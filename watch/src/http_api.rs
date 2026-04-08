//! Tiny HTTP API exposed by the watch engine.
//!
//! Bound to `127.0.0.1:7878` by default. The endpoints are read-only
//! and intentionally narrow — they exist so an external dashboard or
//! Discord bot can ask the engine for current state without coupling
//! to its internals.

use crate::store::Store;
use crate::types::{BridgeId, RiskAssessment};
use anyhow::Result;
use serde::Serialize;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, Serialize)]
pub struct StatusBody {
    pub bridges: usize,
    pub samples_total: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssessmentRow {
    pub bridge: String,
    pub score: u8,
    pub tier: String,
    pub computed_at: i64,
}

pub struct HttpApi {
    pub bind: String,
    pub store: Arc<Store>,
}

impl HttpApi {
    pub fn new(bind: impl Into<String>, store: Arc<Store>) -> Self {
        Self { bind: bind.into(), store }
    }
