//! Configuration loader for the OILSHIP watch engine.

use crate::types::{BridgeConfig, BridgeId, Chain};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub solana_rpc: String,
    pub eth_rpc: String,
    pub arb_rpc: String,
    pub op_rpc: String,
    pub base_rpc: String,
    pub poll_interval_secs: u64,
    pub jitter_secs: u64,
    pub publish_to_chain: bool,
    pub program_id: String,
    pub operator_keypair: String,
    pub alert_webhook: Option<String>,
    pub log_level: String,
    pub bridges: Vec<BridgeConfig>,
}

impl EngineConfig {
    pub fn rpc_for(&self, chain: Chain) -> &str {
        match chain {
            Chain::Solana => &self.solana_rpc,
            Chain::Ethereum => &self.eth_rpc,
            Chain::Arbitrum => &self.arb_rpc,
            Chain::Optimism => &self.op_rpc,
            Chain::Base => &self.base_rpc,
            _ => &self.eth_rpc,
        }
    }

    pub fn defaults() -> Self {
        let mut bridges = vec![];
        let mut mayan = BridgeConfig::placeholder("mayan", "Mayan Finance");
        mayan.evm_contracts.insert(Chain::Ethereum, "0x0000000000000000000000000000000000000000".to_string());
        mayan.evm_contracts.insert(Chain::Base, "0x0000000000000000000000000000000000000000".to_string());
        mayan.solana_program = Some("FC4eXxkyrMPTjiYUpp7EAnkmwMbQyZ6NDChy7vbBdz4S".to_string());
        mayan.healthy_tvl_floor_usd = 5_000_000.0;
        bridges.push(mayan);

        let mut debridge = BridgeConfig::placeholder("debridge", "deBridge");
        debridge.solana_program = Some("DEbrdGj3HsRsAzx6uH4MKyREKxVAfBydijLUF3ygsFfh".to_string());
        debridge.healthy_tvl_floor_usd = 4_000_000.0;
        bridges.push(debridge);

        let mut wormhole = BridgeConfig::placeholder("wormhole", "Wormhole Portal");
        wormhole.solana_program = Some("worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth".to_string());
        wormhole.healthy_tvl_floor_usd = 20_000_000.0;
        wormhole.min_signers = 13;
        bridges.push(wormhole);

        let mut allbridge = BridgeConfig::placeholder("allbridge", "Allbridge Core");
        allbridge.healthy_tvl_floor_usd = 2_000_000.0;
        bridges.push(allbridge);

        Self {
            solana_rpc: "https://api.mainnet-beta.solana.com".to_string(),
            eth_rpc: "https://eth.llamarpc.com".to_string(),
            arb_rpc: "https://arb1.arbitrum.io/rpc".to_string(),
            op_rpc: "https://mainnet.optimism.io".to_string(),
            base_rpc: "https://mainnet.base.org".to_string(),
            poll_interval_secs: 30,
            jitter_secs: 5,
            publish_to_chain: false,
            program_id: "11111111111111111111111111111111".to_string(),
            operator_keypair: "~/.config/solana/id.json".to_string(),
            alert_webhook: None,
            log_level: "info".to_string(),
            bridges,
        }
    }

    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path).with_context(|| format!("reading config at {}", path.display()))?;
        let parsed: Self = serde_json::from_str(&raw).with_context(|| format!("parsing config at {}", path.display()))?;
        parsed.validate()?;
        Ok(parsed)
    }

    pub fn validate(&self) -> Result<()> {
        if self.poll_interval_secs == 0 {
            anyhow::bail!("poll_interval_secs must be > 0");
        }
        if self.bridges.is_empty() {
            anyhow::bail!("at least one bridge must be configured");
        }
        for b in &self.bridges {
            if b.id.as_str().is_empty() {
                anyhow::bail!("bridge id cannot be empty");
            }
        }
        Ok(())
    }
