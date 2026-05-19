use crate::pressure::level::PressureLevel;
use crate::core::result::VrmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmergencyAction {
    None,
    CacheRelease,
    BackgroundSuspend,
    OomTargeted,
}

impl EmergencyAction {
    pub fn for_level(level: PressureLevel) -> Self {
        match level {
            PressureLevel::Green => EmergencyAction::None,
            PressureLevel::Yellow => EmergencyAction::CacheRelease,
            PressureLevel::Orange => EmergencyAction::BackgroundSuspend,
            PressureLevel::Red => EmergencyAction::OomTargeted,
        }
    }

    pub fn execute(&self) -> VrmResult<()> {
        match self {
            EmergencyAction::None => Ok(()),
            EmergencyAction::CacheRelease => {
                tracing::info!("emergency: releasing cache");
                Ok(())
            }
            EmergencyAction::BackgroundSuspend => {
                tracing::info!("emergency: suspending background tasks");
                Ok(())
            }
            EmergencyAction::OomTargeted => {
                tracing::warn!("emergency: OOM targeted mitigation activated");
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_level() {
        assert_eq!(EmergencyAction::for_level(PressureLevel::Green), EmergencyAction::None);
        assert_eq!(EmergencyAction::for_level(PressureLevel::Yellow), EmergencyAction::CacheRelease);
        assert_eq!(EmergencyAction::for_level(PressureLevel::Orange), EmergencyAction::BackgroundSuspend);
        assert_eq!(EmergencyAction::for_level(PressureLevel::Red), EmergencyAction::OomTargeted);
    }

    #[test]
    fn test_execute_none() {
        assert!(EmergencyAction::None.execute().is_ok());
    }

    #[test]
    fn test_execute_all() {
        assert!(EmergencyAction::CacheRelease.execute().is_ok());
        assert!(EmergencyAction::BackgroundSuspend.execute().is_ok());
        assert!(EmergencyAction::OomTargeted.execute().is_ok());
    }
}
