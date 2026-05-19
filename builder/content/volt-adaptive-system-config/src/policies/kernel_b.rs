use crate::hardware::profile::HardwareProfile;

/// Compute the number of kernel background workers.
///
/// Expected values (from specification):
/// | Cores | VM     | Workers |
/// |-------|--------|---------|
/// | 2     | false  | 2       |
/// | 2     | true   | 2       |
/// | 4     | false  | 3       |
/// | 4     | true   | 2       |
/// | 8     | false  | 6       |
/// | 8     | true   | 4       |
/// | 16    | false  | 12      |
/// | 16    | true   | 8       |
/// | 32    | false  | 24      |
/// | 32    | true   | 8       |
/// | 64    | false  | 48      |
/// | 64    | true   | 8       |
pub fn kernel_b_workers(hw: &HardwareProfile) -> usize {
    let base = hw.cpu_cores * 3 / 4;
    if hw.is_vm {
        (hw.cpu_cores / 2).max(2).min(8)
    } else {
        base.max(2).min(48)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(cpu_cores: usize, is_vm: bool) -> HardwareProfile {
        HardwareProfile {
            cpu_cores,
            cpu_threads: cpu_cores,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb: 8192,
            ram_available_mb: 4096,
            swap_total_mb: 0,
            is_laptop: false,
            is_vm,
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
    fn test_vm_workers() {
        assert_eq!(kernel_b_workers(&hw(2, false)), 2);
        assert_eq!(kernel_b_workers(&hw(2, true)), 2);
        assert_eq!(kernel_b_workers(&hw(4, false)), 3);
        assert_eq!(kernel_b_workers(&hw(4, true)), 2);
        assert_eq!(kernel_b_workers(&hw(8, false)), 6);
        assert_eq!(kernel_b_workers(&hw(8, true)), 4);
        assert_eq!(kernel_b_workers(&hw(16, false)), 12);
        assert_eq!(kernel_b_workers(&hw(16, true)), 8);
        assert_eq!(kernel_b_workers(&hw(64, false)), 48);
        assert_eq!(kernel_b_workers(&hw(64, true)), 8);
    }

    #[test]
    fn test_min_workers() {
        assert_eq!(kernel_b_workers(&hw(0, false)), 2);
        assert_eq!(kernel_b_workers(&hw(0, true)), 2);
    }
}
