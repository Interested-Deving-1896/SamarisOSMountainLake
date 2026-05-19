use crate::backend::backend::{GpuBackend, GpuBackendKind, GpuCapabilities};

pub struct VulkanBackend {
    capabilities: GpuCapabilities,
}

impl VulkanBackend {
    pub fn new() -> Self {
        Self {
            capabilities: GpuCapabilities::for_backend(GpuBackendKind::Vulkan),
        }
    }
}

impl Default for VulkanBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl GpuBackend for VulkanBackend {
    fn kind(&self) -> GpuBackendKind {
        GpuBackendKind::Null
    }

    fn capabilities(&self) -> GpuCapabilities {
        self.capabilities.clone()
    }

    fn name(&self) -> &'static str {
        "VulkanBackend (stub)"
    }

    fn is_available(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vulkan_backend_new() {
        let backend = VulkanBackend::new();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }

    #[test]
    fn test_vulkan_backend_not_available() {
        let backend = VulkanBackend::new();
        assert!(!backend.is_available());
    }

    #[test]
    fn test_vulkan_backend_name() {
        let backend = VulkanBackend::new();
        assert_eq!(backend.name(), "VulkanBackend (stub)");
    }

    #[test]
    fn test_vulkan_backend_kind_is_null_stub() {
        let backend = VulkanBackend::new();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }

    #[test]
    fn test_vulkan_backend_default() {
        let backend: VulkanBackend = Default::default();
        assert_eq!(backend.kind(), GpuBackendKind::Null);
    }
}
