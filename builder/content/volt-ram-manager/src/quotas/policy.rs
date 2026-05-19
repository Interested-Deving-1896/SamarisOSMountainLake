use crate::apps::app_profile::AppPriority;
use crate::quotas::app_quota::AppQuota;

#[derive(Debug, Clone)]
pub struct QuotaPolicy {
    pub strict: bool,
    pub warn_on_approach: bool,
    pub auto_compression: bool,
}

impl Default for QuotaPolicy {
    fn default() -> Self {
        Self {
            strict: true,
            warn_on_approach: true,
            auto_compression: true,
        }
    }
}

impl QuotaPolicy {
    pub fn for_priority(priority: AppPriority) -> Self {
        match priority {
            AppPriority::Critical => Self {
                strict: false,
                warn_on_approach: false,
                auto_compression: false,
            },
            AppPriority::High => Self {
                strict: true,
                warn_on_approach: true,
                auto_compression: true,
            },
            AppPriority::Normal => Self {
                strict: true,
                warn_on_approach: true,
                auto_compression: true,
            },
            AppPriority::Low => Self {
                strict: true,
                warn_on_approach: false,
                auto_compression: true,
            },
            AppPriority::Idle => Self {
                strict: true,
                warn_on_approach: false,
                auto_compression: true,
            },
        }
    }

    pub fn is_near_limit(&self, quota: &AppQuota) -> bool {
        if !self.warn_on_approach {
            return false;
        }
        quota.usage_percent() >= 80.0
    }

    pub fn should_auto_compress(&self, quota: &AppQuota) -> bool {
        if !self.auto_compression {
            return false;
        }
        quota.usage_percent() >= 75.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let p = QuotaPolicy::default();
        assert!(p.strict);
        assert!(p.warn_on_approach);
        assert!(p.auto_compression);
    }

    #[test]
    fn test_critical_policy() {
        let p = QuotaPolicy::for_priority(AppPriority::Critical);
        assert!(!p.strict);
        assert!(!p.warn_on_approach);
        assert!(!p.auto_compression);
    }

    #[test]
    fn test_idle_policy() {
        let p = QuotaPolicy::for_priority(AppPriority::Idle);
        assert!(p.strict);
        assert!(!p.warn_on_approach);
    }

    #[test]
    fn test_is_near_limit() {
        let policy = QuotaPolicy::for_priority(AppPriority::Normal);
        let mut q = AppQuota::new(100);
        q.current_usage = 90 * 1024 * 1024;
        assert!(policy.is_near_limit(&q));
    }

    #[test]
    fn test_should_auto_compress() {
        let policy = QuotaPolicy::for_priority(AppPriority::Normal);
        let mut q = AppQuota::new(100);
        q.current_usage = 80 * 1024 * 1024;
        assert!(policy.should_auto_compress(&q));
    }
}
