use crate::backend::backend::GpuBackendKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Apple,
    Qualcomm,
    Arm,
    Virtual,
    Unknown,
}

impl GpuVendor {
    pub fn name(&self) -> &'static str {
        match self {
            GpuVendor::Nvidia => "NVIDIA",
            GpuVendor::Amd => "AMD",
            GpuVendor::Intel => "Intel",
            GpuVendor::Apple => "Apple",
            GpuVendor::Qualcomm => "Qualcomm",
            GpuVendor::Arm => "Arm",
            GpuVendor::Virtual => "Virtual",
            GpuVendor::Unknown => "Unknown",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "nvidia" => GpuVendor::Nvidia,
            "amd" | "ati" => GpuVendor::Amd,
            "intel" => GpuVendor::Intel,
            "apple" => GpuVendor::Apple,
            "qualcomm" => GpuVendor::Qualcomm,
            "arm" => GpuVendor::Arm,
            "virtual" | "vmware" | "virtualbox" | "qemu" => GpuVendor::Virtual,
            _ => GpuVendor::Unknown,
        }
    }
}

impl std::fmt::Display for GpuVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone)]
pub struct GpuHardware {
    pub vendor: GpuVendor,
    pub model: String,
    pub driver: String,
    pub vram_total_mb: u64,
    pub vram_available_mb: u64,
    pub compute_units: u32,
    pub backend: GpuBackendKind,
}

impl GpuHardware {
    pub fn null() -> Self {
        Self {
            vendor: GpuVendor::Unknown,
            model: "Null GPU".into(),
            driver: "null".into(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Null,
        }
    }

    pub fn new(name: &str, vendor: &str, backend: GpuBackendKind, memory_total: u64) -> Self {
        Self {
            vendor: GpuVendor::from_str(vendor),
            model: name.to_string(),
            driver: String::new(),
            vram_total_mb: memory_total / (1024 * 1024),
            vram_available_mb: memory_total / (1024 * 1024),
            compute_units: 0,
            backend,
        }
    }

    pub fn is_integrated(&self) -> bool {
        matches!(self.vendor, GpuVendor::Intel | GpuVendor::Apple | GpuVendor::Arm)
    }

    pub fn is_discrete(&self) -> bool {
        matches!(self.vendor, GpuVendor::Nvidia | GpuVendor::Amd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_vendor_names() {
        assert_eq!(GpuVendor::Nvidia.name(), "NVIDIA");
        assert_eq!(GpuVendor::Amd.name(), "AMD");
        assert_eq!(GpuVendor::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_gpu_vendor_from_str() {
        assert_eq!(GpuVendor::from_str("nvidia"), GpuVendor::Nvidia);
        assert_eq!(GpuVendor::from_str("AMD"), GpuVendor::Amd);
        assert_eq!(GpuVendor::from_str("ati"), GpuVendor::Amd);
        assert_eq!(GpuVendor::from_str("Intel"), GpuVendor::Intel);
        assert_eq!(GpuVendor::from_str("Apple"), GpuVendor::Apple);
        assert_eq!(GpuVendor::from_str("UnknownVendor"), GpuVendor::Unknown);
    }

    #[test]
    fn test_gpu_hardware_null() {
        let hw = GpuHardware::null();
        assert_eq!(hw.vendor, GpuVendor::Unknown);
        assert_eq!(hw.model, "Null GPU");
        assert_eq!(hw.driver, "null");
        assert_eq!(hw.vram_total_mb, 0);
        assert_eq!(hw.backend, GpuBackendKind::Null);
    }

    #[test]
    fn test_gpu_hardware_new_backwards_compat() {
        let hw = GpuHardware::new("RTX 4090", "NVIDIA", GpuBackendKind::Vulkan, 24 * 1024 * 1024 * 1024);
        assert_eq!(hw.model, "RTX 4090");
        assert_eq!(hw.vendor, GpuVendor::Nvidia);
        assert_eq!(hw.vram_total_mb, 24576);
        assert_eq!(hw.backend, GpuBackendKind::Vulkan);
    }

    #[test]
    fn test_is_integrated() {
        let intel = GpuHardware {
            vendor: GpuVendor::Intel,
            model: String::new(),
            driver: String::new(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Wgpu,
        };
        assert!(intel.is_integrated());
        assert!(!intel.is_discrete());
    }

    #[test]
    fn test_is_discrete() {
        let nvidia = GpuHardware {
            vendor: GpuVendor::Nvidia,
            model: String::new(),
            driver: String::new(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Vulkan,
        };
        assert!(nvidia.is_discrete());
        assert!(!nvidia.is_integrated());
    }

    #[test]
    fn test_virtual_is_neither() {
        let virt = GpuHardware {
            vendor: GpuVendor::Virtual,
            model: String::new(),
            driver: String::new(),
            vram_total_mb: 0,
            vram_available_mb: 0,
            compute_units: 0,
            backend: GpuBackendKind::Null,
        };
        assert!(!virt.is_integrated());
        assert!(!virt.is_discrete());
    }

    #[test]
    fn test_vendor_display() {
        assert_eq!(format!("{}", GpuVendor::Qualcomm), "Qualcomm");
    }

    #[test]
    fn test_vendor_from_vm_strings() {
        assert_eq!(GpuVendor::from_str("vmware"), GpuVendor::Virtual);
        assert_eq!(GpuVendor::from_str("qemu"), GpuVendor::Virtual);
        assert_eq!(GpuVendor::from_str("virtualbox"), GpuVendor::Virtual);
    }

    #[test]
    fn test_gpu_hardware_available_vram_initially_equal() {
        let hw = GpuHardware::new("Test", "AMD", GpuBackendKind::Vulkan, 8 * 1024 * 1024 * 1024);
        assert_eq!(hw.vram_total_mb, hw.vram_available_mb);
    }
}
