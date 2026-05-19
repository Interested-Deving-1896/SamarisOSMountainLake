use crate::core::result::VgmResult;
use crate::backend::GpuCapabilities;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityAction {
    AllocateBuffer,
    AllocateTexture,
    CompressResource,
    DecompressResource,
    SubmitCompute,
    SubmitRender,
    UseMultiGpu,
    UseThermalControl,
    UseShaderCache,
}

pub struct GpuCapability;

impl GpuCapability {
    pub fn check(caps: &GpuCapabilities, action: CapabilityAction) -> VgmResult<()> {
        match action {
            CapabilityAction::CompressResource | CapabilityAction::DecompressResource => {
                if !caps.compressed_vram {
                    return Err(crate::core::error::VgmError::UnsupportedFeature(
                        "Compressed VRAM not supported".into(),
                    ));
                }
            }
            CapabilityAction::SubmitCompute => {
                if !caps.compute {
                    return Err(crate::core::error::VgmError::UnsupportedFeature(
                        "Compute not supported".into(),
                    ));
                }
            }
            CapabilityAction::UseMultiGpu => {
                if !caps.multi_gpu {
                    return Err(crate::core::error::VgmError::UnsupportedFeature(
                        "Multi-GPU not supported".into(),
                    ));
                }
            }
            CapabilityAction::UseShaderCache => {
                if !caps.shader_cache {
                    return Err(crate::core::error::VgmError::UnsupportedFeature(
                        "Shader cache not supported".into(),
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn is_allowed(caps: &GpuCapabilities, action: CapabilityAction) -> bool {
        Self::check(caps, action).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::GpuBackendKind;

    #[test]
    fn test_check_compress_allowed() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
        assert!(GpuCapability::check(&caps, CapabilityAction::CompressResource).is_ok());
    }

    #[test]
    fn test_check_compute_allowed() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Vulkan);
        assert!(GpuCapability::check(&caps, CapabilityAction::SubmitCompute).is_ok());
    }

    #[test]
    fn test_check_compute_not_allowed() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Null);
        assert!(GpuCapability::check(&caps, CapabilityAction::SubmitCompute).is_err());
    }

    #[test]
    fn test_is_allowed() {
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Wgpu);
        assert!(GpuCapability::is_allowed(&caps, CapabilityAction::UseShaderCache));
        assert!(!GpuCapability::is_allowed(&caps, CapabilityAction::UseMultiGpu));
    }

    #[test]
    fn test_allocate_always_allowed() {
        let caps = GpuCapabilities::null();
        assert!(GpuCapability::check(&caps, CapabilityAction::AllocateBuffer).is_ok());
        assert!(GpuCapability::check(&caps, CapabilityAction::AllocateTexture).is_ok());
    }
}
