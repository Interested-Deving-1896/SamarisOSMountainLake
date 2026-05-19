use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMetricsSnapshot {
    pub enqueue_count: u64,
    pub dequeue_count: u64,
    pub cancel_count: u64,
    pub peak_depth: u64,
}

#[derive(Debug)]
pub struct QueueMetrics {
    pub enqueue_count: AtomicU64,
    pub dequeue_count: AtomicU64,
    pub cancel_count: AtomicU64,
    pub peak_depth: AtomicU64,
}

impl QueueMetrics {
    pub fn new() -> Self {
        Self {
            enqueue_count: AtomicU64::new(0),
            dequeue_count: AtomicU64::new(0),
            cancel_count: AtomicU64::new(0),
            peak_depth: AtomicU64::new(0),
        }
    }

    pub fn record_enqueue(&self, current_depth: u64) {
        self.enqueue_count.fetch_add(1, Ordering::Relaxed);
        let mut prev = self.peak_depth.load(Ordering::Relaxed);
        while current_depth > prev {
            match self.peak_depth.compare_exchange_weak(
                prev,
                current_depth,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => prev = actual,
            }
        }
    }

    pub fn record_dequeue(&self) {
        self.dequeue_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cancel(&self) {
        self.cancel_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> QueueMetricsSnapshot {
        QueueMetricsSnapshot {
            enqueue_count: self.enqueue_count.load(Ordering::Relaxed),
            dequeue_count: self.dequeue_count.load(Ordering::Relaxed),
            cancel_count: self.cancel_count.load(Ordering::Relaxed),
            peak_depth: self.peak_depth.load(Ordering::Relaxed),
        }
    }
}

impl Default for QueueMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_metrics() {
        let qm = QueueMetrics::new();
        qm.record_enqueue(1);
        qm.record_enqueue(5);
        qm.record_enqueue(3);
        qm.record_dequeue();
        qm.record_cancel();
        let snap = qm.snapshot();
        assert_eq!(snap.enqueue_count, 3);
        assert_eq!(snap.dequeue_count, 1);
        assert_eq!(snap.cancel_count, 1);
        assert_eq!(snap.peak_depth, 5);
    }

    #[test]
    fn test_queue_metrics_empty() {
        let qm = QueueMetrics::new();
        let snap = qm.snapshot();
        assert_eq!(snap.enqueue_count, 0);
        assert_eq!(snap.peak_depth, 0);
    }
}
