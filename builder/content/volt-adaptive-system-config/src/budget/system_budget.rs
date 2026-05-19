use crate::classify::machine_class::MachineClass;
use crate::hardware::profile::{BootMedium, HardwareProfile, StorageType};
use crate::policies::kernel_b::kernel_b_workers;
use crate::policies::vrm::{vrm_cache_mb, vrm_desktop_quota_mb, vrm_orbit_quota_mb};
use crate::policies::vum::{vum_buffer_mb, vum_cache_mb};

use super::ram_budget::compute_os_reservation_mb;

/// Complete system budget with all Samaris allocations.
#[derive(Debug, Clone)]
pub struct SystemBudget {
    pub ram_total_mb: u64,
    pub ram_reserved_for_os_mb: u64,
    pub ram_available_for_samaris_mb: u64,
    pub desktop_mb: u64,
    pub orbit_mb: u64,
    pub vrm_cache_mb: u64,
    pub vum_cache_mb: u64,
    pub vum_buffer_mb: u64,
    pub kernel_b_mb: u64,
    pub safety_margin_mb: u64,
    pub allocated_total: u64,
}

impl SystemBudget {
    pub fn compute(
        hw: &HardwareProfile,
        _classes: &[MachineClass],
        safety_margin_mb: u64,
    ) -> Self {
        let ram_total_mb = hw.ram_total_mb;
        let ram_reserved_for_os_mb = compute_os_reservation_mb(ram_total_mb);
        let ram_available_for_samaris_mb = if ram_total_mb > ram_reserved_for_os_mb {
            ram_total_mb - ram_reserved_for_os_mb
        } else {
            0
        };

        let vrm_cache = vrm_cache_mb(hw);
        let vum_cache = vum_cache_mb(hw);
        let vum_buffer = vum_buffer_mb(hw);
        let orbit = vrm_orbit_quota_mb(hw);
        let desktop = vrm_desktop_quota_mb(hw);
        let kernel_b_mb = (kernel_b_workers(hw) as u64) * 2;

        let mut budget = SystemBudget {
            ram_total_mb,
            ram_reserved_for_os_mb,
            ram_available_for_samaris_mb,
            desktop_mb: desktop,
            orbit_mb: orbit,
            vrm_cache_mb: vrm_cache,
            vum_cache_mb: vum_cache,
            vum_buffer_mb: vum_buffer,
            kernel_b_mb,
            safety_margin_mb,
            allocated_total: 0,
        };

        budget.allocated_total = budget.compute_allocated_total();
        budget
    }

    pub(crate) fn compute_allocated_total(&self) -> u64 {
        self.desktop_mb
            + self.orbit_mb
            + self.vrm_cache_mb
            + self.vum_cache_mb
            + self.vum_buffer_mb
            + self.kernel_b_mb
    }

    pub fn allocated_total(&self) -> u64 {
        self.allocated_total
    }

    pub fn total_with_margin(&self) -> u64 {
        self.allocated_total + self.safety_margin_mb
    }

    pub fn is_within_cap(&self) -> bool {
        self.total_with_margin() <= self.ram_available_for_samaris_mb
    }

    pub fn budget_cap_mb(&self) -> u64 {
        crate::policies::global_budget::samaris_budget_cap(&HardwareProfile {
            cpu_cores: 8,
            cpu_threads: 8,
            cpu_model: "generic".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb: self.ram_total_mb,
            ram_available_mb: self.ram_total_mb,
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
            confidence: crate::hardware::confidence::DetectionConfidence::default(),
        })
    }

    // ---- Compatibility fields for existing code ----

    pub fn desktop_quota_mb(&self) -> u64 {
        self.desktop_mb
    }

    pub fn orbit_quota_mb(&self) -> u64 {
        self.orbit_mb
    }

    pub fn allocated_total_mb(&self) -> u64 {
        self.allocated_total
    }

    pub fn total_ram_mb(&self) -> u64 {
        self.ram_total_mb
    }
}

impl Default for SystemBudget {
    fn default() -> Self {
        SystemBudget {
            ram_total_mb: 0,
            ram_reserved_for_os_mb: 0,
            ram_available_for_samaris_mb: 0,
            desktop_mb: 0,
            orbit_mb: 0,
            vrm_cache_mb: 0,
            vum_cache_mb: 0,
            vum_buffer_mb: 0,
            kernel_b_mb: 0,
            safety_margin_mb: 0,
            allocated_total: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;

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
    fn test_budget_compute() {
        let hw = hw(16384);
        let budget = SystemBudget::compute(&hw, &[], 256);
        assert_eq!(budget.ram_total_mb, 16384);
        assert!(budget.ram_reserved_for_os_mb > 0);
        assert!(budget.ram_available_for_samaris_mb > 0);
        assert!(budget.allocated_total > 0);
    }

    #[test]
    fn test_default() {
        let budget = SystemBudget::default();
        assert_eq!(budget.ram_total_mb, 0);
        assert_eq!(budget.allocated_total, 0);
    }
}
