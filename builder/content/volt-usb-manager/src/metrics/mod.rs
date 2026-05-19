pub mod counters;
pub mod histogram;
pub mod latency;
pub mod report;
pub mod snapshot;
pub mod throughput;

use std::collections::HashMap;
use std::sync::atomic::Ordering;
use std::time::Instant;

use parking_lot::RwLock;

pub use self::counters::MetricsCounters;
pub use self::snapshot::MetricsSnapshot;

#[derive(Debug)]
pub struct MetricsEngine {
    counters: MetricsCounters,
    start_time: Instant,
    events: RwLock<Vec<(String, u64)>>,
    latency_buckets: RwLock<HashMap<String, Vec<f64>>>,
}

impl MetricsEngine {
    pub fn new() -> Self {
        MetricsEngine {
            counters: MetricsCounters::new(),
            start_time: Instant::now(),
            events: RwLock::new(Vec::new()),
            latency_buckets: RwLock::new(HashMap::new()),
        }
    }

    pub fn record_hit(&self) {
        self.counters.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_miss(&self) {
        self.counters.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_write_queued(&self) {
        // Maps to pending write count - stored in write buffer, not directly here.
        // We increment a local counter for the snapshot.
        self.counters.sbp_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_write_flushed(&self) {
        self.counters.flush_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_journal_record(&self) {
        // Track via sbp_requests as a proxy; journal records tracked in journal module.
        self.counters.sbp_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_event(&self, event: &str) {
        let now = self.start_time.elapsed().as_millis() as u64;
        self.events.write().push((event.to_string(), now));
    }

    pub fn record_latency(&self, operation: &str, latency_ms: f64) {
        let mut buckets = self.latency_buckets.write();
        buckets
            .entry(operation.to_string())
            .or_default()
            .push(latency_ms);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let mut snap = self.counters.snapshot();
        snap.uptime_ms = self.start_time.elapsed().as_millis() as u64;
        snap
    }
}

impl Default for MetricsEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_engine_new() {
        let engine = MetricsEngine::new();
        let snapshot = engine.snapshot();
        assert_eq!(snapshot.cache_hit_count, 0);
        assert_eq!(snapshot.cache_miss_count, 0);
    }

    #[test]
    fn test_metrics_engine_recording() {
        let engine = MetricsEngine::new();
        engine.record_hit();
        engine.record_hit();
        engine.record_miss();
        let snapshot = engine.snapshot();
        assert_eq!(snapshot.cache_hit_count, 2);
        assert_eq!(snapshot.cache_miss_count, 1);
    }

    #[test]
    fn test_metrics_engine_hit_ratio() {
        let engine = MetricsEngine::new();
        engine.record_hit();
        engine.record_hit();
        engine.record_miss();
        let snapshot = engine.snapshot();
        assert!((snapshot.cache_hit_ratio - 2.0 / 3.0).abs() < 0.001);
    }

    #[test]
    fn test_metrics_engine_event() {
        let engine = MetricsEngine::new();
        engine.record_event("init");
        engine.record_event("mount");
    }

    #[test]
    fn test_metrics_engine_latency() {
        let engine = MetricsEngine::new();
        engine.record_latency("read", 1.5);
        engine.record_latency("read", 2.3);
        engine.record_latency("write", 0.8);
    }

    #[test]
    fn test_metrics_engine_flush_count() {
        let engine = MetricsEngine::new();
        engine.record_write_flushed();
        engine.record_write_flushed();
        let snapshot = engine.snapshot();
        assert_eq!(snapshot.flush_count, 2);
    }

    #[test]
    fn test_metrics_engine_uptime_increasing() {
        let engine = MetricsEngine::new();
        let t1 = engine.snapshot().uptime_ms;
        std::thread::sleep(std::time::Duration::from_millis(5));
        let t2 = engine.snapshot().uptime_ms;
        assert!(t2 >= t1);
    }
}
