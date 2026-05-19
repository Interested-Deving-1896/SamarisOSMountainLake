#[derive(Debug, Clone)]
pub struct FrameBudget {
    pub budget_ms: u64,
    pub used_ms: u64,
    pub total_time_ms: u64,
    pub miss_count: u64,
    pub frame_count: u64,
}

impl FrameBudget {
    pub fn new(budget_ms: u64) -> Self {
        FrameBudget {
            budget_ms,
            used_ms: 0,
            total_time_ms: 0,
            miss_count: 0,
            frame_count: 0,
        }
    }

    pub fn start_frame(&mut self) {
        self.used_ms = 0;
    }

    pub fn end_frame(&mut self, elapsed_ms: u64) {
        self.used_ms = elapsed_ms;
        self.total_time_ms += elapsed_ms;
        self.frame_count += 1;
        if elapsed_ms > self.budget_ms {
            self.miss_count += 1;
        }
    }

    pub fn is_within_budget(&self) -> bool {
        if self.frame_count == 0 {
            return true;
        }
        self.used_ms <= self.budget_ms
    }

    pub fn average_frame_time(&self) -> f64 {
        if self.frame_count == 0 {
            return 0.0;
        }
        self.total_time_ms as f64 / self.frame_count as f64
    }

    pub fn miss_rate(&self) -> f64 {
        if self.frame_count == 0 {
            return 0.0;
        }
        self.miss_count as f64 / self.frame_count as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_budget() {
        let b = FrameBudget::new(16);
        assert_eq!(b.budget_ms, 16);
        assert_eq!(b.used_ms, 0);
        assert_eq!(b.miss_count, 0);
        assert_eq!(b.frame_count, 0);
    }

    #[test]
    fn test_start_frame_resets_used() {
        let mut b = FrameBudget::new(16);
        b.end_frame(10);
        b.start_frame();
        assert_eq!(b.used_ms, 0);
    }

    #[test]
    fn test_end_frame_within_budget() {
        let mut b = FrameBudget::new(16);
        b.end_frame(10);
        assert!(b.is_within_budget());
        assert_eq!(b.frame_count, 1);
        assert_eq!(b.miss_count, 0);
    }

    #[test]
    fn test_end_frame_exceeds_budget() {
        let mut b = FrameBudget::new(16);
        b.end_frame(20);
        assert!(!b.is_within_budget());
        assert_eq!(b.miss_count, 1);
    }

    #[test]
    fn test_average_frame_time() {
        let mut b = FrameBudget::new(16);
        b.end_frame(10);
        b.start_frame();
        b.end_frame(20);
        assert!((b.average_frame_time() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_miss_rate() {
        let mut b = FrameBudget::new(16);
        b.end_frame(10);
        b.start_frame();
        b.end_frame(20);
        b.start_frame();
        b.end_frame(30);
        assert!((b.miss_rate() - 2.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_miss_rate_zero_frames() {
        let b = FrameBudget::new(16);
        assert_eq!(b.miss_rate(), 0.0);
    }

    #[test]
    fn test_within_budget_zero_frames() {
        let b = FrameBudget::new(16);
        assert!(b.is_within_budget());
    }
}
