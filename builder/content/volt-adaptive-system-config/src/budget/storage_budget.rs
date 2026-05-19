use crate::hardware::profile::HardwareProfile;
use crate::policies::vum::{vum_buffer_mb, vum_cache_mb, vum_flush_interval_ms};

#[derive(Debug, Clone)]
pub struct StorageBudget {
    pub vum_cache_mb: u64,
    pub vum_buffer_mb: u64,
    pub vum_flush_interval_ms: u64,
    pub total_cache_mb: u64,
}

impl StorageBudget {
    pub fn compute(hw: &HardwareProfile) -> Self {
        let cache = vum_cache_mb(hw);
        let buffer = vum_buffer_mb(hw);
        let flush = vum_flush_interval_ms(hw);

        StorageBudget {
            vum_cache_mb: cache,
            vum_buffer_mb: buffer,
            vum_flush_interval_ms: flush,
            total_cache_mb: cache + buffer,
        }
    }
}

impl Default for StorageBudget {
    fn default() -> Self {
        StorageBudget {
            vum_cache_mb: 0,
            vum_buffer_mb: 0,
            vum_flush_interval_ms: 0,
            total_cache_mb: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(ram_total_mb: u64, storage_type: StorageType) -> HardwareProfile {
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
            storage_type,
            usb_speed: None,
            gpu_available: true,
            gpu_vendor: None,
            gpu_model: None,
            gpu_memory_mb: None,
            battery_present: false,
            thermal_available: false,
            confidence: DetectionConfidence::default(),
        }
    }

    #[test]
    fn test_storage_budget_compute() {
        let budget = StorageBudget::compute(&hw(16384, StorageType::Nvme));
        assert!(budget.vum_cache_mb > 0);
        assert!(budget.vum_buffer_mb > 0);
        assert_eq!(budget.vum_flush_interval_ms, 30000);
        assert_eq!(budget.total_cache_mb, budget.vum_cache_mb + budget.vum_buffer_mb);
    }
}
