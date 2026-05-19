use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum VumState {
    #[default]
    Uninitialized,
    ConfigLoaded,
    DeviceDetected,
    BackingMounted,
    JournalOpened,
    RecoveryChecked,
    FuseMounted,
    Running,
    Flushing,
    Ejecting,
    Unmounted,
    Shutdown,
    DeviceMissing,
    DeviceRemoved,
    ReadOnlyFallback,
    JournalDirty,
    RecoveryRequired,
    CorruptionDetected,
    UnsafeToEject,
    FatalError,
}

impl VumState {
    pub fn can_transition_to(&self, next: &VumState) -> bool {
        use VumState::*;
        match (self, next) {
            (Uninitialized, ConfigLoaded | Shutdown) => true,
            (ConfigLoaded, DeviceDetected | DeviceMissing | Shutdown) => true,
            (DeviceDetected, BackingMounted | DeviceRemoved) => true,
            (DeviceDetected, Shutdown) => true,
            (DeviceRemoved, Shutdown) => true,
            (BackingMounted, JournalOpened | ReadOnlyFallback | Shutdown) => true,
            (JournalOpened, RecoveryChecked | JournalDirty | Shutdown) => true,
            (RecoveryChecked, FuseMounted | RecoveryRequired | CorruptionDetected) => true,
            (FuseMounted, Running) => true,
            (Running, Flushing | Ejecting | DeviceRemoved) => true,
            (Flushing, Running | Ejecting | UnsafeToEject) => true,
            (Ejecting, Unmounted) => true,
            (Unmounted, Shutdown) => true,
            (_, FatalError) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert!(VumState::Uninitialized.can_transition_to(&VumState::ConfigLoaded));
        assert!(VumState::Running.can_transition_to(&VumState::Flushing));
        assert!(VumState::Unmounted.can_transition_to(&VumState::Shutdown));
        assert!(VumState::Ejecting.can_transition_to(&VumState::Unmounted));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!VumState::Uninitialized.can_transition_to(&VumState::Running));
        assert!(!VumState::ConfigLoaded.can_transition_to(&VumState::FuseMounted));
        assert!(!VumState::Running.can_transition_to(&VumState::Uninitialized));
        assert!(!VumState::Shutdown.can_transition_to(&VumState::Running));
    }

    #[test]
    fn test_fatal_error_always_allowed() {
        assert!(VumState::Running.can_transition_to(&VumState::FatalError));
        assert!(VumState::Shutdown.can_transition_to(&VumState::FatalError));
    }
}
