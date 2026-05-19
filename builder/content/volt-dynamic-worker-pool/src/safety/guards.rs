use crate::config::schema::WorkerPoolConfig;
use crate::worker::WorkerState;

pub struct SafetyGuards {
    pub desktop_min_workers: u32,
    pub system_min_workers: u32,
    pub orbit_burst_enabled: bool,
    pub thermal_backoff_enabled: bool,
}

impl SafetyGuards {
    pub fn new(config: &WorkerPoolConfig) -> Self {
        Self {
            desktop_min_workers: config.worker_pool.reservations.desktop_min_workers as u32,
            system_min_workers: config.worker_pool.reservations.system_min_workers as u32,
            orbit_burst_enabled: true,
            thermal_backoff_enabled: config.worker_pool.thermal.thermal_backoff_enabled,
        }
    }

    pub fn can_scale_down(&self, high_backlog: bool) -> bool {
        !high_backlog
    }

    pub fn can_scale_up(&self, thermal_active: bool) -> bool {
        !(self.thermal_backoff_enabled && thermal_active)
    }

    pub fn can_burst(&self, desktop_pressure: f64, thermal_active: bool) -> bool {
        if self.thermal_backoff_enabled && thermal_active {
            return false;
        }
        if desktop_pressure > 0.6 {
            return false;
        }
        self.orbit_burst_enabled
    }

    pub fn can_kill_worker(&self, state: WorkerState) -> bool {
        state == WorkerState::Idle
    }
}
