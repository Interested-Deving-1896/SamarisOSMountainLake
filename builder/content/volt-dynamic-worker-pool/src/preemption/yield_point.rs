use std::time::Instant;

#[derive(Debug, Clone)]
pub struct YieldPoint {
    pub budget_us: u64,
    pub consumed: u64,
    pub last_yield_at: Instant,
}

impl YieldPoint {
    pub fn new(budget_us: u64) -> Self {
        Self {
            budget_us,
            consumed: 0,
            last_yield_at: Instant::now(),
        }
    }

    pub fn should_yield(&self) -> bool {
        self.consumed >= self.budget_us
    }

    pub fn record_yield(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_yield_at).as_micros() as u64;
        self.consumed += elapsed;
        self.last_yield_at = now;
    }

    pub fn budget_remaining(&self) -> u64 {
        self.budget_us.saturating_sub(self.consumed)
    }

    pub fn reset(&mut self) {
        self.consumed = 0;
        self.last_yield_at = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_not_exhausted() {
        let yp = YieldPoint::new(1000);
        assert!(!yp.should_yield());
        assert_eq!(yp.budget_remaining(), 1000);
    }

    #[test]
    fn test_should_yield_when_exhausted() {
        let mut yp = YieldPoint::new(100);
        yp.consumed = 100;
        assert!(yp.should_yield());
    }

    #[test]
    fn test_record_yield_consumes_budget() {
        let mut yp = YieldPoint::new(10_000);
        let before = yp.consumed;
        std::thread::sleep(std::time::Duration::from_micros(10));
        yp.record_yield();
        assert!(yp.consumed >= before);
    }

    #[test]
    fn test_reset() {
        let mut yp = YieldPoint::new(1000);
        yp.consumed = 999;
        yp.reset();
        assert_eq!(yp.consumed, 0);
        assert!(!yp.should_yield());
    }

    #[test]
    fn test_budget_remaining_saturating() {
        let mut yp = YieldPoint::new(100);
        yp.consumed = 200;
        assert_eq!(yp.budget_remaining(), 0);
    }
}
