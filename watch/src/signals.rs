//! Signal extraction.

use crate::types::{Anomaly, AnomalyKind, BridgeConfig, BridgeSnapshot, Severity};

pub trait Signal: Send + Sync {
    fn name(&self) -> &'static str;
    fn evaluate(&self, snap: &BridgeSnapshot, cfg: &BridgeConfig) -> Vec<Anomaly>;
}

pub struct TvlDropSignal {
    pub min_drop_pct: f64,
}

impl Signal for TvlDropSignal {
    fn name(&self) -> &'static str { "tvl-drop" }

    fn evaluate(&self, snap: &BridgeSnapshot, cfg: &BridgeConfig) -> Vec<Anomaly> {
        let mut out = vec![];
        if snap.tvl_delta_24h_pct <= -self.min_drop_pct {
            let severity = if snap.tvl_delta_24h_pct <= -50.0 {
                Severity::Critical
            } else if snap.tvl_delta_24h_pct <= -25.0 {
                Severity::High
            } else {
                Severity::Medium
            };
            out.push(Anomaly {
                kind: AnomalyKind::TvlDrop,
                severity,
                message: format!("tvl down {:.1}% in 24h", snap.tvl_delta_24h_pct.abs()),
                captured_at: snap.captured_at,
                source: "tvl-drop".to_string(),
            });
        }
        if snap.tvl_usd > 0.0 && snap.tvl_usd < cfg.healthy_tvl_floor_usd * 0.75 {
            out.push(Anomaly {
                kind: AnomalyKind::TvlDrop,
                severity: Severity::Medium,
                message: format!("tvl ${:.0} below 75% of floor", snap.tvl_usd),
                captured_at: snap.captured_at,
                source: "tvl-floor".to_string(),
            });
        }
        out
    }
}

pub struct AdminKeySignal {
    pub max_moves_24h: u32,
}
