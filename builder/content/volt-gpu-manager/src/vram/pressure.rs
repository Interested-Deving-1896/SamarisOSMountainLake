#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VramPressure {
    Normal,
    Warning,
    Critical,
    OutOfMemory,
}

impl VramPressure {
    pub fn from_usage(used_pct: f64, available_mb: u64) -> Self {
        if available_mb == 0 {
            return VramPressure::OutOfMemory;
        }
        if used_pct >= 100.0 {
            VramPressure::OutOfMemory
        } else if used_pct >= 90.0 {
            VramPressure::Critical
        } else if used_pct >= 75.0 {
            VramPressure::Warning
        } else {
            VramPressure::Normal
        }
    }

    pub fn should_compress(&self) -> bool {
        matches!(self, VramPressure::Warning | VramPressure::Critical)
    }

    pub fn should_evict(&self) -> bool {
        matches!(self, VramPressure::Critical)
    }

    pub fn should_fallback(&self) -> bool {
        matches!(self, VramPressure::OutOfMemory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_pressure() {
        let p = VramPressure::from_usage(50.0, 1024);
        assert_eq!(p, VramPressure::Normal);
        assert!(!p.should_compress());
        assert!(!p.should_evict());
        assert!(!p.should_fallback());
    }

    #[test]
    fn test_warning_pressure() {
        let p = VramPressure::from_usage(80.0, 1024);
        assert_eq!(p, VramPressure::Warning);
        assert!(p.should_compress());
        assert!(!p.should_evict());
        assert!(!p.should_fallback());
    }

    #[test]
    fn test_critical_pressure() {
        let p = VramPressure::from_usage(95.0, 512);
        assert_eq!(p, VramPressure::Critical);
        assert!(p.should_compress());
        assert!(p.should_evict());
        assert!(!p.should_fallback());
    }

    #[test]
    fn test_out_of_memory() {
        let p = VramPressure::from_usage(100.0, 0);
        assert_eq!(p, VramPressure::OutOfMemory);
        assert!(!p.should_compress());
        assert!(!p.should_evict());
        assert!(p.should_fallback());
    }

    #[test]
    fn test_exact_boundaries() {
        assert_eq!(VramPressure::from_usage(74.999, 1024), VramPressure::Normal);
        assert_eq!(VramPressure::from_usage(75.0, 1024), VramPressure::Warning);
        assert_eq!(VramPressure::from_usage(90.0, 1024), VramPressure::Critical);
        assert_eq!(VramPressure::from_usage(100.0, 1), VramPressure::OutOfMemory);
    }

    #[test]
    fn test_zero_available() {
        assert_eq!(VramPressure::from_usage(0.0, 0), VramPressure::OutOfMemory);
    }

    #[test]
    fn test_clone_and_eq() {
        let a = VramPressure::Warning;
        let b = a;
        assert_eq!(a, b);
    }
}
