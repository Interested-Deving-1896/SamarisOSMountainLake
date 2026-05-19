use crate::scheduler::priority::IoPriority;

pub struct FairnessPolicy {
    pub window_ms: u64,
    pub max_per_window: usize,
}

impl Default for FairnessPolicy {
    fn default() -> Self {
        FairnessPolicy {
            window_ms: 100,
            max_per_window: 32,
        }
    }
}

impl FairnessPolicy {
    pub fn can_schedule(&self, priority: IoPriority, used: usize) -> bool {
        let limit = match priority {
            IoPriority::CriticalMetadata => self.max_per_window,
            IoPriority::Desktop => self.max_per_window / 2,
            IoPriority::UserVisible => self.max_per_window / 3,
            IoPriority::Background => self.max_per_window / 4,
            IoPriority::Cache => self.max_per_window / 5,
        };
        let limit = if limit == 0 { 1 } else { limit };
        used < limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_critical_gets_full_slot() {
        let policy = FairnessPolicy::default();
        assert!(policy.can_schedule(IoPriority::CriticalMetadata, 0));
        assert!(policy.can_schedule(IoPriority::CriticalMetadata, 31));
        assert!(!policy.can_schedule(IoPriority::CriticalMetadata, 32));
    }

    #[test]
    fn test_desktop_gets_half() {
        let policy = FairnessPolicy::default();
        assert!(policy.can_schedule(IoPriority::Desktop, 0));
        assert!(policy.can_schedule(IoPriority::Desktop, 15));
        assert!(!policy.can_schedule(IoPriority::Desktop, 16));
    }

    #[test]
    fn test_background_limited() {
        let policy = FairnessPolicy::default();
        assert!(policy.can_schedule(IoPriority::Background, 0));
        assert!(policy.can_schedule(IoPriority::Background, 7));
        assert!(!policy.can_schedule(IoPriority::Background, 8));
    }

    #[test]
    fn test_cache_most_limited() {
        let policy = FairnessPolicy::default();
        assert!(policy.can_schedule(IoPriority::Cache, 0));
        assert!(policy.can_schedule(IoPriority::Cache, 5));
        assert!(!policy.can_schedule(IoPriority::Cache, 6));
    }

    #[test]
    fn test_default_values() {
        let policy = FairnessPolicy::default();
        assert_eq!(policy.window_ms, 100);
        assert_eq!(policy.max_per_window, 32);
    }
}
