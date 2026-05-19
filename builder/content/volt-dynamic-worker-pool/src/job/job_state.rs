#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobState {
    Queued,
    Running,
    Yielded,
    Rescheduled,
    Completed,
    Cancelled,
    Failed,
}

impl JobState {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Yielded => "yielded",
            Self::Rescheduled => "rescheduled",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Failed)
    }

    pub fn can_transition_to(&self, next: JobState) -> bool {
        use JobState::*;
        match self {
            Queued => matches!(next, Running | Cancelled),
            Running => matches!(next, Yielded | Rescheduled | Completed | Cancelled | Failed),
            Yielded => matches!(next, Running | Cancelled),
            Rescheduled => matches!(next, Running | Cancelled),
            Completed => false,
            Cancelled => false,
            Failed => false,
        }
    }
}
