//! Scheduling utilities for the watch engine.

use crate::types::BridgeId;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct Schedule {
    pub interval: Duration,
    pub jitter: Duration,
}

impl Schedule {
    pub fn new(interval_secs: u64, jitter_secs: u64) -> Self {
        Self {
            interval: Duration::from_secs(interval_secs),
            jitter: Duration::from_secs(jitter_secs),
        }
    }
}

pub struct Scheduler {
    base: Schedule,
    last_run: BTreeMap<BridgeId, Instant>,
    rng_state: u64,
}

impl Scheduler {
    pub fn new(base: Schedule) -> Self {
        Self { base, last_run: BTreeMap::new(), rng_state: 0xc01dbeef }
    }
