//! On-chain publisher.

use crate::types::{BridgeId, RiskAssessment};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishOptions {
    pub program_id: String,
    pub operator_pubkey: String,
    pub max_in_flight: u32,
    pub dry_run: bool,
}

impl Default for PublishOptions {
    fn default() -> Self {
        Self {
            program_id: "11111111111111111111111111111111".to_string(),
            operator_pubkey: "11111111111111111111111111111111".to_string(),
            max_in_flight: 4,
            dry_run: true,
        }
    }
}

pub struct Publisher {
    options: PublishOptions,
    in_flight: Arc<Mutex<u32>>,
}
