use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct BackgroundModule;

impl BackgroundModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::background(),
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
    fn test_background_profile() {
        let profile = BackgroundModule::profile();
        assert_eq!(profile.module_id, ModuleId::background());
        assert_eq!(profile.default_priority, PriorityLevel::Low);
        assert!(profile.can_be_preempted);
        assert!(profile.background_only);
        assert!(!profile.can_burst);
        assert!(!profile.latency_sensitive);
    }
}
