use volt_adaptive_system_config::hardware::profile::{HardwareProfile, BootMedium, StorageType};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::hardware::probe::HardwareProbe;

#[test]
fn test_hardware_profile_default() {
    let profile = HardwareProfile::default();
    assert_eq!(profile.cpu_cores, 4);
    assert_eq!(profile.ram_total_mb, 2048);
    assert!(!profile.gpu_available);
    assert_eq!(profile.boot_medium, BootMedium::Unknown);
    assert_eq!(profile.storage_type, StorageType::Unknown);
}

#[test]
fn test_detection_confidence_bounds() {
    let conf = DetectionConfidence::default();
    assert!(conf.cpu >= 0.0 && conf.cpu <= 1.0);
    assert!(conf.ram >= 0.0 && conf.ram <= 1.0);
    assert!(conf.gpu >= 0.0 && conf.gpu <= 1.0);

    let high = DetectionConfidence::new(true);
    assert!((high.cpu - 0.95).abs() < 0.01);
}

#[test]
fn test_hardware_probe_detect() {
    let probe = HardwareProbe::new();
    let profile = probe.detect().expect("Probe should not fail");
    assert!(profile.cpu_cores > 0);
    assert!(profile.ram_total_mb > 0);
    assert!(profile.confidence.cpu > 0.0);
}

#[test]
fn test_hardware_profile_serialize() {
    let profile = HardwareProfile::default();
    let json = serde_json::to_string(&profile).expect("Serialize should work");
    let deserialized: HardwareProfile = serde_json::from_str(&json).expect("Deserialize should work");
    assert_eq!(deserialized.cpu_cores, profile.cpu_cores);
}

#[test]
fn test_storage_type_names() {
    assert_eq!(StorageType::Usb.name(), "usb");
    assert_eq!(StorageType::Ssd.name(), "ssd");
    assert_eq!(StorageType::Unknown.name(), "unknown");
}

#[test]
fn test_boot_medium_display() {
    assert_eq!(format!("{}", BootMedium::Usb), "usb");
    assert_eq!(format!("{}", BootMedium::InternalDisk), "internal_disk");
}
