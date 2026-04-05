//! Admin-action audit log.
//!
//! Whenever the watch engine sees an admin action on a bridge contract
//! it appends a record to a tamper-evident log file. The log is kept
//! append-only; rotation is the operator's job.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAction {
    pub bridge: String,
    pub action: String,
    pub actor: String,
    pub tx_signature: String,
    pub captured_at: i64,
    pub severity: AdminSeverity,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminSeverity {
    Routine,
    Notable,
    Critical,
}

impl AdminSeverity {
    pub fn from_action(action: &str) -> Self {
        match action {
            "key_rotation" | "upgrade" | "pause" | "set_authority" => Self::Critical,
            "fee_change" | "param_change" => Self::Notable,
            _ => Self::Routine,
        }
    }
}

pub struct AdminAuditLog {
    pub path: PathBuf,
}
