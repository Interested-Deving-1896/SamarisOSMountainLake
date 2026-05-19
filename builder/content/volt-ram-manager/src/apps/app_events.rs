#[derive(Debug, Clone)]
pub enum AppEvent {
    Registered {
        app_id: u64,
        name: String,
    },
    Unregistered {
        app_id: u64,
    },
    QuotaExceeded {
        app_id: u64,
        used: u64,
        limit: u64,
    },
    PressureApplied {
        app_id: u64,
        level: String,
    },
    Suspended {
        app_id: u64,
    },
    Resumed {
        app_id: u64,
    },
}

impl AppEvent {
    pub fn app_id(&self) -> u64 {
        match self {
            AppEvent::Registered { app_id, .. } => *app_id,
            AppEvent::Unregistered { app_id } => *app_id,
            AppEvent::QuotaExceeded { app_id, .. } => *app_id,
            AppEvent::PressureApplied { app_id, .. } => *app_id,
            AppEvent::Suspended { app_id } => *app_id,
            AppEvent::Resumed { app_id } => *app_id,
        }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            AppEvent::Registered { .. } => "registered",
            AppEvent::Unregistered { .. } => "unregistered",
            AppEvent::QuotaExceeded { .. } => "quota_exceeded",
            AppEvent::PressureApplied { .. } => "pressure_applied",
            AppEvent::Suspended { .. } => "suspended",
            AppEvent::Resumed { .. } => "resumed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registered_event() {
        let ev = AppEvent::Registered {
            app_id: 42,
            name: "test".into(),
        };
        assert_eq!(ev.app_id(), 42);
        assert_eq!(ev.kind(), "registered");
    }

    #[test]
    fn test_unregistered_event() {
        let ev = AppEvent::Unregistered { app_id: 7 };
        assert_eq!(ev.app_id(), 7);
        assert_eq!(ev.kind(), "unregistered");
    }
}
