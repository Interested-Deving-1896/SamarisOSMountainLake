use std::time::Instant;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Init,
    Config,
    Backend,
    Devices,
    Resources,
    VramReady,
    Running,
    Shutdown,
}

#[derive(Debug, Clone)]
pub struct Lifecycle {
    phase: LifecyclePhase,
    start_time: Instant,
    transitions: Vec<(LifecyclePhase, Instant)>,
}

impl Lifecycle {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            phase: LifecyclePhase::Init,
            start_time: now,
            transitions: vec![(LifecyclePhase::Init, now)],
        }
    }

    pub fn transition(&mut self, next: LifecyclePhase) -> VgmResult<()> {
        let allowed = match (self.phase, next) {
            (LifecyclePhase::Init, LifecyclePhase::Config) => true,
            (LifecyclePhase::Config, LifecyclePhase::Backend) => true,
            (LifecyclePhase::Backend, LifecyclePhase::Devices) => true,
            (LifecyclePhase::Devices, LifecyclePhase::Resources) => true,
            (LifecyclePhase::Resources, LifecyclePhase::VramReady) => true,
            (LifecyclePhase::VramReady, LifecyclePhase::Running) => true,
            (LifecyclePhase::Running, LifecyclePhase::Shutdown) => true,
            _ => false,
        };
        if !allowed {
            return Err(VgmError::InvalidState(format!(
                "Cannot transition lifecycle from {:?} to {:?}",
                self.phase, next
            )));
        }
        self.phase = next;
        self.transitions.push((next, Instant::now()));
        Ok(())
    }

    pub fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    pub fn uptime_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    pub fn phase_duration_ms(&self) -> u64 {
        if let Some((_, start)) = self.transitions.last() {
            start.elapsed().as_millis() as u64
        } else {
            0
        }
    }

    pub fn start_time(&self) -> Instant {
        self.start_time
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
    fn test_lifecycle_ordering() {
        let mut lc = Lifecycle::new();
        assert_eq!(lc.phase(), LifecyclePhase::Init);
        assert!(lc.transition(LifecyclePhase::Config).is_ok());
        assert!(lc.transition(LifecyclePhase::Backend).is_ok());
        assert!(lc.transition(LifecyclePhase::Devices).is_ok());
        assert!(lc.transition(LifecyclePhase::Resources).is_ok());
        assert!(lc.transition(LifecyclePhase::VramReady).is_ok());
        assert!(lc.transition(LifecyclePhase::Running).is_ok());
        assert!(lc.transition(LifecyclePhase::Shutdown).is_ok());
    }

    #[test]
    fn test_invalid_lifecycle_transition() {
        let mut lc = Lifecycle::new();
        assert!(lc.transition(LifecyclePhase::Running).is_err());
        assert_eq!(lc.phase(), LifecyclePhase::Init);
    }

    #[test]
    fn test_uptime() {
        let lc = Lifecycle::new();
        assert!(lc.uptime_ms() >= 0);
    }
}
