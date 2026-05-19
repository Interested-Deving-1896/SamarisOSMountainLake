use crate::backend::backend::{GpuBackend, GpuBackendKind, GpuCapabilities};
use crate::core::result::VgmResult;

pub struct WgpuBackend {
    capabilities: GpuCapabilities,
}

impl WgpuBackend {
    pub fn init() -> VgmResult<Self> {
        #[cfg(not(feature = "wgpu_backend"))]
        {
            return Err(crate::core::error::VgmError::BackendUnavailable(
                "wgpu backend feature not enabled".into(),
            ));
        }
        #[cfg(feature = "wgpu_backend")]
        {
            Self::try_init_wgpu()
        }
    }

    #[cfg(feature = "wgpu_backend")]
    fn try_init_wgpu() -> VgmResult<Self> {
        Err(crate::core::error::VgmError::BackendUnavailable(
            "wgpu adapter not found".into(),
        ))
    }
}

impl GpuBackend for WgpuBackend {
    fn kind(&self) -> GpuBackendKind {
        GpuBackendKind::Wgpu
    }

    fn capabilities(&self) -> GpuCapabilities {
        self.capabilities.clone()
    }

    fn name(&self) -> &'static str {
        "WgpuBackend"
    }

    fn is_available(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgpu_init_returns_err_without_feature() {
        let result = WgpuBackend::init();
        assert!(result.is_err());
    }

    #[test]
    fn test_wgpu_backend_kind() {
        let backend = WgpuBackend {
            capabilities: GpuCapabilities::for_backend(GpuBackendKind::Wgpu),
        };
        assert_eq!(backend.kind(), GpuBackendKind::Wgpu);
    }

    #[test]
    fn test_wgpu_backend_name() {
        let backend = WgpuBackend {
            capabilities: GpuCapabilities::for_backend(GpuBackendKind::Wgpu),
        };
        assert_eq!(backend.name(), "WgpuBackend");
    }

    #[test]
    fn test_wgpu_backend_not_available() {
        let backend = WgpuBackend {
            capabilities: GpuCapabilities::for_backend(GpuBackendKind::Wgpu),
        };
        assert!(!backend.is_available());
    }

    #[test]
    fn test_wgpu_capabilities_preset() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
        let backend = WgpuBackend {
            capabilities: caps.clone(),
        };
        assert_eq!(backend.capabilities().max_texture_size, caps.max_texture_size);
    }
}
