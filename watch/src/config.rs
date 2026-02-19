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
