use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::Profile;

pub struct BalancedProfile;

impl Profile for BalancedProfile {
    fn name(&self) -> &'static str {
        "balanced"
    }

    fn apply(&self, _config: &mut GeneratedConfig, _hw: &HardwareProfile, _budget: &SystemBudget) {
    }

    fn description(&self) -> &'static str {
        "Default balanced profile — no modifications applied"
    }
}
