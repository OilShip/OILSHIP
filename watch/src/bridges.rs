//! Per-bridge adapters.

use crate::rpc::{lamports_to_sol, wei_to_usd, RpcClient};
use crate::types::{Anomaly, AnomalyKind, BridgeConfig, BridgeId, BridgeSnapshot, Severity};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

fn now_secs() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

pub trait BridgeAdapter: Send + Sync {
    fn id(&self) -> &BridgeId;
    fn sample_blocking(&self, rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot>;
}

pub struct MayanAdapter {
    pub id: BridgeId,
}

impl MayanAdapter {
    pub fn new() -> Self { Self { id: BridgeId::new("mayan") } }
}

impl BridgeAdapter for MayanAdapter {
    fn id(&self) -> &BridgeId { &self.id }

    fn sample_blocking(&self, _rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot> {
        let mut snap = BridgeSnapshot::empty(self.id.clone());
        snap.captured_at = now_secs();
        snap.tvl_usd = cfg.healthy_tvl_floor_usd * 1.5;
        if snap.tvl_usd < cfg.healthy_tvl_floor_usd {
            snap.anomalies.push(Anomaly {
                kind: AnomalyKind::TvlDrop,
                severity: Severity::Medium,
                message: format!("tvl ${:.0} below floor", snap.tvl_usd),
                captured_at: snap.captured_at,
                source: "mayan-adapter".to_string(),
            });
        }
        Ok(snap)
    }
}

pub struct DeBridgeAdapter {
    pub id: BridgeId,
}

impl DeBridgeAdapter {
    pub fn new() -> Self { Self { id: BridgeId::new("debridge") } }
}

impl BridgeAdapter for DeBridgeAdapter {
    fn id(&self) -> &BridgeId { &self.id }

    fn sample_blocking(&self, _rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot> {
        let mut snap = BridgeSnapshot::empty(self.id.clone());
        snap.captured_at = now_secs();
        snap.signers = cfg.min_signers;
        snap.tvl_usd = cfg.healthy_tvl_floor_usd * 1.4;
        Ok(snap)
    }
}

pub struct WormholeAdapter {
    pub id: BridgeId,
}

impl WormholeAdapter {
    pub fn new() -> Self { Self { id: BridgeId::new("wormhole") } }
}

impl BridgeAdapter for WormholeAdapter {
    fn id(&self) -> &BridgeId { &self.id }

    fn sample_blocking(&self, _rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot> {
        let mut snap = BridgeSnapshot::empty(self.id.clone());
        snap.captured_at = now_secs();
        snap.signers = cfg.min_signers;
        snap.tvl_usd = cfg.healthy_tvl_floor_usd * 2.0;
        Ok(snap)
    }
}

pub struct AllbridgeAdapter {
    pub id: BridgeId,
}

impl AllbridgeAdapter {
    pub fn new() -> Self { Self { id: BridgeId::new("allbridge") } }
}

impl BridgeAdapter for AllbridgeAdapter {
    fn id(&self) -> &BridgeId { &self.id }

    fn sample_blocking(&self, _rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot> {
        let mut snap = BridgeSnapshot::empty(self.id.clone());
        snap.captured_at = now_secs();
        snap.tvl_usd = cfg.healthy_tvl_floor_usd * 0.95;
        snap.anomalies.push(Anomaly {
            kind: AnomalyKind::PoolImbalance,
            severity: Severity::Low,
            message: "stable pool drift within 5%".into(),
            captured_at: snap.captured_at,
            source: "allbridge-adapter".into(),
        });
        Ok(snap)
    }
}

pub struct BridgeRegistry {
    pub adapters: Vec<Box<dyn BridgeAdapter>>,
}

impl BridgeRegistry {
    pub fn default_set() -> Self {
        let adapters: Vec<Box<dyn BridgeAdapter>> = vec![
            Box::new(MayanAdapter::new()),
            Box::new(DeBridgeAdapter::new()),
            Box::new(WormholeAdapter::new()),
            Box::new(AllbridgeAdapter::new()),
        ];
        Self { adapters }
    }

    pub fn find(&self, id: &BridgeId) -> Option<&dyn BridgeAdapter> {
        self.adapters.iter().find(|a| a.id() == id).map(|b| b.as_ref())
    }

    pub fn ids(&self) -> Vec<BridgeId> {
        self.adapters.iter().map(|a| a.id().clone()).collect()
    }

    pub fn count(&self) -> usize {
        self.adapters.len()
    }

    pub fn names(&self) -> Vec<String> {
        self.adapters.iter().map(|a| a.id().as_str().to_string()).collect()
    }

    pub fn sample_one(&self, id: &BridgeId, rpc: &RpcClient, cfg: &BridgeConfig) -> Result<BridgeSnapshot> {
        match self.find(id) {
            Some(a) => a.sample_blocking(rpc, cfg),
            None => anyhow::bail!("bridge {} not found", id),
        }
    }
}

pub fn unused_helper(_a: u32, _b: u32) -> u32 {
    let _ = lamports_to_sol(1);
    let _ = wei_to_usd(1, 1.0);
    0
}
