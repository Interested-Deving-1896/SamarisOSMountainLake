use super::latency_guard::LatencyGuard;
use super::ui_signal::{UiSignal, UISignalHandler};

pub struct DesktopProtection {
    pub enabled: bool,
    pub guard: LatencyGuard,
    pub signal: UISignalHandler,
    pub frame_budget_ms: u64,
}

impl DesktopProtection {
    pub fn new(
        frame_budget_ms: u64,
        latency_guard_ms: u64,
        desktop_min_workers: u32,
        enabled: bool,
    ) -> Self {
        Self {
            enabled,
            guard: LatencyGuard::new(frame_budget_ms, latency_guard_ms, desktop_min_workers),
            signal: UISignalHandler::new(),
            frame_budget_ms,
        }
    }

    pub fn should_protect(&self) -> bool {
        self.guard.should_protect() || self.signal.is_pressure()
    }

    pub fn reduce_background(&self) -> bool {
        self.enabled && self.should_protect()
    }

    pub fn block_orbit_burst(&self) -> bool {
        self.signal.current_signal() == UiSignal::CriticalPressure
    }

    pub fn set_pressure_from_frame_time(&self, time_ms: u64) {
        self.guard.record_frame_time(time_ms);

        let signal = if self.guard.is_exceeding_budget() {
            if time_ms > self.frame_budget_ms * 2 {
                UiSignal::CriticalPressure
            } else {
                UiSignal::HighPressure
            }
        } else if self.guard.should_protect() {
            UiSignal::MediumPressure
        } else if time_ms > 0 {
            UiSignal::LowPressure
        } else {
            UiSignal::NoPressure
        };

        self.signal.set_signal(signal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_desktop_protection() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        assert!(dp.enabled);
        assert_eq!(dp.frame_budget_ms, 16);
        assert!(!dp.should_protect());
        assert!(!dp.reduce_background());
        assert!(!dp.block_orbit_burst());
    }

    #[test]
    fn test_protection_disabled() {
        let dp = DesktopProtection::new(16, 4, 2, false);
        dp.guard.record_frame_time(100);
        assert!(dp.should_protect());
        assert!(!dp.reduce_background());
    }

    #[test]
    fn test_set_pressure_from_frame_time_no_pressure() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        dp.set_pressure_from_frame_time(0);
        assert_eq!(dp.signal.current_signal(), UiSignal::NoPressure);
    }

    #[test]
    fn test_set_pressure_from_frame_time_low() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        dp.set_pressure_from_frame_time(5);
        assert_eq!(dp.signal.current_signal(), UiSignal::LowPressure);
    }

    #[test]
    fn test_set_pressure_from_frame_time_high() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        dp.set_pressure_from_frame_time(20);
        assert_eq!(dp.signal.current_signal(), UiSignal::HighPressure);
    }

    #[test]
    fn test_set_pressure_from_frame_time_critical() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        dp.set_pressure_from_frame_time(40);
        assert_eq!(dp.signal.current_signal(), UiSignal::CriticalPressure);
    }

    #[test]
    fn test_block_orbit_burst() {
        let dp = DesktopProtection::new(16, 4, 2, true);
        assert!(!dp.block_orbit_burst());
        dp.set_pressure_from_frame_time(40);
        assert!(dp.block_orbit_burst());
    }
}
