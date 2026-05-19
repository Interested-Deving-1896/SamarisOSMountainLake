use crate::hardware::profile::HardwareProfile;
use crate::policies::vrm::vrm_orbit_quota_mb;
use crate::policies::worker_pool::{
    orbit_burst_max_workers, orbit_burst_window_ms, orbit_default_max_workers,
};

#[derive(Debug, Clone)]
pub struct OrbitPolicy {
    pub default_max_workers: usize,
    pub burst_max_workers: usize,
    pub burst_window_ms: u64,
    pub vram_quota_mb: u64,
    pub enable_gpu_accel: bool,
    pub priority: u8,
}

impl OrbitPolicy {
    pub fn from_hardware(hw: &HardwareProfile) -> Self {
        OrbitPolicy {
            default_max_workers: orbit_default_max_workers(hw),
            burst_max_workers: orbit_burst_max_workers(hw),
            burst_window_ms: orbit_burst_window_ms(hw),
            vram_quota_mb: vrm_orbit_quota_mb(hw),
            enable_gpu_accel: hw.gpu_available,
            priority: if hw.is_vm { 5 } else { 8 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(cpu_cores: usize, is_vm: bool, gpu: bool) -> HardwareProfile {
        HardwareProfile {
            cpu_cores,
            cpu_threads: cpu_cores,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb: 16384,
            ram_available_mb: 8192,
            swap_total_mb: 0,
            is_laptop: false,
            is_vm,
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
    fn test_orbit_policy_basic() {
        let policy = OrbitPolicy::from_hardware(&hw(8, false, true));
        assert!(policy.default_max_workers > 0);
        assert!(policy.burst_max_workers >= policy.default_max_workers);
        assert!(policy.vram_quota_mb > 0);
        assert!(policy.enable_gpu_accel);
        assert_eq!(policy.priority, 8);
    }

    #[test]
    fn test_orbit_policy_vm_priority() {
        let policy = OrbitPolicy::from_hardware(&hw(8, true, false));
        assert_eq!(policy.priority, 5);
    }
}
