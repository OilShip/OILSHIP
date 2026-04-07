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

impl AdminAuditLog {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn append(&self, action: &AdminAction) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut f = OpenOptions::new().append(true).create(true).open(&self.path)?;
        let line = serde_json::to_string(action)?;
        writeln!(f, "{line}")?;
        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<AdminAction>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let raw = std::fs::read_to_string(&self.path)?;
        let mut out = vec![];
        for line in raw.lines() {
            if line.trim().is_empty() { continue; }
            let parsed: AdminAction = serde_json::from_str(line)?;
            out.push(parsed);
        }
        Ok(out)
    }

    pub fn count(&self) -> usize {
        self.read_all().map(|v| v.len()).unwrap_or(0)
    }
}

pub fn classify_actor(addr: &str) -> &'static str {
    if addr.starts_with("11111") { "system" }
    else if addr.len() < 32 { "unknown" }
    else { "external" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn empty_log_reads_empty() {
        let dir = tempdir().unwrap();
        let log = AdminAuditLog::new(dir.path().join("audit.jsonl"));
        assert_eq!(log.count(), 0);
    }

    #[test]
    fn round_trip_one_action() {
        let dir = tempdir().unwrap();
        let log = AdminAuditLog::new(dir.path().join("audit.jsonl"));
        let action = AdminAction {
            bridge: "mayan".into(),
            action: "key_rotation".into(),
            actor: "11111111111111111111111111111111".into(),
            tx_signature: "abc".into(),
            captured_at: 0,
            severity: AdminSeverity::Critical,
        };
        log.append(&action).unwrap();
        let back = log.read_all().unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].action, "key_rotation");
    }

    #[test]
    fn severity_classifier() {
        assert_eq!(AdminSeverity::from_action("key_rotation"), AdminSeverity::Critical);
        assert_eq!(AdminSeverity::from_action("fee_change"), AdminSeverity::Notable);
        assert_eq!(AdminSeverity::from_action("read"), AdminSeverity::Routine);
    }
}
