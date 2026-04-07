//! Backfill — pulls historical bridge state from a snapshot file and
//! re-runs the scoring pipeline. Used by the team to regression-test
//! the engine against past incidents.

use crate::score;
use crate::signals::SignalSet;
use crate::types::{BridgeConfig, BridgeId, BridgeSnapshot, RiskAssessment};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackfillFile {
    pub bridge_cfg: BridgeConfig,
    pub timeline: Vec<BridgeSnapshot>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BackfillReport {
    pub bridge: BridgeId,
    pub total_samples: usize,
    pub max_score: u8,
    pub min_score: u8,
    pub final_tier: String,
    pub assessments: Vec<RiskAssessment>,
}
