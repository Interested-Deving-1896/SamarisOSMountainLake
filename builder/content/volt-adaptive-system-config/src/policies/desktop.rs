use crate::hardware::profile::HardwareProfile;
use crate::policies::vrm::vrm_desktop_quota_mb;

#[derive(Debug, Clone)]
pub struct DesktopPolicy {
    pub min_workers: usize,
    pub vram_quota_mb: u64,
    pub enable_transparency: bool,
    pub enable_animations: bool,
    pub enable_blur: bool,
}

impl DesktopPolicy {
    pub fn from_hardware(hw: &HardwareProfile) -> Self {
        DesktopPolicy {
            min_workers: 1,
            vram_quota_mb: vrm_desktop_quota_mb(hw),
            enable_transparency: hw.gpu_available,
            enable_animations: hw.gpu_available && hw.ram_total_mb >= 4096,
            enable_blur: hw.gpu_available && hw.ram_total_mb >= 8192,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(ram_total_mb: u64, gpu: bool) -> HardwareProfile {
        HardwareProfile {
            cpu_cores: 8,
            cpu_threads: 8,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb,
            ram_available_mb: ram_total_mb,
            swap_total_mb: 0,
            is_laptop: false,
            is_vm: false,
            boot_medium: BootMedium::Unknown,
            storage_type: StorageType::Unknown,
            usb_speed: None,
            gpu_available: gpu,
            gpu_vendor: None,
            gpu_model: None,
            gpu_memory_mb: None,
            battery_present: false,
            thermal_available: false,
            confidence: DetectionConfidence::default(),
        }
    }

    #[test]
    fn test_desktop_policy_basics() {
        let policy = DesktopPolicy::from_hardware(&hw(8192, true));
        assert_eq!(policy.min_workers, 1);
        assert!(policy.enable_transparency);
        assert!(policy.enable_animations);
        assert!(policy.enable_blur);
    }

    #[test]
    fn test_desktop_policy_no_gpu() {
        let policy = DesktopPolicy::from_hardware(&hw(8192, false));
        assert!(!policy.enable_transparency);
        assert!(!policy.enable_animations);
        assert!(!policy.enable_blur);
    }
}
