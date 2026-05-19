#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Priority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Idle = 4,
}

impl Priority {
    pub fn from_byte(b: u8) -> Self {
        match b {
            0 => Self::Critical,
            1 => Self::High,
            2 => Self::Normal,
            3 => Self::Low,
            _ => Self::Idle,
        }
    }

    pub fn to_byte(self) -> u8 {
        self as u8
    }

    /// Maximum number of tasks to process from this priority level per cycle.
    /// CRITICAL = unbounded (all tasks execute), others are capped.
    pub fn max_per_cycle(self) -> Option<usize> {
        match self {
            Self::Critical => None,
            Self::High => Some(8),
            Self::Normal => Some(4),
            Self::Low => Some(2),
            Self::Idle => Some(1),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::High => "HIGH",
            Self::Normal => "NORMAL",
            Self::Low => "LOW",
            Self::Idle => "IDLE",
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
