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

pub fn replay(input: ReplayInput, signals: &SignalSet) -> ReplayOutput {
    let mut assessments = Vec::with_capacity(input.timeline.len());
    let mut max_score = 0u8;
    let mut quarantined_at: Option<usize> = None;
    for (i, snap) in input.timeline.iter().enumerate() {
        let anomalies = signals.evaluate_all(snap, &input.bridge_cfg);
        let assessment = score::compute(input.bridge_cfg.id.clone(), &anomalies);
        if assessment.score > max_score {
            max_score = assessment.score;
        }
        if assessment.score >= 81 && quarantined_at.is_none() {
            quarantined_at = Some(i);
        }
        assessments.push(assessment);
    }
    let final_score = assessments.last().map(|a| a.score).unwrap_or(0);
    ReplayOutput {
        bridge: input.bridge_cfg.id,
        assessments,
        max_score,
        final_score,
        quarantined_at,
    }
}

pub fn into_store(output: &ReplayOutput, store: Arc<Store>) {
    for a in &output.assessments {
        let snap = BridgeSnapshot::empty(output.bridge.clone());
        store.record_sample(snap, a.clone());
    }
}

pub fn synthesize_timeline(
    bridge: BridgeId,
    events: &[(crate::types::AnomalyKind, crate::types::Severity, &str)],
) -> Vec<BridgeSnapshot> {
    let mut out = vec![];
    for (i, (kind, sev, msg)) in events.iter().enumerate() {
        let mut snap = BridgeSnapshot::empty(bridge.clone());
        snap.captured_at = 1_700_000_000 + (i as i64);
        snap.anomalies.push(crate::types::Anomaly {
            kind: *kind,
            severity: *sev,
            message: msg.to_string(),
            captured_at: snap.captured_at,
            source: "synth".to_string(),
        });
        out.push(snap);
    }
    out
}
