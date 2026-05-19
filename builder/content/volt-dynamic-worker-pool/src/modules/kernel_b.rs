use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::priority::level::PriorityLevel;

pub struct KernelBModule;

impl KernelBModule {
    pub fn profile() -> ModuleProfile {
        ModuleProfile::new_full(
            ModuleId::kernel_b(),
            PriorityLevel::Normal,
            0.4,
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
    fn test_kernel_b_profile() {
        let profile = KernelBModule::profile();
        assert_eq!(profile.module_id, ModuleId::kernel_b());
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!((profile.max_workers_fraction - 0.4).abs() < 1e-10);
        assert!(!profile.can_burst);
        assert!(profile.can_be_preempted);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }
}
