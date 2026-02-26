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

    pub fn record_sample(&self, snap: BridgeSnapshot, assessment: RiskAssessment) {
        let mut g = self.inner.write();
        let entry = g.entry(snap.bridge.clone()).or_default();
        entry.last_snapshot = Some(snap);
        entry.push_score(assessment.score);
        entry.last_assessment = Some(assessment);
        entry.samples_taken += 1;
    }

    pub fn snapshot_for(&self, id: &BridgeId) -> Option<BridgeSnapshot> {
        self.inner.read().get(id).and_then(|e| e.last_snapshot.clone())
    }

    pub fn assessment_for(&self, id: &BridgeId) -> Option<RiskAssessment> {
        self.inner.read().get(id).and_then(|e| e.last_assessment.clone())
    }

    pub fn last_tier(&self, id: &BridgeId) -> Option<Tier> {
        self.inner.read().get(id).and_then(|e| e.last_tier())
    }

    pub fn all_bridges(&self) -> Vec<BridgeId> {
        self.inner.read().keys().cloned().collect()
    }

    pub fn samples_total(&self) -> u64 {
        self.inner.read().values().map(|e| e.samples_taken).sum()
    }
}
