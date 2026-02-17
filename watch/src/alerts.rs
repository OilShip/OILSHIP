//! Alerting sinks.

use crate::types::{Anomaly, RiskAssessment, Tier};
use anyhow::Result;
use serde::Serialize;
use std::sync::Arc;

pub trait AlertSink: Send + Sync {
    fn name(&self) -> &'static str;
    fn handle(&self, alert: &Alert) -> Result<()>;
}

#[derive(Debug, Clone, Serialize)]
pub struct Alert {
    pub kind: AlertKind,
    pub assessment: RiskAssessment,
    pub anomalies: Vec<Anomaly>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AlertKind {
    NewSample,
    TierDowngrade,
    Quarantined,
    Recovered,
}

impl Alert {
    pub fn classify(prev_tier: Option<Tier>, assessment: RiskAssessment) -> Option<Self> {
        let kind = match (prev_tier, assessment.tier) {
            (None, _) => AlertKind::NewSample,
            (Some(prev), now) if prev == now => AlertKind::NewSample,
            (Some(prev), now) if (now as u8) > (prev as u8) => {
                if now == Tier::Quarantined { AlertKind::Quarantined } else { AlertKind::TierDowngrade }
            }
            (Some(_), _) => AlertKind::Recovered,
        };
        Some(Self { kind, assessment, anomalies: vec![] })
    }
}
