use std::sync::atomic::{AtomicU8, Ordering};

use crate::core::result::VrmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V8PressureSignal {
    Low,
    Moderate,
    Critical,
}

impl V8PressureSignal {
    pub fn from_threshold(usage_ratio: f64) -> Self {
        if usage_ratio >= 0.90 {
            Self::Critical
        } else if usage_ratio >= 0.70 {
            Self::Moderate
        } else {
            Self::Low
        }
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, Self::Critical)
    }

    pub fn is_moderate(&self) -> bool {
        matches!(self, Self::Moderate)
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Moderate => 1,
            Self::Critical => 2,
        }
    }
}

pub struct V8SignalHandler {
    current: AtomicU8,
}

impl V8SignalHandler {
    pub fn new() -> Self {
        Self {
            current: AtomicU8::new(V8PressureSignal::Low as u8),
        }
    }

    pub fn handle_signal(&self, signal: V8PressureSignal) -> VrmResult<()> {
        self.current.store(signal as u8, Ordering::SeqCst);
        tracing::debug!("V8 pressure signal: {:?}", signal);
        Ok(())
    }

    pub fn current_pressure(&self) -> V8PressureSignal {
        match self.current.load(Ordering::SeqCst) {
            1 => V8PressureSignal::Moderate,
            2 => V8PressureSignal::Critical,
            _ => V8PressureSignal::Low,
        }
    }

    pub fn is_pressured(&self) -> bool {
        self.current_pressure() != V8PressureSignal::Low
    }

    pub fn is_critical(&self) -> bool {
        self.current_pressure() == V8PressureSignal::Critical
    }

    pub fn reset(&self) {
        self.current.store(V8PressureSignal::Low as u8, Ordering::SeqCst);
    }
}

impl Default for V8SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_from_threshold() {
        assert_eq!(V8PressureSignal::from_threshold(0.50), V8PressureSignal::Low);
        assert_eq!(V8PressureSignal::from_threshold(0.75), V8PressureSignal::Moderate);
        assert_eq!(V8PressureSignal::from_threshold(0.95), V8PressureSignal::Critical);
    }

    #[test]
    fn test_signal_properties() {
        assert!(V8PressureSignal::Critical.is_critical());
        assert!(!V8PressureSignal::Low.is_critical());
        assert!(V8PressureSignal::Moderate.is_moderate());
        assert!(!V8PressureSignal::Low.is_moderate());
    }

    #[test]
    fn test_signal_priority() {
        assert_eq!(V8PressureSignal::Low.priority(), 0);
        assert_eq!(V8PressureSignal::Moderate.priority(), 1);
        assert_eq!(V8PressureSignal::Critical.priority(), 2);
    }

    #[test]
    fn test_handler_default() {
        let handler = V8SignalHandler::new();
        assert_eq!(handler.current_pressure(), V8PressureSignal::Low);
        assert!(!handler.is_pressured());
        assert!(!handler.is_critical());
    }

    #[test]
    fn test_handle_signal() {
        let handler = V8SignalHandler::new();
        handler.handle_signal(V8PressureSignal::Moderate).unwrap();
        assert_eq!(handler.current_pressure(), V8PressureSignal::Moderate);
        assert!(handler.is_pressured());
        assert!(!handler.is_critical());
    }

    #[test]
    fn test_handle_critical() {
        let handler = V8SignalHandler::new();
        handler.handle_signal(V8PressureSignal::Critical).unwrap();
        assert_eq!(handler.current_pressure(), V8PressureSignal::Critical);
        assert!(handler.is_pressured());
        assert!(handler.is_critical());
    }

    #[test]
    fn test_reset() {
        let handler = V8SignalHandler::new();
        handler.handle_signal(V8PressureSignal::Critical).unwrap();
        assert!(handler.is_critical());
        handler.reset();
        assert_eq!(handler.current_pressure(), V8PressureSignal::Low);
    }
}
