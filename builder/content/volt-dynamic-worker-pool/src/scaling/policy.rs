use crate::config::schema::ScalingSection;
use crate::scaling::decision::ScalingDecision;
use crate::scaling::thermal::ThermalState;

pub struct ScalingPolicy {
    pub min_workers: u32,
    pub max_workers: u32,
    pub scale_up_queue_factor: f64,
    pub scale_down_queue_factor: f64,
    pub scale_cooldown_ms: u64,
    pub cpu_scale_up_threshold: f64,
    pub cpu_scale_down_threshold: f64,
}

impl ScalingPolicy {
    pub fn new(config: &ScalingSection) -> Self {
        ScalingPolicy {
            min_workers: config.min_workers_override.unwrap_or(2) as u32,
            max_workers: config.max_workers_override.unwrap_or(48) as u32,
            scale_up_queue_factor: config.scale_up_queue_factor,
            scale_down_queue_factor: config.scale_down_queue_factor,
            scale_cooldown_ms: config.scale_cooldown_ms,
            cpu_scale_up_threshold: config.scale_up_cpu_threshold,
            cpu_scale_down_threshold: config.scale_down_cpu_threshold,
        }
    }

    pub fn should_scale_up(
        &self,
        queue_depth: u32,
        current_workers: u32,
        cpu_util: f64,
        thermal: &ThermalState,
    ) -> ScalingDecision {
        if current_workers >= self.max_workers {
            return ScalingDecision::NoChange {
                reason: "already at maximum workers".to_string(),
            };
        }

        let queue_threshold = (current_workers as f64 * self.scale_up_queue_factor) as u32;
        if queue_depth <= queue_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "queue depth {} does not exceed threshold {}",
                    queue_depth, queue_threshold
                ),
            };
        }

        if cpu_util >= self.cpu_scale_up_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "cpu util {:.2} at or above threshold {:.2}",
                    cpu_util, self.cpu_scale_up_threshold
                ),
            };
        }

        if thermal.is_throttled() {
            return ScalingDecision::NoChange {
                reason: format!("thermal backoff active: {:?}", thermal),
            };
        }

        let new_count = (current_workers + 1).min(self.max_workers);
        ScalingDecision::ScaleUp {
            new_count,
            reason: format!(
                "queue depth {} exceeds threshold {}, cpu util {:.2} < {:.2}",
                queue_depth, queue_threshold, cpu_util, self.cpu_scale_up_threshold
            ),
        }
    }

    pub fn should_scale_down(
        &self,
        queue_depth: u32,
        current_workers: u32,
        cpu_util: f64,
        thermal: &ThermalState,
        has_critical_backlog: bool,
    ) -> ScalingDecision {
        if current_workers <= self.min_workers {
            return ScalingDecision::NoChange {
                reason: "already at minimum workers".to_string(),
            };
        }

        if has_critical_backlog {
            return ScalingDecision::NoChange {
                reason: "critical backlog present, cannot scale down".to_string(),
            };
        }

        let queue_threshold = (current_workers as f64 * self.scale_down_queue_factor) as u32;
        if queue_depth >= queue_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "queue depth {} at or above threshold {}",
                    queue_depth, queue_threshold
                ),
            };
        }

        if cpu_util >= self.cpu_scale_down_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "cpu util {:.2} at or above threshold {:.2}",
                    cpu_util, self.cpu_scale_down_threshold
                ),
            };
        }

        if *thermal == ThermalState::Critical {
            return ScalingDecision::NoChange {
                reason: "critical thermal state prevents scale down".to_string(),
            };
        }

        let new_count = current_workers.saturating_sub(1).max(self.min_workers);
        ScalingDecision::ScaleDown {
            new_count,
            reason: format!(
                "queue depth {} below threshold {}, cpu util {:.2} < {:.2}",
                queue_depth, queue_threshold, cpu_util, self.cpu_scale_down_threshold
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_policy() -> ScalingPolicy {
        ScalingPolicy {
            min_workers: 2,
            max_workers: 16,
            scale_up_queue_factor: 2.0,
            scale_down_queue_factor: 0.5,
            scale_cooldown_ms: 5000,
            cpu_scale_up_threshold: 0.80,
            cpu_scale_down_threshold: 0.30,
        }
    }

    #[test]
    fn test_scale_up_sufficient_queue_depth() {
        let policy = default_policy();
        let decision = policy.should_scale_up(10, 4, 0.50, &ThermalState::Normal);
        assert!(decision.is_scale_up());
        assert_eq!(decision.new_count(), Some(5));
    }

    #[test]
    fn test_scale_up_insufficient_queue_depth() {
        let policy = default_policy();
        let decision = policy.should_scale_up(5, 4, 0.50, &ThermalState::Normal);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_up_cpu_util_too_high() {
        let policy = default_policy();
        let decision = policy.should_scale_up(10, 4, 0.85, &ThermalState::Normal);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_up_at_max_workers() {
        let policy = default_policy();
        let decision = policy.should_scale_up(10, 16, 0.50, &ThermalState::Normal);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_up_thermal_backoff() {
        let policy = default_policy();
        let decision = policy.should_scale_up(10, 4, 0.50, &ThermalState::Hot);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_sufficiently_low() {
        let policy = default_policy();
        let decision = policy.should_scale_down(1, 4, 0.15, &ThermalState::Normal, false);
        assert!(decision.is_scale_down());
        assert_eq!(decision.new_count(), Some(3));
    }

    #[test]
    fn test_scale_down_critical_backlog() {
        let policy = default_policy();
        let decision = policy.should_scale_down(1, 4, 0.15, &ThermalState::Normal, true);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_at_min_workers() {
        let policy = default_policy();
        let decision = policy.should_scale_down(1, 2, 0.15, &ThermalState::Normal, false);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_cpu_util_too_high() {
        let policy = default_policy();
        let decision = policy.should_scale_down(1, 4, 0.35, &ThermalState::Normal, false);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_critical_thermal() {
        let policy = default_policy();
        let decision = policy.should_scale_down(1, 4, 0.15, &ThermalState::Critical, false);
        assert!(decision.is_no_change());
    }
}
