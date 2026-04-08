//! Notifier — collapses a stream of alerts into a single human-friendly
//! summary suitable for posting into a chat channel.

use crate::alerts::{Alert, AlertKind};
use crate::types::{RiskAssessment, Tier};
use std::collections::BTreeMap;

pub struct NotifierState {
    pub last_score_per_bridge: BTreeMap<String, u8>,
    pub last_tier_per_bridge: BTreeMap<String, Tier>,
}

impl Default for NotifierState {
    fn default() -> Self {
        Self {
            last_score_per_bridge: BTreeMap::new(),
            last_tier_per_bridge: BTreeMap::new(),
        }
    }
}

pub struct Notifier {
    state: NotifierState,
}

impl Notifier {
    pub fn new() -> Self { Self { state: NotifierState::default() } }

    pub fn observe(&mut self, assessment: &RiskAssessment) -> Option<String> {
        let bridge = assessment.bridge.to_string();
        let prev_score = self.state.last_score_per_bridge.get(&bridge).copied();
        let prev_tier = self.state.last_tier_per_bridge.get(&bridge).copied();
        self.state.last_score_per_bridge.insert(bridge.clone(), assessment.score);
        self.state.last_tier_per_bridge.insert(bridge.clone(), assessment.tier);
        match (prev_score, prev_tier) {
            (None, _) => Some(format!("⛴ {} watching with score {}", bridge, assessment.score)),
            (Some(p), Some(pt)) if pt != assessment.tier => Some(format!(
                "⚠ {} tier {:?} → {:?} (score {} → {})",
                bridge, pt, assessment.tier, p, assessment.score
            )),
            (Some(p), _) if assessment.score.saturating_sub(p) >= 10 => Some(format!(
                "⚠ {} score climbed {} → {}",
                bridge, p, assessment.score
            )),
            _ => None,
        }
    }

    pub fn from_alert(&mut self, alert: &Alert) -> Option<String> {
        match alert.kind {
            AlertKind::NewSample => self.observe(&alert.assessment),
            AlertKind::TierDowngrade => Some(format!(
                "⚠ {} downgraded to {:?} at score {}",
                alert.assessment.bridge, alert.assessment.tier, alert.assessment.score
            )),
            AlertKind::Quarantined => Some(format!(
                "❌ {} quarantined at score {}",
                alert.assessment.bridge, alert.assessment.score
            )),
            AlertKind::Recovered => Some(format!(
                "✓ {} recovered to {:?}",
                alert.assessment.bridge, alert.assessment.tier
            )),
        }
    }

    pub fn snapshot(&self) -> NotifierState {
        NotifierState {
            last_score_per_bridge: self.state.last_score_per_bridge.clone(),
            last_tier_per_bridge: self.state.last_tier_per_bridge.clone(),
        }
    }
}

impl Default for Notifier {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BridgeId;
