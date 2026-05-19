use serde::Serialize;

use crate::budget::system_budget::SystemBudget;
use crate::classify::machine_class::MachineClass;
use crate::classify::profile_kind::ProfileKind;
use crate::hardware::profile::HardwareProfile;
use crate::policies::global_budget::samaris_budget_cap;
use crate::policies::kernel_b::kernel_b_workers;
use crate::policies::vrm::{
    vrm_cache_mb_from_budget, vrm_desktop_quota, vrm_orbit_quota, vrm_pressure_policy,
};
use crate::policies::vum::{
    vum_batch_size, vum_buffer_mb_from_budget, vum_cache_mb_from_budget, vum_flush_interval,
    vum_journal_mode, vum_prefetch_boot_assets,
};
use crate::policies::worker_pool::{
    worker_pool_desktop_min, worker_pool_max_workers, worker_pool_min_workers,
    worker_pool_orbit_burst_max, worker_pool_orbit_burst_window, worker_pool_orbit_default_max,
    worker_pool_system_min,
};

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedConfig {
    pub kernel_b: GeneratedKernelBConfig,
    pub worker_pool: GeneratedWorkerPoolConfig,
    pub vrm: GeneratedVrmConfig,
    pub vum: GeneratedVumConfig,
    pub budget: GeneratedBudgetSection,
    pub asc: GeneratedAscSection,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedKernelBConfig {
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedWorkerPoolConfig {
    pub min_workers: usize,
    pub max_workers: usize,
    pub desktop_min: usize,
    pub system_min: usize,
    pub orbit_default_max: usize,
    pub orbit_burst_max: usize,
    pub orbit_burst_window_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedVrmConfig {
    pub desktop_quota_mb: u64,
    pub orbit_quota_mb: u64,
    pub cache_mb: u64,
    pub pressure_green_max_percent: u8,
    pub pressure_yellow_enter_percent: u8,
    pub pressure_yellow_exit_percent: u8,
    pub pressure_orange_enter_percent: u8,
    pub pressure_orange_exit_percent: u8,
    pub pressure_red_enter_percent: u8,
    pub pressure_red_exit_percent: u8,
    pub min_free_mb_yellow: u64,
    pub min_free_mb_orange: u64,
    pub min_free_mb_red: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedVumConfig {
    pub cache_mb: u64,
    pub buffer_mb: u64,
    pub flush_interval_ms: u64,
    pub batch_size_kb: u64,
    pub journal_mode: String,
    pub prefetch_boot_assets: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedBudgetSection {
    pub samaris_budget_cap_mb: u64,
    pub allocated_total_mb: u64,
    pub safety_margin_mb: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedAscSection {
    pub profile: String,
    pub machine_classes: Vec<String>,
    pub generated_at: String,
    pub safe_mode: bool,
}

impl GeneratedConfig {
    pub fn from_profile(
        hw: &HardwareProfile,
        classes: &[MachineClass],
        budget: &SystemBudget,
        kind: ProfileKind,
    ) -> Self {
        let pressure = vrm_pressure_policy(hw);

        let mut config = Self {
            kernel_b: GeneratedKernelBConfig {
                workers: kernel_b_workers(hw),
            },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: worker_pool_min_workers(hw),
                max_workers: worker_pool_max_workers(hw),
                desktop_min: worker_pool_desktop_min(hw),
                system_min: worker_pool_system_min(hw),
                orbit_default_max: worker_pool_orbit_default_max(hw),
                orbit_burst_max: worker_pool_orbit_burst_max(hw),
                orbit_burst_window_ms: worker_pool_orbit_burst_window(hw),
            },
            vrm: GeneratedVrmConfig {
                desktop_quota_mb: vrm_desktop_quota(hw, budget),
                orbit_quota_mb: vrm_orbit_quota(hw, budget),
                cache_mb: vrm_cache_mb_from_budget(budget),
                pressure_green_max_percent: pressure.green_max_percent,
                pressure_yellow_enter_percent: pressure.yellow_enter_percent,
                pressure_yellow_exit_percent: pressure.yellow_exit_percent,
                pressure_orange_enter_percent: pressure.orange_enter_percent,
                pressure_orange_exit_percent: pressure.orange_exit_percent,
                pressure_red_enter_percent: pressure.red_enter_percent,
                pressure_red_exit_percent: pressure.red_exit_percent,
                min_free_mb_yellow: pressure.min_free_mb_yellow,
                min_free_mb_orange: pressure.min_free_mb_orange,
                min_free_mb_red: pressure.min_free_mb_red,
            },
            vum: GeneratedVumConfig {
                cache_mb: vum_cache_mb_from_budget(budget),
                buffer_mb: vum_buffer_mb_from_budget(budget),
                flush_interval_ms: vum_flush_interval(hw),
                batch_size_kb: vum_batch_size(hw),
                journal_mode: vum_journal_mode(hw),
                prefetch_boot_assets: vum_prefetch_boot_assets(hw.boot_medium.clone()),
            },
            budget: GeneratedBudgetSection {
                samaris_budget_cap_mb: samaris_budget_cap(hw),
                allocated_total_mb: budget.allocated_total_mb(),
                safety_margin_mb: budget.safety_margin_mb,
            },
            asc: GeneratedAscSection {
                profile: kind.name().into(),
                machine_classes: classes.iter().map(|c| c.name().to_string()).collect(),
                generated_at: time::OffsetDateTime::now_utc()
                    .format(&time::format_description::well_known::Rfc3339)
                    .unwrap_or_else(|_| "unknown".into()),
                safe_mode: false,
            },
        };

        config.apply_profile(kind, hw, budget);

        config
    }

    fn apply_profile(&mut self, kind: ProfileKind, hw: &HardwareProfile, budget: &SystemBudget) {
        #[cfg(feature = "profiles")]
        {
            use crate::profiles::balanced::BalancedProfile;
            use crate::profiles::debug::DebugProfile;
            use crate::profiles::low_ram::LowRamProfile;
            use crate::profiles::performance::PerformanceProfile;
            use crate::profiles::powersave::PowersaveProfile;
            use crate::profiles::safe::SafeProfile;
            use crate::profiles::usb_boot::UsbBootProfile;
            use crate::profiles::vm::VmProfile;
            use crate::profiles::Profile;

            let prof: Box<dyn Profile> = match kind {
                ProfileKind::Balanced => Box::new(BalancedProfile),
                ProfileKind::Powersave => Box::new(PowersaveProfile),
                ProfileKind::Performance => Box::new(PerformanceProfile),
                ProfileKind::Safe => Box::new(SafeProfile),
                ProfileKind::Debug => Box::new(DebugProfile),
                ProfileKind::Vm => Box::new(VmProfile),
                ProfileKind::UsbBoot => Box::new(UsbBootProfile),
                ProfileKind::LowRam => Box::new(LowRamProfile),
                _ => return,
            };
            prof.apply(self, hw, budget);
        }
    }
}
