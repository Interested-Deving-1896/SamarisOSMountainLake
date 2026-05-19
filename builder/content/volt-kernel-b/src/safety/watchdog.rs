use crate::system::thermal::ThermalMetrics;

#[derive(Debug, Clone)]
pub enum WatchdogState {
    Normal,
    Throttled,
    Emergency,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ThermalWatchdog {
    state: WatchdogState,
    pub last_temp_c: f64,
    pub throttle_started_at: Option<u64>,
}

impl ThermalWatchdog {
    pub fn new() -> Self {
        Self {
            state: WatchdogState::Normal,
            last_temp_c: 0.0,
            throttle_started_at: None,
        }
    }

    pub fn evaluate(&mut self, metrics: &ThermalMetrics) -> WatchdogAction {
        self.last_temp_c = metrics.max_temp;

        if metrics.max_temp >= 100.0 {
            self.state = WatchdogState::Critical;
            WatchdogAction::EmergencyShutdown
        } else if metrics.max_temp >= 95.0 {
            self.state = WatchdogState::Emergency;
            WatchdogAction::ReleaseCoresAndNotify
        } else if metrics.max_temp >= 85.0 {
            self.state = WatchdogState::Throttled;
            if self.throttle_started_at.is_none() {
                self.throttle_started_at = Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                );
            }
            WatchdogAction::ThrottleTo50Percent
        } else {
            self.state = WatchdogState::Normal;
            self.throttle_started_at = None;
            WatchdogAction::Normal
        }
    }

    pub fn state(&self) -> &WatchdogState {
        &self.state
    }

    pub fn reset(&mut self) {
        self.state = WatchdogState::Normal;
        self.last_temp_c = 0.0;
        self.throttle_started_at = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchdogAction {
    Normal,
    ThrottleTo50Percent,
    ReleaseCoresAndNotify,
    EmergencyShutdown,
}
