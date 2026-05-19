pub struct OrbitPolicy {
    pub default_fraction: f64,
    pub burst_window_ms: u64,
    pub burst_cooldown_ms: u64,
    pub max_consecutive_bursts: u32,
}

impl OrbitPolicy {
    pub fn new(default_fraction: f64, burst_window_ms: u64, burst_cooldown_ms: u64, max_consecutive_bursts: u32) -> Self {
        Self {
            default_fraction: default_fraction.clamp(0.0, 1.0),
            burst_window_ms,
            burst_cooldown_ms,
            max_consecutive_bursts,
        }
    }

    pub fn max_workers_for_orbit(&self, max_workers: u32) -> u32 {
        ((max_workers as f64) * self.default_fraction).ceil() as u32
    }

    pub fn burst_max_workers(&self, max_workers: u32) -> u32 {
        max_workers
    }

    pub fn can_burst(&self, current_consecutive: u32, desktop_pressure: f64, thermal: f64) -> bool {
        if current_consecutive >= self.max_consecutive_bursts {
            return false;
        }
        if desktop_pressure > 0.8 {
            return false;
        }
        if thermal > 0.85 {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_workers_for_orbit() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 3);
        assert_eq!(policy.max_workers_for_orbit(10), 5);
        assert_eq!(policy.max_workers_for_orbit(1), 1);
        assert_eq!(policy.max_workers_for_orbit(0), 0);
    }

    #[test]
    fn test_burst_max_workers() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 3);
        assert_eq!(policy.burst_max_workers(10), 10);
    }

    #[test]
    fn test_can_burst_below_consecutive_limit() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 3);
        assert!(policy.can_burst(0, 0.5, 0.5));
        assert!(policy.can_burst(2, 0.5, 0.5));
    }

    #[test]
    fn test_can_burst_at_consecutive_limit() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 3);
        assert!(!policy.can_burst(3, 0.5, 0.5));
        assert!(!policy.can_burst(5, 0.5, 0.5));
    }

    #[test]
    fn test_can_burst_rejects_high_pressure() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 5);
        assert!(!policy.can_burst(0, 0.9, 0.5));
    }

    #[test]
    fn test_can_burst_rejects_high_thermal() {
        let policy = OrbitPolicy::new(0.5, 1000, 500, 5);
        assert!(!policy.can_burst(0, 0.5, 0.9));
    }

    #[test]
    fn test_default_fraction_clamped() {
        let policy = OrbitPolicy::new(1.5, 1000, 500, 3);
        assert_eq!(policy.max_workers_for_orbit(10), 10);
        let policy = OrbitPolicy::new(-0.5, 1000, 500, 3);
        assert_eq!(policy.max_workers_for_orbit(10), 0);
    }

    #[test]
    fn test_max_workers_rounding() {
        let policy = OrbitPolicy::new(0.33, 1000, 500, 3);
        assert_eq!(policy.max_workers_for_orbit(10), 4);
    }
}
