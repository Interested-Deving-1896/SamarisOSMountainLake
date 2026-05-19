#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoPriority {
    CriticalMetadata = 0,
    Desktop = 1,
    UserVisible = 2,
    Background = 3,
    Cache = 4,
}

impl IoPriority {
    pub fn name(&self) -> &'static str {
        match self {
            IoPriority::CriticalMetadata => "critical_metadata",
            IoPriority::Desktop => "desktop",
            IoPriority::UserVisible => "user_visible",
            IoPriority::Background => "background",
            IoPriority::Cache => "cache",
        }
    }

    pub fn max_concurrent(&self) -> usize {
        match self {
            IoPriority::CriticalMetadata => 8,
            IoPriority::Desktop => 6,
            IoPriority::UserVisible => 4,
            IoPriority::Background => 2,
            IoPriority::Cache => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(IoPriority::CriticalMetadata < IoPriority::Desktop);
        assert!(IoPriority::Desktop < IoPriority::UserVisible);
        assert!(IoPriority::UserVisible < IoPriority::Background);
        assert!(IoPriority::Background < IoPriority::Cache);
    }

    #[test]
    fn test_discriminant_values() {
        assert_eq!(IoPriority::CriticalMetadata as isize, 0);
        assert_eq!(IoPriority::Desktop as isize, 1);
        assert_eq!(IoPriority::UserVisible as isize, 2);
        assert_eq!(IoPriority::Background as isize, 3);
        assert_eq!(IoPriority::Cache as isize, 4);
    }

    #[test]
    fn test_name() {
        assert_eq!(IoPriority::CriticalMetadata.name(), "critical_metadata");
        assert_eq!(IoPriority::Desktop.name(), "desktop");
        assert_eq!(IoPriority::UserVisible.name(), "user_visible");
        assert_eq!(IoPriority::Background.name(), "background");
        assert_eq!(IoPriority::Cache.name(), "cache");
    }

    #[test]
    fn test_max_concurrent() {
        assert_eq!(IoPriority::CriticalMetadata.max_concurrent(), 8);
        assert_eq!(IoPriority::Desktop.max_concurrent(), 6);
        assert_eq!(IoPriority::Cache.max_concurrent(), 1);
    }
}
