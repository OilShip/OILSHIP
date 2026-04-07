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

pub fn load(path: impl AsRef<Path>) -> Result<BackfillFile> {
    let path = path.as_ref();
    let raw = std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let parsed: BackfillFile = serde_json::from_str(&raw).with_context(|| format!("parsing {}", path.display()))?;
    Ok(parsed)
}

pub fn run(file: &BackfillFile, signals: &SignalSet) -> BackfillReport {
    let mut assessments = vec![];
    let mut max_score = 0u8;
    let mut min_score = 100u8;
    for snap in &file.timeline {
        let anomalies = signals.evaluate_all(snap, &file.bridge_cfg);
        let assessment = score::compute(file.bridge_cfg.id.clone(), &anomalies);
        if assessment.score > max_score { max_score = assessment.score; }
        if assessment.score < min_score { min_score = assessment.score; }
        assessments.push(assessment);
    }
    let final_tier = assessments.last().map(|a| format!("{:?}", a.tier)).unwrap_or_else(|| "Unknown".into());
    BackfillReport {
        bridge: file.bridge_cfg.id.clone(),
        total_samples: file.timeline.len(),
        max_score,
        min_score: if file.timeline.is_empty() { 0 } else { min_score },
        final_tier,
        assessments,
    }
}

pub fn save_report(report: &BackfillReport, path: impl AsRef<Path>) -> Result<()> {
    let s = serde_json::to_string_pretty(report)?;
    std::fs::write(path, s)?;
    Ok(())
}

pub fn summarise(report: &BackfillReport) -> String {
    format!(
        "bridge={} samples={} score=[{}..{}] final_tier={}",
        report.bridge, report.total_samples, report.min_score, report.max_score, report.final_tier,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_backfill_summary() {
        let f = BackfillFile {
            bridge_cfg: BridgeConfig::placeholder("x", "X"),
            timeline: vec![],
        };
        let report = run(&f, &SignalSet::standard());
        assert_eq!(report.total_samples, 0);
        assert_eq!(report.max_score, 0);
    }
}
