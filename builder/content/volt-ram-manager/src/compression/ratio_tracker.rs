use std::sync::atomic::{AtomicU64, Ordering};

pub struct RatioTracker {
    total_raw: AtomicU64,
    total_compressed: AtomicU64,
    compress_count: AtomicU64,
}

impl RatioTracker {
    pub fn new() -> Self {
        RatioTracker {
            total_raw: AtomicU64::new(0),
            total_compressed: AtomicU64::new(0),
            compress_count: AtomicU64::new(0),
        }
    }

    pub fn record(&self, raw: u64, compressed: u64) {
        self.total_raw.fetch_add(raw, Ordering::Relaxed);
        self.total_compressed.fetch_add(compressed, Ordering::Relaxed);
        self.compress_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn average_ratio(&self) -> f64 {
        let raw = self.total_raw.load(Ordering::Relaxed);
        if raw == 0 {
            return 1.0;
        }
        self.total_compressed.load(Ordering::Relaxed) as f64 / raw as f64
    }

    pub fn total_raw(&self) -> u64 {
        self.total_raw.load(Ordering::Relaxed)
    }

    pub fn total_compressed(&self) -> u64 {
        self.total_compressed.load(Ordering::Relaxed)
    }

    pub fn savings(&self) -> u64 {
        let raw = self.total_raw.load(Ordering::Relaxed);
        let comp = self.total_compressed.load(Ordering::Relaxed);
        if raw > comp {
            raw - comp
        } else {
            0
        }
    }

    pub fn count(&self) -> u64 {
        self.compress_count.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tracker() {
        let t = RatioTracker::new();
        assert_eq!(t.total_raw(), 0);
        assert_eq!(t.total_compressed(), 0);
        assert_eq!(t.count(), 0);
        assert!((t.average_ratio() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_record() {
        let t = RatioTracker::new();
        t.record(100, 30);
        assert_eq!(t.total_raw(), 100);
        assert_eq!(t.total_compressed(), 30);
        assert_eq!(t.count(), 1);
        assert!((t.average_ratio() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_savings() {
        let t = RatioTracker::new();
        t.record(200, 50);
        assert_eq!(t.savings(), 150);
    }

    #[test]
    fn test_no_savings() {
        let t = RatioTracker::new();
        t.record(50, 100);
        assert_eq!(t.savings(), 0);
    }

    #[test]
    fn test_multiple_records() {
        let t = RatioTracker::new();
        t.record(100, 30);
        t.record(200, 60);
        assert_eq!(t.total_raw(), 300);
        assert_eq!(t.total_compressed(), 90);
        assert_eq!(t.count(), 2);
        assert!((t.average_ratio() - 0.3).abs() < 1e-10);
    }
}
