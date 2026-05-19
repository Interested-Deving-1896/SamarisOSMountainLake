use crate::device::hardware::GpuHardware;
use crate::device::adapter::GpuAdapter;
use crate::device::probe::GpuProbe;
use crate::backend::backend::GpuBackendKind;
use crate::core::result::VgmResult;

pub struct GpuSelection;

impl GpuSelection {
    pub fn auto() -> VgmResult<GpuAdapter> {
        let devices = GpuProbe::probe_all();
        let adapters: Vec<GpuAdapter> = devices.into_iter().map(GpuAdapter::new).collect();
        GpuAdapter::select_best(adapters).ok_or_else(|| {
            crate::core::error::VgmError::AdapterNotFound(
                "No suitable GPU adapter found during auto-selection".into(),
            )
        })
    }

    pub fn specific(backend: GpuBackendKind) -> VgmResult<GpuAdapter> {
        let hw = GpuProbe::probe_backend(backend).ok_or_else(|| {
            crate::core::error::VgmError::AdapterNotFound(format!(
                "No adapter available for backend {:?}",
                backend
            ))
        })?;
        Ok(GpuAdapter::new(hw))
    }

    pub fn fallback() -> GpuAdapter {
        GpuAdapter::new(GpuHardware::null())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_returns_null_adapter() {
        let adapter = GpuSelection::fallback();
        assert_eq!(adapter.backend, GpuBackendKind::Null);
    }

    #[test]
    fn test_specific_null_succeeds() {
        let result = GpuSelection::specific(GpuBackendKind::Null);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().backend, GpuBackendKind::Null);
    }

    #[test]
    fn test_specific_cpu_fallback_succeeds() {
        let result = GpuSelection::specific(GpuBackendKind::CpuFallback);
        assert!(result.is_ok());
    }

    #[test]
    fn test_specific_real_gpu_fails_in_stub() {
        let result = GpuSelection::specific(GpuBackendKind::Vulkan);
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_returns_adapter() {
        let result = GpuSelection::auto();
        assert!(result.is_ok());
    }

    #[test]
    fn test_auto_adapter_has_null_fallback() {
        let adapter = GpuSelection::auto().unwrap();
        assert_eq!(adapter.hardware.backend, GpuBackendKind::Null);
    }
}
