use crate::modules::module_id::ModuleId;
use crate::priority::level::PriorityLevel;

#[derive(Debug, Clone)]
pub struct ModuleProfile {
    pub module_id: ModuleId,
    pub default_priority: PriorityLevel,
    pub max_workers_fraction: f64,
    pub can_burst: bool,
    pub can_be_preempted: bool,
    pub latency_sensitive: bool,
    pub background_only: bool,
}

impl ModuleProfile {
    pub fn new(module_id: ModuleId, default_priority: PriorityLevel) -> Self {
        Self {
            module_id,
            default_priority,
            max_workers_fraction: 1.0,
            can_burst: false,
            can_be_preempted: true,
            latency_sensitive: false,
            background_only: false,
        }
    }

    pub fn new_full(
        module_id: ModuleId,
        default_priority: PriorityLevel,
        max_workers_fraction: f64,
        can_burst: bool,
        can_be_preempted: bool,
        latency_sensitive: bool,
        background_only: bool,
    ) -> Self {
        Self {
            module_id,
            default_priority,
            max_workers_fraction,
            can_burst,
            can_be_preempted,
            latency_sensitive,
            background_only,
        }
    }

    pub fn with_resource_fraction(mut self, fraction: f64) -> Self {
        self.max_workers_fraction = fraction;
        self
    }

    pub fn with_can_burst(mut self) -> Self {
        self.can_burst = true;
        self
    }

    pub fn with_can_be_preempted(mut self) -> Self {
        self.can_be_preempted = true;
        self
    }

    pub fn with_latency_sensitive(mut self) -> Self {
        self.latency_sensitive = true;
        self
    }

    pub fn with_background_only(mut self) -> Self {
        self.background_only = true;
        self
    }

    pub fn with_no_preemption(mut self) -> Self {
        self.can_be_preempted = false;
        self
    }

    pub fn from_config(module_id_str: &str) -> Self {
        let module_id = ModuleId::new(module_id_str);
        match module_id_str {
            "orbit" => Self::new(module_id, PriorityLevel::Critical)
                .with_can_burst(),
            "desktop" => Self::new(module_id, PriorityLevel::High)
                .with_latency_sensitive()
                .with_no_preemption(),
            "kernel_a" => Self::new(module_id, PriorityLevel::Normal)
                .with_resource_fraction(0.3),
            "kernel_b" => Self::new(module_id, PriorityLevel::Normal)
                .with_resource_fraction(0.4),
            "vrm" => Self::new(module_id, PriorityLevel::Low)
                .with_background_only(),
            "vum" => Self::new(module_id, PriorityLevel::Normal),
            "vgm" => Self::new(module_id, PriorityLevel::Normal),
            "background" => Self::new(module_id, PriorityLevel::Low)
                .with_background_only(),
            "electron" => Self::new(module_id, PriorityLevel::Normal),
            _ => Self::new(module_id, PriorityLevel::Normal),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_profile() {
        let profile = ModuleProfile::from_config("orbit");
        assert_eq!(profile.module_id.as_str(), "orbit");
        assert_eq!(profile.default_priority, PriorityLevel::Critical);
        assert!(profile.can_burst);
        assert!(profile.can_be_preempted);
        assert!(!profile.latency_sensitive);
        assert!(!profile.background_only);
    }

    #[test]
    fn test_desktop_profile() {
        let profile = ModuleProfile::from_config("desktop");
        assert_eq!(profile.module_id.as_str(), "desktop");
        assert_eq!(profile.default_priority, PriorityLevel::High);
        assert!(!profile.can_burst);
        assert!(!profile.can_be_preempted);
        assert!(profile.latency_sensitive);
        assert!(!profile.background_only);
    }

    #[test]
    fn test_kernel_a_profile() {
        let profile = ModuleProfile::from_config("kernel_a");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!((profile.max_workers_fraction - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_kernel_b_profile() {
        let profile = ModuleProfile::from_config("kernel_b");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!((profile.max_workers_fraction - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_vrm_profile() {
        let profile = ModuleProfile::from_config("vrm");
        assert_eq!(profile.default_priority, PriorityLevel::Low);
        assert!(profile.can_be_preempted);
        assert!(profile.background_only);
    }

    #[test]
    fn test_vum_profile() {
        let profile = ModuleProfile::from_config("vum");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
        assert!(!profile.background_only);
    }

    #[test]
    fn test_vgm_profile() {
        let profile = ModuleProfile::from_config("vgm");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
    }

    #[test]
    fn test_background_profile() {
        let profile = ModuleProfile::from_config("background");
        assert_eq!(profile.default_priority, PriorityLevel::Low);
        assert!(profile.can_be_preempted);
        assert!(profile.background_only);
    }

    #[test]
    fn test_electron_profile() {
        let profile = ModuleProfile::from_config("electron");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
        assert!(!profile.background_only);
    }

    #[test]
    fn test_unknown_module_defaults() {
        let profile = ModuleProfile::from_config("unknown_module");
        assert_eq!(profile.default_priority, PriorityLevel::Normal);
        assert!(profile.can_be_preempted);
        assert!(!profile.background_only);
        assert!(!profile.can_burst);
        assert!(!profile.latency_sensitive);
    }

    #[test]
    fn test_new_full_constructor() {
        let id = ModuleId::new("custom");
        let profile = ModuleProfile::new_full(
            id.clone(),
            PriorityLevel::Realtime,
            0.5,
            true,
            false,
            true,
            false,
        );
        assert_eq!(profile.module_id, id);
        assert_eq!(profile.default_priority, PriorityLevel::Realtime);
        assert!((profile.max_workers_fraction - 0.5).abs() < 1e-10);
        assert!(profile.can_burst);
        assert!(!profile.can_be_preempted);
        assert!(profile.latency_sensitive);
        assert!(!profile.background_only);
    }

    #[test]
    fn test_builder_pattern() {
        let id = ModuleId::new("builder_test");
        let profile = ModuleProfile::new(id.clone(), PriorityLevel::Low)
            .with_resource_fraction(0.25)
            .with_can_burst()
            .with_latency_sensitive();
        assert_eq!(profile.module_id, id);
        assert_eq!(profile.default_priority, PriorityLevel::Low);
        assert!((profile.max_workers_fraction - 0.25).abs() < 1e-10);
        assert!(profile.can_burst);
        assert!(profile.latency_sensitive);
    }

    #[test]
    fn test_background_only_builder() {
        let id = ModuleId::new("bg_test");
        let profile = ModuleProfile::new(id, PriorityLevel::Low)
            .with_background_only();
        assert!(profile.background_only);
        assert!(profile.can_be_preempted);
    }
}
