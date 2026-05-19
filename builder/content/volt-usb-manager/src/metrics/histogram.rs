use std::sync::atomic::{AtomicU64, Ordering};

pub struct LatencyHistogram {
    buckets: Vec<AtomicU64>,
    bounds: Vec<u64>,
}

impl LatencyHistogram {
    pub fn new(bounds: Vec<u64>) -> Self {
        let len = bounds.len() + 1;
        let mut buckets = Vec::with_capacity(len);
        for _ in 0..len {
            buckets.push(AtomicU64::new(0));
        }
        LatencyHistogram { buckets, bounds }
    }

    pub fn record(&self, value_us: u64) {
        let idx = self
            .bounds
            .iter()
            .position(|&b| value_us < b)
            .unwrap_or(self.buckets.len() - 1);
        self.buckets[idx].fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> Vec<u64> {
        self.buckets
            .iter()
            .map(|b| b.load(Ordering::Relaxed))
            .collect()
    }

    pub fn average(&self) -> f64 {
        let snap = self.snapshot();
        let total: u64 = snap.iter().sum();
        if total == 0 {
            return 0.0;
        }
        let mut weighted_sum: u128 = 0;
        for (i, &count) in snap.iter().enumerate() {
            let prev_bound = if i == 0 { 0 } else { self.bounds[i - 1] };
            let bound = if i < self.bounds.len() {
                self.bounds[i]
            } else {
                u64::MAX
            };
            let mid = if bound == u64::MAX {
                prev_bound as u128 * 2
            } else {
                (prev_bound as u128 + bound as u128) / 2
            };
            weighted_sum += mid * count as u128;
        }
        weighted_sum as f64 / total as f64
    }

    pub fn reset(&self) {
        for b in &self.buckets {
            b.store(0, Ordering::Relaxed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_histogram() {
        let h = LatencyHistogram::new(vec![100, 200, 500, 1000]);
        assert_eq!(h.snapshot().len(), 5);
    }

    #[test]
    fn test_record_in_first_bucket() {
        let h = LatencyHistogram::new(vec![100, 200]);
        h.record(50);
        let snap = h.snapshot();
        assert_eq!(snap[0], 1);
        assert_eq!(snap[1], 0);
        assert_eq!(snap[2], 0);
    }

    #[test]
    fn test_record_in_last_bucket() {
        let h = LatencyHistogram::new(vec![100, 200]);
        h.record(999);
        let snap = h.snapshot();
        assert_eq!(snap[0], 0);
        assert_eq!(snap[1], 0);
        assert_eq!(snap[2], 1);
    }

    #[test]
    fn test_average_single_value() {
        let h = LatencyHistogram::new(vec![100, 200]);
        h.record(50);
        let avg = h.average();
        assert!((avg - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_average_empty() {
        let h = LatencyHistogram::new(vec![100, 200]);
        assert!((h.average() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_reset_clears_all() {
        let h = LatencyHistogram::new(vec![100, 200]);
        h.record(50);
        h.record(150);
        h.reset();
        let snap = h.snapshot();
        assert_eq!(snap.iter().sum::<u64>(), 0);
    }

    #[test]
    fn test_record_on_boundary() {
        let h = LatencyHistogram::new(vec![100, 200]);
        h.record(100);
        let snap = h.snapshot();
        assert_eq!(snap[0], 0);
        assert_eq!(snap[1], 1);
    }

    #[test]
    fn test_empty_bounds() {
        let h = LatencyHistogram::new(vec![]);
        h.record(42);
        let snap = h.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0], 1);
    }

    #[test]
    fn test_multiple_records() {
        let h = LatencyHistogram::new(vec![10, 20, 30]);
        h.record(5);
        h.record(15);
        h.record(25);
        h.record(35);
        let snap = h.snapshot();
        assert_eq!(snap[0], 1);
        assert_eq!(snap[1], 1);
        assert_eq!(snap[2], 1);
        assert_eq!(snap[3], 1);
    }
}
