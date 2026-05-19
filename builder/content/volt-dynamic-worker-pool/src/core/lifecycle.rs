use std::time::Instant;

use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;
use crate::core::state::WorkerPoolState;

#[derive(Debug, Clone)]
pub struct Lifecycle {
    state: WorkerPoolState,
    started_at: Option<Instant>,
    stopped_at: Option<Instant>,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            state: WorkerPoolState::Uninitialized,
            started_at: None,
            stopped_at: None,
        }
    }

    pub fn transition(&mut self, next: WorkerPoolState) -> WorkerPoolResult<()> {
        let current = self.state;
        if !current.can_transition_to(&next) {
            return Err(WorkerPoolError::InvalidStateTransition {
                from: current.name(),
                to: next.name(),
            });
        }
        self.state = next;
        Ok(())
    }

    pub fn state(&self) -> WorkerPoolState {
        self.state
    }

    pub fn uptime_ms(&self) -> u64 {
        match (self.started_at, self.stopped_at) {
            (Some(start), None) => {
                start.elapsed().as_millis() as u64
            }
            (Some(start), Some(stop)) => {
                stop.duration_since(start).as_millis() as u64
            }
            (None, _) => 0,
        }
    }

    pub fn start(&mut self) -> WorkerPoolResult<()> {
        if self.state != WorkerPoolState::Uninitialized {
            return Err(WorkerPoolError::PoolAlreadyStarted);
        }
        self.transition(WorkerPoolState::Starting)?;
        self.transition(WorkerPoolState::Running)?;
        self.started_at = Some(Instant::now());
        self.stopped_at = None;
        Ok(())
    }

    pub fn stop(&mut self) -> WorkerPoolResult<()> {
        if !self.state.is_running() && self.state != WorkerPoolState::Draining {
            return Err(WorkerPoolError::PoolNotStarted);
        }
        self.transition(WorkerPoolState::Draining)?;
        self.transition(WorkerPoolState::Shutdown)?;
        self.stopped_at = Some(Instant::now());
        Ok(())
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_lifecycle() {
        let lc = Lifecycle::new();
        assert_eq!(lc.state(), WorkerPoolState::Uninitialized);
        assert_eq!(lc.uptime_ms(), 0);
    }

    #[test]
    fn test_start_and_stop() {
        let mut lc = Lifecycle::new();
        assert!(lc.start().is_ok());
        assert_eq!(lc.state(), WorkerPoolState::Running);
        assert!(lc.started_at.is_some());
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert!(lc.uptime_ms() > 0);

        assert!(lc.stop().is_ok());
        assert_eq!(lc.state(), WorkerPoolState::Shutdown);
        assert!(lc.stopped_at.is_some());
    }

    #[test]
    fn test_double_start_fails() {
        let mut lc = Lifecycle::new();
        assert!(lc.start().is_ok());
        assert!(lc.start().is_err());
    }

    #[test]
    fn test_stop_before_start_fails() {
        let mut lc = Lifecycle::new();
        assert!(lc.stop().is_err());
    }
}
