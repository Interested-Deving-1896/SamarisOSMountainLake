#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThermalLevel {
    Unknown,
    Normal,
    Warm,
    Hot,
    Throttle,
    Critical,
    Emergency,
    Fatal,
}

impl ThermalLevel {
    pub fn name(&self) -> &'static str {
        match self {
            ThermalLevel::Unknown => "unknown",
            ThermalLevel::Normal => "normal",
            ThermalLevel::Warm => "warm",
            ThermalLevel::Hot => "hot",
            ThermalLevel::Throttle => "throttle",
            ThermalLevel::Critical => "critical",
            ThermalLevel::Emergency => "emergency",
            ThermalLevel::Fatal => "fatal",
        }
    }

    pub fn should_reduce_compute(&self) -> bool {
        matches!(
            self,
            ThermalLevel::Hot | ThermalLevel::Throttle | ThermalLevel::Critical
        )
    }

    pub fn should_stop_compute(&self) -> bool {
        matches!(self, ThermalLevel::Emergency | ThermalLevel::Fatal)
    }
}

#[derive(Debug, Clone)]
pub struct ThermalState {
    pub level: ThermalLevel,
    pub temperature_c: f64,
}

impl ThermalState {
    pub fn new(level: ThermalLevel, temperature_c: f64) -> Self {
        ThermalState {
            level,
            temperature_c,
        }
    }

    pub fn name(&self) -> &'static str {
        self.level.name()
    }

    pub fn should_reduce_compute(&self) -> bool {
        self.level.should_reduce_compute()
    }

    pub fn should_stop_compute(&self) -> bool {
        self.level.should_stop_compute()
    }
}

impl Default for ThermalState {
    fn default() -> Self {
        ThermalState {
            level: ThermalLevel::Normal,
            temperature_c: 40.0,
        }
    }
}

impl PartialEq for ThermalState {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level
    }
}

impl Eq for ThermalState {}

impl PartialOrd for ThermalState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.level.partial_cmp(&other.level)
    }
}

impl Ord for ThermalState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level.cmp(&other.level)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let s = ThermalState::default();
        assert_eq!(s.level, ThermalLevel::Normal);
        assert!((s.temperature_c - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_ordering() {
        assert!(ThermalLevel::Unknown < ThermalLevel::Normal);
        assert!(ThermalLevel::Normal < ThermalLevel::Warm);
        assert!(ThermalLevel::Warm < ThermalLevel::Hot);
        assert!(ThermalLevel::Hot < ThermalLevel::Throttle);
        assert!(ThermalLevel::Throttle < ThermalLevel::Critical);
        assert!(ThermalLevel::Critical < ThermalLevel::Emergency);
        assert!(ThermalLevel::Emergency < ThermalLevel::Fatal);
    }

    #[test]
    fn test_thermal_state_methods() {
        let s = ThermalState::new(ThermalLevel::Critical, 88.0);
        assert_eq!(s.name(), "critical");
        assert!(s.should_reduce_compute());
        assert!(!s.should_stop_compute());
    }

    #[test]
    fn test_name() {
        assert_eq!(ThermalLevel::Unknown.name(), "unknown");
        assert_eq!(ThermalLevel::Normal.name(), "normal");
        assert_eq!(ThermalLevel::Warm.name(), "warm");
        assert_eq!(ThermalLevel::Hot.name(), "hot");
        assert_eq!(ThermalLevel::Throttle.name(), "throttle");
        assert_eq!(ThermalLevel::Critical.name(), "critical");
        assert_eq!(ThermalLevel::Emergency.name(), "emergency");
        assert_eq!(ThermalLevel::Fatal.name(), "fatal");
    }

    #[test]
    fn test_should_reduce_compute() {
        assert!(!ThermalLevel::Unknown.should_reduce_compute());
        assert!(!ThermalLevel::Normal.should_reduce_compute());
        assert!(!ThermalLevel::Warm.should_reduce_compute());
        assert!(ThermalLevel::Hot.should_reduce_compute());
        assert!(ThermalLevel::Throttle.should_reduce_compute());
        assert!(ThermalLevel::Critical.should_reduce_compute());
        assert!(!ThermalLevel::Emergency.should_reduce_compute());
        assert!(!ThermalLevel::Fatal.should_reduce_compute());
    }

    #[test]
    fn test_should_stop_compute() {
        assert!(!ThermalLevel::Unknown.should_stop_compute());
        assert!(!ThermalLevel::Normal.should_stop_compute());
        assert!(!ThermalLevel::Warm.should_stop_compute());
        assert!(!ThermalLevel::Hot.should_stop_compute());
        assert!(!ThermalLevel::Throttle.should_stop_compute());
        assert!(!ThermalLevel::Critical.should_stop_compute());
        assert!(ThermalLevel::Emergency.should_stop_compute());
        assert!(ThermalLevel::Fatal.should_stop_compute());
    }

    #[test]
    fn test_temperature_field() {
        let s = ThermalState::new(ThermalLevel::Hot, 72.5);
        assert!((s.temperature_c - 72.5).abs() < 0.001);
    }
}
