#[derive(Debug, Clone)]
pub struct GcReport {
    pub pages_freed: u64,
    pub bytes_reclaimed: u64,
    pub duration_ms: u64,
    pub aggressive: bool,
}

impl GcReport {
    pub fn new() -> Self {
        Self {
            pages_freed: 0,
            bytes_reclaimed: 0,
            duration_ms: 0,
            aggressive: false,
        }
    }

    pub fn new_aggressive() -> Self {
        Self {
            aggressive: true,
            ..Self::new()
        }
    }

    pub fn merge(&mut self, other: &GcReport) {
        self.pages_freed += other.pages_freed;
        self.bytes_reclaimed += other.bytes_reclaimed;
        self.duration_ms += other.duration_ms;
        self.aggressive = self.aggressive || other.aggressive;
    }

    pub fn is_empty(&self) -> bool {
        self.pages_freed == 0 && self.bytes_reclaimed == 0
    }

    pub fn bytes_per_second(&self) -> f64 {
        if self.duration_ms == 0 {
            return 0.0;
        }
        (self.bytes_reclaimed as f64 / self.duration_ms as f64) * 1000.0
    }

    pub fn avg_page_size(&self) -> f64 {
        if self.pages_freed == 0 {
            return 0.0;
        }
        self.bytes_reclaimed as f64 / self.pages_freed as f64
    }
}

impl Default for GcReport {
    fn default() -> Self {
        Self::new()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_report_empty() {
        let report = GcReport::new();
        assert!(report.is_empty());
        assert!(!report.aggressive);
    }

    #[test]
    fn test_new_aggressive() {
        let report = GcReport::new_aggressive();
        assert!(report.is_empty());
        assert!(report.aggressive);
    }

    #[test]
    fn test_merge() {
        let mut r1 = GcReport::new();
        r1.pages_freed = 5;
        r1.bytes_reclaimed = 1000;
        r1.duration_ms = 10;

        let mut r2 = GcReport::new();
        r2.pages_freed = 3;
        r2.bytes_reclaimed = 500;
        r2.duration_ms = 5;

        r1.merge(&r2);
        assert_eq!(r1.pages_freed, 8);
        assert_eq!(r1.bytes_reclaimed, 1500);
        assert_eq!(r1.duration_ms, 15);
    }

    #[test]
    fn test_merge_sets_aggressive() {
        let mut r1 = GcReport::new();
        let r2 = GcReport::new_aggressive();
        r1.merge(&r2);
        assert!(r1.aggressive);
    }

    #[test]
    fn test_bytes_per_second() {
        let mut report = GcReport::new();
        report.bytes_reclaimed = 1000;
        report.duration_ms = 100;
        let bps = report.bytes_per_second();
        assert!((bps - 10000.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bytes_per_second_zero_duration() {
        let report = GcReport::new();
        assert_eq!(report.bytes_per_second(), 0.0);
    }

    #[test]
    fn test_avg_page_size() {
        let mut report = GcReport::new();
        report.pages_freed = 10;
        report.bytes_reclaimed = 40960;
        assert!((report.avg_page_size() - 4096.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_avg_page_size_zero() {
        let report = GcReport::new();
        assert_eq!(report.avg_page_size(), 0.0);
    }

    #[test]
    fn test_default() {
        let report: GcReport = Default::default();
        assert!(report.is_empty());
        assert!(!report.aggressive);
    }
}
