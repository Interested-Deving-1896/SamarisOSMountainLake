use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::classify;
use volt_adaptive_system_config::budget::system_budget::SystemBudget;
use volt_adaptive_system_config::classify::profile_kind::ProfileKind;
use volt_adaptive_system_config::generator::generated_config::GeneratedConfig;
use volt_adaptive_system_config::generator::toml_writer::generated_config_toml;

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
fn test_generated_config_has_all_sections() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    assert!(config.kernel_b.workers > 0);
    assert!(config.worker_pool.min_workers > 0);
    assert!(config.worker_pool.max_workers >= config.worker_pool.min_workers);
    assert!(config.vrm.desktop_quota_mb > 0);
    assert!(config.vum.cache_mb > 0);
    assert!(config.budget.samaris_budget_cap_mb > 0);
    assert!(!config.asc.machine_classes.is_empty());
}

#[test]
fn test_generated_config_serializes_to_toml() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    let toml = generated_config_toml(&config).expect("TOML generation");
    assert!(toml.contains("[kernel_b]"));
    assert!(toml.contains("[worker_pool]"));
    assert!(toml.contains("[vrm]"));
    assert!(toml.contains("[vum]"));
    assert!(toml.contains("[budget]"));
    assert!(toml.contains("[asc]"));
}

#[test]
fn test_generated_config_roundtrip() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    let toml = generated_config_toml(&config).expect("TOML");
    let parsed: toml::Value = toml::from_str(&toml).expect("Parse TOML");
    assert!(parsed.get("kernel_b").is_some());
    assert!(parsed.get("vrm").is_some());
    assert!(parsed.get("vum").is_some());
}

#[test]
fn test_generated_config_values_coherent() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    assert!(config.worker_pool.orbit_default_max <= config.worker_pool.max_workers);
    assert!(config.worker_pool.orbit_burst_max >= config.worker_pool.orbit_default_max);
    assert!(config.worker_pool.desktop_min >= 1);
}
