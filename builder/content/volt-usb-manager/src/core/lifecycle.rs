use std::time::Instant;

use crate::core::error::VumError;
use crate::core::result::VumResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Init = 0,
    ConfigLoaded = 1,
    JournalOpened = 2,
    RecoveryDone = 3,
    FuseMounted = 4,
    Running = 5,
    Shutdown = 6,
}

pub struct Lifecycle {
    phase: LifecyclePhase,
    start_time: Instant,
}

impl Lifecycle {
    pub fn new() -> Self {
        Lifecycle {
            phase: LifecyclePhase::Init,
            start_time: Instant::now(),
        }
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> VumResult<()> {
        if (next as u8) < (self.phase as u8) && next != LifecyclePhase::Shutdown {
            return Err(VumError::InternalInvariantViolation(format!(
                "Cannot transition from {:?} to {:?}",
                self.phase, next
            )));
        }
        self.phase = next;
        Ok(())
    }

    pub fn phase(&self) -> &LifecyclePhase {
        &self.phase
    }

    pub fn uptime_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifecycle_new() {
        let lc = Lifecycle::new();
        assert_eq!(*lc.phase(), LifecyclePhase::Init);
    }

    #[test]
    fn test_lifecycle_transition() {
        let mut lc = Lifecycle::new();
        assert!(lc.transition(LifecyclePhase::ConfigLoaded).is_ok());
        assert!(lc.transition(LifecyclePhase::Running).is_ok());
    }

    #[test]
    fn test_lifecycle_invalid_transition() {
        let mut lc = Lifecycle::new();
        assert!(lc.transition(LifecyclePhase::Running).is_ok());
        assert!(lc.transition(LifecyclePhase::ConfigLoaded).is_err());
    }

    #[test]
    fn test_uptime_monotonic() {
        let lc = Lifecycle::new();
        let u1 = lc.uptime_ms();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let u2 = lc.uptime_ms();
        assert!(u2 >= u1);
    }
}
