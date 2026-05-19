#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlushTrigger {
    Timer(u64),
    Threshold(u8),
    Manual,
    Shutdown,
    Eject,
}

#[derive(Debug, Clone)]
pub struct FlushPolicy {
    pub interval_ms: u64,
    pub threshold_pct: u8,
    pub batch_size_kb: u64,
    pub metadata_priority: bool,
}

impl Default for FlushPolicy {
    fn default() -> Self {
        Self {
            interval_ms: 5000,
            threshold_pct: 80,
            batch_size_kb: 64,
            metadata_priority: true,
        }
    }
}

impl FlushPolicy {
    pub fn should_flush(&self, usage_pct: f64, time_since_last_flush_ms: u64) -> bool {
        if usage_pct >= self.threshold_pct as f64 {
            return true;
        }
        if time_since_last_flush_ms >= self.interval_ms {
            return true;
        }
        false
    }

    pub fn trigger_reason(&self, usage_pct: f64, time_since_last_flush_ms: u64) -> Option<FlushTrigger> {
        if usage_pct >= self.threshold_pct as f64 {
            return Some(FlushTrigger::Threshold(self.threshold_pct));
        }
        if time_since_last_flush_ms >= self.interval_ms {
            return Some(FlushTrigger::Timer(self.interval_ms));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let p = FlushPolicy::default();
        assert_eq!(p.interval_ms, 5000);
        assert_eq!(p.threshold_pct, 80);
        assert_eq!(p.batch_size_kb, 64);
        assert!(p.metadata_priority);
    }

    #[test]
    fn test_should_flush_threshold() {
        let p = FlushPolicy::default();
        assert!(p.should_flush(85.0, 0));
        assert!(!p.should_flush(50.0, 0));
    }

    #[test]
    fn test_should_flush_timer() {
        let p = FlushPolicy::default();
        assert!(p.should_flush(0.0, 10_000));
        assert!(!p.should_flush(0.0, 1000));
    }

    #[test]
    fn test_should_flush_both() {
        let p = FlushPolicy::default();
        assert!(p.should_flush(90.0, 10_000));
    }

    #[test]
    fn test_should_flush_neither() {
        let p = FlushPolicy::default();
        assert!(!p.should_flush(10.0, 100));
    }

    #[test]
    fn test_trigger_reason_threshold() {
        let p = FlushPolicy::default();
        let r = p.trigger_reason(90.0, 0);
        assert_eq!(r, Some(FlushTrigger::Threshold(80)));
    }

    #[test]
    fn test_trigger_reason_timer() {
        let p = FlushPolicy::default();
        let r = p.trigger_reason(10.0, 10_000);
        assert_eq!(r, Some(FlushTrigger::Timer(5000)));
    }

    #[test]
    fn test_trigger_reason_none() {
        let p = FlushPolicy::default();
        let r = p.trigger_reason(10.0, 100);
        assert_eq!(r, None);
    }

    #[test]
    fn test_custom_policy() {
        let p = FlushPolicy {
            interval_ms: 1000,
            threshold_pct: 50,
            batch_size_kb: 128,
            metadata_priority: false,
        };
        assert!(p.should_flush(60.0, 0));
        assert!(p.should_flush(10.0, 2000));
        assert!(!p.should_flush(10.0, 500));
    }
}
