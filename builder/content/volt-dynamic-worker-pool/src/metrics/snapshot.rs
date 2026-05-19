use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub total_jobs_submitted: u64,
    pub total_jobs_completed: u64,
    pub total_jobs_failed: u64,
    pub total_jobs_cancelled: u64,
    pub total_jobs_timed_out: u64,
    pub active_workers: u32,
    pub idle_workers: u32,
    pub queue_depth: usize,
    pub high_priority_queue_depth: usize,
    pub avg_completion_time_ms: f64,
    pub avg_queue_time_ms: f64,
    pub throughput_jobs_per_sec: f64,
    pub uptime_ms: u64,
    pub yield_count: u64,
    pub preemption_count: u64,
    pub orbit_burst_count: u64,
    pub scaling_events: u64,
    pub desktop_pressure: f64,
    pub thermal_throttle_count: u64,
    pub worker_pool_state: String,
}

impl MetricsSnapshot {
    pub fn new() -> Self {
        Self {
            total_jobs_submitted: 0,
            total_jobs_completed: 0,
            total_jobs_failed: 0,
            total_jobs_cancelled: 0,
            total_jobs_timed_out: 0,
            active_workers: 0,
            idle_workers: 0,
            queue_depth: 0,
            high_priority_queue_depth: 0,
            avg_completion_time_ms: 0.0,
            avg_queue_time_ms: 0.0,
            throughput_jobs_per_sec: 0.0,
            uptime_ms: 0,
            yield_count: 0,
            preemption_count: 0,
            orbit_burst_count: 0,
            scaling_events: 0,
            desktop_pressure: 0.0,
            thermal_throttle_count: 0,
            worker_pool_state: String::from("uninitialized"),
        }
    }
}

impl Default for MetricsSnapshot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_creation() {
        let s = MetricsSnapshot::new();
        assert_eq!(s.total_jobs_submitted, 0);
        assert_eq!(s.active_workers, 0);
        assert_eq!(&s.worker_pool_state, "uninitialized");
    }
}
