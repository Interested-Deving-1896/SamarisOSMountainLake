use crate::priority::level::PriorityLevel;

pub struct AgingPolicy {
    pub enabled: bool,
    pub aging_after_ms: u64,
    pub starvation_limit_ms: u64,
    pub priority_boost_on_starvation: bool,
}

impl AgingPolicy {
    pub fn new() -> Self {
        Self {
            enabled: true,
            aging_after_ms: 5_000,
            starvation_limit_ms: 30_000,
            priority_boost_on_starvation: true,
        }
    }

    pub fn is_starved(&self, wait_ms: u64) -> bool {
        wait_ms >= self.starvation_limit_ms
    }

    pub fn compute_boost(&self, wait_ms: u64) -> Option<PriorityLevel> {
        if !self.enabled || !self.priority_boost_on_starvation {
            return None;
        }
        if wait_ms < self.aging_after_ms {
            return None;
        }
        if wait_ms >= self.starvation_limit_ms {
            return Some(PriorityLevel::High);
        }
        None
    }
}

impl Default for AgingPolicy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_starved() {
        let policy = AgingPolicy::new();
        assert!(!policy.is_starved(10_000));
    }

    #[test]
    fn test_starved() {
        let policy = AgingPolicy::new();
        assert!(policy.is_starved(30_000));
    }

    #[test]
    fn test_no_boost_below_aging_threshold() {
        let policy = AgingPolicy::new();
        assert_eq!(policy.compute_boost(1_000), None);
    }

    #[test]
    fn test_boost_at_starvation_limit() {
        let policy = AgingPolicy::new();
        assert_eq!(policy.compute_boost(30_000), Some(PriorityLevel::High));
    }

    #[test]
    fn test_no_boost_when_disabled() {
        let mut policy = AgingPolicy::new();
        policy.enabled = false;
        assert_eq!(policy.compute_boost(30_000), None);
    }

    #[test]
    fn test_no_boost_when_boost_disabled() {
        let mut policy = AgingPolicy::new();
        policy.priority_boost_on_starvation = false;
        assert_eq!(policy.compute_boost(30_000), None);
    }

    #[test]
    fn test_aging_without_starvation() {
        let policy = AgingPolicy::new();
        assert_eq!(policy.compute_boost(15_000), None);
    }

    #[test]
    fn test_default() {
        let policy = AgingPolicy::default();
        assert!(policy.enabled);
        assert_eq!(policy.aging_after_ms, 5_000);
        assert_eq!(policy.starvation_limit_ms, 30_000);
        assert!(policy.priority_boost_on_starvation);
    }
}
