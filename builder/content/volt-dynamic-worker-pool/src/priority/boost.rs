use std::time::{Duration, Instant};

use crate::priority::level::PriorityLevel;

pub struct PriorityBoost {
    pub original: PriorityLevel,
    pub boosted: PriorityLevel,
    pub expires_at: Instant,
    pub reason: String,
}

impl PriorityBoost {
    pub fn new(
        original: PriorityLevel,
        boosted: PriorityLevel,
        duration_ms: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            original,
            boosted,
            expires_at: Instant::now() + Duration::from_millis(duration_ms),
            reason: reason.into(),
        }
    }

    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }

    pub fn revert(&self) -> PriorityLevel {
        self.original
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_boost() {
        let boost = PriorityBoost::new(
            PriorityLevel::Normal,
            PriorityLevel::High,
            5_000,
            "starvation recovery",
        );
        assert_eq!(boost.original, PriorityLevel::Normal);
        assert_eq!(boost.boosted, PriorityLevel::High);
        assert_eq!(boost.reason, "starvation recovery");
    }

    #[test]
    fn test_revert() {
        let boost = PriorityBoost::new(
            PriorityLevel::Normal,
            PriorityLevel::High,
            5_000,
            "test",
        );
        assert_eq!(boost.revert(), PriorityLevel::Normal);
    }

    #[test]
    fn test_is_expired_returns_false_initially() {
        let boost = PriorityBoost::new(
            PriorityLevel::Low,
            PriorityLevel::Normal,
            60_000,
            "test",
        );
        assert!(!boost.is_expired());
    }

    #[test]
    fn test_zero_duration_expires() {
        let boost = PriorityBoost::new(
            PriorityLevel::Low,
            PriorityLevel::Normal,
            0,
            "immediate",
        );
        assert!(boost.is_expired());
    }
}
