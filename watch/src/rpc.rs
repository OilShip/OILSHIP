//! Multi-chain RPC client used by the OILSHIP watch engine.

use crate::types::Chain;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcRequest<'a> {
    pub jsonrpc: &'a str,
    pub id: u64,
    pub method: &'a str,
    pub params: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcResponse {
    #[allow(dead_code)]
    pub jsonrpc: String,
    #[allow(dead_code)]
    pub id: u64,
    pub result: Option<Value>,
    pub error: Option<RpcError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Clone)]
pub struct RpcClient {
    http: reqwest::Client,
    endpoints: BTreeMap<Chain, String>,
}

impl RpcClient {
    pub fn new(endpoints: BTreeMap<Chain, String>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(15))
            .user_agent("oilship-watch/0.1")
            .build()
            .expect("reqwest client");
        Self { http, endpoints }
    }

    pub fn endpoint(&self, chain: Chain) -> Result<&str> {
        self.endpoints.get(&chain).map(|s| s.as_str()).ok_or_else(|| anyhow::anyhow!("no endpoint configured for {:?}", chain))
    }
