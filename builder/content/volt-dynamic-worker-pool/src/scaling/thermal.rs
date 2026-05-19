#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThermalState {
    Normal,
    Warm,
    Hot,
    Critical,
}

impl ThermalState {
    pub fn is_throttled(&self) -> bool {
        matches!(self, ThermalState::Hot | ThermalState::Critical)
    }

    pub fn is_safe(&self) -> bool {
        matches!(self, ThermalState::Normal | ThermalState::Warm)
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            ThermalState::Normal => 0.0,
            ThermalState::Warm => 0.33,
            ThermalState::Hot => 0.66,
            ThermalState::Critical => 1.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThermalState::Normal => "normal",
            ThermalState::Warm => "warm",
            ThermalState::Hot => "hot",
            ThermalState::Critical => "critical",
        }
    }
}

impl Default for ThermalState {
    fn default() -> Self {
        ThermalState::Normal
    }
}

pub struct ThermalMonitor {
    backoff_enabled: bool,
    state: ThermalState,
}

impl ThermalMonitor {
    pub fn new(backoff_enabled: bool) -> Self {
        ThermalMonitor {
            backoff_enabled,
            state: ThermalState::Normal,
        }
    }

    pub fn state(&self) -> ThermalState {
        self.state
    }

    pub fn backoff_enabled(&self) -> bool {
        self.backoff_enabled
    }

    pub fn set_state(&mut self, state: ThermalState) {
        self.state = state;
    }

    pub fn set_backoff_enabled(&mut self, enabled: bool) {
        self.backoff_enabled = enabled;
    }

    pub fn should_backoff(&self) -> bool {
        self.backoff_enabled && self.state.is_throttled()
    }

    pub fn sample(&mut self) -> ThermalState {
        self.state = self.read_sensor();
        self.state
    }

    fn read_sensor(&self) -> ThermalState {
        #[cfg(feature = "thermal")]
        {
            std::fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
                .ok()
                .and_then(|s| s.trim().parse::<u64>().ok())
                .map(|millidegrees| {
                    let celsius = millidegrees / 1000;
                    match celsius {
                        0..=50 => ThermalState::Normal,
                        51..=70 => ThermalState::Warm,
                        71..=85 => ThermalState::Hot,
                        _ => ThermalState::Critical,
                    }
                })
                .unwrap_or(ThermalState::Normal)
        }

        #[cfg(not(feature = "thermal"))]
        {
            ThermalState::Normal
        }
    }
}

impl Default for ThermalMonitor {
    fn default() -> Self {
        ThermalMonitor::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_state_default() {
        assert_eq!(ThermalState::default(), ThermalState::Normal);
    }

    #[test]
    fn test_thermal_state_is_throttled() {
        assert!(!ThermalState::Normal.is_throttled());
        assert!(!ThermalState::Warm.is_throttled());
        assert!(ThermalState::Hot.is_throttled());
        assert!(ThermalState::Critical.is_throttled());
    }

    #[test]
    fn test_thermal_state_is_safe() {
        assert!(ThermalState::Normal.is_safe());
        assert!(ThermalState::Warm.is_safe());
        assert!(!ThermalState::Hot.is_safe());
        assert!(!ThermalState::Critical.is_safe());
    }

    #[test]
    fn test_thermal_state_names() {
        assert_eq!(ThermalState::Normal.name(), "normal");
        assert_eq!(ThermalState::Warm.name(), "warm");
        assert_eq!(ThermalState::Hot.name(), "hot");
        assert_eq!(ThermalState::Critical.name(), "critical");
    }

    #[test]
    fn test_thermal_monitor_default() {
        let monitor = ThermalMonitor::default();
        assert!(monitor.backoff_enabled());
        assert_eq!(monitor.state(), ThermalState::Normal);
        assert!(!monitor.should_backoff());
    }

    #[test]
    fn test_thermal_monitor_should_backoff() {
        let mut monitor = ThermalMonitor::new(true);
        monitor.set_state(ThermalState::Hot);
        assert!(monitor.should_backoff());

        monitor.set_backoff_enabled(false);
        assert!(!monitor.should_backoff());

        monitor.set_backoff_enabled(true);
        monitor.set_state(ThermalState::Normal);
        assert!(!monitor.should_backoff());
    }

    #[test]
    fn test_thermal_monitor_sample() {
        let mut monitor = ThermalMonitor::new(true);
        let state = monitor.sample();
        assert!(state == ThermalState::Normal);
    }

    #[test]
    fn test_thermal_state_ordering() {
        assert!(ThermalState::Normal < ThermalState::Warm);
        assert!(ThermalState::Warm < ThermalState::Hot);
        assert!(ThermalState::Hot < ThermalState::Critical);
    }
}
