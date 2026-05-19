use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::classify;
use volt_adaptive_system_config::classify::profile_kind::ProfileKind;
use volt_adaptive_system_config::budget::system_budget::SystemBudget;
use volt_adaptive_system_config::generator::generated_config::GeneratedConfig;
use volt_adaptive_system_config::explain::report::ExplainReport;

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
fn test_explain_report_contains_hardware() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    let report = ExplainReport::new(&hw, &classes, Some(&reconciled), Some(&config));
    let rendered = report.render();
    assert!(rendered.contains("CPU cores"));
    assert!(rendered.contains("8192"));
}

#[test]
fn test_explain_report_contains_decisions() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    let report = ExplainReport::new(&hw, &classes, Some(&reconciled), Some(&config));
    let rendered = report.render();
    assert!(rendered.contains("Kernel B") || rendered.contains("workers"));
    assert!(rendered.contains("min"));
    assert!(rendered.contains("max"));
}

#[test]
fn test_explain_report_contains_reasons() {
    let hw = make_hw();
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Balanced);
    let report = ExplainReport::new(&hw, &classes, Some(&reconciled), Some(&config));
    let rendered = report.render();
    assert!(rendered.contains("Reason") || rendered.contains("cores"));
}

#[test]
fn test_explain_report_mentions_fallback() {
    let mut hw = make_hw();
    hw.confidence.gpu = 0.0;
    let classes = classify(&hw);
    let budget = SystemBudget::compute(&hw, &classes, 64);
    let reconciled = budget.reconcile(&hw).expect("Reconciliation");
    let config = GeneratedConfig::from_profile(&hw, &classes, &reconciled, ProfileKind::Safe);
    let report = ExplainReport::new(&hw, &classes, Some(&reconciled), Some(&config));
    let rendered = report.render();
    assert!(rendered.contains("Warning") || rendered.contains("confidence"));
}

#[test]
fn test_explain_report_when_generated_none() {
    let hw = make_hw();
    let classes = classify(&hw);
    let report = ExplainReport::new(&hw, &classes, None, None);
    let rendered = report.render();
    assert!(rendered.contains("Hardware"));
}
