//! Lightweight indexer that materialises engine output into a JSONL log.

use crate::store::Store;
use crate::types::{BridgeId, RiskAssessment};
use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct IndexedRow {
    pub bridge: String,
    pub score: u8,
    pub tier: String,
    pub computed_at: i64,
}

pub struct Indexer {
    pub root: PathBuf,
    pub store: Arc<Store>,
    last_seen: BTreeMap<BridgeId, i64>,
}

impl Indexer {
    pub fn new(root: impl Into<PathBuf>, store: Arc<Store>) -> Self {
        Self { root: root.into(), store, last_seen: BTreeMap::new() }
    }

    pub async fn run(mut self, interval: Duration) -> Result<()> {
        create_dir_all(&self.root)?;
        loop {
            self.tick()?;
            tokio::time::sleep(interval).await;
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let bridges = self.store.all_bridges();
        for id in bridges {
            let assessment = match self.store.assessment_for(&id) {
                Some(a) => a,
                None => continue,
            };
            let last = self.last_seen.get(&id).copied().unwrap_or(0);
            if assessment.computed_at <= last { continue; }
            self.write_row(&assessment)?;
            self.last_seen.insert(id, assessment.computed_at);
        }
        Ok(())
    }

    fn write_row(&self, a: &RiskAssessment) -> Result<()> {
        let row = IndexedRow {
            bridge: a.bridge.to_string(),
            score: a.score,
            tier: format!("{:?}", a.tier),
            computed_at: a.computed_at,
        };
        let day = chrono::DateTime::<chrono::Utc>::from_timestamp(a.computed_at, 0)
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());
        let path = self.root.join(format!("oilship-{day}.jsonl"));
        let mut f = OpenOptions::new().append(true).create(true).open(&path)?;
        let line = serde_json::to_string(&row)?;
        writeln!(f, "{line}")?;
        Ok(())
    }
}
