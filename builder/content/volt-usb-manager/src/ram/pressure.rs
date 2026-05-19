#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RamPressure {
    Normal,
    Warning,
    Critical,
}

impl RamPressure {
    pub fn from_usage(usage_pct: f64) -> Self {
        if usage_pct >= 90.0 {
            RamPressure::Critical
        } else if usage_pct >= 70.0 {
            RamPressure::Warning
        } else {
            RamPressure::Normal
        }
    }

    pub fn should_backoff(&self) -> bool {
        matches!(self, RamPressure::Warning | RamPressure::Critical)
    }

    pub fn should_evict(&self) -> bool {
        matches!(self, RamPressure::Critical)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_pressure() {
        let p = RamPressure::from_usage(30.0);
        assert_eq!(p, RamPressure::Normal);
        assert!(!p.should_backoff());
        assert!(!p.should_evict());
    }

    #[test]
    fn test_warning_pressure() {
        let p = RamPressure::from_usage(75.0);
        assert_eq!(p, RamPressure::Warning);
        assert!(p.should_backoff());
        assert!(!p.should_evict());
    }

    #[test]
    fn test_critical_pressure() {
        let p = RamPressure::from_usage(95.0);
        assert_eq!(p, RamPressure::Critical);
        assert!(p.should_backoff());
        assert!(p.should_evict());
    }

    #[test]
    fn test_boundary_exact_70() {
        let p = RamPressure::from_usage(70.0);
        assert_eq!(p, RamPressure::Warning);
    }

    #[test]
    fn test_boundary_exact_90() {
        let p = RamPressure::from_usage(90.0);
        assert_eq!(p, RamPressure::Critical);
    }

    #[test]
    fn test_full_usage() {
        let p = RamPressure::from_usage(100.0);
        assert_eq!(p, RamPressure::Critical);
    }
}
