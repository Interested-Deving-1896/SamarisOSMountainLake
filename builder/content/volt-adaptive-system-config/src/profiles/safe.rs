use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct SafeProfile;

impl Profile for SafeProfile {
    fn name(&self) -> &'static str {
        "safe"
    }

    fn apply(&self, config: &mut GeneratedConfig, _hw: &HardwareProfile, _budget: &SystemBudget) {
        config.vrm.desktop_quota_mb = (config.vrm.desktop_quota_mb as f64 * 0.8) as u64;
        config.vrm.orbit_quota_mb = (config.vrm.orbit_quota_mb as f64 * 0.8) as u64;
        config.vrm.cache_mb = (config.vrm.cache_mb as f64 * 0.8) as u64;
        config.vum.cache_mb = (config.vum.cache_mb as f64 * 0.8) as u64;
        config.vum.buffer_mb = (config.vum.buffer_mb as f64 * 0.8) as u64;
        config.worker_pool.max_workers = (config.worker_pool.max_workers as f64 * 0.75) as usize;
        if config.worker_pool.max_workers < 2 {
            config.worker_pool.max_workers = 2;
        }
        if config.worker_pool.min_workers > config.worker_pool.max_workers {
            config.worker_pool.min_workers = config.worker_pool.max_workers;
        }
        config.worker_pool.orbit_default_max =
            (config.worker_pool.orbit_default_max as f64 * 0.75) as usize;
        config.worker_pool.orbit_burst_max =
            (config.worker_pool.orbit_burst_max as f64 * 0.75) as usize;
        config.vum.flush_interval_ms = (config.vum.flush_interval_ms as f64 * 0.5) as u64;
        if config.vum.flush_interval_ms < 50 {
            config.vum.flush_interval_ms = 50;
        }
        config.asc.safe_mode = true;
        config.asc.profile = "safe".into();
    }

    fn description(&self) -> &'static str {
        "Safe profile — reduces all quotas by 20%, limits workers, strict VUM writeback, detailed logging"
    }
}
