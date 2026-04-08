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

    pub async fn run(self) -> Result<()> {
        let listener = TcpListener::bind(&self.bind).await?;
        tracing::info!(target: "oilship.http", "listening on {}", self.bind);
        loop {
            let (socket, _peer) = listener.accept().await?;
            let store = self.store.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_one(socket, store).await {
                    tracing::warn!("http handler error: {}", e);
                }
            });
        }
    }
}

async fn handle_one(socket: TcpStream, store: Arc<Store>) -> Result<()> {
    let (read, mut write) = socket.into_split();
    let mut reader = BufReader::new(read);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).await?;
    let path = parse_path(&request_line);

    let body = match path.as_str() {
        "/healthz" => "ok".to_string(),
        "/status" => {
            let bridges = store.all_bridges().len();
            let samples = store.samples_total();
            let body = StatusBody { bridges, samples_total: samples };
            serde_json::to_string(&body)?
        }
        "/assessments" => {
            let mut rows: Vec<AssessmentRow> = vec![];
            for id in store.all_bridges() {
                if let Some(a) = store.assessment_for(&id) {
                    rows.push(row_from(&a));
                }
            }
            serde_json::to_string(&rows)?
        }
        _ => "not found".to_string(),
    };

    let status = if path == "/healthz" || path.starts_with("/status") || path == "/assessments" {
        "200 OK"
    } else {
        "404 Not Found"
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    write.write_all(response.as_bytes()).await?;
    write.shutdown().await?;
    Ok(())
}

fn parse_path(request_line: &str) -> String {
    let mut parts = request_line.split_whitespace();
    let _method = parts.next();
    let path = parts.next().unwrap_or("/");
    path.split('?').next().unwrap_or("/").to_string()
}

fn row_from(a: &RiskAssessment) -> AssessmentRow {
    AssessmentRow {
        bridge: a.bridge.to_string(),
        score: a.score,
        tier: format!("{:?}", a.tier),
        computed_at: a.computed_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get_root() {
        assert_eq!(parse_path("GET / HTTP/1.1\r\n"), "/");
    }

    #[test]
    fn parses_query_strings() {
        assert_eq!(parse_path("GET /status?fmt=json HTTP/1.1\r\n"), "/status");
    }

    #[test]
    fn parses_assessments() {
        assert_eq!(parse_path("GET /assessments HTTP/1.1\r\n"), "/assessments");
    }
}
