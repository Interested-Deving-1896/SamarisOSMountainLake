use crate::budget::system_budget::SystemBudget;
use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::policies::vrm::PressurePolicy;

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check_min_workers_positive(min: usize) -> AscResult<()> {
        if min == 0 {
            return Err(AscError::InternalInvariantViolation(
                "min_workers must be positive".into(),
            ));
        }
        Ok(())
    }

    pub fn check_desktop_protected(budget: &SystemBudget) -> AscResult<()> {
        if budget.desktop_mb < 64 {
            return Err(AscError::InternalInvariantViolation(format!(
                "desktop_mb ({}) is below minimum required (64 MB)",
                budget.desktop_mb
            )));
        }
        Ok(())
    }

    pub fn check_quotas_within_ram(budget: &SystemBudget, ram_total: u64) -> AscResult<()> {
        let total_quota = budget
            .desktop_mb
            .saturating_add(budget.orbit_mb)
            .saturating_add(budget.vrm_cache_mb)
            .saturating_add(budget.vum_cache_mb)
            .saturating_add(budget.vum_buffer_mb);
        if total_quota > ram_total {
            return Err(AscError::InternalInvariantViolation(format!(
                "total quota ({}) exceeds total RAM ({})",
                total_quota, ram_total
            )));
        }
        Ok(())
    }

    pub fn check_pressure_policy_valid(policy: &PressurePolicy) -> AscResult<()> {
        if policy.green_max_percent >= policy.yellow_enter_percent {
            return Err(AscError::InternalInvariantViolation(
                "green_max_percent must be less than yellow_enter_percent".into(),
            ));
        }
        if policy.yellow_enter_percent >= policy.orange_enter_percent {
            return Err(AscError::InternalInvariantViolation(
                "yellow_enter_percent must be less than orange_enter_percent".into(),
            ));
        }
        if policy.orange_enter_percent >= policy.red_enter_percent {
            return Err(AscError::InternalInvariantViolation(
                "orange_enter_percent must be less than red_enter_percent".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policies::vrm::PressurePolicy;

    fn valid_policy() -> PressurePolicy {
        PressurePolicy {
            green_max_percent: 30,
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

    #[test]
    fn test_min_workers_positive_ok() {
        assert!(InvariantChecker::check_min_workers_positive(1).is_ok());
    }

    #[test]
    fn test_min_workers_positive_fail() {
        assert!(InvariantChecker::check_min_workers_positive(0).is_err());
    }

    #[test]
    fn test_pressure_policy_valid_ok() {
        assert!(InvariantChecker::check_pressure_policy_valid(&valid_policy()).is_ok());
    }

    #[test]
    fn test_pressure_policy_invalid() {
        let p = PressurePolicy {
            green_max_percent: 60,
            yellow_enter_percent: 50,
            ..valid_policy()
        };
        assert!(InvariantChecker::check_pressure_policy_valid(&p).is_err());
    }
}
