#[derive(Debug, Clone)]
pub struct LatencyMetrics {
    pub avg_us: f64,
    pub min_us: u64,
    pub max_us: u64,
    pub count: u64,
}

impl LatencyMetrics {
    pub fn new() -> Self {
        LatencyMetrics {
            avg_us: 0.0,
            min_us: u64::MAX,
            max_us: 0,
            count: 0,
        }
    }

    pub fn record(&mut self, elapsed_us: u64) {
        self.count += 1;
        self.avg_us += (elapsed_us as f64 - self.avg_us) / self.count as f64;
        if elapsed_us < self.min_us {
            self.min_us = elapsed_us;
        }
        if elapsed_us > self.max_us {
            self.max_us = elapsed_us;
        }
    }

    pub fn snapshot(&self) -> Self {
        self.clone()
    }
}

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_latency_metrics() {
        let lm = LatencyMetrics::new();
        assert!((lm.avg_us - 0.0).abs() < 0.001);
        assert_eq!(lm.count, 0);
        assert_eq!(lm.min_us, u64::MAX);
        assert_eq!(lm.max_us, 0);
    }

    #[test]
    fn test_record_single_value() {
        let mut lm = LatencyMetrics::new();
        lm.record(100);
        assert!((lm.avg_us - 100.0).abs() < 0.001);
        assert_eq!(lm.count, 1);
        assert_eq!(lm.min_us, 100);
        assert_eq!(lm.max_us, 100);
    }

    #[test]
    fn test_record_multiple_values() {
        let mut lm = LatencyMetrics::new();
        lm.record(100);
        lm.record(200);
        lm.record(300);
        assert!((lm.avg_us - 200.0).abs() < 0.001);
        assert_eq!(lm.count, 3);
        assert_eq!(lm.min_us, 100);
        assert_eq!(lm.max_us, 300);
    }

    #[test]
    fn test_snapshot_returns_clone() {
        let mut lm = LatencyMetrics::new();
        lm.record(42);
        let snap = lm.snapshot();
        assert!((snap.avg_us - 42.0).abs() < 0.001);
        assert_eq!(snap.count, 1);
    }

    #[test]
    fn test_snapshot_independent_from_original() {
        let mut lm = LatencyMetrics::new();
        lm.record(10);
        let mut snap = lm.snapshot();
        snap.record(999);
        assert_eq!(lm.count, 1);
        assert_eq!(snap.count, 2);
    }
}
