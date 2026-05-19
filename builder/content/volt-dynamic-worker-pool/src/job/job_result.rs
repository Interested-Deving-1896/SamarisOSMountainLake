use std::time::Instant;

use crate::job::job_id::JobId;
use crate::job::job_state::JobState;

#[derive(Debug, Clone)]
pub struct JobResult<T> {
    pub job_id: JobId,
    pub state: JobState,
    pub result: Option<T>,
    pub error: Option<String>,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
    pub duration_us: u64,
}

impl<T> JobResult<T> {
    pub fn new(
        job_id: JobId,
        state: JobState,
        result: Option<T>,
        error: Option<String>,
        started_at: Option<Instant>,
        completed_at: Option<Instant>,
        duration_us: u64,
    ) -> Self {
        Self {
            job_id,
            state,
            result,
            error,
            started_at,
            completed_at,
            duration_us,
        }
    }

    pub fn success(job_id: JobId, result: T, started_at: Instant, completed_at: Instant) -> Self {
        let duration_us = completed_at.duration_since(started_at).as_micros() as u64;
        Self {
            job_id,
            state: JobState::Completed,
            result: Some(result),
            error: None,
            started_at: Some(started_at),
            completed_at: Some(completed_at),
            duration_us,
        }
    }

    pub fn failure(
        job_id: JobId,
        error: String,
        started_at: Instant,
        completed_at: Instant,
    ) -> Self {
        let duration_us = completed_at.duration_since(started_at).as_micros() as u64;
        Self {
            job_id,
            state: JobState::Failed,
            result: None,
            error: Some(error),
            started_at: Some(started_at),
            completed_at: Some(completed_at),
            duration_us,
        }
    }

    pub fn cancelled(job_id: JobId, started_at: Instant, completed_at: Instant) -> Self {
        let duration_us = completed_at.duration_since(started_at).as_micros() as u64;
        Self {
            job_id,
            state: JobState::Cancelled,
            result: None,
            error: None,
            started_at: Some(started_at),
            completed_at: Some(completed_at),
            duration_us,
        }
    }

    pub fn elapsed_us(&self) -> u64 {
        if self.duration_us > 0 {
            return self.duration_us;
        }
        if let (Some(start), Some(end)) = (self.started_at, self.completed_at) {
            return end.duration_since(start).as_micros() as u64;
        }
        if let Some(start) = self.started_at {
            return start.elapsed().as_micros() as u64;
        }
        0
    }
}
