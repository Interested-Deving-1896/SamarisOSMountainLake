use crate::hardware::profile::HardwareProfile;

pub fn worker_pool_min_workers(hw: &HardwareProfile) -> usize {
    (hw.cpu_cores / 3).max(2).min(12)
}

pub fn worker_pool_max_workers(hw: &HardwareProfile) -> usize {
    if hw.is_vm {
        (hw.cpu_cores / 2).max(2).min(8)
    } else {
        let min = worker_pool_min_workers(hw);
        (hw.cpu_cores * 3 / 4).max(min).min(48)
    }
}

pub fn worker_pool_desktop_min(_hw: &HardwareProfile) -> usize {
    1
}

pub fn worker_pool_system_min(hw: &HardwareProfile) -> usize {
    if hw.cpu_cores >= 8 { 2 } else { 1 }
}

pub fn worker_pool_orbit_default_max(hw: &HardwareProfile) -> usize {
    worker_pool_max_workers(hw) * 3 / 4
}

pub fn worker_pool_orbit_burst_max(hw: &HardwareProfile) -> usize {
    worker_pool_max_workers(hw)
}

pub fn worker_pool_orbit_burst_window(hw: &HardwareProfile) -> u64 {
    if hw.is_laptop || hw.battery_present {
        250
    } else {
        500
    }
}

pub fn worker_pool_desktop_from_budget(budget: &crate::budget::system_budget::SystemBudget) -> usize {
    if budget.desktop_mb >= 256 { 2 } else { 1 }
}

pub fn desktop_min_workers(hw: &HardwareProfile) -> usize {
    worker_pool_desktop_min(hw)
}

pub fn dwp_min_workers(hw: &HardwareProfile) -> usize {
    worker_pool_min_workers(hw)
}

pub fn dwp_max_workers(hw: &HardwareProfile) -> usize {
    worker_pool_max_workers(hw)
}

pub fn system_min_workers(hw: &HardwareProfile) -> usize {
    worker_pool_system_min(hw)
}

pub fn orbit_default_max_workers(hw: &HardwareProfile) -> usize {
    worker_pool_orbit_default_max(hw)
}

pub fn orbit_burst_max_workers(hw: &HardwareProfile) -> usize {
    worker_pool_orbit_burst_max(hw)
}

pub fn orbit_burst_window_ms(hw: &HardwareProfile) -> u64 {
    worker_pool_orbit_burst_window(hw)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hw(cores: usize) -> HardwareProfile {
        crate::hardware::profile::HardwareProfile {
            cpu_cores: cores, cpu_threads: cores, cpu_model: "test".into(), cpu_arch: "x86_64".into(),
            ram_total_mb: 8192, ram_available_mb: 6000, swap_total_mb: 2048,
            gpu_available: true, gpu_vendor: None, gpu_model: None, gpu_memory_mb: None,
            boot_medium: crate::hardware::profile::BootMedium::InternalDisk,
            storage_type: crate::hardware::profile::StorageType::Ssd,
            usb_speed: None, is_vm: false, is_laptop: false, battery_present: false, thermal_available: true,
            confidence: crate::hardware::confidence::DetectionConfidence::new(true),
        }
    }

    #[test]
    fn test_min_workers() {
        assert_eq!(worker_pool_min_workers(&make_hw(4)), 2);
        assert_eq!(worker_pool_min_workers(&make_hw(8)), 2);
        assert_eq!(worker_pool_min_workers(&make_hw(16)), 5);
    }

    #[test]
    fn test_max_workers() {
        assert_eq!(worker_pool_max_workers(&make_hw(4)), 3);
        assert_eq!(worker_pool_max_workers(&make_hw(8)), 6);
        assert_eq!(worker_pool_max_workers(&make_hw(16)), 12);
    }
}
