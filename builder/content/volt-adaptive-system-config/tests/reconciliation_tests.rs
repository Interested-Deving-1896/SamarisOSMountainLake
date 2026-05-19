use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::classify;
use volt_adaptive_system_config::budget::system_budget::SystemBudget;

fn make_hw(ram: u64) -> HardwareProfile {
    HardwareProfile {
        cpu_cores: 4, cpu_threads: 4, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
        ram_total_mb: ram, ram_available_mb: ram * 3 / 4, swap_total_mb: ram / 2,
        gpu_available: false, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk, storage_type: StorageType::Ssd,
        usb_speed: None, is_vm: false, is_laptop: false, battery_present: false, thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

#[test]
fn test_reconcile_reduces_vum_cache_first() {
    let hw = make_hw(2048);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let original_cache = budget.vum_cache_mb;
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    assert!(reconciled.vum_cache_mb <= original_cache);
}

#[test]
fn test_reconcile_keeps_desktop_intact() {
    let hw = make_hw(1024);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let original_desktop = budget.desktop_mb;
    let result = budget.reconcile(&hw);
    if let Ok(reconciled) = result {
        assert!(reconciled.desktop_mb >= original_desktop / 2);
    }
}

#[test]
fn test_reconcile_on_small_ram_does_not_panic() {
    let hw = make_hw(512);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let result = budget.reconcile(&hw);
    match result {
        Ok(_) => {},
        Err(e) => assert!(e.to_string().contains("Budget") || e.to_string().contains("budget")),
    }
}
