use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct PowersaveProfile;

impl Profile for PowersaveProfile {
    fn name(&self) -> &'static str {
        "powersave"
    }

    fn apply(&self, config: &mut GeneratedConfig, _hw: &HardwareProfile, _budget: &SystemBudget) {
        config.vrm.orbit_quota_mb = (config.vrm.orbit_quota_mb as f64 * 0.75) as u64;
        let reduced_max = (config.worker_pool.max_workers as f64 * 0.8) as usize;
        config.worker_pool.max_workers = reduced_max.max(1);
        if config.worker_pool.min_workers > config.worker_pool.max_workers {
            config.worker_pool.min_workers = config.worker_pool.max_workers;
        }
        config.worker_pool.orbit_burst_window_ms =
            (config.worker_pool.orbit_burst_window_ms as f64 * 0.7) as u64;
        config.worker_pool.orbit_burst_max =
            (config.worker_pool.orbit_burst_max as f64 * 0.75) as usize;
        if config.worker_pool.orbit_burst_max < 1 {
            config.worker_pool.orbit_burst_max = 1;
        }
        config.asc.profile = "powersave".into();
    }

    fn description(&self) -> &'static str {
        "Powersave profile — reduces orbit quota by 25%, reduces max workers by 20%, shortens burst window"
    }
}
