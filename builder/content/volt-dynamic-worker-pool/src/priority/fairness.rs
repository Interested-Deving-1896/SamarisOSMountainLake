use crate::priority::aging::AgingPolicy;
use crate::priority::level::PriorityLevel;

pub struct FairnessPolicy {
    pub aging: AgingPolicy,
    pub starvation_count: u64,
    pub boost_count: u64,
    pub max_wait_time_ms: u64,
}

impl FairnessPolicy {
    pub fn new() -> Self {
        Self {
            aging: AgingPolicy::new(),
            starvation_count: 0,
            boost_count: 0,
            max_wait_time_ms: 30_000,
        }
    }

    pub fn check_fairness(&mut self, wait_ms: u64) -> Option<PriorityLevel> {
        if self.aging.is_starved(wait_ms) {
            self.starvation_count += 1;
        }
        let boost = self.aging.compute_boost(wait_ms);
        if boost.is_some() {
            self.boost_count += 1;
        }
        boost
    }

    pub fn record_boost(&mut self) {
        self.boost_count += 1;
    }

    pub fn record_starvation(&mut self) {
        self.starvation_count += 1;
    }

    pub fn snapshot(&self) -> FairnessSnapshot {
        FairnessSnapshot {
            starvation_count: self.starvation_count,
            boost_count: self.boost_count,
            max_wait_time_ms: self.max_wait_time_ms,
            aging_enabled: self.aging.enabled,
        }
    }
}

impl Default for FairnessPolicy {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FairnessSnapshot {
    pub starvation_count: u64,
    pub boost_count: u64,
    pub max_wait_time_ms: u64,
    pub aging_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_policy() {
        let policy = FairnessPolicy::new();
        assert_eq!(policy.starvation_count, 0);
        assert_eq!(policy.boost_count, 0);
    }

    #[test]
    fn test_check_fairness_no_boost() {
        let mut policy = FairnessPolicy::new();
        assert_eq!(policy.check_fairness(1_000), None);
        assert_eq!(policy.starvation_count, 0);
        assert_eq!(policy.boost_count, 0);
    }

    #[test]
    fn test_check_fairness_triggers_boost() {
        let mut policy = FairnessPolicy::new();
        assert_eq!(
            policy.check_fairness(30_000),
            Some(PriorityLevel::High)
        );
        assert_eq!(policy.starvation_count, 1);
        assert_eq!(policy.boost_count, 1);
    }

    #[test]
    fn test_record_boost_and_starvation() {
        let mut policy = FairnessPolicy::new();
        policy.record_boost();
        policy.record_boost();
        policy.record_boost();
        assert_eq!(policy.boost_count, 3);
        policy.record_starvation();
        assert_eq!(policy.starvation_count, 1);
    }

    #[test]
    fn test_snapshot() {
        let mut policy = FairnessPolicy::new();
        policy.record_starvation();
        policy.record_boost();
        let snap = policy.snapshot();
        assert_eq!(snap.starvation_count, 1);
        assert_eq!(snap.boost_count, 1);
        assert!(snap.aging_enabled);
    }

    #[test]
    fn test_default() {
        let policy = FairnessPolicy::default();
        assert_eq!(policy.max_wait_time_ms, 30_000);
    }
}
