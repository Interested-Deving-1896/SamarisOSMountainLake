use crate::hardware::profile::HardwareProfile;

pub fn samaris_budget_cap(hw: &HardwareProfile) -> u64 {
    let percent = if hw.ram_total_mb < 2048 {
        55
    } else if hw.ram_total_mb < 8192 {
        65
    } else {
        75
    };
    (hw.ram_total_mb * percent) / 100
}

pub fn compute_os_reservation_mb(total_ram_mb: u64) -> u64 {
    if total_ram_mb < 2048 {
        512
    } else if total_ram_mb < 4096 {
        768
    } else if total_ram_mb < 8192 {
        1024
    } else if total_ram_mb < 16384 {
        1536
    } else {
        2048
    }
}

pub fn available_for_samaris_mb(total_ram_mb: u64) -> u64 {
    let reserved = compute_os_reservation_mb(total_ram_mb);
    if total_ram_mb > reserved {
        total_ram_mb - reserved
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(ram_total_mb: u64) -> HardwareProfile {
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
    fn test_budget_cap() {
        assert_eq!(samaris_budget_cap(&hw(1024)), 563);
        assert_eq!(samaris_budget_cap(&hw(4096)), 2662);
        assert_eq!(samaris_budget_cap(&hw(16384)), 12288);
    }

    #[test]
    fn test_os_reservation() {
        assert_eq!(compute_os_reservation_mb(1024), 512);
        assert_eq!(compute_os_reservation_mb(6144), 1024);
        assert_eq!(compute_os_reservation_mb(32768), 2048);
    }
}
