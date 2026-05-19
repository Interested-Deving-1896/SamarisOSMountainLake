use volt_adaptive_system_config::hardware::profile::{HardwareProfile, StorageType, UsbSpeed, BootMedium};
use volt_adaptive_system_config::hardware::confidence::DetectionConfidence;
use volt_adaptive_system_config::policies::kernel_b::kernel_b_workers;
use volt_adaptive_system_config::policies::worker_pool::*;
use volt_adaptive_system_config::policies::vrm::*;
use volt_adaptive_system_config::policies::vum::*;
use volt_adaptive_system_config::policies::global_budget::samaris_budget_cap;

fn make_hw(cpu: usize, ram: u64, laptop: bool, vm: bool) -> HardwareProfile {
    HardwareProfile {
        cpu_cores: cpu, cpu_threads: cpu, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
        ram_total_mb: ram, ram_available_mb: ram * 3 / 4, swap_total_mb: ram / 2,
        gpu_available: true, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk,
        storage_type: StorageType::Ssd, usb_speed: Some(UsbSpeed::Usb3Plus),
        is_vm: vm, is_laptop: laptop,
        battery_present: laptop, thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

fn make_hw_storage(st: StorageType, us: Option<UsbSpeed>) -> HardwareProfile {
    HardwareProfile {
        cpu_cores: 8, cpu_threads: 8, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
        ram_total_mb: 16384, ram_available_mb: 12000, swap_total_mb: 4096,
        gpu_available: true, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
        boot_medium: BootMedium::InternalDisk,
        storage_type: st, usb_speed: us,
        is_vm: false, is_laptop: false, battery_present: false, thermal_available: true,
        confidence: DetectionConfidence::new(true),
    }
}

#[test]
fn test_kernel_b_workers_pi5() { assert_eq!(kernel_b_workers(&make_hw(4, 2048, false, false)), 3); }
#[test]
fn test_kernel_b_workers_laptop() { assert_eq!(kernel_b_workers(&make_hw(8, 8192, true, false)), 6); }
#[test]
fn test_kernel_b_workers_desktop() { assert_eq!(kernel_b_workers(&make_hw(16, 16384, false, false)), 12); }
#[test]
fn test_kernel_b_workers_server() { assert_eq!(kernel_b_workers(&make_hw(64, 65536, false, false)), 48); }
#[test]
fn test_kernel_b_workers_vm() { assert_eq!(kernel_b_workers(&make_hw(4, 4096, false, true)), 2); }

#[test]
fn test_dwp_formulas() {
    let hw = make_hw(8, 8192, false, false);
    assert_eq!(dwp_min_workers(&hw), 2);
    assert_eq!(dwp_max_workers(&hw), 6);
    assert_eq!(desktop_min_workers(&hw), 1);
    assert_eq!(system_min_workers(&hw), 2);
}

#[test]
fn test_orbit_burst_window_laptop() {
    assert_eq!(orbit_burst_window_ms(&make_hw(8, 8192, true, false)), 250);
    assert_eq!(orbit_burst_window_ms(&make_hw(8, 8192, false, false)), 500);
}

#[test]
fn test_vrm_quotas() {
    let hw = make_hw(8, 8192, false, false);
    let desk = vrm_desktop_quota_mb(&hw);
    let orbit = vrm_orbit_quota_mb(&hw);
    assert!(desk >= 64 && desk <= 512);
    assert!(orbit >= 256 && orbit <= 4096);
}

#[test]
fn test_vum_cache_by_storage() {
    let usb = make_hw_storage(StorageType::Usb, Some(UsbSpeed::Usb3Plus));
    let nvme = make_hw_storage(StorageType::Nvme, Some(UsbSpeed::Usb3Plus));
    assert!(vum_cache_mb(&usb) >= vum_cache_mb(&nvme));
}

#[test]
fn test_vum_flush_interval() {
    let usb = make_hw_storage(StorageType::Usb, Some(UsbSpeed::Usb3Plus));
    let nvme = make_hw_storage(StorageType::Nvme, Some(UsbSpeed::Usb3Plus));
    assert_eq!(vum_flush_interval_ms(&usb), 5000);
    assert_eq!(vum_flush_interval_ms(&nvme), 30000);
}

#[test]
fn test_vum_batch_size() {
    let usb3 = make_hw_storage(StorageType::Usb, Some(UsbSpeed::Usb3Plus));
    let none = make_hw_storage(StorageType::Ssd, None);
    assert_eq!(vum_batch_size_kb(&usb3), 256);
    assert_eq!(vum_batch_size_kb(&none), 128);
}

#[test]
fn test_samaris_budget_cap() {
    let hw_low = make_hw(4, 1024, false, false);
    let hw_mid = make_hw(4, 4096, false, false);
    let hw_high = make_hw(8, 16384, false, false);
    assert_eq!(samaris_budget_cap(&hw_low), 563);
    assert_eq!(samaris_budget_cap(&hw_mid), 2662);
    assert_eq!(samaris_budget_cap(&hw_high), 12288);
}
