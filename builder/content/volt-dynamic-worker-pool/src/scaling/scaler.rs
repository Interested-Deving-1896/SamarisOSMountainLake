use std::time::{Duration, Instant};

use crate::config::schema::WorkerPoolConfig;
use crate::scaling::decision::ScalingDecision;
use crate::scaling::thermal::ThermalState;

pub struct AdaptiveScaler {
    min_workers: u32,
    max_workers: u32,
    current_workers: u32,
    last_scale_up: Instant,
    last_scale_down: Instant,
    scale_cooldown: Duration,
    scale_up_queue_factor: f64,
    scale_down_queue_factor: f64,
    cpu_scale_up_threshold: f64,
    cpu_scale_down_threshold: f64,
}

impl AdaptiveScaler {
    pub fn new(config: &WorkerPoolConfig) -> Self {
        let scaling = &config.worker_pool.scaling;
        let cooldown = Duration::from_millis(scaling.scale_cooldown_ms);
        let past = Instant::now() - cooldown - Duration::from_secs(1);
        AdaptiveScaler {
            min_workers: scaling.min_workers_override.unwrap_or(2) as u32,
            max_workers: scaling.max_workers_override.unwrap_or(48) as u32,
            current_workers: scaling.min_workers_override.unwrap_or(2) as u32,
            last_scale_up: past,
            last_scale_down: past,
            scale_cooldown: Duration::from_millis(scaling.scale_cooldown_ms),
            scale_up_queue_factor: scaling.scale_up_queue_factor,
            scale_down_queue_factor: scaling.scale_down_queue_factor,
            cpu_scale_up_threshold: scaling.scale_up_cpu_threshold,
            cpu_scale_down_threshold: scaling.scale_down_cpu_threshold,
        }
    }

    pub fn new_with_params(
        min_workers: u32,
        max_workers: u32,
        scale_cooldown_ms: u64,
        scale_up_queue_factor: f64,
        scale_down_queue_factor: f64,
        cpu_scale_up_threshold: f64,
        cpu_scale_down_threshold: f64,
    ) -> Self {
        let current_workers = if min_workers > max_workers {
            max_workers
        } else {
            min_workers
        };
        let cooldown = Duration::from_millis(scale_cooldown_ms);
        let past = Instant::now() - cooldown - Duration::from_secs(1);
        AdaptiveScaler {
            min_workers,
            max_workers,
            current_workers,
            last_scale_up: past,
            last_scale_down: past,
            scale_cooldown: cooldown,
            scale_up_queue_factor,
            scale_down_queue_factor,
            cpu_scale_up_threshold,
            cpu_scale_down_threshold,
        }
    }

    pub fn current_workers(&self) -> u32 {
        self.current_workers
    }

    pub fn min_workers(&self) -> u32 {
        self.min_workers
    }

    pub fn max_workers(&self) -> u32 {
        self.max_workers
    }

    pub fn scale_cooldown(&self) -> Duration {
        self.scale_cooldown
    }

    pub fn set_current_workers(&mut self, count: u32) {
        self.current_workers = count.clamp(self.min_workers, self.max_workers);
    }

    pub fn should_scale_up(
        &self,
        queue_depth: u32,
        cpu_util: f64,
        thermal: &ThermalState,
        _desktop_pressure: f64,
    ) -> ScalingDecision {
        if self.current_workers >= self.max_workers {
            return ScalingDecision::NoChange {
                reason: "already at maximum worker count".to_string(),
            };
        }

        let queue_threshold = (self.current_workers as f64 * self.scale_up_queue_factor) as u32;
        if queue_depth <= queue_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "queue depth {} does not exceed threshold {} (current_workers={} * factor={})",
                    queue_depth, queue_threshold, self.current_workers, self.scale_up_queue_factor
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

        if !self.scale_up_cooldown_elapsed() {
            return ScalingDecision::NoChange {
                reason: "scale up cooldown not elapsed".to_string(),
            };
        }

        if thermal.is_throttled() {
            return ScalingDecision::NoChange {
                reason: format!("thermal backoff active: {:?}", thermal),
            };
        }

        let new_count = (self.current_workers + 1).min(self.max_workers);
        ScalingDecision::ScaleUp {
            new_count,
            reason: format!(
                "queue depth {} > threshold {}, cpu {:.2} < {:.2}, thermal not throttled",
                queue_depth, queue_threshold, cpu_util, self.cpu_scale_up_threshold
            ),
        }
    }

    pub fn should_scale_down(
        &self,
        queue_depth: u32,
        cpu_util: f64,
        thermal: &ThermalState,
        _desktop_pressure: f64,
    ) -> ScalingDecision {
        if self.current_workers <= self.min_workers {
            return ScalingDecision::NoChange {
                reason: "already at minimum worker count".to_string(),
            };
        }

        let queue_threshold = (self.current_workers as f64 * self.scale_down_queue_factor) as u32;
        if queue_depth >= queue_threshold {
            return ScalingDecision::NoChange {
                reason: format!(
                    "queue depth {} at or above threshold {} (current_workers={} * factor={})",
                    queue_depth, queue_threshold, self.current_workers, self.scale_down_queue_factor
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

        if !self.scale_down_cooldown_elapsed() {
            return ScalingDecision::NoChange {
                reason: "scale down cooldown not elapsed".to_string(),
            };
        }

        if *thermal == ThermalState::Critical {
            return ScalingDecision::NoChange {
                reason: "critical thermal state prevents scale down".to_string(),
            };
        }

        if self.current_workers > 0 && queue_depth >= self.current_workers {
            return ScalingDecision::NoChange {
                reason: "critical backlog present, cannot scale down".to_string(),
            };
        }

        let new_count = self.current_workers.saturating_sub(1).max(self.min_workers);
        ScalingDecision::ScaleDown {
            new_count,
            reason: format!(
                "queue depth {} < threshold {}, cpu {:.2} < {:.2}, no backlog",
                queue_depth, queue_threshold, cpu_util, self.cpu_scale_down_threshold
            ),
        }
    }

    pub fn record_scale_up(&mut self) {
        self.current_workers = (self.current_workers + 1).min(self.max_workers);
        self.last_scale_up = Instant::now();
    }

    pub fn record_scale_down(&mut self) {
        self.current_workers = self.current_workers.saturating_sub(1).max(self.min_workers);
        self.last_scale_down = Instant::now();
    }

    fn scale_up_cooldown_elapsed(&self) -> bool {
        Instant::now().duration_since(self.last_scale_up) >= self.scale_cooldown
    }

    fn scale_down_cooldown_elapsed(&self) -> bool {
        Instant::now().duration_since(self.last_scale_down) >= self.scale_cooldown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> WorkerPoolConfig {
        WorkerPoolConfig::default()
    }

    #[test]
    fn test_scaler_initial_workers() {
        let scaler = AdaptiveScaler::new(&test_config());
        assert!(scaler.current_workers() >= scaler.min_workers());
        assert!(scaler.current_workers() <= scaler.max_workers());
    }

    #[test]
    fn test_scale_up_when_queue_depth_exceeds_threshold() {
        let scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        let decision = scaler.should_scale_up(10, 0.50, &ThermalState::Normal, 0.0);
        assert!(decision.is_scale_up());
        assert_eq!(decision.new_count(), Some(3));
    }

    #[test]
    fn test_scale_up_no_change_when_at_max() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 4, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(4);
        let decision = scaler.should_scale_up(10, 0.50, &ThermalState::Normal, 0.0);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_up_no_change_when_cpu_high() {
        let scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        let decision = scaler.should_scale_up(10, 0.85, &ThermalState::Normal, 0.0);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_up_no_change_when_thermal_throttled() {
        let scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        let decision = scaler.should_scale_up(10, 0.50, &ThermalState::Hot, 0.0);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_when_queue_low() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(8);
        let decision = scaler.should_scale_down(1, 0.15, &ThermalState::Normal, 0.0);
        assert!(decision.is_scale_down());
        assert_eq!(decision.new_count(), Some(7));
    }

    #[test]
    fn test_scale_down_no_change_when_at_min() {
        let scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        let decision = scaler.should_scale_down(1, 0.15, &ThermalState::Normal, 0.0);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_scale_down_no_change_with_backlog() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(8);
        let decision = scaler.should_scale_down(8, 0.15, &ThermalState::Normal, 0.0);
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_record_scale_up_increases_count() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        let before = scaler.current_workers();
        scaler.record_scale_up();
        assert_eq!(scaler.current_workers(), before + 1);
    }

    #[test]
    fn test_record_scale_down_decreases_count() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(8);
        scaler.record_scale_down();
        assert_eq!(scaler.current_workers(), 7);
    }

    #[test]
    fn test_record_scale_down_respects_min() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.record_scale_down();
        assert_eq!(scaler.current_workers(), 2);
    }

    #[test]
    fn test_record_scale_up_respects_max() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 2, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.record_scale_up();
        assert_eq!(scaler.current_workers(), 2);
    }

    #[test]
    fn test_scale_up_insufficient_queue_depth() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(8);
        let decision = scaler.should_scale_up(10, 0.50, &ThermalState::Normal, 0.0);
        // 10 <= 8*2.0 = 16, so no scale
        assert!(decision.is_no_change());
    }

    #[test]
    fn test_set_current_workers_clamps() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.set_current_workers(100);
        assert_eq!(scaler.current_workers(), 16);
        scaler.set_current_workers(0);
        assert_eq!(scaler.current_workers(), 2);
    }

    #[test]
    fn test_scale_up_cooldown_respected() {
        let mut scaler = AdaptiveScaler::new_with_params(2, 16, 5000, 2.0, 0.5, 0.80, 0.30);
        scaler.record_scale_up();
        let decision = scaler.should_scale_up(10, 0.50, &ThermalState::Normal, 0.0);
        assert!(decision.is_no_change());
    }
}
