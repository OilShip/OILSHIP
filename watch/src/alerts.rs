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

pub struct LogSink;

impl AlertSink for LogSink {
    fn name(&self) -> &'static str { "log" }

    fn handle(&self, alert: &Alert) -> Result<()> {
        tracing::info!(
            target: "oilship.alert",
            bridge = %alert.assessment.bridge,
            score = alert.assessment.score,
            tier = ?alert.assessment.tier,
            kind = ?alert.kind,
            "alert"
        );
        Ok(())
    }
}

pub struct WebhookSink {
    pub url: String,
    pub http: reqwest::Client,
}

impl WebhookSink {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into(), http: reqwest::Client::new() }
    }
}

impl AlertSink for WebhookSink {
    fn name(&self) -> &'static str { "webhook" }

    fn handle(&self, alert: &Alert) -> Result<()> {
        let url = self.url.clone();
        let body = serde_json::to_string(alert)?;
        let http = self.http.clone();
        tokio::spawn(async move {
            match http.post(&url).body(body).send().await {
                Ok(resp) if resp.status().is_success() => {}
                Ok(resp) => tracing::warn!("webhook non-success: {}", resp.status()),
                Err(e) => tracing::warn!("webhook error: {}", e),
            }
        });
        Ok(())
    }
}

pub struct SinkSet(pub Vec<Arc<dyn AlertSink>>);

impl SinkSet {
    pub fn empty() -> Self { Self(vec![]) }
    pub fn add(&mut self, sink: Arc<dyn AlertSink>) { self.0.push(sink); }

    pub fn dispatch(&self, alert: &Alert) {
        for sink in &self.0 {
            if let Err(e) = sink.handle(alert) {
                tracing::warn!("sink {} failed: {}", sink.name(), e);
            }
        }
    }
}
