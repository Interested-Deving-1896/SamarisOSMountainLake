use volt_gpu_manager::backend::null_backend::NullBackend;
use volt_gpu_manager::backend::{GpuBackend, GpuBackendKind, GpuCapabilities};

#[test]
fn null_backend_inits() {
    let backend = NullBackend::new();
    assert_eq!(backend.kind(), GpuBackendKind::Null);
    assert!(backend.is_available());
    assert_eq!(backend.name(), "NullBackend");
}

#[test]
fn capabilities_exposed() {
    let backend = NullBackend::new();
    let caps = backend.capabilities();
    assert_eq!(caps.backend, GpuBackendKind::Null);
    assert!(!caps.compute);
    assert!(!caps.native_texture_compression);
}

#[test]
fn backend_fallback_works() {
    let null_caps = GpuCapabilities::null();
    let backend = NullBackend::new_with_capabilities(null_caps.clone());
    assert_eq!(backend.capabilities().backend, GpuBackendKind::Null);
    let wgpu_caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
    assert!(wgpu_caps.compute);
    assert!(wgpu_caps.supports_feature("compressed_vram"));
}

#[test]
fn unsupported_feature_returns_error() {
    let caps = GpuCapabilities::null();
    assert!(!caps.supports_feature("gpu_zstd"));
    assert!(!caps.supports_feature("native_texture_compression"));
    assert!(!caps.supports_feature("does_not_exist"));
}

#[test]
fn backend_kind_real_gpu() {
    assert!(GpuBackendKind::Wgpu.is_real_gpu());
    assert!(GpuBackendKind::Vulkan.is_real_gpu());
    assert!(GpuBackendKind::Metal.is_real_gpu());
    assert!(!GpuBackendKind::Null.is_real_gpu());
    assert!(!GpuBackendKind::CpuFallback.is_real_gpu());
}

#[test]
fn null_backend_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NullBackend>();
}
