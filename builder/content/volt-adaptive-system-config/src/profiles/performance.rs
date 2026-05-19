use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct PerformanceProfile;

impl Profile for PerformanceProfile {
    fn name(&self) -> &'static str {
        "performance"
    }

    fn apply(&self, config: &mut GeneratedConfig, _hw: &HardwareProfile, _budget: &SystemBudget) {
        let boosted = (config.vrm.orbit_quota_mb as f64 * 1.25) as u64;
        config.vrm.orbit_quota_mb = boosted.min(config.vrm.desktop_quota_mb);
        config.worker_pool.max_workers = (config.worker_pool.max_workers as f64 * 1.25) as usize;
        if config.worker_pool.max_workers > 32 {
            config.worker_pool.max_workers = 32;
        }
        config.vum.cache_mb = (config.vum.cache_mb as f64 * 1.5) as u64;
        config.vum.flush_interval_ms = (config.vum.flush_interval_ms as f64 * 1.5) as u64;
        config.asc.profile = "performance".into();
    }

    fn description(&self) -> &'static str {
        "Performance profile — increases orbit quota by 25%, increases max workers, increases VUM cache"
    }
}
