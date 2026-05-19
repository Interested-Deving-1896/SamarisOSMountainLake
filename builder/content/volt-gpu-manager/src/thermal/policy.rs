use crate::scheduler::priority::GpuPriority;
use crate::thermal::state::{ThermalLevel, ThermalState};

#[derive(Debug, Clone)]
pub struct ThermalPolicy {
    pub backoff_at_80: bool,
    pub disable_orbit_burst_at_85: bool,
    pub desktop_only_at_95: bool,
    pub cpu_fallback_at_100: bool,
}

impl Default for ThermalPolicy {
    fn default() -> Self {
        ThermalPolicy {
            backoff_at_80: true,
            disable_orbit_burst_at_85: true,
            desktop_only_at_95: true,
            cpu_fallback_at_100: true,
        }
    }
}

impl ThermalPolicy {
    pub fn should_block_priority(&self, state: &ThermalState, priority: GpuPriority) -> bool {
        if state.level >= ThermalLevel::Emergency {
            return true;
        }
        if state.level >= ThermalLevel::Critical {
            return priority != GpuPriority::Critical;
        }
        if state.level >= ThermalLevel::Throttle && self.backoff_at_80 {
            return matches!(priority, GpuPriority::Idle);
        }
        if state.level >= ThermalLevel::Hot {
            return matches!(priority, GpuPriority::Idle);
        }
        false
    }

    pub fn should_cpu_fallback(&self, state: &ThermalState) -> bool {
        if !self.cpu_fallback_at_100 {
            return false;
        }
        matches!(state.level, ThermalLevel::Emergency | ThermalLevel::Fatal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(level: ThermalLevel) -> ThermalState {
        ThermalState::new(level, 0.0)
    }

    #[test]
    fn test_default_policy() {
        let p = ThermalPolicy::default();
        assert!(p.backoff_at_80);
        assert!(p.disable_orbit_burst_at_85);
        assert!(p.desktop_only_at_95);
        assert!(p.cpu_fallback_at_100);
    }

    #[test]
    fn test_should_block_emergency() {
        let p = ThermalPolicy::default();
        let s = make_state(ThermalLevel::Emergency);
        assert!(p.should_block_priority(&s, GpuPriority::Critical));
        assert!(p.should_block_priority(&s, GpuPriority::Idle));
    }

    #[test]
    fn test_should_block_critical() {
        let p = ThermalPolicy::default();
        let s = make_state(ThermalLevel::Critical);
        assert!(!p.should_block_priority(&s, GpuPriority::Critical));
        assert!(p.should_block_priority(&s, GpuPriority::Normal));
    }

    #[test]
    fn test_should_block_throttle() {
        let p = ThermalPolicy::default();
        let s = make_state(ThermalLevel::Throttle);
        assert!(!p.should_block_priority(&s, GpuPriority::Critical));
        assert!(!p.should_block_priority(&s, GpuPriority::High));
        assert!(!p.should_block_priority(&s, GpuPriority::Normal));
        assert!(p.should_block_priority(&s, GpuPriority::Idle));
    }

    #[test]
    fn test_should_block_normal_no_pressure() {
        let p = ThermalPolicy::default();
        let s = make_state(ThermalLevel::Normal);
        assert!(!p.should_block_priority(&s, GpuPriority::Idle));
        let w = make_state(ThermalLevel::Warm);
        assert!(!p.should_block_priority(&w, GpuPriority::Normal));
    }

    #[test]
    fn test_cpu_fallback() {
        let p = ThermalPolicy::default();
        let n = make_state(ThermalLevel::Normal);
        assert!(!p.should_cpu_fallback(&n));
        let c = make_state(ThermalLevel::Critical);
        assert!(!p.should_cpu_fallback(&c));
        let e = make_state(ThermalLevel::Emergency);
        assert!(p.should_cpu_fallback(&e));
        let f = make_state(ThermalLevel::Fatal);
        assert!(p.should_cpu_fallback(&f));
    }

    #[test]
    fn test_disabled_fallback() {
        let p = ThermalPolicy {
            cpu_fallback_at_100: false,
            ..ThermalPolicy::default()
        };
        let f = make_state(ThermalLevel::Fatal);
        assert!(!p.should_cpu_fallback(&f));
    }
}
