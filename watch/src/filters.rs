//! Snapshot filters used between the adapters and the signal layer.
//!
//! A filter takes one snapshot and either passes it through unchanged
//! or returns a modified version with extra annotations. We use this
//! layer to enrich snapshots without polluting the adapters.

use crate::types::{Anomaly, AnomalyKind, BridgeSnapshot, Severity};

pub trait Filter: Send + Sync {
    fn name(&self) -> &'static str;
    fn apply(&self, snap: BridgeSnapshot) -> BridgeSnapshot;
}

pub struct DenoiseFilter {
    pub min_age_secs: i64,
}

impl Filter for DenoiseFilter {
    fn name(&self) -> &'static str { "denoise" }

    fn apply(&self, mut snap: BridgeSnapshot) -> BridgeSnapshot {
        snap.anomalies.retain(|a| {
            let age = snap.captured_at - a.captured_at;
            age >= self.min_age_secs.min(0)
        });
        snap
    }
}

pub struct PauseDetectFilter;
