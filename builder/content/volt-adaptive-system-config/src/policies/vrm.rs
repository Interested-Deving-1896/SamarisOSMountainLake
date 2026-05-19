use serde::{Deserialize, Serialize};

use crate::hardware::profile::HardwareProfile;

use super::super::budget::system_budget::SystemBudget;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PressurePolicy {
    pub green_max_percent: u8,
    pub yellow_enter_percent: u8,
    pub yellow_exit_percent: u8,
    pub orange_enter_percent: u8,
    pub orange_exit_percent: u8,
    pub red_enter_percent: u8,
    pub red_exit_percent: u8,
    pub min_free_mb_yellow: u64,
    pub min_free_mb_orange: u64,
    pub min_free_mb_red: u64,
}

impl Default for PressurePolicy {
    fn default() -> Self {
        Self {
            green_max_percent: 50,
            yellow_enter_percent: 50,
            yellow_exit_percent: 45,
            orange_enter_percent: 70,
            orange_exit_percent: 60,
            red_enter_percent: 85,
            red_exit_percent: 75,
            min_free_mb_yellow: 1024,
            min_free_mb_orange: 512,
            min_free_mb_red: 256,
        }
    }
}

/// VRM desktop quota computed from hardware (spec function)
pub fn vrm_desktop_quota_mb(hw: &HardwareProfile) -> u64 {
    (hw.ram_total_mb / 16).max(64).min(512)
}

/// VRM Orbit quota computed from hardware (spec function)
pub fn vrm_orbit_quota_mb(hw: &HardwareProfile) -> u64 {
    let base = hw.ram_total_mb / 4;
    let cap = if hw.is_vm {
        2048
    } else if hw.gpu_available {
        if hw.ram_total_mb >= 32768 { 8192 } else { 4096 }
    } else {
        if hw.ram_total_mb >= 32768 { 4096 } else { 2048 }
    };
    base.min(cap)
}

/// VRM cache from hardware (spec function)
pub fn vrm_cache_mb(hw: &HardwareProfile) -> u64 {
    let base = hw.ram_total_mb / 16;
    let cap = if hw.ram_total_mb >= 65536 {
        4096
    } else if hw.ram_total_mb >= 32768 {
        2048
    } else if hw.ram_total_mb >= 16384 {
        1024
    } else if hw.ram_total_mb >= 8192 {
        512
    } else {
        256
    };
    base.min(cap)
}

/// Desktop quota passthrough from budget (used by generator)
pub fn vrm_desktop_quota(_hw: &HardwareProfile, budget: &SystemBudget) -> u64 {
    budget.desktop_mb
}

/// Orbit quota passthrough from budget (used by generator)
pub fn vrm_orbit_quota(_hw: &HardwareProfile, budget: &SystemBudget) -> u64 {
    budget.orbit_mb
}

/// VRM cache passthrough from budget (used by generator)
pub fn vrm_cache_mb_from_budget(budget: &SystemBudget) -> u64 {
    budget.vrm_cache_mb
}

/// Compute VRM pressure policy based on hardware profile.
pub fn vrm_pressure_policy(hw: &HardwareProfile) -> PressurePolicy {
    if hw.ram_total_mb < 2048 {
        PressurePolicy {
            green_max_percent: 50,
            yellow_enter_percent: 50,
            yellow_exit_percent: 40,
            orange_enter_percent: 65,
            orange_exit_percent: 50,
            red_enter_percent: 80,
            red_exit_percent: 60,
            min_free_mb_yellow: 128,
            min_free_mb_orange: 64,
            min_free_mb_red: 32,
        }
    } else if hw.ram_total_mb < 8192 {
        PressurePolicy {
            green_max_percent: 60,
            yellow_enter_percent: 60,
            yellow_exit_percent: 50,
            orange_enter_percent: 75,
            orange_exit_percent: 60,
            red_enter_percent: 90,
            red_exit_percent: 70,
            min_free_mb_yellow: 256,
            min_free_mb_orange: 128,
            min_free_mb_red: 64,
        }
    } else {
        PressurePolicy {
            green_max_percent: 70,
            yellow_enter_percent: 70,
            yellow_exit_percent: 60,
            orange_enter_percent: 85,
            orange_exit_percent: 70,
            red_enter_percent: 95,
            red_exit_percent: 80,
            min_free_mb_yellow: 512,
            min_free_mb_orange: 256,
            min_free_mb_red: 128,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(ram_total_mb: u64, is_vm: bool, gpu: bool) -> HardwareProfile {
        HardwareProfile {
            cpu_cores: 8,
            cpu_threads: 8,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb,
            ram_available_mb: ram_total_mb,
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
    fn test_desktop_quota_bounds() {
        assert_eq!(vrm_desktop_quota_mb(&hw(1024, false, true)), 64);
        assert_eq!(vrm_desktop_quota_mb(&hw(65536, false, true)), 512);
    }

    #[test]
    fn test_orbit_quota_vm_cap() {
        assert_eq!(vrm_orbit_quota_mb(&hw(32768, true, true)), 2048);
    }

    #[test]
    fn test_cache_caps() {
        assert_eq!(vrm_cache_mb(&hw(4096, false, true)), 256);
        assert_eq!(vrm_cache_mb(&hw(16384, false, true)), 1024);
        assert_eq!(vrm_cache_mb(&hw(65536, false, true)), 4096);
    }

    #[test]
    fn test_pressure_tiers() {
        let p = vrm_pressure_policy(&hw(1024, false, true));
        assert_eq!(p.green_max_percent, 50);
        assert_eq!(p.min_free_mb_yellow, 128);

        let p = vrm_pressure_policy(&hw(4096, false, true));
        assert_eq!(p.green_max_percent, 60);
        assert_eq!(p.min_free_mb_yellow, 256);

        let p = vrm_pressure_policy(&hw(16384, false, true));
        assert_eq!(p.green_max_percent, 70);
        assert_eq!(p.min_free_mb_yellow, 512);
    }
}
