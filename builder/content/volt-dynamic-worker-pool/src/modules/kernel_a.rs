use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct KernelAModule;

impl KernelAModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::kernel_a(),
            PriorityLevel::Normal,
            0.3,
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
    fn test_kernel_a_profile() {
        let profile = KernelAModule::profile();
        assert_eq!(profile.module_id, ModuleId::kernel_a());
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!((profile.max_workers_fraction - 0.3).abs() < 1e-10);
        assert!(!profile.can_burst);
        assert!(profile.can_be_preempted);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
