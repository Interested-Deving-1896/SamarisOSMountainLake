use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct VmProfile;

impl Profile for VmProfile {
    fn name(&self) -> &'static str {
        "vm"
    }

    fn apply(&self, config: &mut GeneratedConfig, hw: &HardwareProfile, budget: &SystemBudget) {
        let vm_cap = hw.cpu_cores.max(2);
        if config.worker_pool.max_workers > vm_cap {
            config.worker_pool.max_workers = vm_cap;
        }
        if config.worker_pool.min_workers > config.worker_pool.max_workers {
            config.worker_pool.min_workers = config.worker_pool.max_workers;
        }
        config.worker_pool.orbit_default_max =
            (config.worker_pool.orbit_default_max as f64 * 0.5) as usize;
        config.worker_pool.orbit_burst_max =
            (config.worker_pool.orbit_burst_max as f64 * 0.5) as usize;
        config.worker_pool.orbit_burst_window_ms =
            (config.worker_pool.orbit_burst_window_ms as f64 * 1.5) as u64;
        config.vrm.orbit_quota_mb = (config.vrm.orbit_quota_mb as f64 * 0.6) as u64;
        config.vum.cache_mb = (budget.vum_cache_mb as f64 * 0.7) as u64;
        config.vum.buffer_mb = (budget.vum_buffer_mb as f64 * 0.7) as u64;
        config.asc.profile = "vm".into();
    }

    fn description(&self) -> &'static str {
        "VM profile — limits workers to VM cap, limits orbit, moderate VUM cache"
    }
}
