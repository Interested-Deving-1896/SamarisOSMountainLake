use std::time::Instant;

pub struct ThermalBackoff {
    pub last_throttle: Option<Instant>,
    pub throttle_count: u64,
}

impl ThermalBackoff {
    pub fn new() -> Self {
        ThermalBackoff {
            last_throttle: None,
            throttle_count: 0,
        }
    }

    pub fn record_throttle(&mut self) {
        self.last_throttle = Some(Instant::now());
        self.throttle_count += 1;
    }

    pub fn time_since_last_throttle_ms(&self) -> u64 {
        match self.last_throttle {
            Some(instant) => instant.elapsed().as_millis() as u64,
            None => u64::MAX,
        }
    }

    pub fn throttle_count(&self) -> u64 {
        self.throttle_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_new_backoff() {
        let b = ThermalBackoff::new();
        assert!(b.last_throttle.is_none());
        assert_eq!(b.throttle_count(), 0);
        assert_eq!(b.time_since_last_throttle_ms(), u64::MAX);
    }

    #[test]
    fn test_record_throttle() {
        let mut b = ThermalBackoff::new();
        b.record_throttle();
        assert!(b.last_throttle.is_some());
        assert_eq!(b.throttle_count(), 1);
    }

    #[test]
    fn test_time_since_last_throttle() {
        let mut b = ThermalBackoff::new();
        b.record_throttle();
        let t = b.time_since_last_throttle_ms();
        assert!(t < 100);
    }

    #[test]
    fn test_multiple_records() {
        let mut b = ThermalBackoff::new();
        b.record_throttle();
        b.record_throttle();
        b.record_throttle();
        assert_eq!(b.throttle_count(), 3);
    }

    #[test]
    fn test_time_increases() {
        let mut b = ThermalBackoff::new();
        b.record_throttle();
        let t1 = b.time_since_last_throttle_ms();
        thread::sleep(Duration::from_millis(5));
        let t2 = b.time_since_last_throttle_ms();
        assert!(t2 >= t1);
    }
}
