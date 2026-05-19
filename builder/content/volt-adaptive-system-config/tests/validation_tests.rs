use volt_adaptive_system_config::validation::constraints::*;
use volt_adaptive_system_config::validation::safety_caps::SafetyCaps;
use volt_adaptive_system_config::validation::invariants::InvariantChecker;
use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::classify;
use volt_adaptive_system_config::budget::system_budget::SystemBudget;
use volt_adaptive_system_config::classify::profile_kind::ProfileKind;
use volt_adaptive_system_config::generator::generated_config::GeneratedConfig;

fn make_hw() -> HardwareProfile {
    HardwareProfile {
        cpu_cores: 8, cpu_threads: 8, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
        ram_total_mb: 8192, ram_available_mb: 6000, swap_total_mb: 2048,
        gpu_available: true, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk, storage_type: StorageType::Ssd,
        usb_speed: None, is_vm: false, is_laptop: false, battery_present: false, thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

#[test]
fn test_validate_worker_bounds() {
    assert!(validate_worker_bounds(2, 8).is_ok());
    assert!(validate_worker_bounds(10, 5).is_err());
}

#[test]
fn test_safety_caps_clamp_excessive() {
    let caps = SafetyCaps::default_caps();
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let mut config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    config.worker_pool.max_workers = 999;
    let clamped = caps.apply(&mut config);
    assert!(config.worker_pool.max_workers < 999);
    assert!(!clamped.is_empty());
}

#[test]
fn test_invariant_min_workers() {
    assert!(InvariantChecker::check_min_workers_positive(0).is_err());
    assert!(InvariantChecker::check_min_workers_positive(1).is_ok());
}

#[test]
fn test_invariant_desktop_protected() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    assert!(InvariantChecker::check_desktop_protected(&budget).is_ok());
}
