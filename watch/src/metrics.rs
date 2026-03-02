//! Lightweight metrics counters for the watch engine.

use parking_lot::RwLock;
use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

#[derive(Default)]
pub struct Counters {
    inner: RwLock<BTreeMap<&'static str, u64>>,
}

impl Counters {
    pub fn shared() -> Arc<Self> { Arc::new(Self::default()) }

    pub fn incr(&self, name: &'static str) {
        let mut g = self.inner.write();
        *g.entry(name).or_insert(0) += 1;
    }

    pub fn add(&self, name: &'static str, n: u64) {
        let mut g = self.inner.write();
        *g.entry(name).or_insert(0) += n;
    }

    pub fn snapshot(&self) -> BTreeMap<&'static str, u64> {
        self.inner.read().clone()
    }

    pub fn render_prom(&self) -> String {
        let snap = self.snapshot();
        let mut out = String::new();
        for (k, v) in snap {
            out.push_str(&format!("# TYPE {k} counter\n{k} {v}\n"));
        }
        out
    }
}

pub struct UptimeClock {
    started: Instant,
}

impl UptimeClock {
    pub fn new() -> Self { Self { started: Instant::now() } }
    pub fn uptime_secs(&self) -> u64 { self.started.elapsed().as_secs() }
}

impl Default for UptimeClock {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counters_basic() {
        let c = Counters::default();
        c.incr("samples");
        c.incr("samples");
        c.add("anomalies", 5);
        let snap = c.snapshot();
        assert_eq!(snap["samples"], 2);
        assert_eq!(snap["anomalies"], 5);
    }

    #[test]
    fn render_includes_lines() {
        let c = Counters::default();
        c.add("foo", 3);
        let s = c.render_prom();
        assert!(s.contains("foo 3"));
    }
}
