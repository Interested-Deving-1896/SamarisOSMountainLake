use volt_dynamic_worker_pool::*;

#[test]
fn test_hardware_probe_defaults() {
    let hw_cfg = config::schema::HardwareConfig::default();
    let probe = scaling::hardware_probe::HardwareProbe::new(hw_cfg);
    assert!(probe.min_workers >= 2);
    assert!(probe.max_workers <= 48);
    assert!(probe.min_workers <= probe.max_workers);
}

#[test]
fn test_hardware_probe_with_overrides() {
    let hw_cfg = config::schema::HardwareConfig {
        min_workers_override: 8,
        max_workers_override: 24,
        ..config::schema::HardwareConfig::default()
    };
    let probe = scaling::hardware_probe::HardwareProbe::new(hw_cfg);
    assert_eq!(probe.min_workers, 8);
    assert_eq!(probe.max_workers, 24);
}

#[test]
fn test_hardware_profile_roundtrip() {
    let hw_cfg = config::schema::HardwareConfig::default();
    let probe = scaling::hardware_probe::HardwareProbe::new(hw_cfg);
    let profile = probe.profile();
    assert_eq!(profile.cpu_cores, probe.cpu_cores);
    assert_eq!(profile.ram_bytes, probe.ram_bytes);
}
