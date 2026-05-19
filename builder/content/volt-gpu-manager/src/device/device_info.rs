use crate::device::hardware::{GpuHardware, GpuVendor};

#[derive(Debug, Clone)]
pub struct GpuDeviceInfo {
    pub name: String,
    pub vendor: GpuVendor,
    pub vram_mb: u64,
    pub driver_version: String,
    pub backend_name: String,
}

impl GpuDeviceInfo {
    pub fn from_hardware(hw: &GpuHardware) -> Self {
        Self {
            name: hw.model.clone(),
            vendor: hw.vendor,
            vram_mb: hw.vram_total_mb,
            driver_version: hw.driver.clone(),
            backend_name: hw.backend.name().to_string(),
        }
    }

    pub fn null() -> Self {
        Self {
            name: "Null Device".into(),
            vendor: GpuVendor::Unknown,
            vram_mb: 0,
            driver_version: "0.0.0".into(),
            backend_name: "null".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::backend::GpuBackendKind;

    #[test]
    fn test_from_hardware() {
        let hw = GpuHardware {
            vendor: GpuVendor::Nvidia,
            model: "RTX 4090".into(),
            driver: "535.98".into(),
            vram_total_mb: 24576,
            vram_available_mb: 24576,
            compute_units: 128,
            backend: GpuBackendKind::Vulkan,
        };
        let info = GpuDeviceInfo::from_hardware(&hw);
        assert_eq!(info.name, "RTX 4090");
        assert_eq!(info.vendor, GpuVendor::Nvidia);
        assert_eq!(info.vram_mb, 24576);
        assert_eq!(info.driver_version, "535.98");
        assert_eq!(info.backend_name, "vulkan");
    }

    #[test]
    fn test_null() {
        let info = GpuDeviceInfo::null();
        assert_eq!(info.name, "Null Device");
        assert_eq!(info.vendor, GpuVendor::Unknown);
        assert_eq!(info.vram_mb, 0);
        assert_eq!(info.driver_version, "0.0.0");
        assert_eq!(info.backend_name, "null");
    }

    #[test]
    fn test_from_null_hardware() {
        let hw = GpuHardware::null();
        let info = GpuDeviceInfo::from_hardware(&hw);
        assert_eq!(info.name, "Null GPU");
        assert_eq!(info.vram_mb, 0);
    }

    #[test]
    fn test_backend_name_matches() {
        let hw = GpuHardware {
            vendor: GpuVendor::Amd,
            model: String::new(),
            driver: String::new(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Wgpu,
        };
        let info = GpuDeviceInfo::from_hardware(&hw);
        assert_eq!(info.backend_name, "wgpu");
    }

    #[test]
    fn test_vendor_preserved() {
        let hw = GpuHardware {
            vendor: GpuVendor::Apple,
            model: String::new(),
            driver: String::new(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Metal,
        };
        let info = GpuDeviceInfo::from_hardware(&hw);
        assert_eq!(info.vendor, GpuVendor::Apple);
    }
}
