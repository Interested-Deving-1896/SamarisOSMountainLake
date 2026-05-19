#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GpuPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Idle = 3,
}

impl GpuPriority {
    pub fn name(&self) -> &'static str {
        match self {
            GpuPriority::Critical => "critical",
            GpuPriority::High => "high",
            GpuPriority::Normal => "normal",
            GpuPriority::Idle => "idle",
        }
    }

    pub fn batch_size(&self) -> usize {
        match self {
            GpuPriority::Critical => 1,
            GpuPriority::High => 4,
            GpuPriority::Normal => 8,
            GpuPriority::Idle => 16,
        }
    }

    pub fn max_concurrent(&self) -> usize {
        match self {
            GpuPriority::Critical => 1,
            GpuPriority::High => 2,
            GpuPriority::Normal => 4,
            GpuPriority::Idle => 8,
        }
    }

    pub fn can_pause(&self) -> bool {
        matches!(self, GpuPriority::Normal | GpuPriority::Idle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(GpuPriority::Critical < GpuPriority::High);
        assert!(GpuPriority::High < GpuPriority::Normal);
        assert!(GpuPriority::Normal < GpuPriority::Idle);
    }

    #[test]
    fn test_priority_name() {
        assert_eq!(GpuPriority::Critical.name(), "critical");
        assert_eq!(GpuPriority::High.name(), "high");
        assert_eq!(GpuPriority::Normal.name(), "normal");
        assert_eq!(GpuPriority::Idle.name(), "idle");
    }

    #[test]
    fn test_batch_size() {
        assert_eq!(GpuPriority::Critical.batch_size(), 1);
        assert_eq!(GpuPriority::High.batch_size(), 4);
        assert_eq!(GpuPriority::Normal.batch_size(), 8);
        assert_eq!(GpuPriority::Idle.batch_size(), 16);
    }

    #[test]
    fn test_max_concurrent() {
        assert_eq!(GpuPriority::Critical.max_concurrent(), 1);
        assert_eq!(GpuPriority::High.max_concurrent(), 2);
        assert_eq!(GpuPriority::Normal.max_concurrent(), 4);
        assert_eq!(GpuPriority::Idle.max_concurrent(), 8);
    }

    #[test]
    fn test_can_pause() {
        assert!(!GpuPriority::Critical.can_pause());
        assert!(!GpuPriority::High.can_pause());
        assert!(GpuPriority::Normal.can_pause());
        assert!(GpuPriority::Idle.can_pause());
    }
}
