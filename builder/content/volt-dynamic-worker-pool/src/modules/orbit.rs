use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct OrbitModule;

impl OrbitModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::orbit(),
            PriorityLevel::Critical,
            1.0,
            true,
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
    fn test_orbit_profile() {
        let profile = OrbitModule::profile();
        assert_eq!(profile.module_id, ModuleId::orbit());
        assert_eq!(profile.default_priority, PriorityLevel::Critical);
        assert!(profile.can_burst);
        assert!(profile.can_be_preempted);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
