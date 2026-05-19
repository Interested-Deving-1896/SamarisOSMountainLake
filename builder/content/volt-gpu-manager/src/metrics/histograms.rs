use std::sync::atomic::{AtomicU64, Ordering};

const BUCKET_COUNT: usize = 16;

#[derive(Debug)]
pub struct GpuHistogram {
    buckets: [AtomicU64; BUCKET_COUNT],
    low: u64,
    high: u64,
}

impl GpuHistogram {
    pub fn new(low: u64, high: u64) -> Self {
        let buckets = [
            AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
            AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
            AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
            AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
        ];
        Self { buckets, low, high }
    }

    pub fn record(&self, value: u64) {
        let bucket = if value <= self.low {
            0
        } else if value >= self.high {
            BUCKET_COUNT - 1
        } else {
            let range = self.high - self.low;
            let scaled = ((value - self.low) as u128 * (BUCKET_COUNT - 2) as u128) / range as u128;
            1 + (scaled as usize).min(BUCKET_COUNT - 2)
        };
        self.buckets[bucket].fetch_add(1, Ordering::Relaxed);
    }

    pub fn get(&self, bucket: usize) -> u64 {
        if bucket >= BUCKET_COUNT {
            return 0;
        }
        self.buckets[bucket].load(Ordering::Relaxed)
    }

    pub fn total(&self) -> u64 {
        self.buckets.iter().map(|b| b.load(Ordering::Relaxed)).sum()
    }

    pub fn percentile(&self, pct: f64) -> u64 {
        let total = self.total();
        if total == 0 {
            return 0;
        }
        let target = (total as f64 * pct / 100.0).ceil() as u64;
        let mut cumulative = 0u64;
        for (i, bucket) in self.buckets.iter().enumerate() {
            cumulative += bucket.load(Ordering::Relaxed);
            if cumulative >= target {
                let range = self.high - self.low;
                return self.low + (range as u128 * i as u128 / (BUCKET_COUNT - 1) as u128) as u64;
            }
        }
        self.high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_histogram() {
        let h = GpuHistogram::new(0, 1000);
        assert_eq!(h.total(), 0);
    }

    #[test]
    fn test_record_single() {
        let h = GpuHistogram::new(0, 1000);
        h.record(500);
        assert_eq!(h.total(), 1);
    }

    #[test]
    fn test_record_below_range() {
        let h = GpuHistogram::new(100, 1000);
        h.record(50);
        assert_eq!(h.get(0), 1);
    }

    #[test]
    fn test_record_above_range() {
        let h = GpuHistogram::new(0, 1000);
        h.record(2000);
        assert_eq!(h.get(BUCKET_COUNT - 1), 1);
    }

    #[test]
    fn test_percentile() {
        let h = GpuHistogram::new(0, 1000);
        for _ in 0..100 {
            h.record(500);
        }
        let p50 = h.percentile(50.0);
        assert!(p50 <= 1000);
    }

    #[test]
    fn test_percentile_empty() {
        let h = GpuHistogram::new(0, 100);
        assert_eq!(h.percentile(50.0), 0);
    }
}
