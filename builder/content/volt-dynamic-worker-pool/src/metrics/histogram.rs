use std::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramSnapshot {
    pub queue_wait: Vec<(u64, u64)>,
    pub execution: Vec<(u64, u64)>,
    pub preemption_overhead: Vec<(u64, u64)>,
    pub yield_overhead: Vec<(u64, u64)>,
    pub p50_queue_wait: u64,
    pub p90_queue_wait: u64,
    pub p99_queue_wait: u64,
    pub p50_execution: u64,
    pub p90_execution: u64,
    pub p99_execution: u64,
    pub p50_preemption: u64,
    pub p90_preemption: u64,
    pub p99_preemption: u64,
    pub p50_yield: u64,
    pub p90_yield: u64,
    pub p99_yield: u64,
}

#[derive(Debug)]
pub struct Histogram {
    buckets: Vec<u64>,
    counts: Vec<AtomicU64>,
    total_samples: AtomicU64,
}

impl Histogram {
    pub fn new(mut buckets: Vec<u64>) -> Self {
        buckets.sort_unstable();
        buckets.dedup();
        let len = buckets.len();
        Self {
            buckets,
            counts: (0..len).map(|_| AtomicU64::new(0)).collect(),
            total_samples: AtomicU64::new(0),
        }
    }

    pub fn record(&self, value: u64) {
        let idx = match self.buckets.binary_search(&value) {
            Ok(i) => i,
            Err(i) if i >= self.buckets.len() => self.buckets.len() - 1,
            Err(i) => i,
        };
        self.counts[idx].fetch_add(1, Ordering::Relaxed);
        self.total_samples.fetch_add(1, Ordering::Relaxed);
    }

    pub fn percentile(&self, p: f64) -> u64 {
        let total = self.total_samples.load(Ordering::Relaxed);
        if total == 0 {
            return 0;
        }
        let target = (p / 100.0) * total as f64;
        let mut cumulative = 0u64;
        for (i, count) in self.counts.iter().enumerate() {
            cumulative += count.load(Ordering::Relaxed);
            if cumulative as f64 >= target {
                return self.buckets[i];
            }
        }
        *self.buckets.last().unwrap_or(&0)
    }

    pub fn snapshot(&self) -> Vec<(u64, u64)> {
        self.buckets
            .iter()
            .zip(self.counts.iter())
            .map(|(b, c)| (*b, c.load(Ordering::Relaxed)))
            .collect()
    }

    pub fn clear(&self) {
        for count in &self.counts {
            count.store(0, Ordering::Relaxed);
        }
        self.total_samples.store(0, Ordering::Relaxed);
    }

    pub fn total(&self) -> u64 {
        self.total_samples.load(Ordering::Relaxed)
    }

    pub fn avg(&self) -> f64 {
        let total = self.total_samples.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let mut sum = 0u64;
        let mut prev = 0u64;
        for (i, count) in self.counts.iter().enumerate() {
            let c = count.load(Ordering::Relaxed);
            let upper = self.buckets[i];
            let midpoint = (prev + upper) / 2;
            sum += midpoint * c;
            prev = upper;
        }
        sum as f64 / total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_new_and_record() {
        let h = Histogram::new(vec![100, 500, 1000, 5000]);
        assert_eq!(h.total(), 0);
        h.record(50);
        assert_eq!(h.total(), 1);
        h.record(100);
        assert_eq!(h.total(), 2);
        h.record(250);
        assert_eq!(h.total(), 3);
        h.record(5000);
        assert_eq!(h.total(), 4);
        h.record(10000);
        assert_eq!(h.total(), 5);
    }

    #[test]
    fn test_histogram_percentile() {
        let h = Histogram::new(vec![10, 20, 30, 40, 50]);
        for _ in 0..100 {
            h.record(5);
        }
        for _ in 0..100 {
            h.record(15);
        }
        for _ in 0..100 {
            h.record(25);
        }
        for _ in 0..100 {
            h.record(35);
        }
        for _ in 0..100 {
            h.record(45);
        }
        assert_eq!(h.percentile(50.0), 30);
        assert_eq!(h.percentile(90.0), 50);
    }

    #[test]
    fn test_histogram_clear() {
        let h = Histogram::new(vec![100, 500]);
        h.record(50);
        h.record(250);
        assert_eq!(h.total(), 2);
        h.clear();
        assert_eq!(h.total(), 0);
    }

    #[test]
    fn test_histogram_snapshot() {
        let h = Histogram::new(vec![100, 500]);
        h.record(50);
        h.record(250);
        let snap = h.snapshot();
        assert_eq!(snap.len(), 2);
        assert_eq!(snap[0], (100, 1));
        assert_eq!(snap[1], (500, 1));
    }

    #[test]
    fn test_histogram_avg() {
        let h = Histogram::new(vec![100, 200]);
        h.record(50);
        h.record(150);
        let avg = h.avg();
        assert!(avg > 0.0);
    }
}
