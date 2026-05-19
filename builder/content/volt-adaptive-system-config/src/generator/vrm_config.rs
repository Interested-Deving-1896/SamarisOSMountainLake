use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedVrmConfig;
use crate::hardware::profile::HardwareProfile;
use crate::policies::vrm::{vrm_cache_mb_from_budget, vrm_desktop_quota, vrm_orbit_quota, vrm_pressure_policy};

pub fn generate_vrm_config(hw: &HardwareProfile, budget: &SystemBudget) -> GeneratedVrmConfig {
    let pressure = vrm_pressure_policy(hw);
    GeneratedVrmConfig {
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
    }
}
