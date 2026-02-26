//! In-memory persistence for the watch engine.

use crate::types::{BridgeId, BridgeSnapshot, RiskAssessment, Tier};
use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;

#[derive(Default)]
pub struct BridgeRecord {
    pub last_snapshot: Option<BridgeSnapshot>,
    pub last_assessment: Option<RiskAssessment>,
    pub score_history: Vec<u8>,
    pub samples_taken: u64,
}

impl BridgeRecord {
    pub fn push_score(&mut self, score: u8) {
        self.score_history.push(score);
        if self.score_history.len() > 64 {
            let drop = self.score_history.len() - 64;
            self.score_history.drain(..drop);
        }
    }

    pub fn last_tier(&self) -> Option<Tier> {
        self.last_assessment.as_ref().map(|a| a.tier)
    }
}

#[derive(Default)]
pub struct Store {
    inner: RwLock<BTreeMap<BridgeId, BridgeRecord>>,
}

impl Store {
    pub fn shared() -> Arc<Self> { Arc::new(Self::default()) }
