use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilizationSnapshot {
    pub total_worker_time_us: u64,
    pub busy_worker_time_us: u64,
    pub utilization: f64,
}

#[derive(Debug)]
pub struct UtilizationTracker {
    total_worker_time_us: AtomicU64,
    busy_worker_time_us: AtomicU64,
    samples: AtomicU64,
}

impl UtilizationTracker {
    pub fn new() -> Self {
        Self {
            total_worker_time_us: AtomicU64::new(0),
            busy_worker_time_us: AtomicU64::new(0),
            samples: AtomicU64::new(0),
        }
    }

    pub fn record_busy(&self, duration_us: u64) {
        self.busy_worker_time_us.fetch_add(duration_us, Ordering::Relaxed);
        self.total_worker_time_us.fetch_add(duration_us, Ordering::Relaxed);
        self.samples.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_idle(&self, duration_us: u64) {
        self.total_worker_time_us.fetch_add(duration_us, Ordering::Relaxed);
        self.samples.fetch_add(1, Ordering::Relaxed);
    }

    pub fn utilization(&self) -> f64 {
        let total = self.total_worker_time_us.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let busy = self.busy_worker_time_us.load(Ordering::Relaxed);
        busy as f64 / total as f64
    }

    pub fn snapshot(&self) -> UtilizationSnapshot {
        let total = self.total_worker_time_us.load(Ordering::Relaxed);
        let busy = self.busy_worker_time_us.load(Ordering::Relaxed);
        let utilization = if total > 0 {
            busy as f64 / total as f64
        } else {
            0.0
        };
        UtilizationSnapshot {
            total_worker_time_us: total,
            busy_worker_time_us: busy,
            utilization,
        }
    }
}

impl Default for UtilizationTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilization_tracker() {
        let ut = UtilizationTracker::new();
        assert!((ut.utilization() - 0.0).abs() < f64::EPSILON);
        ut.record_busy(100);
        ut.record_idle(100);
        let u = ut.utilization();
        assert!((u - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_utilization_snapshot() {
        let ut = UtilizationTracker::new();
        ut.record_busy(75);
        ut.record_idle(25);
        let snap = ut.snapshot();
        assert_eq!(snap.total_worker_time_us, 100);
        assert_eq!(snap.busy_worker_time_us, 75);
        assert!((snap.utilization - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_utilization_empty() {
        let ut = UtilizationTracker::new();
        let snap = ut.snapshot();
        assert_eq!(snap.total_worker_time_us, 0);
        assert!((snap.utilization - 0.0).abs() < f64::EPSILON);
    }
}
