use crate::metrics::histogram::{Histogram, HistogramSnapshot};

const LATENCY_BUCKETS: &[u64] = &[
    100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000,
];

#[derive(Debug)]
pub struct LatencyTracker {
    pub queue_wait: Histogram,
    pub execution: Histogram,
    pub preemption_overhead: Histogram,
    pub yield_overhead: Histogram,
}

impl LatencyTracker {
    pub fn new() -> Self {
        Self {
            queue_wait: Histogram::new(LATENCY_BUCKETS.to_vec()),
            execution: Histogram::new(LATENCY_BUCKETS.to_vec()),
            preemption_overhead: Histogram::new(LATENCY_BUCKETS.to_vec()),
            yield_overhead: Histogram::new(LATENCY_BUCKETS.to_vec()),
        }
    }

    pub fn record_queue_wait(&self, ns: u64) {
        self.queue_wait.record(ns);
    }

    pub fn record_execution(&self, ns: u64) {
        self.execution.record(ns);
    }

    pub fn record_preemption(&self, ns: u64) {
        self.preemption_overhead.record(ns);
    }

    pub fn record_yield(&self, ns: u64) {
        self.yield_overhead.record(ns);
    }

    pub fn snapshot(&self) -> HistogramSnapshot {
        HistogramSnapshot {
            queue_wait: self.queue_wait.snapshot(),
            execution: self.execution.snapshot(),
            preemption_overhead: self.preemption_overhead.snapshot(),
            yield_overhead: self.yield_overhead.snapshot(),
            p50_queue_wait: self.queue_wait.percentile(50.0),
            p90_queue_wait: self.queue_wait.percentile(90.0),
            p99_queue_wait: self.queue_wait.percentile(99.0),
            p50_execution: self.execution.percentile(50.0),
            p90_execution: self.execution.percentile(90.0),
            p99_execution: self.execution.percentile(99.0),
            p50_preemption: self.preemption_overhead.percentile(50.0),
            p90_preemption: self.preemption_overhead.percentile(90.0),
            p99_preemption: self.preemption_overhead.percentile(99.0),
            p50_yield: self.yield_overhead.percentile(50.0),
            p90_yield: self.yield_overhead.percentile(90.0),
            p99_yield: self.yield_overhead.percentile(99.0),
        }
    }
}

impl Default for LatencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_tracker() {
        let lt = LatencyTracker::new();
        lt.record_queue_wait(500);
        lt.record_execution(2000);
        lt.record_preemption(150);
        lt.record_yield(300);
        let snap = lt.snapshot();
        assert!(!snap.queue_wait.is_empty());
        assert!(!snap.execution.is_empty());
        assert!(!snap.preemption_overhead.is_empty());
        assert!(!snap.yield_overhead.is_empty());
    }

    #[test]
    fn test_latency_tracker_empty() {
        let lt = LatencyTracker::new();
        let snap = lt.snapshot();
        assert_eq!(snap.p50_queue_wait, 0);
    }
}
