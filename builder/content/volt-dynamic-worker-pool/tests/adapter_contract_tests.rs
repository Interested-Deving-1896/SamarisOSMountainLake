use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_module_id_named_constructors() {
    assert_eq!(ModuleId::orbit().as_str(), "orbit");
    assert_eq!(ModuleId::desktop().as_str(), "desktop");
    assert_eq!(ModuleId::kernel_a().as_str(), "kernel_a");
    assert_eq!(ModuleId::kernel_b().as_str(), "kernel_b");
    assert_eq!(ModuleId::background().as_str(), "background");
}

#[test]
fn test_module_profile_from_config_orbit() {
    let profile = ModuleProfile::from_config("orbit");
    assert_eq!(profile.module_id, ModuleId::orbit());
    assert_eq!(profile.default_priority, priority::level::PriorityLevel::Critical);
    assert!(profile.can_burst);
}

#[test]
fn test_module_profile_from_config_desktop() {
    let profile = ModuleProfile::from_config("desktop");
    assert_eq!(profile.default_priority, priority::level::PriorityLevel::High);
    assert!(profile.latency_sensitive);
    assert!(!profile.can_be_preempted);
}

#[test]
fn test_module_registry_register_and_lookup() {
    use modules::registry::ModuleRegistry;
    let mut reg = ModuleRegistry::new();
    assert!(reg.is_empty());

    let profile = ModuleProfile::from_config("orbit");
    reg.register(profile.clone()).unwrap();
    assert!(reg.is_registered(&ModuleId::orbit()));
    assert_eq!(reg.len(), 1);
}

#[test]
fn test_module_registry_prevents_duplicates() {
    use modules::registry::ModuleRegistry;
    let mut reg = ModuleRegistry::new();
    let profile = ModuleProfile::from_config("orbit");
    reg.register(profile).unwrap();
    let dup = ModuleProfile::from_config("orbit");
    assert!(reg.register(dup).is_err());
}
