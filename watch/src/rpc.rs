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

    pub async fn call(&self, chain: Chain, method: &str, params: Value) -> Result<Value> {
        let url = self.endpoint(chain)?;
        let req = JsonRpcRequest { jsonrpc: "2.0", id: 1, method, params };
        let res = self.http.post(url).json(&req).send().await.with_context(|| format!("rpc {} on {:?}", method, chain))?;
        let body: JsonRpcResponse = res.json().await.context("decoding rpc body")?;
        if let Some(err) = body.error {
            anyhow::bail!("rpc error {} {}: {}", chain.slug(), err.code, err.message);
        }
        body.result.ok_or_else(|| anyhow::anyhow!("rpc {} returned empty result", method))
    }

    pub async fn solana_get_slot(&self) -> Result<u64> {
        let v = self.call(Chain::Solana, "getSlot", json!([])).await?;
        Ok(v.as_u64().unwrap_or(0))
    }

    pub async fn solana_get_balance(&self, pubkey: &str) -> Result<u64> {
        let v = self.call(Chain::Solana, "getBalance", json!([pubkey])).await?;
        Ok(v.get("value").and_then(|x| x.as_u64()).unwrap_or(0))
    }

    pub async fn solana_get_account_info(&self, pubkey: &str) -> Result<Option<Value>> {
        let v = self.call(Chain::Solana, "getAccountInfo", json!([pubkey, { "encoding": "base64" }])).await?;
        Ok(v.get("value").cloned())
    }

    pub async fn solana_get_signatures_for_address(&self, pubkey: &str, limit: u32) -> Result<Vec<Value>> {
        let v = self.call(Chain::Solana, "getSignaturesForAddress", json!([pubkey, { "limit": limit }])).await?;
        Ok(v.as_array().cloned().unwrap_or_default())
    }

    pub async fn eth_block_number(&self, chain: Chain) -> Result<u64> {
        let v = self.call(chain, "eth_blockNumber", json!([])).await?;
        let s = v.as_str().unwrap_or("0x0");
        Ok(u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0))
    }

    pub async fn eth_get_balance(&self, chain: Chain, address: &str) -> Result<u128> {
        let v = self.call(chain, "eth_getBalance", json!([address, "latest"])).await?;
        let s = v.as_str().unwrap_or("0x0");
        Ok(u128::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0))
    }

    pub async fn eth_get_logs(&self, chain: Chain, from_block: u64, to_block: u64, address: &str) -> Result<Vec<Value>> {
        let v = self.call(chain, "eth_getLogs", json!([{
            "fromBlock": format!("0x{:x}", from_block),
            "toBlock": format!("0x{:x}", to_block),
            "address": address,
        }])).await?;
        Ok(v.as_array().cloned().unwrap_or_default())
    }

    pub async fn eth_call(&self, chain: Chain, to: &str, data: &str) -> Result<String> {
        let v = self.call(chain, "eth_call", json!([{ "to": to, "data": data }, "latest"])).await?;
        Ok(v.as_str().unwrap_or("0x").to_string())
    }
}

pub fn wei_to_usd(wei: u128, price_usd: f64) -> f64 {
    let eth = wei as f64 / 1e18;
    eth * price_usd
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1e9
}

pub fn delta_pct(now: f64, then: f64) -> f64 {
    if then <= 0.0 { return 0.0; }
    ((now - then) / then) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_basic() {
        assert_eq!(delta_pct(110.0, 100.0), 10.0);
        assert_eq!(delta_pct(90.0, 100.0), -10.0);
        assert_eq!(delta_pct(100.0, 0.0), 0.0);
    }

    #[test]
    fn lamports_round_trip() {
        assert_eq!(lamports_to_sol(1_000_000_000), 1.0);
    }
}
