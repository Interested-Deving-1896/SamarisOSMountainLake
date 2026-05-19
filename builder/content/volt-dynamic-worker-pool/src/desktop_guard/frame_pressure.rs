use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum FramePressure {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl FramePressure {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::None => 0.0,
            Self::Low => 0.25,
            Self::Medium => 0.50,
            Self::High => 0.75,
            Self::Critical => 1.0,
        }
    }

    pub fn from_f64(value: f64) -> Self {
        let clamped = value.clamp(0.0, 1.0);
        if clamped <= 0.1 {
            Self::None
        } else if clamped <= 0.3 {
            Self::Low
        } else if clamped <= 0.6 {
            Self::Medium
        } else if clamped <= 0.85 {
            Self::High
        } else {
            Self::Critical
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn is_throttled(&self) -> bool {
        matches!(self, Self::High | Self::Critical)
    }
}

impl Default for FramePressure {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub struct DesktopGuard {
    pressure: AtomicU64,
    throttle_enabled: bool,
    throttle_threshold: FramePressure,
    yield_multiplier: f64,
}

impl DesktopGuard {
    pub fn new(throttle_enabled: bool, throttle_threshold: FramePressure, yield_multiplier: f64) -> Self {
        Self {
            pressure: AtomicU64::new(0),
            throttle_enabled,
            throttle_threshold,
            yield_multiplier,
        }
    }

    pub fn set_pressure(&self, pressure: FramePressure) {
        let bits = (pressure.as_f64() * u64::MAX as f64) as u64;
        self.pressure.store(bits, Ordering::SeqCst);
    }

    pub fn current_pressure(&self) -> FramePressure {
        let bits = self.pressure.load(Ordering::SeqCst);
        let value = bits as f64 / u64::MAX as f64;
        FramePressure::from_f64(value)
    }

    pub fn should_throttle(&self) -> bool {
        if !self.throttle_enabled {
            return false;
        }
        let current = self.current_pressure();
        current >= self.throttle_threshold
    }

    pub fn yield_multiplier(&self) -> f64 {
        if self.should_throttle() {
            self.yield_multiplier
        } else {
            1.0
        }
    }

    pub fn throttle_threshold(&self) -> FramePressure {
        self.throttle_threshold
    }

    pub fn is_throttle_enabled(&self) -> bool {
        self.throttle_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_pressure_ordering() {
        assert!(FramePressure::None < FramePressure::Low);
        assert!(FramePressure::Low < FramePressure::Medium);
        assert!(FramePressure::Medium < FramePressure::High);
        assert!(FramePressure::High < FramePressure::Critical);
    }

    #[test]
    fn test_frame_pressure_from_f64() {
        assert_eq!(FramePressure::from_f64(0.0), FramePressure::None);
        assert_eq!(FramePressure::from_f64(0.2), FramePressure::Low);
        assert_eq!(FramePressure::from_f64(0.5), FramePressure::Medium);
        assert_eq!(FramePressure::from_f64(0.7), FramePressure::High);
        assert_eq!(FramePressure::from_f64(0.9), FramePressure::Critical);
    }

    #[test]
    fn test_desktop_guard_throttling() {
        let guard = DesktopGuard::new(true, FramePressure::High, 2.0);
        assert!(!guard.should_throttle());
        guard.set_pressure(FramePressure::High);
        assert!(guard.should_throttle());
        assert!((guard.yield_multiplier() - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_desktop_guard_disabled() {
        let guard = DesktopGuard::new(false, FramePressure::Low, 1.0);
        guard.set_pressure(FramePressure::Critical);
        assert!(!guard.should_throttle());
    }
}
