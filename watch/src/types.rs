//! Shared types for the OILSHIP monitoring engine.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Solana,
    Ethereum,
    Arbitrum,
    Optimism,
    Base,
    Polygon,
    Avalanche,
    Bsc,
}

impl Chain {
    pub fn slug(&self) -> &'static str {
        match self {
            Chain::Solana => "solana",
            Chain::Ethereum => "ethereum",
            Chain::Arbitrum => "arbitrum",
            Chain::Optimism => "optimism",
            Chain::Base => "base",
            Chain::Polygon => "polygon",
            Chain::Avalanche => "avalanche",
            Chain::Bsc => "bsc",
        }
    }

    pub fn is_evm(&self) -> bool {
        !matches!(self, Chain::Solana)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BridgeId(pub String);

impl BridgeId {
    pub fn new(s: impl Into<String>) -> Self { Self(s.into()) }
    pub fn as_str(&self) -> &str { &self.0 }
}

impl std::fmt::Display for BridgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSnapshot {
    pub bridge: BridgeId,
    pub captured_at: i64,
    pub tvl_usd: f64,
    pub tvl_delta_24h_pct: f64,
    pub signers: u32,
    pub admin_key_recent_moves: u32,
    pub oracle_drift_bps: u32,
    pub anomalies: Vec<Anomaly>,
    pub raw: BTreeMap<String, serde_json::Value>,
}

impl BridgeSnapshot {
    pub fn empty(bridge: BridgeId) -> Self {
        Self {
            bridge,
            captured_at: 0,
            tvl_usd: 0.0,
            tvl_delta_24h_pct: 0.0,
            signers: 0,
            admin_key_recent_moves: 0,
            oracle_drift_bps: 0,
            anomalies: vec![],
            raw: BTreeMap::new(),
        }
    }

    pub fn merge(&mut self, other: BridgeSnapshot) {
        if other.captured_at > self.captured_at {
            self.captured_at = other.captured_at;
            self.tvl_usd = other.tvl_usd;
            self.tvl_delta_24h_pct = other.tvl_delta_24h_pct;
            self.signers = other.signers;
            self.admin_key_recent_moves = other.admin_key_recent_moves;
            self.oracle_drift_bps = other.oracle_drift_bps;
        }
        for a in other.anomalies {
            self.anomalies.push(a);
        }
        for (k, v) in other.raw {
            self.raw.insert(k, v);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub kind: AnomalyKind,
    pub severity: Severity,
    pub message: String,
    pub captured_at: i64,
    pub source: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyKind {
    TvlDrop,
    AdminKeyRotation,
    SignerCollusion,
    OracleDrift,
    UnusualWithdrawal,
    PauseFlagSet,
    ContractUpgrade,
    GuardianOffline,
    PoolImbalance,
    SuspiciousMemo,
}
