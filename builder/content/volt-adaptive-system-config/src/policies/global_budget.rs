use crate::budget::system_budget::SystemBudget;
use crate::hardware::profile::HardwareProfile;

/// Global Samaris budget: the maximum percentage of total RAM Samaris may consume.
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

/// OS reservation in MB based on total RAM.
pub fn os_reservation_mb(hw: &HardwareProfile) -> u64 {
    if hw.ram_total_mb < 2048 {
        512
    } else if hw.ram_total_mb < 4096 {
        768
    } else if hw.ram_total_mb < 8192 {
        1024
    } else if hw.ram_total_mb < 16384 {
        1536
    } else {
        2048
    }
}

/// Available RAM for Samaris after OS reservation.
pub fn available_for_samaris_mb(hw: &HardwareProfile) -> u64 {
    let reserved = os_reservation_mb(hw);
    if hw.ram_total_mb > reserved {
        hw.ram_total_mb - reserved
    } else {
        0
    }
}

/// Budget cap passthrough from system budget — wraps hw-based cap.
pub fn samaris_budget_cap_from_budget(budget: &SystemBudget, hw: &HardwareProfile) -> u64 {
    let _ = budget;
    samaris_budget_cap(hw)
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
    fn test_budget_cap_tiers() {
        assert_eq!(samaris_budget_cap(&hw(1024)), 563);
        assert_eq!(samaris_budget_cap(&hw(4096)), 2662);
        assert_eq!(samaris_budget_cap(&hw(16384)), 12288);
    }

    #[test]
    fn test_os_reservation() {
        assert_eq!(os_reservation_mb(&hw(1024)), 512);
        assert_eq!(os_reservation_mb(&hw(3072)), 768);
        assert_eq!(os_reservation_mb(&hw(6144)), 1024);
        assert_eq!(os_reservation_mb(&hw(12288)), 1536);
        assert_eq!(os_reservation_mb(&hw(32768)), 2048);
    }
}
