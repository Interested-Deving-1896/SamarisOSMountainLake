use crate::backend::backend::{GpuBackend, GpuBackendKind, GpuCapabilities};

pub struct MetalBackend {
    capabilities: GpuCapabilities,
}

impl MetalBackend {
    pub fn new() -> Self {
        Self {
            capabilities: GpuCapabilities::for_backend(GpuBackendKind::Metal),
        }
    }
}

impl Default for MetalBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuBackend for MetalBackend {
    fn kind(&self) -> GpuBackendKind {
        GpuBackendKind::Null
    }

    fn capabilities(&self) -> GpuCapabilities {
        self.capabilities.clone()
    }

    fn name(&self) -> &'static str {
        "MetalBackend (stub)"
    }

    fn is_available(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metal_backend_new() {
        let backend = MetalBackend::new();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }

    #[test]
    fn test_metal_backend_not_available() {
        let backend = MetalBackend::new();
        assert!(!backend.is_available());
    }

    #[test]
    fn test_metal_backend_name() {
        let backend = MetalBackend::new();
        assert_eq!(backend.name(), "MetalBackend (stub)");
    }

    #[test]
    fn test_metal_backend_kind_is_null_stub() {
        let backend = MetalBackend::new();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }

    #[test]
    fn test_metal_backend_default() {
        let backend: MetalBackend = Default::default();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }
}
