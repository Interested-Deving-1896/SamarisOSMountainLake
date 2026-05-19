use volt_gpu_manager::device::probe::GpuProbe;
use volt_gpu_manager::device::hardware::GpuHardware;
use volt_gpu_manager::backend::GpuBackendKind;

#[test]
fn probe_doesnt_panic_without_gpu() {
    let devices = GpuProbe::probe_all();
    assert!(!devices.is_empty());
}

#[test]
fn no_gpu_gives_null_backend() {
    let devices = GpuProbe::probe_all();
    let has_null = devices.iter().any(|d| d.backend == GpuBackendKind::Null);
    assert!(has_null);
}

#[test]
fn hardware_profile_minimal_valid() {
    let hw = GpuHardware::null();
    assert_eq!(hw.model, "Null GPU");
    assert_eq!(hw.vendor.name(), "Unknown");
    assert_eq!(hw.vram_total_mb, 0);
    assert_eq!(hw.backend, GpuBackendKind::Null);
}

#[test]
fn probe_null_backend_direct() {
    let hw = GpuProbe::probe_backend(GpuBackendKind::Null);
    assert!(hw.is_some());
    assert_eq!(hw.unwrap().backend, GpuBackendKind::Null);
}

#[test]
fn probe_real_gpu_returns_none_in_stub() {
    let hw = GpuProbe::probe_backend(GpuBackendKind::Vulkan);
    assert!(hw.is_none());
}
