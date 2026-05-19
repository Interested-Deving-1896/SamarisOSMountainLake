use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct VumModule;

impl VumModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::vum(),
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
    fn test_vum_profile() {
        let profile = VumModule::profile();
        assert_eq!(profile.module_id, ModuleId::vum());
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
        assert!(!profile.can_burst);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
