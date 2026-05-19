pub mod balanced;
pub mod debug;
pub mod low_ram;
pub mod performance;
pub mod powersave;
pub mod safe;
pub mod usb_boot;
pub mod vm;

use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;

pub trait Profile: Send + Sync {
    fn name(&self) -> &'static str;
    fn apply(&self, config: &mut GeneratedConfig, hw: &HardwareProfile, budget: &SystemBudget);
    fn description(&self) -> &'static str;
}
