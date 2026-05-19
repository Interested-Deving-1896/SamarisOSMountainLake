#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuBackendKind {
    Wgpu,
    Vulkan,
    Metal,
    Null,
    CpuFallback,
}

impl GpuBackendKind {
    pub fn name(&self) -> &'static str {
        match self {
            GpuBackendKind::Wgpu => "wgpu",
            GpuBackendKind::Vulkan => "vulkan",
            GpuBackendKind::Metal => "metal",
            GpuBackendKind::Null => "null",
            GpuBackendKind::CpuFallback => "cpu_fallback",
        }
    }

    pub fn is_real_gpu(&self) -> bool {
        matches!(self, GpuBackendKind::Wgpu | GpuBackendKind::Vulkan | GpuBackendKind::Metal)
    }
}

#[derive(Debug, Clone)]
pub struct GpuCapabilities {
    pub backend: GpuBackendKind,
    pub compressed_vram: bool,
    pub native_texture_compression: bool,
    pub gpu_zstd: bool,
    pub gpu_lz4: bool,
    pub multi_gpu: bool,
    pub shader_cache: bool,
    pub compute: bool,
    pub max_buffer_size: u64,
    pub max_texture_size: u32,
    pub max_compute_workgroups: u32,
}

pub trait GpuBackend: Send + Sync {
    fn kind(&self) -> GpuBackendKind;
    fn capabilities(&self) -> GpuCapabilities;
    fn name(&self) -> &'static str;
    fn is_available(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_kind_names() {
        assert_eq!(GpuBackendKind::Wgpu.name(), "wgpu");
        assert_eq!(GpuBackendKind::Vulkan.name(), "vulkan");
        assert_eq!(GpuBackendKind::Metal.name(), "metal");
        assert_eq!(GpuBackendKind::Null.name(), "null");
        assert_eq!(GpuBackendKind::CpuFallback.name(), "cpu_fallback");
    }

    #[test]
    fn test_is_real_gpu() {
        assert!(GpuBackendKind::Wgpu.is_real_gpu());
        assert!(GpuBackendKind::Vulkan.is_real_gpu());
        assert!(GpuBackendKind::Metal.is_real_gpu());
        assert!(!GpuBackendKind::Null.is_real_gpu());
        assert!(!GpuBackendKind::CpuFallback.is_real_gpu());
    }

    #[test]
    fn test_gpu_capabilities_default_backend() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Vulkan);
        assert_eq!(caps.backend, GpuBackendKind::Vulkan);
        assert!(caps.compute);
        assert_eq!(caps.max_texture_size, 32768);
        assert_eq!(caps.max_buffer_size, 4 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_gpu_capabilities_null() {
        let caps = GpuCapabilities::null();
        assert_eq!(caps.backend, GpuBackendKind::Null);
        assert!(!caps.compute);
        assert!(!caps.native_texture_compression);
    }

    #[test]
    fn test_supports_feature() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
        assert!(caps.supports_feature("compressed_vram"));
        assert!(caps.supports_feature("shader_cache"));
        assert!(!caps.supports_feature("gpu_zstd"));
        assert!(!caps.supports_feature("unknown_feature"));
    }

    #[test]
    fn test_backend_kind_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GpuBackendKind::Wgpu);
        set.insert(GpuBackendKind::Vulkan);
        set.insert(GpuBackendKind::Wgpu);
        assert_eq!(set.len(), 2);
    }
}
