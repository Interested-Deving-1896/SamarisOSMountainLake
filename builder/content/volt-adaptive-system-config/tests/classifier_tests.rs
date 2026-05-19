use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::classify::machine_class::{MachineClass, classify};
use volt_adaptive_system_config::classify::profile_kind::ProfileKind;

fn make_hw() -> HardwareProfile {
    HardwareProfile {
        cpu_cores: 8,
        cpu_threads: 8,
        cpu_model: "test".into(),
        cpu_arch: "x86_64".into(),
        ram_total_mb: 16384,
        ram_available_mb: 12000,
        swap_total_mb: 4096,
        gpu_available: true,
        gpu_vendor: None,
        gpu_model: None,
        gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk,
        storage_type: StorageType::Ssd,
        usb_speed: None,
        is_vm: false,
        is_laptop: true,
        battery_present: true,
        thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

#[test]
fn test_classify_low_ram() {
    let mut hw = make_hw();
    hw.ram_total_mb = 2048;
    let classes = classify(&hw);
    assert!(classes.contains(&MachineClass::LowRam));
}

#[test]
fn test_classify_usb_boot() {
    let mut hw = make_hw();
    hw.boot_medium = BootMedium::Usb;
    let classes = classify(&hw);
    assert!(classes.contains(&MachineClass::UsbBoot));
}

#[test]
fn test_classify_vm() {
    let mut hw = make_hw();
    hw.is_vm = true;
    let classes = classify(&hw);
    assert!(classes.contains(&MachineClass::VirtualMachine));
}

#[test]
fn test_classify_high_memory() {
    let mut hw = make_hw();
    hw.ram_total_mb = 65536;
    let classes = classify(&hw);
    assert!(classes.contains(&MachineClass::HighMemory));
}

#[test]
fn test_classify_laptop_classes() {
    let hw = make_hw();
    let classes = classify(&hw);
    assert!(classes.contains(&MachineClass::PerformanceLaptop));
    assert!(classes.contains(&MachineClass::BatteryPowered));
    assert!(classes.contains(&MachineClass::ThermalSensitive));
}

#[test]
fn test_profile_kind_from_config() {
    assert_eq!(ProfileKind::from_config("balanced"), ProfileKind::Balanced);
    assert_eq!(ProfileKind::from_config("performance"), ProfileKind::Performance);
    assert_eq!(ProfileKind::from_config("unknown"), ProfileKind::Balanced);
}

#[test]
fn test_classify_names() {
    assert_eq!(MachineClass::LowRam.name(), "low-ram");
    assert_eq!(MachineClass::Server.name(), "server");
}
