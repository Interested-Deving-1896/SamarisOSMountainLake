use crate::backend::backend::{GpuBackendKind, GpuCapabilities};

impl Default for GpuCapabilities {
    fn default() -> Self {
        Self::for_backend(GpuBackendKind::Null)
    }
}

impl GpuCapabilities {
    pub fn for_backend(kind: GpuBackendKind) -> Self {
        let (max_buffer, max_tex, max_wg) = match kind {
            GpuBackendKind::Null | GpuBackendKind::CpuFallback => (256 * 1024 * 1024, 4096, 64),
            GpuBackendKind::Wgpu => (2 * 1024 * 1024 * 1024, 16384, 256),
            GpuBackendKind::Vulkan => (4 * 1024 * 1024 * 1024, 32768, 512),
            GpuBackendKind::Metal => (2 * 1024 * 1024 * 1024, 16384, 256),
        };
        let is_real = kind.is_real_gpu();
        GpuCapabilities {
            backend: kind,
            compressed_vram: true,
            native_texture_compression: is_real,
            gpu_zstd: false,
            gpu_lz4: false,
            multi_gpu: false,
            shader_cache: true,
            compute: is_real,
            max_buffer_size: max_buffer,
            max_texture_size: max_tex,
            max_compute_workgroups: max_wg,
        }
    }

    pub fn null() -> Self {
        Self::for_backend(GpuBackendKind::Null)
    }

    pub fn supports_feature(&self, feature: &str) -> bool {
        match feature {
            "compressed_vram" => self.compressed_vram,
            "native_texture_compression" => self.native_texture_compression,
            "gpu_zstd" => self.gpu_zstd,
            "gpu_lz4" => self.gpu_lz4,
            "multi_gpu" => self.multi_gpu,
            "shader_cache" => self.shader_cache,
            "compute" => self.compute,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::backend::GpuBackendKind;
    use crate::backend::backend::GpuCapabilities;

    #[test]
    fn test_for_backend_wgpu() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
        assert!(caps.compute);
        assert!(caps.compressed_vram);
        assert!(caps.native_texture_compression);
        assert_eq!(caps.max_texture_size, 16384);
    }

    #[test]
    fn test_for_backend_null() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Null);
        assert!(!caps.compute);
        assert!(!caps.native_texture_compression);
        assert_eq!(caps.max_texture_size, 4096);
    }

    #[test]
    fn test_for_backend_cpu_fallback() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::CpuFallback);
        assert!(!caps.compute);
        assert_eq!(caps.max_buffer_size, 256 * 1024 * 1024);
    }

    #[test]
    fn test_null_caps() {
        let caps = GpuCapabilities::null();
        assert_eq!(caps.backend, GpuBackendKind::Null);
        assert!(!caps.compute);
    }

    #[test]
    fn test_supports_feature_edge_cases() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Metal);
        assert!(caps.supports_feature("compressed_vram"));
        assert!(!caps.supports_feature("gpu_zstd"));
        assert!(!caps.supports_feature(""));
        assert!(!caps.supports_feature("ray_tracing"));
    }
}
