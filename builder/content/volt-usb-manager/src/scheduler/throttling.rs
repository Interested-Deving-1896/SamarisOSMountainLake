#[derive(Debug, Clone)]
pub struct ThrottlingPolicy {
    pub enabled: bool,
    pub max_ios_per_sec: u64,
    pub max_bytes_per_sec: u64,
}

impl Default for ThrottlingPolicy {
    fn default() -> Self {
        ThrottlingPolicy {
            enabled: false,
            max_ios_per_sec: 1000,
            max_bytes_per_sec: 100 * 1024 * 1024,
        }
    }
}

impl ThrottlingPolicy {
    pub fn should_throttle(&self, current_iops: u64, current_bps: u64) -> bool {
        if !self.enabled {
            return false;
        }
        current_iops > self.max_ios_per_sec || current_bps > self.max_bytes_per_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_not_enabled() {
        let policy = ThrottlingPolicy::default();
        assert!(!policy.enabled);
    }

    #[test]
    fn test_disabled_never_throttles() {
        let policy = ThrottlingPolicy::default();
        assert!(!policy.should_throttle(999999, 999999));
    }

    #[test]
    fn test_throttle_on_iops() {
        let policy = ThrottlingPolicy {
            enabled: true,
            max_ios_per_sec: 100,
            max_bytes_per_sec: 999999,
        };
        assert!(!policy.should_throttle(50, 0));
        assert!(policy.should_throttle(150, 0));
    }

    #[test]
    fn test_throttle_on_bps() {
        let policy = ThrottlingPolicy {
            enabled: true,
            max_ios_per_sec: 999999,
            max_bytes_per_sec: 1024,
        };
        assert!(!policy.should_throttle(0, 500));
        assert!(policy.should_throttle(0, 2048));
    }

    #[test]
    fn test_throttle_on_both() {
        let policy = ThrottlingPolicy {
            enabled: true,
            max_ios_per_sec: 10,
            max_bytes_per_sec: 1000,
        };
        assert!(policy.should_throttle(20, 2000));
    }
}
