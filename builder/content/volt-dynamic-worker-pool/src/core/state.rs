#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkerPoolState {
    Uninitialized,
    Starting,
    Running,
    ScalingUp,
    ScalingDown,
    Draining,
    Shutdown,
    Error,
}

impl WorkerPoolState {
    pub fn can_transition_to(&self, next: &Self) -> bool {
        use WorkerPoolState::*;
        match (self, next) {
            (Uninitialized, Starting) => true,
            (Starting, Running) => true,
            (Starting, Error) => true,
            (Running, ScalingUp) => true,
            (Running, ScalingDown) => true,
            (Running, Draining) => true,
            (Running, Error) => true,
            (Running, Shutdown) => true,
            (ScalingUp, Running) => true,
            (ScalingUp, ScalingDown) => true,
            (ScalingUp, Error) => true,
            (ScalingDown, Running) => true,
            (ScalingDown, ScalingUp) => true,
            (ScalingDown, Error) => true,
            (Draining, Shutdown) => true,
            (Draining, Error) => true,
            (Error, Uninitialized) => true,
            (Shutdown, Uninitialized) => true,
            _ => false,
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running | Self::ScalingUp | Self::ScalingDown)
    }

    pub fn name(&self) -> &'static str {
        use WorkerPoolState::*;
        match self {
            Uninitialized => "uninitialized",
            Starting => "starting",
            Running => "running",
            ScalingUp => "scaling_up",
            ScalingDown => "scaling_down",
            Draining => "draining",
            Shutdown => "shutdown",
            Error => "error",
        }
    }
}

impl Default for WorkerPoolState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        assert_eq!(WorkerPoolState::default(), WorkerPoolState::Uninitialized);
    }

    #[test]
    fn test_valid_transitions() {
        assert!(WorkerPoolState::Uninitialized.can_transition_to(&WorkerPoolState::Starting));
        assert!(WorkerPoolState::Starting.can_transition_to(&WorkerPoolState::Running));
        assert!(WorkerPoolState::Running.can_transition_to(&WorkerPoolState::ScalingUp));
        assert!(WorkerPoolState::Running.can_transition_to(&WorkerPoolState::Shutdown));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!WorkerPoolState::Uninitialized.can_transition_to(&WorkerPoolState::Running));
        assert!(!WorkerPoolState::Shutdown.can_transition_to(&WorkerPoolState::Running));
    }

    #[test]
    fn test_is_running() {
        assert!(WorkerPoolState::Running.is_running());
        assert!(WorkerPoolState::ScalingUp.is_running());
        assert!(WorkerPoolState::ScalingDown.is_running());
        assert!(!WorkerPoolState::Uninitialized.is_running());
        assert!(!WorkerPoolState::Shutdown.is_running());
    }

    #[test]
    fn test_name() {
        assert_eq!(WorkerPoolState::Uninitialized.name(), "uninitialized");
        assert_eq!(WorkerPoolState::Running.name(), "running");
    }
}
