pub use super::WorkerState;

impl WorkerState {
    pub fn can_transition_to(&self, next: &Self) -> bool {
        use WorkerState::*;
        match (self, next) {
            (Idle, Busy) => true,
            (Idle, Draining) => true,
            (Idle, Stopped) => true,
            (Busy, Idle) => true,
            (Busy, Error) => true,
            (Busy, Draining) => true,
            (Busy, Stopped) => true,
            (Draining, Stopped) => true,
            (Error, Idle) => true,
            (Error, Stopped) => true,
            _ => false,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Busy => 1,
            Self::Draining => 2,
            Self::Stopped => 3,
            Self::Error => 4,
        }
    }

    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::Idle),
            1 => Some(Self::Busy),
            2 => Some(Self::Draining),
            3 => Some(Self::Stopped),
            4 => Some(Self::Error),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Stopped | Self::Error)
    }
}
