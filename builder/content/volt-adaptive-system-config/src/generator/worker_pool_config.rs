use crate::generator::generated_config::GeneratedWorkerPoolConfig;
use crate::hardware::profile::HardwareProfile;
use crate::policies::worker_pool::{
    worker_pool_desktop_min, worker_pool_max_workers, worker_pool_min_workers,
    worker_pool_orbit_burst_max, worker_pool_orbit_burst_window, worker_pool_orbit_default_max,
    worker_pool_system_min,
};

pub fn generate_worker_pool_config(hw: &HardwareProfile) -> GeneratedWorkerPoolConfig {
    GeneratedWorkerPoolConfig {
        min_workers: worker_pool_min_workers(hw),
        max_workers: worker_pool_max_workers(hw),
        desktop_min: worker_pool_desktop_min(hw),
        system_min: worker_pool_system_min(hw),
        orbit_default_max: worker_pool_orbit_default_max(hw),
        orbit_burst_max: worker_pool_orbit_burst_max(hw),
        orbit_burst_window_ms: worker_pool_orbit_burst_window(hw),
    }
}
