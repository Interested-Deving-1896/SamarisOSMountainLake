#[derive(Clone, Debug)]
pub struct OrbitBudgetSnapshot {
    pub default_fraction: f64,
    pub burst_enabled: bool,
    pub current_fraction: f64,
}

pub struct OrbitBudget {
    pub default_fraction: f64,
    pub burst_enabled: bool,
    pub current_fraction: f64,
}

impl OrbitBudget {
    pub fn new(default_fraction: f64) -> Self {
        Self {
            default_fraction,
            burst_enabled: false,
            current_fraction: default_fraction,
        }
    }

    pub fn enable_burst(&mut self, burst_fraction: f64) {
        self.burst_enabled = true;
        self.current_fraction = burst_fraction;
    }

    pub fn disable_burst(&mut self) {
        self.burst_enabled = false;
        self.current_fraction = self.default_fraction;
    }

    pub fn max_workers(&self, total: u32) -> u32 {
        (total as f64 * self.current_fraction) as u32
    }

    pub fn snapshot(&self) -> OrbitBudgetSnapshot {
        OrbitBudgetSnapshot {
            default_fraction: self.default_fraction,
            burst_enabled: self.burst_enabled,
            current_fraction: self.current_fraction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let b = OrbitBudget::new(0.3);
        assert!((b.default_fraction - 0.3).abs() < 1e-9);
        assert!(!b.burst_enabled);
        assert!((b.current_fraction - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_enable_burst() {
        let mut b = OrbitBudget::new(0.3);
        b.enable_burst(0.6);
        assert!(b.burst_enabled);
        assert!((b.current_fraction - 0.6).abs() < 1e-9);
    }

    #[test]
    fn test_disable_burst() {
        let mut b = OrbitBudget::new(0.3);
        b.enable_burst(0.6);
        b.disable_burst();
        assert!(!b.burst_enabled);
        assert!((b.current_fraction - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_max_workers() {
        let b = OrbitBudget::new(0.25);
        assert_eq!(b.max_workers(100), 25);
        assert_eq!(b.max_workers(10), 2);
    }

    #[test]
    fn test_max_workers_with_burst() {
        let mut b = OrbitBudget::new(0.25);
        b.enable_burst(0.5);
        assert_eq!(b.max_workers(100), 50);
    }

    #[test]
    fn test_snapshot() {
        let mut b = OrbitBudget::new(0.3);
        b.enable_burst(0.6);
        let s = b.snapshot();
        assert!((s.default_fraction - 0.3).abs() < 1e-9);
        assert!(s.burst_enabled);
        assert!((s.current_fraction - 0.6).abs() < 1e-9);
    }
}
