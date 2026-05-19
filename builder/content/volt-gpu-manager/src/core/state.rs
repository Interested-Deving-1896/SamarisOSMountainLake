use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorState {
    #[default]
    GpuUnavailable,
    BackendFailed,
    ThermalEmergency,
    FatalError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VgmState {
    #[default]
    Uninitialized,
    ConfigLoaded,
    BackendSelected,
    DeviceProbed,
    ResourcesReady,
    Running,
    Shutdown,
    Error(ErrorState),
}

impl VgmState {
    pub fn can_transition_to(&self, next: &VgmState) -> bool {
        use VgmState::*;
        match (self, next) {
            (Uninitialized, ConfigLoaded) => true,
            (Uninitialized, Error(_)) => true,
            (ConfigLoaded, BackendSelected) => true,
            (ConfigLoaded, Error(_)) => true,
            (BackendSelected, DeviceProbed) => true,
            (BackendSelected, Error(_)) => true,
            (DeviceProbed, ResourcesReady) => true,
            (DeviceProbed, Error(_)) => true,
            (ResourcesReady, Running) => true,
            (ResourcesReady, Error(_)) => true,
            (Running, Shutdown) => true,
            (Running, Error(_)) => true,
            (Shutdown, Uninitialized) => true,
            (Error(_), ConfigLoaded) => true,
            (Error(_), Uninitialized) => true,
            _ => false,
        }
    }

    pub fn transition(&mut self, next: VgmState) -> VgmResult<()> {
        if self.can_transition_to(&next) {
            *self = next;
            Ok(())
        } else {
            Err(VgmError::InvalidState(format!(
                "Cannot transition from {:?} to {:?}",
                self, next
            )))
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, VgmState::Error(_))
    }

    pub fn is_running(&self) -> bool {
        *self == VgmState::Running
    }

    pub fn is_shutdown(&self) -> bool {
        *self == VgmState::Shutdown
    }

    pub fn is_terminal(&self) -> bool {
        *self == VgmState::Shutdown
    }

    pub fn error_state(&self) -> Option<ErrorState> {
        if let VgmState::Error(e) = self {
            Some(*e)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        let mut state = VgmState::Uninitialized;
        assert!(state.transition(VgmState::ConfigLoaded).is_ok());
        assert!(state.transition(VgmState::BackendSelected).is_ok());
        assert!(state.transition(VgmState::DeviceProbed).is_ok());
        assert!(state.transition(VgmState::ResourcesReady).is_ok());
        assert!(state.transition(VgmState::Running).is_ok());
        assert!(state.transition(VgmState::Shutdown).is_ok());
        assert!(state.is_shutdown());
    }

    #[test]
    fn test_invalid_transition() {
        let mut state = VgmState::Uninitialized;
        assert!(state.transition(VgmState::Running).is_err());
    }

    #[test]
    fn test_error_recovery() {
        let mut state = VgmState::Uninitialized;
        assert!(state.transition(VgmState::Error(ErrorState::GpuUnavailable)).is_ok());
        assert!(state.is_error());
        assert_eq!(state.error_state(), Some(ErrorState::GpuUnavailable));
        assert!(state.transition(VgmState::ConfigLoaded).is_ok());
        assert!(!state.is_error());
    }

    #[test]
    fn test_thermal_emergency_from_running() {
        let mut state = VgmState::Uninitialized;
        state.transition(VgmState::ConfigLoaded).unwrap();
        state.transition(VgmState::BackendSelected).unwrap();
        state.transition(VgmState::DeviceProbed).unwrap();
        state.transition(VgmState::ResourcesReady).unwrap();
        state.transition(VgmState::Running).unwrap();
        assert!(state.transition(VgmState::Error(ErrorState::ThermalEmergency)).is_ok());
        assert!(state.is_error());
    }

    #[test]
    fn test_terminal() {
        assert!(VgmState::Shutdown.is_terminal());
        assert!(!VgmState::Running.is_terminal());
    }
}
