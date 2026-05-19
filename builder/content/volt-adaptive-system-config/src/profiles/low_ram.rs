use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct LowRamProfile;

impl Profile for LowRamProfile {
    fn name(&self) -> &'static str {
        "low_ram"
    }

    fn apply(&self, config: &mut GeneratedConfig, _hw: &HardwareProfile, _budget: &SystemBudget) {
        let target_desktop = config.vrm.desktop_quota_mb;
        config.vrm.orbit_quota_mb = (config.vrm.orbit_quota_mb as f64 * 0.5) as u64;
        config.vrm.cache_mb = (config.vrm.cache_mb as f64 * 0.5) as u64;
        config.vum.cache_mb = (config.vum.cache_mb as f64 * 0.5) as u64;
        config.vum.buffer_mb = (config.vum.buffer_mb as f64 * 0.5) as u64;
        config.worker_pool.max_workers = (config.worker_pool.max_workers as f64 * 0.5) as usize;
        if config.worker_pool.max_workers < 2 {
            config.worker_pool.max_workers = 2;
        }
        if config.worker_pool.min_workers > config.worker_pool.max_workers {
            config.worker_pool.min_workers = config.worker_pool.max_workers;
        }
        config.worker_pool.orbit_default_max =
            (config.worker_pool.orbit_default_max as f64 * 0.5) as usize;
        config.worker_pool.orbit_burst_max =
            (config.worker_pool.orbit_burst_max as f64 * 0.5) as usize;
        config.worker_pool.desktop_min = 1.max(config.worker_pool.desktop_min);
        config.vrm.desktop_quota_mb = config.vrm.desktop_quota_mb.max(target_desktop);
        config.asc.profile = "low_ram".into();
    }

    fn description(&self) -> &'static str {
        "Low RAM profile — protects desktop quota, reduces orbit/caches, aggressive VRM compression"
    }
}
