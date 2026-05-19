use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct VrmModule;

impl VrmModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::vrm(),
            PriorityLevel::Low,
            1.0,
            false,
            true,
            false,
            true,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vrm_profile() {
        let profile = VrmModule::profile();
        assert_eq!(profile.module_id, ModuleId::vrm());
        assert_eq!(profile.default_priority, PriorityLevel::Low);
        assert!(profile.can_be_preempted);
        assert!(profile.background_only);
        assert!(!profile.can_burst);
        assert!(!profile.latency_sensitive);
    }
}
