use crate::backend::GpuHardware;
use crate::compute::compute_job::GpuComputeJobKind;

pub struct GpuRoutingPolicy;

impl GpuRoutingPolicy {
    pub fn route(devices: &[GpuHardware], kind: &GpuComputeJobKind) -> Option<usize> {
        if devices.is_empty() {
            return None;
        }
        match kind {
            GpuComputeJobKind::Blur
            | GpuComputeJobKind::Shadow
            | GpuComputeJobKind::Composite
            | GpuComputeJobKind::Transform2D => {
                devices.iter().position(|d| d.is_integrated())
                    .or(Some(0))
            }
            GpuComputeJobKind::MatMul
            | GpuComputeJobKind::VramCompress
            | GpuComputeJobKind::VramDecompress => {
                devices.iter().position(|d| d.is_discrete())
                    .or(Some(0))
            }
            _ => Some(0),
        }
    }

    pub fn round_robin(devices: &[GpuHardware], counter: &mut usize) -> Option<usize> {
        if devices.is_empty() {
            return None;
        }
        let idx = *counter % devices.len();
        *counter += 1;
        Some(idx)
    }

    pub fn least_loaded(devices: &[GpuHardware], loads: &[u64]) -> Option<usize> {
        if devices.is_empty() || loads.len() != devices.len() {
            return None;
        }
        loads.iter()
            .enumerate()
            .min_by_key(|&(_, &load)| load)
            .map(|(idx, _)| idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::GpuBackendKind;

    fn make_devices() -> Vec<GpuHardware> {
        vec![
            GpuHardware::new("Intel UHD", "Intel", GpuBackendKind::Wgpu, 512 * 1024 * 1024),
            GpuHardware::new("RTX 4060", "NVIDIA", GpuBackendKind::Vulkan, 8 * 1024 * 1024 * 1024),
        ]
    }

    #[test]
    fn test_route_desktop_to_igpu() {
        let devices = make_devices();
        let idx = GpuRoutingPolicy::route(&devices, &GpuComputeJobKind::Blur);
        assert_eq!(idx, Some(0));
    }

    #[test]
    fn test_route_compute_to_dgpu() {
        let devices = make_devices();
        let idx = GpuRoutingPolicy::route(&devices, &GpuComputeJobKind::MatMul);
        assert_eq!(idx, Some(1));
    }

    #[test]
    fn test_empty_devices() {
        assert_eq!(GpuRoutingPolicy::route(&[], &GpuComputeJobKind::Blur), None);
    }

    #[test]
    fn test_round_robin() {
        let devices = make_devices();
        let mut counter = 0;
        assert_eq!(GpuRoutingPolicy::round_robin(&devices, &mut counter), Some(0));
        assert_eq!(GpuRoutingPolicy::round_robin(&devices, &mut counter), Some(1));
        assert_eq!(GpuRoutingPolicy::round_robin(&devices, &mut counter), Some(0));
    }

    #[test]
    fn test_least_loaded() {
        let devices = make_devices();
        let loads = vec![100, 50];
        assert_eq!(GpuRoutingPolicy::least_loaded(&devices, &loads), Some(1));
    }
}
