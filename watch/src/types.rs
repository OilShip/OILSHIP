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
