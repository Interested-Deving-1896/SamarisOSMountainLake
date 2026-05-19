#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WriteStatus {
    Pending,
    Flushing,
    Flushed,
    Durable,
    Failed,
}

impl WriteStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Flushed | Self::Durable | Self::Failed)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Flushing => "flushing",
            Self::Flushed => "flushed",
            Self::Durable => "durable",
            Self::Failed => "failed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_not_terminal() {
        assert!(!WriteStatus::Pending.is_terminal());
        assert!(!WriteStatus::Flushing.is_terminal());
    }

    #[test]
    fn test_terminal_states() {
        assert!(WriteStatus::Flushed.is_terminal());
        assert!(WriteStatus::Durable.is_terminal());
        assert!(WriteStatus::Failed.is_terminal());
    }

    #[test]
    fn test_names() {
        assert_eq!(WriteStatus::Pending.name(), "pending");
        assert_eq!(WriteStatus::Flushing.name(), "flushing");
        assert_eq!(WriteStatus::Flushed.name(), "flushed");
        assert_eq!(WriteStatus::Durable.name(), "durable");
        assert_eq!(WriteStatus::Failed.name(), "failed");
    }

    #[test]
    fn test_equality() {
        assert_eq!(WriteStatus::Pending, WriteStatus::Pending);
        assert_ne!(WriteStatus::Pending, WriteStatus::Flushing);
    }

    #[test]
    fn test_clone_copy() {
        let a = WriteStatus::Durable;
        let b = a;
        let c = a;
        assert_eq!(b, c);
    }

    #[test]
    fn test_all_variants_distinct() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(WriteStatus::Pending);
        set.insert(WriteStatus::Flushing);
        set.insert(WriteStatus::Flushed);
        set.insert(WriteStatus::Durable);
        set.insert(WriteStatus::Failed);
        assert_eq!(set.len(), 5);
    }
}
