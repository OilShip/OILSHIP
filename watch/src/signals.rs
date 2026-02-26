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

impl Signal for AdminKeySignal {
    fn name(&self) -> &'static str { "admin-key" }

    fn evaluate(&self, snap: &BridgeSnapshot, _cfg: &BridgeConfig) -> Vec<Anomaly> {
        let mut out = vec![];
        if snap.admin_key_recent_moves > self.max_moves_24h {
            let sev = if snap.admin_key_recent_moves > 5 { Severity::Critical } else { Severity::High };
            out.push(Anomaly {
                kind: AnomalyKind::AdminKeyRotation,
                severity: sev,
                message: format!("admin key moved {} times in 24h", snap.admin_key_recent_moves),
                captured_at: snap.captured_at,
                source: "admin-key".to_string(),
            });
        }
        out
    }
}

pub struct SignerSignal;

impl Signal for SignerSignal {
    fn name(&self) -> &'static str { "signer" }

    fn evaluate(&self, snap: &BridgeSnapshot, cfg: &BridgeConfig) -> Vec<Anomaly> {
        let mut out = vec![];
        if snap.signers > 0 && snap.signers < cfg.min_signers {
            let missing = cfg.min_signers - snap.signers;
            let sev = if missing >= 3 { Severity::High } else { Severity::Medium };
            out.push(Anomaly {
                kind: AnomalyKind::SignerCollusion,
                severity: sev,
                message: format!("{} signers active, expected {}", snap.signers, cfg.min_signers),
                captured_at: snap.captured_at,
                source: "signer".to_string(),
            });
        }
        out
    }
}

pub struct OracleDriftSignal {
    pub max_drift_bps: u32,
}

impl Signal for OracleDriftSignal {
    fn name(&self) -> &'static str { "oracle-drift" }

    fn evaluate(&self, snap: &BridgeSnapshot, _cfg: &BridgeConfig) -> Vec<Anomaly> {
        let mut out = vec![];
        if snap.oracle_drift_bps > self.max_drift_bps {
            let bps = snap.oracle_drift_bps;
            let sev = if bps > 500 { Severity::Critical }
                else if bps > 200 { Severity::High }
                else { Severity::Medium };
            out.push(Anomaly {
                kind: AnomalyKind::OracleDrift,
                severity: sev,
                message: format!("oracle drift {} bps", bps),
                captured_at: snap.captured_at,
                source: "oracle-drift".to_string(),
            });
        }
        out
    }
}

pub struct SignalSet {
    pub signals: Vec<Box<dyn Signal>>,
}

impl SignalSet {
    pub fn standard() -> Self {
        Self {
            signals: vec![
                Box::new(TvlDropSignal { min_drop_pct: 10.0 }),
                Box::new(AdminKeySignal { max_moves_24h: 1 }),
                Box::new(SignerSignal),
                Box::new(OracleDriftSignal { max_drift_bps: 100 }),
            ],
        }
    }

    pub fn evaluate_all(&self, snap: &BridgeSnapshot, cfg: &BridgeConfig) -> Vec<Anomaly> {
        let mut out = vec![];
        for s in &self.signals {
            out.extend(s.evaluate(snap, cfg));
        }
        out.extend(snap.anomalies.iter().cloned());
        out
    }
}
