use std::time::Instant;

use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;
use crate::job::job_id::JobId;
use crate::worker::worker_id::WorkerId;
pub use crate::worker::WorkerState;
use crate::worker::worker_stats::WorkerStats;

pub struct Worker {
    pub id: WorkerId,
    pub state: WorkerState,
    pub current_job_id: Option<JobId>,
    pub created_at: Instant,
    pub stats: WorkerStats,
}

impl Worker {
    pub fn new(id: WorkerId) -> Self {
        Self {
            id,
            state: WorkerState::Idle,
            current_job_id: None,
            created_at: Instant::now(),
            stats: WorkerStats::new(),
        }
    }

    pub fn start_job(&mut self, job_id: JobId) -> WorkerPoolResult<()> {
        if !self.state.can_transition_to(&WorkerState::Busy) {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: self.state.name(),
                to: WorkerState::Busy.name(),
            });
        }
        self.stats.record_job_start(job_id.clone());
        self.state = WorkerState::Busy;
        self.current_job_id = Some(job_id);
        Ok(())
    }

    pub fn finish_job(&mut self) -> WorkerPoolResult<()> {
        if !self.state.can_transition_to(&WorkerState::Idle) {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: self.state.name(),
                to: WorkerState::Idle.name(),
            });
        }
        self.current_job_id = None;
        self.state = WorkerState::Idle;
        Ok(())
    }

    pub fn is_idle(&self) -> bool {
        self.state == WorkerState::Idle
    }

    pub fn retire(&mut self) -> WorkerPoolResult<()> {
        let target = if self.is_idle() {
            WorkerState::Stopped
        } else {
            WorkerState::Draining
        };
        if !self.state.can_transition_to(&target) {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: self.state.name(),
                to: target.name(),
            });
        }
        self.state = target;
        Ok(())
    }

    pub fn record_job_result(&mut self, success: bool, elapsed_us: u64) -> WorkerPoolResult<()> {
        if self.state != WorkerState::Busy {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: self.state.name(),
                to: if success { "complete" } else { "error" },
            });
        }
        self.stats.record_job_complete(success, elapsed_us);
        if success {
            self.state = WorkerState::Idle;
        } else {
            self.state = WorkerState::Error;
        }
        self.current_job_id = None;
        Ok(())
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.created_at.elapsed().as_secs()
    }

    pub fn reset(&mut self) {
        self.state = WorkerState::Idle;
        self.current_job_id = None;
        self.stats = WorkerStats::new();
    }
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worker")
            .field("id", &self.id)
            .field("state", &self.state.name())
            .field("current_job_id", &self.current_job_id)
            .field("created_at", &self.created_at.elapsed().as_secs_f64())
            .field("stats", &self.stats)
            .finish()
    }
}

impl std::fmt::Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Worker({}, state={}, jobs={})",
            self.id,
            self.state.name(),
            self.stats.total_jobs(),
        )
    }
}
