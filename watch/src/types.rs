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

impl AnomalyKind {
    pub fn weight(&self) -> u32 {
        match self {
            Self::TvlDrop => 25,
            Self::AdminKeyRotation => 30,
            Self::SignerCollusion => 35,
            Self::OracleDrift => 12,
            Self::UnusualWithdrawal => 18,
            Self::PauseFlagSet => 8,
            Self::ContractUpgrade => 22,
            Self::GuardianOffline => 14,
            Self::PoolImbalance => 10,
            Self::SuspiciousMemo => 6,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn factor(&self) -> f64 {
        match self {
            Self::Info => 0.25,
            Self::Low => 0.5,
            Self::Medium => 1.0,
            Self::High => 1.6,
            Self::Critical => 2.4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub bridge: BridgeId,
    pub score: u8,
    pub tier: Tier,
    pub computed_at: i64,
    pub factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub contribution: u32,
    pub note: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    Tier1,
    Tier2,
    Tier3,
    Quarantined,
}

impl Tier {
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=30 => Tier::Tier1,
            31..=55 => Tier::Tier2,
            56..=80 => Tier::Tier3,
            _ => Tier::Quarantined,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Tier::Tier1 => "TIER 1",
            Tier::Tier2 => "TIER 2",
            Tier::Tier3 => "TIER 3",
            Tier::Quarantined => "QUARANTINED",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub id: BridgeId,
    pub display_name: String,
    pub solana_program: Option<String>,
    pub evm_contracts: BTreeMap<Chain, String>,
    pub guardian_endpoint: Option<String>,
    pub min_signers: u32,
    pub healthy_tvl_floor_usd: f64,
}

impl BridgeConfig {
    pub fn placeholder(id: &str, display_name: &str) -> Self {
        Self {
            id: BridgeId::new(id),
            display_name: display_name.to_string(),
            solana_program: None,
            evm_contracts: BTreeMap::new(),
            guardian_endpoint: None,
            min_signers: 5,
            healthy_tvl_floor_usd: 1_000_000.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    SampleAll,
    SampleOne(BridgeId),
    Quarantine(BridgeId),
    LiftQuarantine(BridgeId),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EngineStatus {
    pub uptime_secs: u64,
    pub bridges_watched: u32,
    pub anomalies_seen: u64,
    pub samples_taken: u64,
    pub last_sample_at: i64,
}
