use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedVumConfig;
use crate::hardware::profile::HardwareProfile;
use crate::policies::vum::{
    vum_batch_size, vum_buffer_mb_from_budget, vum_cache_mb_from_budget, vum_flush_interval,
    vum_journal_mode, vum_prefetch_boot_assets,
};

pub fn generate_vum_config(hw: &HardwareProfile, budget: &SystemBudget) -> GeneratedVumConfig {
    GeneratedVumConfig {
        cache_mb: vum_cache_mb_from_budget(budget),
        buffer_mb: vum_buffer_mb_from_budget(budget),
        flush_interval_ms: vum_flush_interval(hw),
        batch_size_kb: vum_batch_size(hw),
        journal_mode: vum_journal_mode(hw),
        prefetch_boot_assets: vum_prefetch_boot_assets(hw.boot_medium.clone()),
    }
}
