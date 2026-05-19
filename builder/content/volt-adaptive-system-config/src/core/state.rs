#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AscState {
    Uninitialized,
    Probing,
    Profiling,
    Budgeting,
    Generating,
    Validating,
    Complete,
    Error,
}

impl AscState {
    pub fn can_transition_to(&self, next: &Self) -> bool {
        use AscState::*;
        matches!(
            (self, next),
            (Uninitialized, Probing)
                | (Probing, Profiling)
                |             (Probing, Error)
                | (Probing, Complete)
                | (Profiling, Budgeting)
                | (Profiling, Error)
                | (Budgeting, Generating)
                | (Budgeting, Error)
                | (Generating, Validating)
                | (Generating, Error)
                | (Validating, Complete)
                | (Validating, Error)
                | (Error, Uninitialized)
                | (Complete, Uninitialized)
        )
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Self::Probing | Self::Profiling | Self::Budgeting | Self::Generating | Self::Validating)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Uninitialized => "uninitialized",
            Self::Probing => "probing",
            Self::Profiling => "profiling",
            Self::Budgeting => "budgeting",
            Self::Generating => "generating",
            Self::Validating => "validating",
            Self::Complete => "complete",
            Self::Error => "error",
        }
    }
}

impl Default for AscState {
    fn default() -> Self {
        Self::Uninitialized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_state() {
        assert_eq!(AscState::default(), AscState::Uninitialized);
    }

    #[test]
    fn test_valid_transitions() {
        assert!(AscState::Uninitialized.can_transition_to(&AscState::Probing));
        assert!(AscState::Probing.can_transition_to(&AscState::Profiling));
        assert!(AscState::Profiling.can_transition_to(&AscState::Budgeting));
        assert!(AscState::Budgeting.can_transition_to(&AscState::Generating));
        assert!(AscState::Generating.can_transition_to(&AscState::Validating));
        assert!(AscState::Validating.can_transition_to(&AscState::Complete));
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(!AscState::Uninitialized.can_transition_to(&AscState::Complete));
        assert!(!AscState::Complete.can_transition_to(&AscState::Probing));
    }

    #[test]
    fn test_is_active() {
        assert!(AscState::Probing.is_active());
        assert!(!AscState::Complete.is_active());
    }

    #[test]
    fn test_name() {
        assert_eq!(AscState::Uninitialized.name(), "uninitialized");
        assert_eq!(AscState::Complete.name(), "complete");
    }
}
