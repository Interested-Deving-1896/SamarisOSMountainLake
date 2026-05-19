use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct DesktopModule;

impl DesktopModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::desktop(),
            PriorityLevel::High,
            1.0,
            false,
            false,
            true,
            false,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_profile() {
        let profile = DesktopModule::profile();
        assert_eq!(profile.module_id, ModuleId::desktop());
        assert_eq!(profile.default_priority, PriorityLevel::High);
        assert!(!profile.can_burst);
        assert!(!profile.can_be_preempted);
        assert!(profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
