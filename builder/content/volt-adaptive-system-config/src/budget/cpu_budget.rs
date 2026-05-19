use crate::hardware::profile::HardwareProfile;
use crate::policies::kernel_b::kernel_b_workers;

#[derive(Debug, Clone)]
pub struct CpuBudget {
    pub kernel_b_workers: usize,
    pub estimated_total_min_workers: usize,
    pub estimated_total_max_workers: usize,
}

impl CpuBudget {
    pub fn compute(hw: &HardwareProfile) -> Self {
        let kb = kernel_b_workers(hw);

        CpuBudget {
            kernel_b_workers: kb,
            estimated_total_min_workers: kb + 2,
            estimated_total_max_workers: kb + 16,
        }
    }
}

impl Default for CpuBudget {
    fn default() -> Self {
        CpuBudget {
            kernel_b_workers: 0,
            estimated_total_min_workers: 0,
            estimated_total_max_workers: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(cpu_cores: usize) -> HardwareProfile {
        HardwareProfile {
            cpu_cores,
            cpu_threads: cpu_cores,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb: 8192,
            ram_available_mb: 4096,
            swap_total_mb: 0,
            is_laptop: false,
            is_vm: false,
            boot_medium: BootMedium::Unknown,
            storage_type: StorageType::Unknown,
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
    fn test_cpu_budget_compute() {
        let budget = CpuBudget::compute(&hw(8));
        assert!(budget.kernel_b_workers > 0);
        assert!(budget.estimated_total_min_workers > 0);
    }

    #[test]
    fn test_default() {
        let budget = CpuBudget::default();
        assert_eq!(budget.kernel_b_workers, 0);
    }
}
