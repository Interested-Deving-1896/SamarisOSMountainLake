use crate::device::hardware::GpuHardware;
use crate::backend::backend::GpuBackendKind;

#[derive(Debug, Clone)]
pub struct GpuAdapter {
    pub hardware: GpuHardware,
    pub is_preferred: bool,
    pub backend: GpuBackendKind,
}

impl GpuAdapter {
    pub fn new(hardware: GpuHardware) -> Self {
        let backend = hardware.backend;
        Self {
            hardware,
            is_preferred: false,
            backend,
        }
    }

    pub fn select_best(adapters: Vec<Self>) -> Option<Self> {
        if adapters.is_empty() {
            return None;
        }
        let mut best: Option<Self> = None;
        for adapter in adapters {
            let replace = match (&best, &adapter) {
                (None, _) => true,
                (Some(b), a) => {
                    if a.is_preferred && !b.is_preferred {
                        true
                    } else if a.is_preferred == b.is_preferred {
                        a.hardware.vram_total_mb > b.hardware.vram_total_mb
                    } else {
                        false
                    }
                }
            };
            if replace {
                best = Some(adapter);
            }
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::hardware::GpuVendor;

    fn make_adapter(vram_mb: u64, preferred: bool, backend: GpuBackendKind) -> GpuAdapter {
        GpuAdapter {
            hardware: GpuHardware {
                vendor: GpuVendor::Nvidia,
                model: format!("GPU {}MB", vram_mb),
                driver: String::new(),
                vram_total_mb: vram_mb,
                vram_available_mb: vram_mb,
                compute_units: 0,
                backend,
            },
            is_preferred: preferred,
            backend,
        }
    }

    #[test]
    fn test_adapter_new() {
        let hw = GpuHardware::null();
        let adapter = GpuAdapter::new(hw);
        assert_eq!(adapter.backend, GpuBackendKind::Null);
        assert!(!adapter.is_preferred);
    }

    #[test]
    fn test_select_best_empty() {
        let result = GpuAdapter::select_best(vec![]);
        assert!(result.is_none());
    }

    #[test]
    fn test_select_best_single() {
        let adapters = vec![make_adapter(1024, false, GpuBackendKind::Vulkan)];
        let best = GpuAdapter::select_best(adapters).unwrap();
        assert_eq!(best.hardware.vram_total_mb, 1024);
    }

    #[test]
    fn test_select_best_preferred_over_vram() {
        let adapters = vec![
            make_adapter(8192, false, GpuBackendKind::Vulkan),
            make_adapter(4096, true, GpuBackendKind::Wgpu),
        ];
        let best = GpuAdapter::select_best(adapters).unwrap();
        assert!(best.is_preferred);
        assert_eq!(best.hardware.vram_total_mb, 4096);
    }

    #[test]
    fn test_select_best_largest_vram_when_equal_preferred() {
        let adapters = vec![
            make_adapter(2048, true, GpuBackendKind::Vulkan),
            make_adapter(8192, true, GpuBackendKind::Wgpu),
        ];
        let best = GpuAdapter::select_best(adapters).unwrap();
        assert_eq!(best.hardware.vram_total_mb, 8192);
    }

    #[test]
    fn test_select_best_no_preferred_uses_vram() {
        let adapters = vec![
            make_adapter(1024, false, GpuBackendKind::Vulkan),
            make_adapter(4096, false, GpuBackendKind::Metal),
            make_adapter(2048, false, GpuBackendKind::Wgpu),
        ];
        let best = GpuAdapter::select_best(adapters).unwrap();
        assert_eq!(best.hardware.vram_total_mb, 4096);
        assert_eq!(best.backend, GpuBackendKind::Metal);
    }
}
