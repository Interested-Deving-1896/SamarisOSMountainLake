use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct VgmModule;

impl VgmModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::vgm(),
            PriorityLevel::Normal,
            1.0,
            false,
            true,
            false,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vgm_profile() {
        let profile = VgmModule::profile();
        assert_eq!(profile.module_id, ModuleId::vgm());
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
        assert!(!profile.can_burst);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
