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
