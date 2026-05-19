use std::time::Instant;

use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::core::state::AscState;

#[derive(Debug, Clone)]
pub struct Lifecycle {
    state: AscState,
    started_at: Option<Instant>,
    completed_at: Option<Instant>,
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            state: AscState::Uninitialized,
            started_at: None,
            completed_at: None,
        }
    }

    pub fn transition(&mut self, next: AscState) -> AscResult<()> {
        let current = self.state;
        if !current.can_transition_to(&next) {
            return Err(AscError::InvalidStateTransition {
                from: current.name().into(),
                to: next.name().into(),
            });
        }
        self.state = next;
        Ok(())
    }

    pub fn state(&self) -> AscState {
        self.state
    }

    pub fn elapsed_ms(&self) -> u64 {
        match (self.started_at, self.completed_at) {
            (Some(start), None) => start.elapsed().as_millis() as u64,
            (Some(start), Some(stop)) => stop.duration_since(start).as_millis() as u64,
            (None, _) => 0,
        }
    }

    pub fn start(&mut self) -> AscResult<()> {
        self.transition(AscState::Probing)?;
        self.started_at = Some(Instant::now());
        Ok(())
    }

    pub fn complete(&mut self) -> AscResult<()> {
        self.transition(AscState::Complete)?;
        self.completed_at = Some(Instant::now());
        Ok(())
    }

    pub fn fail(&mut self) {
        self.state = AscState::Error;
        self.completed_at = Some(Instant::now());
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
        assert_eq!(lc.state(), AscState::Uninitialized);
    }

    #[test]
    fn test_start_complete() {
        let mut lc = Lifecycle::new();
        assert!(lc.start().is_ok());
        assert_eq!(lc.state(), AscState::Probing);
        assert!(lc.complete().is_ok());
        assert_eq!(lc.state(), AscState::Complete);
        let _ = lc.elapsed_ms(); // type check only
    }

    #[test]
    fn test_invalid_transition() {
        let mut lc = Lifecycle::new();
        assert!(lc.complete().is_err());
    }
}
