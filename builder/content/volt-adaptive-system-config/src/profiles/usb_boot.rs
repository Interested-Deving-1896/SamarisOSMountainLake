use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct UsbBootProfile;

impl Profile for UsbBootProfile {
    fn name(&self) -> &'static str {
        "usb_boot"
    }

    fn apply(&self, config: &mut GeneratedConfig, _hw: &HardwareProfile, budget: &SystemBudget) {
        config.vum.cache_mb = (budget.vum_cache_mb as f64 * 2.0) as u64;
        config.vum.buffer_mb = (budget.vum_buffer_mb as f64 * 1.5) as u64;
        config.vum.flush_interval_ms = 50;
        config.vum.journal_mode = "wal".into();
        config.vum.prefetch_boot_assets = true;
        config.asc.profile = "usb_boot".into();
    }

    fn description(&self) -> &'static str {
        "USB Boot profile — increased VUM cache, boot prefetch enabled, strict flush, WAL journal"
    }
}
