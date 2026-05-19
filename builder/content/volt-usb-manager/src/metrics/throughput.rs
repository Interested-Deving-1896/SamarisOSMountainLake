use std::time::Instant;

pub struct ThroughputMetrics {
    bytes: u64,
    start_time: Instant,
}

impl ThroughputMetrics {
    pub fn new() -> Self {
        ThroughputMetrics {
            bytes: 0,
            start_time: Instant::now(),
        }
    }

    pub fn record_bytes(&mut self, n: u64) {
        self.bytes += n;
    }

    pub fn bytes_per_sec(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        self.bytes as f64 / elapsed
    }

    pub fn reset(&mut self) {
        self.bytes = 0;
        self.start_time = Instant::now();
    }
}

impl Default for ThroughputMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_throughput_zero() {
        let tm = ThroughputMetrics::new();
        assert!((tm.bytes_per_sec() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_record_bytes_increases_count() {
        let mut tm = ThroughputMetrics::new();
        tm.record_bytes(1024);
        assert_eq!(tm.bytes, 1024);
    }

    #[test]
    fn test_bytes_per_sec_after_delay() {
        let mut tm = ThroughputMetrics::new();
        tm.record_bytes(1000);
        thread::sleep(Duration::from_millis(100));
        let bps = tm.bytes_per_sec();
        assert!(bps > 0.0);
        assert!(bps < 100_000.0);
    }

    #[test]
    fn test_reset_clears_bytes_and_timer() {
        let mut tm = ThroughputMetrics::new();
        tm.record_bytes(5000);
        thread::sleep(Duration::from_millis(10));
        tm.reset();
        assert_eq!(tm.bytes, 0);
        assert!((tm.bytes_per_sec() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_multiple_record_bytes() {
        let mut tm = ThroughputMetrics::new();
        tm.record_bytes(100);
        tm.record_bytes(200);
        tm.record_bytes(300);
        assert_eq!(tm.bytes, 600);
    }
}
