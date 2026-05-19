use crate::budget::system_budget::SystemBudget;
use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::generator::generated_config::GeneratedConfig;

pub fn validate_worker_bounds(min: usize, max: usize) -> AscResult<()> {
    if min > max {
        return Err(AscError::ValidationFailed(format!(
            "min_workers ({}) exceeds max_workers ({})",
            min, max
        )));
    }
    Ok(())
}

pub fn validate_budget_non_negative(budget: &SystemBudget) -> AscResult<()> {
    if budget.desktop_mb > budget.ram_total_mb {
        return Err(AscError::ValidationFailed(format!(
            "desktop_mb ({}) exceeds total RAM ({})",
            budget.desktop_mb, budget.ram_total_mb
        )));
    }
    if budget.orbit_mb > budget.ram_total_mb {
        return Err(AscError::ValidationFailed(format!(
            "orbit_mb ({}) exceeds total RAM ({})",
            budget.orbit_mb, budget.ram_total_mb
        )));
    }
    if budget.allocated_total > budget.ram_total_mb {
        return Err(AscError::ValidationFailed(format!(
            "allocated_total ({}) exceeds total RAM ({})",
            budget.allocated_total, budget.ram_total_mb
        )));
    }
    Ok(())
}

pub fn validate_config_consistency(generated: &GeneratedConfig) -> AscResult<()> {
    validate_worker_bounds(
        generated.worker_pool.min_workers,
        generated.worker_pool.max_workers,
    )?;

    if generated.worker_pool.desktop_min > generated.worker_pool.max_workers {
        return Err(AscError::ValidationFailed(format!(
            "desktop_min ({}) exceeds max_workers ({})",
            generated.worker_pool.desktop_min, generated.worker_pool.max_workers
        )));
    }

    if generated.worker_pool.orbit_default_max > generated.worker_pool.max_workers {
        return Err(AscError::ValidationFailed(format!(
            "orbit_default_max ({}) exceeds max_workers ({})",
            generated.worker_pool.orbit_default_max, generated.worker_pool.max_workers
        )));
    }

    if generated.worker_pool.orbit_burst_max > generated.worker_pool.max_workers * 2 {
        return Err(AscError::ValidationFailed(format!(
            "orbit_burst_max ({}) exceeds 2x max_workers ({})",
            generated.worker_pool.orbit_burst_max,
            generated.worker_pool.max_workers
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::system_budget::SystemBudget;
    use crate::generator::generated_config::{
        GeneratedAscSection, GeneratedBudgetSection, GeneratedConfig, GeneratedKernelBConfig,
        GeneratedVrmConfig, GeneratedVumConfig, GeneratedWorkerPoolConfig,
    };

    fn make_test_config(min: usize, max: usize) -> GeneratedConfig {
        GeneratedConfig {
            kernel_b: GeneratedKernelBConfig { workers: 4 },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: min,
                max_workers: max,
                desktop_min: 1,
                system_min: 1,
                orbit_default_max: 2,
                orbit_burst_max: 4,
                orbit_burst_window_ms: 200,
            },
            vrm: GeneratedVrmConfig {
                desktop_quota_mb: 1024,
                orbit_quota_mb: 512,
                cache_mb: 256,
                pressure_green_max_percent: 50,
                pressure_yellow_enter_percent: 50,
                pressure_yellow_exit_percent: 45,
                pressure_orange_enter_percent: 70,
                pressure_orange_exit_percent: 60,
                pressure_red_enter_percent: 85,
                pressure_red_exit_percent: 75,
                min_free_mb_yellow: 1024,
                min_free_mb_orange: 512,
                min_free_mb_red: 256,
            },
            vum: GeneratedVumConfig {
                cache_mb: 128,
                buffer_mb: 64,
                flush_interval_ms: 1000,
                batch_size_kb: 32,
                journal_mode: "wal".into(),
                prefetch_boot_assets: false,
            },
            budget: GeneratedBudgetSection {
                samaris_budget_cap_mb: 4096,
                allocated_total_mb: 2048,
                safety_margin_mb: 512,
            },
            asc: GeneratedAscSection {
                profile: "balanced".into(),
                machine_classes: vec![],
                generated_at: "now".into(),
                safe_mode: false,
            },
        }
    }

    #[test]
    fn test_validate_worker_bounds_ok() {
        assert!(validate_worker_bounds(2, 8).is_ok());
    }

    #[test]
    fn test_validate_worker_bounds_fail() {
        assert!(validate_worker_bounds(8, 2).is_err());
    }

    #[test]
    fn test_validate_config_consistency_ok() {
        let config = make_test_config(2, 8);
        assert!(validate_config_consistency(&config).is_ok());
    }

    #[test]
    fn test_validate_config_consistency_fail() {
        let config = make_test_config(8, 2);
        assert!(validate_config_consistency(&config).is_err());
    }

    #[test]
    fn test_validate_budget_non_negative_ok() {
        let budget = SystemBudget::default();
        assert!(validate_budget_non_negative(&budget).is_ok());
    }
}
