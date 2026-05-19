use crate::budget::system_budget::SystemBudget;
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;
use crate::profiles::safe::SafeProfile;
use crate::profiles::Profile;

pub struct DebugProfile;

impl Profile for DebugProfile {
    fn name(&self) -> &'static str {
        "debug"
    }

    fn apply(&self, config: &mut GeneratedConfig, hw: &HardwareProfile, budget: &SystemBudget) {
        SafeProfile.apply(config, hw, budget);
        config.asc.profile = "debug".into();
    }

    fn description(&self) -> &'static str {
        "Debug profile — same as safe but with more verbose explain output"
    }
}
