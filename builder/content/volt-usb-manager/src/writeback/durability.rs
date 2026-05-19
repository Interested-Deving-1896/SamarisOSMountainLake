#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurabilityStatus {
    Buffered,
    Journaled,
    Durable,
}

impl DurabilityStatus {
    pub fn is_durable(&self) -> bool {
        matches!(self, Self::Durable)
    }

    pub fn is_journaled(&self) -> bool {
        matches!(self, Self::Journaled | Self::Durable)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Buffered => "buffered",
            Self::Journaled => "journaled",
            Self::Durable => "durable",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffered_properties() {
        let s = DurabilityStatus::Buffered;
        assert!(!s.is_durable());
        assert!(!s.is_journaled());
        assert_eq!(s.name(), "buffered");
    }

    #[test]
    fn test_journaled_properties() {
        let s = DurabilityStatus::Journaled;
        assert!(!s.is_durable());
        assert!(s.is_journaled());
        assert_eq!(s.name(), "journaled");
    }

    #[test]
    fn test_durable_properties() {
        let s = DurabilityStatus::Durable;
        assert!(s.is_durable());
        assert!(s.is_journaled());
        assert_eq!(s.name(), "durable");
    }

    #[test]
    fn test_equality() {
        assert_eq!(DurabilityStatus::Buffered, DurabilityStatus::Buffered);
        assert_ne!(DurabilityStatus::Buffered, DurabilityStatus::Durable);
    }

    #[test]
    fn test_clone() {
        let a = DurabilityStatus::Journaled;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_copy() {
        let a = DurabilityStatus::Durable;
        let b = a;
        let c = a;
        assert_eq!(b, c);
    }
}
