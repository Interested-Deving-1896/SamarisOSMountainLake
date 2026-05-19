pub struct GpuCompressionRatioTracker {
    total_raw: u64,
    total_compressed: u64,
    count: u64,
}

impl GpuCompressionRatioTracker {
    pub fn new() -> Self {
        GpuCompressionRatioTracker {
            total_raw: 0,
            total_compressed: 0,
            count: 0,
        }
    }

    pub fn record(&mut self, raw: u64, compressed: u64) {
        self.total_raw += raw;
        self.total_compressed += compressed;
        self.count += 1;
    }

    pub fn average_ratio(&self) -> f64 {
        if self.total_raw == 0 {
            return 1.0;
        }
        self.total_compressed as f64 / self.total_raw as f64
    }

    pub fn saved_bytes(&self) -> u64 {
        if self.total_compressed >= self.total_raw {
            0
        } else {
            self.total_raw - self.total_compressed
        }
    }

    pub fn compress_count(&self) -> u64 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tracker() {
        let t = GpuCompressionRatioTracker::new();
        assert_eq!(t.average_ratio(), 1.0);
        assert_eq!(t.saved_bytes(), 0);
        assert_eq!(t.compress_count(), 0);
    }

    #[test]
    fn test_record_updates_stats() {
        let mut t = GpuCompressionRatioTracker::new();
        t.record(100, 80);
        assert_eq!(t.compress_count(), 1);
        assert!((t.average_ratio() - 0.8).abs() < 1e-10);
        assert_eq!(t.saved_bytes(), 20);
    }

    #[test]
    fn test_multiple_records() {
        let mut t = GpuCompressionRatioTracker::new();
        t.record(100, 80);
        t.record(100, 60);
        assert_eq!(t.compress_count(), 2);
        assert!((t.average_ratio() - 0.7).abs() < 1e-10);
        assert_eq!(t.saved_bytes(), 60);
    }

    #[test]
    fn test_no_savings_if_expanded() {
        let mut t = GpuCompressionRatioTracker::new();
        t.record(100, 150);
        assert_eq!(t.saved_bytes(), 0);
        assert!((t.average_ratio() - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_average_ratio_precision() {
        let mut t = GpuCompressionRatioTracker::new();
        t.record(3, 1);
        assert!((t.average_ratio() - 1.0 / 3.0).abs() < 1e-10);
    }
}
