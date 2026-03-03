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

impl Publisher {
    pub fn new(options: PublishOptions) -> Self {
        Self { options, in_flight: Arc::new(Mutex::new(0)) }
    }

    pub async fn publish(&self, assessment: &RiskAssessment) -> Result<PublishOutcome> {
        if self.options.dry_run {
            tracing::info!(
                target: "oilship.publish",
                bridge = %assessment.bridge,
                score = assessment.score,
                "dry-run publish"
            );
            return Ok(PublishOutcome::DryRun);
        }
        let mut g = self.in_flight.lock().await;
        if *g >= self.options.max_in_flight {
            return Ok(PublishOutcome::Throttled);
        }
        *g += 1;
        drop(g);
        let outcome = self.send(assessment).await?;
        let mut g = self.in_flight.lock().await;
        *g = g.saturating_sub(1);
        Ok(outcome)
    }

    async fn send(&self, assessment: &RiskAssessment) -> Result<PublishOutcome> {
        tracing::info!(
            target: "oilship.publish",
            program = %self.options.program_id,
            bridge = %assessment.bridge,
            score = assessment.score,
            "publishing risk update"
        );
        Ok(PublishOutcome::Sent { tx_signature: "noop".to_string(), slot: 0 })
    }
}

#[derive(Debug, Clone)]
pub enum PublishOutcome {
    DryRun,
    Throttled,
    Sent { tx_signature: String, slot: u64 },
}
