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
