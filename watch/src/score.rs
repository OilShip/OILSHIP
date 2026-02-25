//! Risk score computation.

use crate::types::{Anomaly, BridgeId, RiskAssessment, RiskFactor, Tier};
use std::time::{SystemTime, UNIX_EPOCH};

const BASELINE_SCORE: u32 = 18;

pub fn compute(bridge: BridgeId, anomalies: &[Anomaly]) -> RiskAssessment {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    let mut total: u32 = BASELINE_SCORE;
    let mut factors: Vec<RiskFactor> = vec![
        RiskFactor {
            name: "baseline".to_string(),
            contribution: BASELINE_SCORE,
            note: "every bridge starts at 18".to_string(),
        }
    ];
    let mut category_caps: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
    for a in anomalies {
        let key = format!("{:?}", a.kind);
        let raw = (a.kind.weight() as f64 * a.severity.factor()).round() as u32;
        let cap = category_caps.entry(key.clone()).or_insert(0);
        let allowed = (40u32).saturating_sub(*cap);
        if allowed == 0 { continue; }
        let used = raw.min(allowed);
        *cap += used;
        total = total.saturating_add(used);
        factors.push(RiskFactor {
            name: key,
            contribution: used,
            note: a.message.clone(),
        });
    }
    let score = total.min(100) as u8;
    let tier = Tier::from_score(score);
    RiskAssessment { bridge, score, tier, computed_at: now, factors }
}

pub fn ewma_smooth(history: &[u8], alpha: f64) -> u8 {
    if history.is_empty() { return 0; }
    let mut acc = history[0] as f64;
    for s in &history[1..] {
        acc = alpha * (*s as f64) + (1.0 - alpha) * acc;
    }
    acc.round().clamp(0.0, 100.0) as u8
}

pub fn label(score: u8) -> &'static str {
    match score {
        0..=20 => "calm seas",
        21..=40 => "watch",
        41..=60 => "elevated",
        61..=80 => "rough",
        _ => "do not sail",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AnomalyKind, Severity};
