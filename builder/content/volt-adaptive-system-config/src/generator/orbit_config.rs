use crate::hardware::profile::HardwareProfile;
use crate::policies::worker_pool::{
    worker_pool_orbit_burst_max, worker_pool_orbit_burst_window, worker_pool_orbit_default_max,
};

pub struct GeneratedOrbitConfig {
    pub default_max: usize,
    pub burst_max: usize,
    pub burst_window_ms: u64,
}

pub fn generate_orbit_config(hw: &HardwareProfile) -> GeneratedOrbitConfig {
    GeneratedOrbitConfig {
        default_max: worker_pool_orbit_default_max(hw),
        burst_max: worker_pool_orbit_burst_max(hw),
        burst_window_ms: worker_pool_orbit_burst_window(hw),
    }
}
