use std::time::Instant;

use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;
use crate::worker::worker_id::WorkerId;
use crate::worker::worker_state::WorkerState;

#[derive(Debug, Clone)]
pub struct WorkerLifecycle {
    pub id: WorkerId,
    pub created_at: Instant,
    state: WorkerState,
}

impl WorkerLifecycle {
    pub fn new(id: WorkerId) -> Self {
        Self {
            id,
            created_at: Instant::now(),
            state: WorkerState::Idle,
        }
    }

    pub fn state(&self) -> WorkerState {
        self.state
    }

    pub fn transition(&mut self, next: WorkerState) -> WorkerPoolResult<()> {
        if !self.state.can_transition_to(&next) {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: self.state.name(),
                to: next.name(),
            });
        }
        self.state = next;
        Ok(())
    }

    pub fn start(&mut self) -> WorkerPoolResult<()> {
        self.transition(WorkerState::Idle)
    }

    pub fn stop(&mut self) -> WorkerPoolResult<()> {
        self.transition(WorkerState::Stopped)
    }

    pub fn retire(&mut self) -> WorkerPoolResult<()> {
        let target = match self.state {
            WorkerState::Idle => WorkerState::Stopped,
            WorkerState::Busy => WorkerState::Draining,
            _ => {
                return Err(WorkerPoolError::InvalidStateTransition {
                    from: self.state.name(),
                    to: "retired",
                });
            }
        };
        self.transition(target)
    }

    pub fn mark_busy(&mut self) -> WorkerPoolResult<()> {
        self.transition(WorkerState::Busy)
    }

    pub fn mark_error(&mut self) -> WorkerPoolResult<()> {
        self.transition(WorkerState::Error)
    }

    pub fn elapsed_secs(&self) -> u64 {
        self.created_at.elapsed().as_secs()
    }

    pub fn age_ms(&self) -> u64 {
        self.created_at.elapsed().as_millis() as u64
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    pub fn reset(&mut self) {
        self.state = WorkerState::Idle;
    }
}

impl Default for WorkerLifecycle {
    fn default() -> Self {
        Self::new(WorkerId::zero())
    }
}
