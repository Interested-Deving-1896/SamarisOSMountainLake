pub struct RatioTracker {
    total_raw: u64,
    total_compressed: u64,
    compress_count: u64,
}

impl RatioTracker {
    pub fn new() -> Self {
        RatioTracker {
            total_raw: 0,
            total_compressed: 0,
            compress_count: 0,
        }
    }

    pub fn record(&mut self, raw: u64, compressed: u64) {
        self.total_raw += raw;
        self.total_compressed += compressed;
        self.compress_count += 1;
    }

    pub fn average_ratio(&self) -> f64 {
        if self.total_raw == 0 {
            return 1.0;
        }
        self.total_compressed as f64 / self.total_raw as f64
    }

    pub fn savings(&self) -> u64 {
        self.total_raw.saturating_sub(self.total_compressed)
    }

    pub fn count(&self) -> u64 {
        self.compress_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_ratio_tracker() {
        let tracker = RatioTracker::new();
        assert_eq!(tracker.count(), 0);
        assert!((tracker.average_ratio() - 1.0).abs() < 0.001);
        assert_eq!(tracker.savings(), 0);
    }

    #[test]
    fn test_record_and_average() {
        let mut tracker = RatioTracker::new();
        tracker.record(100, 30);
        tracker.record(200, 60);
        assert!((tracker.average_ratio() - 0.3).abs() < 0.001);
        assert_eq!(tracker.count(), 2);
    }

    #[test]
    fn test_savings() {
        let mut tracker = RatioTracker::new();
        tracker.record(1000, 300);
        assert_eq!(tracker.savings(), 700);
    }

    #[test]
    fn test_no_savings_when_larger() {
        let mut tracker = RatioTracker::new();
        tracker.record(100, 200);
        assert_eq!(tracker.savings(), 0);
    }

    #[test]
    fn test_multiple_records() {
        let mut tracker = RatioTracker::new();
        tracker.record(50, 25);
        tracker.record(50, 25);
        assert!((tracker.average_ratio() - 0.5).abs() < 0.001);
        assert_eq!(tracker.savings(), 50);
    }
}
