use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low,
    Normal,
    High,
    Critical,
    Realtime,
}

impl PriorityLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Normal => 1,
            Self::High => 2,
            Self::Critical => 3,
            Self::Realtime => 4,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Low),
            1 => Some(Self::Normal),
            2 => Some(Self::High),
            3 => Some(Self::Critical),
            4 => Some(Self::Realtime),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
            Self::Realtime => "realtime",
        }
    }
}

impl Default for PriorityLevel {
    fn default() -> Self {
        Self::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(PriorityLevel::Low < PriorityLevel::Normal);
        assert!(PriorityLevel::Normal < PriorityLevel::High);
        assert!(PriorityLevel::High < PriorityLevel::Critical);
        assert!(PriorityLevel::Critical < PriorityLevel::Realtime);
    }

    #[test]
    fn test_as_u8_roundtrip() {
        for level in &[
            PriorityLevel::Low,
            PriorityLevel::Normal,
            PriorityLevel::High,
            PriorityLevel::Critical,
            PriorityLevel::Realtime,
        ] {
            assert_eq!(PriorityLevel::from_u8(level.as_u8()), Some(*level));
        }
    }

    #[test]
    fn test_name() {
        assert_eq!(PriorityLevel::Low.name(), "low");
        assert_eq!(PriorityLevel::Realtime.name(), "realtime");
    }

    #[test]
    fn test_default() {
        assert_eq!(PriorityLevel::default(), PriorityLevel::Normal);
    }
}
