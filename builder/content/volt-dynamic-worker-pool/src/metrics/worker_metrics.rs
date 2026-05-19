use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerMetricsSnapshot {
    pub spawn_count: u64,
    pub retire_count: u64,
    pub max_concurrent: u64,
    pub crash_count: u64,
}

#[derive(Debug)]
pub struct WorkerMetrics {
    pub spawn_count: AtomicU64,
    pub retire_count: AtomicU64,
    pub max_concurrent: AtomicU64,
    pub crash_count: AtomicU64,
}

impl WorkerMetrics {
    pub fn new() -> Self {
        Self {
            spawn_count: AtomicU64::new(0),
            retire_count: AtomicU64::new(0),
            max_concurrent: AtomicU64::new(0),
            crash_count: AtomicU64::new(0),
        }
    }

    pub fn record_spawn(&self, concurrent: u64) {
        self.spawn_count.fetch_add(1, Ordering::Relaxed);
        let mut prev = self.max_concurrent.load(Ordering::Relaxed);
        while concurrent > prev {
            match self.max_concurrent.compare_exchange_weak(
                prev,
                concurrent,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => prev = actual,
            }
        }
    }

    pub fn record_retire(&self) {
        self.retire_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_crash(&self) {
        self.crash_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> WorkerMetricsSnapshot {
        WorkerMetricsSnapshot {
            spawn_count: self.spawn_count.load(Ordering::Relaxed),
            retire_count: self.retire_count.load(Ordering::Relaxed),
            max_concurrent: self.max_concurrent.load(Ordering::Relaxed),
            crash_count: self.crash_count.load(Ordering::Relaxed),
        }
    }
}

impl Default for WorkerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_metrics() {
        let wm = WorkerMetrics::new();
        wm.record_spawn(1);
        wm.record_spawn(3);
        wm.record_spawn(2);
        wm.record_retire();
        wm.record_crash();
        let snap = wm.snapshot();
        assert_eq!(snap.spawn_count, 3);
        assert_eq!(snap.retire_count, 1);
        assert_eq!(snap.crash_count, 1);
        assert_eq!(snap.max_concurrent, 3);
    }

    #[test]
    fn test_worker_metrics_empty() {
        let wm = WorkerMetrics::new();
        let snap = wm.snapshot();
        assert_eq!(snap.spawn_count, 0);
        assert_eq!(snap.max_concurrent, 0);
    }
}
