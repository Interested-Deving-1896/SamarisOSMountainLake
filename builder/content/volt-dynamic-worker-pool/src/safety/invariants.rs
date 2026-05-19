use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;
use crate::core::state::WorkerPoolState;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::worker::WorkerState;

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check_worker_count(current: u32, min: u32, max: u32) -> WorkerPoolResult<()> {
        if current < min {
            return Err(WorkerPoolError::InternalInvariantViolation(format!(
                "Worker count {} is below minimum {}",
                current, min
            )));
        }
        if current > max {
            return Err(WorkerPoolError::InternalInvariantViolation(format!(
                "Worker count {} exceeds maximum {}",
                current, max
            )));
        }
        Ok(())
    }

    pub fn check_desktop_min(current: u32, min: u32) -> WorkerPoolResult<()> {
        if current < min {
            return Err(WorkerPoolError::InternalInvariantViolation(format!(
                "Desktop worker count {} is below minimum {}",
                current, min
            )));
        }
        Ok(())
    }

    pub fn check_no_busy_kill(state: WorkerState) -> WorkerPoolResult<()> {
        if state == WorkerState::Busy {
            return Err(WorkerPoolError::InternalInvariantViolation(
                "Cannot kill a busy worker".into(),
            ));
        }
        Ok(())
    }

    pub fn check_valid_transition(
        from: WorkerPoolState,
        to: WorkerPoolState,
    ) -> WorkerPoolResult<()> {
        if !from.can_transition_to(&to) {
            return Err(WorkerPoolError::InternalInvariantViolation(format!(
                "Invalid state transition from {:?} to {:?}",
                from, to
            )));
        }
        Ok(())
    }

    pub fn check_metrics_non_negative(snapshot: &MetricsSnapshot) -> WorkerPoolResult<()> {
        if snapshot.avg_completion_time_ms < 0.0 {
            return Err(WorkerPoolError::InternalInvariantViolation(
                "avg_completion_time_ms is negative".into(),
            ));
        }
        if snapshot.avg_queue_time_ms < 0.0 {
            return Err(WorkerPoolError::InternalInvariantViolation(
                "avg_queue_time_ms is negative".into(),
            ));
        }
        if snapshot.throughput_jobs_per_sec < 0.0 {
            return Err(WorkerPoolError::InternalInvariantViolation(
                "throughput_jobs_per_sec is negative".into(),
            ));
        }
        if snapshot.desktop_pressure < 0.0 {
            return Err(WorkerPoolError::InternalInvariantViolation(
                "desktop_pressure is negative".into(),
            ));
        }
        Ok(())
    }
}
