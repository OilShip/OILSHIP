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

    pub fn due(&self, id: &BridgeId, now: Instant) -> bool {
        match self.last_run.get(id) {
            None => true,
            Some(t) => now.duration_since(*t) >= self.base.interval,
        }
    }

    pub fn mark(&mut self, id: BridgeId, now: Instant) {
        self.last_run.insert(id, now);
    }

    pub fn jitter(&mut self) -> Duration {
        if self.base.jitter.is_zero() { return Duration::ZERO; }
        self.rng_state = self.rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let max = self.base.jitter.as_millis() as u64;
        let pick = (self.rng_state >> 32) as u64 % max.max(1);
        Duration::from_millis(pick)
    }

    pub fn next_after(&mut self, now: Instant) -> Duration {
        let cycle = self.base.interval + self.jitter();
        let oldest = self.last_run.values().min().cloned().unwrap_or(now);
        let elapsed = now.duration_since(oldest);
        if elapsed >= cycle { Duration::ZERO } else { cycle - elapsed }
    }

    pub fn known_bridges(&self) -> Vec<&BridgeId> {
        self.last_run.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_call_is_due() {
        let s = Scheduler::new(Schedule::new(60, 0));
        assert!(s.due(&BridgeId::new("x"), Instant::now()));
    }

    #[test]
    fn marked_is_not_due_immediately() {
        let mut s = Scheduler::new(Schedule::new(60, 0));
        let now = Instant::now();
        s.mark(BridgeId::new("x"), now);
        assert!(!s.due(&BridgeId::new("x"), now));
    }

    #[test]
    fn jitter_within_bounds() {
        let mut s = Scheduler::new(Schedule::new(60, 5));
        for _ in 0..1000 {
            let j = s.jitter();
            assert!(j <= Duration::from_secs(5));
        }
    }
}
