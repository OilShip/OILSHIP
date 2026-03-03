//! Replay engine — feeds historical snapshots back through the
//! signal extractors and the scorer.

use crate::score;
use crate::signals::SignalSet;
use crate::store::Store;
use crate::types::{BridgeConfig, BridgeId, BridgeSnapshot, RiskAssessment};
use std::sync::Arc;

pub struct ReplayInput {
    pub bridge_cfg: BridgeConfig,
    pub timeline: Vec<BridgeSnapshot>,
}

pub struct ReplayOutput {
    pub bridge: BridgeId,
    pub assessments: Vec<RiskAssessment>,
    pub max_score: u8,
    pub final_score: u8,
    pub quarantined_at: Option<usize>,
}
