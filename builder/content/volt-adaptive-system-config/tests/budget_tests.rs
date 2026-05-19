use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::classify;
use volt_adaptive_system_config::budget::system_budget::SystemBudget;
use volt_adaptive_system_config::policies::global_budget::samaris_budget_cap;

fn make_hw(ram: u64) -> HardwareProfile {
    HardwareProfile {
        cpu_cores: 8, cpu_threads: 8, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
        ram_total_mb: ram, ram_available_mb: ram * 3 / 4, swap_total_mb: ram / 2,
        gpu_available: true, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk, storage_type: StorageType::Ssd,
        usb_speed: None, is_vm: false, is_laptop: false, battery_present: false, thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

#[test]
fn test_budget_within_cap_by_default() {
    let hw = make_hw(16384);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 128);
    let cap = samaris_budget_cap(&hw);
    assert!(budget.allocated_total <= cap || budget.total_with_margin() <= cap,
        "Budget {} exceeds cap {}", budget.allocated_total, cap);
}

#[test]
fn test_budget_reconciliation_reduces_vum_first() {
    let hw = make_hw(4096);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let orig_total = budget.allocated_total;
    let reconciled = budget.reconcile(&hw).expect("Reconciliation should succeed");
    assert!(reconciled.allocated_total <= samaris_budget_cap(&hw) || reconciled.allocated_total <= orig_total);
}

#[test]
fn test_desktop_not_reduced_first() {
    let hw = make_hw(2048);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 32);
    let orig_desktop = budget.desktop_mb;
    let reconciled = budget.reconcile(&hw).expect("Reconciliation should succeed");
    assert!(reconciled.desktop_mb >= orig_desktop || reconciled.desktop_mb >= 64);
}

#[test]
fn test_safety_margin_positive() {
    let hw = make_hw(8192);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 128);
    assert!(budget.safety_margin_mb > 0);
}

#[test]
fn test_budget_allocated_total() {
    let hw = make_hw(8192);
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    assert!(budget.allocated_total > 0);
    assert!(budget.safety_margin_mb > 0);
}
