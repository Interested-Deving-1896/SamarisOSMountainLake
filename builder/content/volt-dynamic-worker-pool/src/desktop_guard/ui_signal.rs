use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum UiSignal {
    NoPressure,
    LowPressure,
    MediumPressure,
    HighPressure,
    CriticalPressure,
}

impl UiSignal {
    pub fn as_u8(self) -> u8 {
        match self {
            Self::NoPressure => 0,
            Self::LowPressure => 1,
            Self::MediumPressure => 2,
            Self::HighPressure => 3,
            Self::CriticalPressure => 4,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::NoPressure,
            1 => Self::LowPressure,
            2 => Self::MediumPressure,
            3 => Self::HighPressure,
            4 => Self::CriticalPressure,
            _ => Self::NoPressure,
        }
    }
}

impl Default for UiSignal {
    fn default() -> Self {
        Self::NoPressure
    }
}

pub struct UISignalHandler {
    current_signal: Arc<AtomicU8>,
}

impl UISignalHandler {
    pub fn new() -> Self {
        Self {
            current_signal: Arc::new(AtomicU8::new(0)),
        }
    }

    pub fn set_signal(&self, signal: UiSignal) {
        self.current_signal.store(signal.as_u8(), Ordering::SeqCst);
    }

    pub fn current_signal(&self) -> UiSignal {
        UiSignal::from_u8(self.current_signal.load(Ordering::SeqCst))
    }

    pub fn is_pressure(&self) -> bool {
        self.current_signal() != UiSignal::NoPressure
    }

    pub fn should_reduce_background(&self) -> bool {
        matches!(
            self.current_signal(),
            UiSignal::HighPressure | UiSignal::CriticalPressure,
        )
    }
}

impl Default for UISignalHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_signal_default() {
        assert_eq!(UiSignal::default(), UiSignal::NoPressure);
    }

    #[test]
    fn test_ui_signal_as_from_u8() {
        let signals = [
            UiSignal::NoPressure,
            UiSignal::LowPressure,
            UiSignal::MediumPressure,
            UiSignal::HighPressure,
            UiSignal::CriticalPressure,
        ];
        for &s in &signals {
            assert_eq!(UiSignal::from_u8(s.as_u8()), s);
        }
    }

    #[test]
    fn test_signal_handler_new() {
        let handler = UISignalHandler::new();
        assert_eq!(handler.current_signal(), UiSignal::NoPressure);
        assert!(!handler.is_pressure());
        assert!(!handler.should_reduce_background());
    }

    #[test]
    fn test_signal_handler_set() {
        let handler = UISignalHandler::new();
        handler.set_signal(UiSignal::HighPressure);
        assert_eq!(handler.current_signal(), UiSignal::HighPressure);
        assert!(handler.is_pressure());
        assert!(handler.should_reduce_background());
    }

    #[test]
    fn test_signal_handler_low_pressure() {
        let handler = UISignalHandler::new();
        handler.set_signal(UiSignal::LowPressure);
        assert!(handler.is_pressure());
        assert!(!handler.should_reduce_background());
    }

    #[test]
    fn test_signal_handler_no_pressure() {
        let handler = UISignalHandler::new();
        handler.set_signal(UiSignal::NoPressure);
        assert!(!handler.is_pressure());
        assert!(!handler.should_reduce_background());
    }

    #[test]
    fn test_from_u8_invalid() {
        assert_eq!(UiSignal::from_u8(255), UiSignal::NoPressure);
    }
}
