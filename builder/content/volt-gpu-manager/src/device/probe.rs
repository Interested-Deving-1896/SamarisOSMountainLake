use crate::device::hardware::GpuHardware;
use crate::backend::backend::GpuBackendKind;

pub struct GpuProbe;

impl GpuProbe {
    pub fn probe_all() -> Vec<GpuHardware> {
        let mut devices = Vec::new();
        for kind in &[GpuBackendKind::Wgpu, GpuBackendKind::Vulkan, GpuBackendKind::Metal] {
            if let Some(hw) = Self::probe_backend(*kind) {
                devices.push(hw);
            }
        }
        if devices.is_empty() {
            devices.push(Self::probe_null());
        }
        devices
    }

    pub fn probe_backend(kind: GpuBackendKind) -> Option<GpuHardware> {
        match kind {
            GpuBackendKind::Null | GpuBackendKind::CpuFallback => {
                Some(GpuHardware::null())
            }
            _ => {
                if kind.is_real_gpu() {
                    // In a real implementation this would query the actual GPU.
                    // For the stub, we return None to indicate no real device found.
                    None
                } else {
                    None
                }
            }
        }
    }

    pub fn probe_null() -> GpuHardware {
        GpuHardware::null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_null_backend() {
        let hw = GpuProbe::probe_backend(GpuBackendKind::Null);
        assert!(hw.is_some());
        assert_eq!(hw.unwrap().backend, GpuBackendKind::Null);
    }

    #[test]
    fn test_probe_cpu_fallback() {
        let hw = GpuProbe::probe_backend(GpuBackendKind::CpuFallback);
        assert!(hw.is_some());
    }

    #[test]
    fn test_probe_real_gpu_returns_none_in_stub() {
        let hw = GpuProbe::probe_backend(GpuBackendKind::Vulkan);
        assert!(hw.is_none());
    }

    #[test]
    fn test_probe_all_returns_null_fallback_when_no_real_gpus() {
        let devices = GpuProbe::probe_all();
        assert!(!devices.is_empty());
        let has_null = devices.iter().any(|d| d.backend == GpuBackendKind::Null);
        assert!(has_null);
    }

    #[test]
    fn test_probe_null() {
        let hw = GpuProbe::probe_null();
        assert_eq!(hw.backend, GpuBackendKind::Null);
        assert_eq!(hw.model, "Null GPU");
    }
}
