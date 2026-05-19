use crate::backend::GpuHardware;
use crate::backend::GpuCapabilities;

pub struct MultiGpuPolicy;

impl MultiGpuPolicy {
    pub fn igpu_for_desktop(devices: &[GpuHardware]) -> Option<&GpuHardware> {
        devices.iter().find(|d| d.is_integrated())
    }

    pub fn dgpu_for_compute(devices: &[GpuHardware]) -> Option<&GpuHardware> {
        devices.iter().find(|d| d.is_discrete())
    }

    pub fn is_multi_gpu_viable(devices: &[GpuHardware]) -> bool {
        let igpu = devices.iter().any(|d| d.is_integrated());
        let dgpu = devices.iter().any(|d| d.is_discrete());
        igpu && dgpu && devices.len() >= 2
    }

    pub fn preferred_compute_device<'a>(devices: &'a [GpuHardware], caps: &GpuCapabilities) -> Option<&'a GpuHardware> {
        if caps.compute {
            Self::dgpu_for_compute(devices).or_else(|| devices.first())
        } else {
            devices.first()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::GpuBackendKind;

    fn integrated_gpu() -> GpuHardware {
        GpuHardware::new("Intel UHD", "Intel", GpuBackendKind::Wgpu, 512 * 1024 * 1024)
    }

    fn discrete_gpu() -> GpuHardware {
        GpuHardware::new("RTX 4060", "NVIDIA", GpuBackendKind::Vulkan, 8 * 1024 * 1024 * 1024)
    }

    #[test]
    fn test_igpu_for_desktop() {
        let devices = vec![discrete_gpu(), integrated_gpu()];
        let igpu = MultiGpuPolicy::igpu_for_desktop(&devices);
        assert!(igpu.is_some());
        assert!(igpu.unwrap().is_integrated());
    }

    #[test]
    fn test_dgpu_for_compute() {
        let devices = vec![integrated_gpu(), discrete_gpu()];
        let dgpu = MultiGpuPolicy::dgpu_for_compute(&devices);
        assert!(dgpu.is_some());
        assert!(dgpu.unwrap().is_discrete());
    }

    #[test]
    fn test_is_multi_gpu_viable() {
        let devices = vec![integrated_gpu(), discrete_gpu()];
        assert!(MultiGpuPolicy::is_multi_gpu_viable(&devices));
        assert!(!MultiGpuPolicy::is_multi_gpu_viable(&[integrated_gpu()]));
    }

    #[test]
    fn test_preferred_compute_device() {
        let devices = vec![integrated_gpu(), discrete_gpu()];
        let caps = GpuCapabilities::for_backend(GpuBackendKind::Vulkan);
        let preferred = MultiGpuPolicy::preferred_compute_device(&devices, &caps);
        assert!(preferred.is_some());
        assert!(preferred.unwrap().is_discrete());
    }
}
