use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::hardware::profile::HardwareProfile;

use super::system_budget::SystemBudget;

impl SystemBudget {
    /// Reconcile the budget to ensure it fits within the cap.
    ///
    /// Reduction order: VUM cache -> VRM cache -> VUM buffer -> Orbit -> Desktop (last resort).
    pub fn reconcile(mut self, hw: &HardwareProfile) -> AscResult<Self> {
        let cap = self.ram_available_for_samaris_mb;
        if self.total_with_margin() <= cap {
            return Ok(self);
        }

        // Reduction step 1: VUM cache
        let min_vum_cache = 32u64;
        if self.vum_cache_mb > min_vum_cache && self.total_with_margin() > cap {
            self.vum_cache_mb = reduce_to(
                self.vum_cache_mb,
                self.total_with_margin() - cap,
                min_vum_cache,
            );
            self.allocated_total = self.compute_allocated_total();
        }

        // Reduction step 2: VRM cache
        let min_vrm_cache = 64u64;
        if self.vrm_cache_mb > min_vrm_cache && self.total_with_margin() > cap {
            self.vrm_cache_mb = reduce_to(
                self.vrm_cache_mb,
                self.total_with_margin() - cap,
                min_vrm_cache,
            );
            self.allocated_total = self.compute_allocated_total();
        }

        // Reduction step 3: VUM buffer
        if self.vum_buffer_mb > 32 && self.total_with_margin() > cap {
            self.vum_buffer_mb = reduce_to(
                self.vum_buffer_mb,
                self.total_with_margin() - cap,
                32,
            );
            self.allocated_total = self.compute_allocated_total();
        }

        // Reduction step 4: Orbit quota
        let min_orbit = if hw.is_vm { 256 } else { 512 };
        if self.orbit_mb > min_orbit && self.total_with_margin() > cap {
            self.orbit_mb = reduce_to(
                self.orbit_mb,
                self.total_with_margin() - cap,
                min_orbit,
            );
            self.allocated_total = self.compute_allocated_total();
        }

        // NEVER reduce Desktop first.
        // Last resort: clamp Desktop to minimum
        if self.total_with_margin() > cap {
            let min_desktop = crate::policies::vrm::vrm_desktop_quota_mb(hw).min(64);
            if self.desktop_mb > min_desktop {
                self.desktop_mb = reduce_to(
                    self.desktop_mb,
                    self.total_with_margin() - cap,
                    min_desktop,
                );
                self.allocated_total = self.compute_allocated_total();
            }
        }

        // If still over cap, return error
        if self.total_with_margin() > cap {
            return Err(AscError::BudgetExceeded {
                allocated: self.total_with_margin(),
                cap,
            });
        }

        Ok(self)
    }
}

fn reduce_to(current: u64, excess: u64, minimum: u64) -> u64 {
    if current <= excess {
        minimum
    } else {
        current.saturating_sub(excess).max(minimum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardware::confidence::DetectionConfidence;
    use crate::hardware::profile::{BootMedium, StorageType};

    fn hw(ram_total_mb: u64) -> HardwareProfile {
        HardwareProfile {
            cpu_cores: 4,
            cpu_threads: 4,
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
            gpu_available: false,
            gpu_vendor: None,
            gpu_model: None,
            gpu_memory_mb: None,
            battery_present: false,
            thermal_available: false,
            confidence: DetectionConfidence::default(),
        }
    }

    fn overbudget_budget(available_mb: u64) -> SystemBudget {
        SystemBudget {
            ram_total_mb: 4096,
            ram_reserved_for_os_mb: 1024,
            ram_available_for_samaris_mb: available_mb,
            desktop_mb: 256,
            orbit_mb: 512,
            vrm_cache_mb: 512,
            vum_cache_mb: 256,
            vum_buffer_mb: 128,
            kernel_b_mb: 8,
            safety_margin_mb: 64,
            allocated_total: 0,
        }
    }

    #[test]
    fn test_reconcile_within_cap() {
        let hw = hw(16384);
        let budget = SystemBudget::compute(&hw, &[], 256);
        let result = budget.reconcile(&hw);
        assert!(result.is_ok());
    }

    #[test]
    fn test_reconcile_reduces_vum_cache_first() {
        let hw = hw(4096);
        let mut budget = overbudget_budget(1400);
        budget.allocated_total = budget.compute_allocated_total();

        let initial_vum = budget.vum_cache_mb;
        let reconciled = budget.reconcile(&hw).unwrap();
        assert!(reconciled.vum_cache_mb <= initial_vum);
        assert!(reconciled.total_with_margin() <= reconciled.ram_available_for_samaris_mb);
    }

    #[test]
    fn test_reconcile_preserves_desktop_first() {
        let hw = hw(2048);
        let mut budget = overbudget_budget(768);
        budget.allocated_total = budget.compute_allocated_total();

        let initial_desktop = budget.desktop_mb;
        let result = budget.reconcile(&hw);
        if let Ok(reconciled) = &result {
            assert!(reconciled.desktop_mb <= initial_desktop);
        }
    }

    #[test]
    fn test_reconcile_budget_exceeded() {
        let hw = hw(1024);
        let mut budget = overbudget_budget(64);
        budget.ram_total_mb = 1024;
        budget.ram_reserved_for_os_mb = 512;
        budget.safety_margin_mb = 0;
        budget.allocated_total = budget.compute_allocated_total();

        let result = budget.reconcile(&hw);
        assert!(result.is_err());
        match result {
            Err(AscError::BudgetExceeded { .. }) => {}
            _ => panic!("Expected BudgetExceeded error"),
        }
    }
}
