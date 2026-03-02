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
