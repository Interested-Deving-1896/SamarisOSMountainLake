pub mod lifecycle;
pub mod worker;
pub mod worker_handle;
pub mod worker_id;
pub mod worker_loop;
pub mod worker_state;
pub mod worker_stats;

pub use worker::Worker;
pub use worker_handle::WorkerHandle;
pub use worker_stats::WorkerStats;
pub use lifecycle::WorkerLifecycle;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkerId(u32);

impl WorkerId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for WorkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "worker-{}", self.0)
    }
}

impl From<u32> for WorkerId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkerState {
    Idle,
    Busy,
    Draining,
    Stopped,
    Error,
}

impl WorkerState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Busy => "busy",
            Self::Draining => "draining",
            Self::Stopped => "stopped",
            Self::Error => "error",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Idle | Self::Busy)
    }
}

impl Default for WorkerState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub struct WorkerInfo {
    pub id: WorkerId,
    pub state: WorkerState,
    pub current_job_id: Option<crate::job::job_id::JobId>,
    pub started_at: std::time::Instant,
    pub jobs_completed: u64,
}

impl WorkerInfo {
    pub fn new(id: WorkerId) -> Self {
        Self {
            id,
            state: WorkerState::Idle,
            current_job_id: None,
            started_at: std::time::Instant::now(),
            jobs_completed: 0,
        }
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_id() {
        let wid = WorkerId::new(42);
        assert_eq!(wid.as_u32(), 42);
        assert_eq!(wid.to_string(), "worker-42");
    }

    #[test]
    fn test_worker_state() {
        assert_eq!(WorkerState::Idle.name(), "idle");
        assert!(WorkerState::Idle.is_active());
        assert!(WorkerState::Busy.is_active());
        assert!(!WorkerState::Stopped.is_active());
    }

    #[test]
    fn test_default_state() {
        assert_eq!(WorkerState::default(), WorkerState::Idle);
    }

    #[test]
    fn test_worker_info() {
        let info = WorkerInfo::new(WorkerId::new(1));
        assert_eq!(info.state, WorkerState::Idle);
        assert!(info.current_job_id.is_none());
        assert_eq!(info.jobs_completed, 0);
    }
}
