#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PressureLevel {
    Low,
    Moderate,
    High,
    Critical,
}

impl PressureLevel {
    pub fn is_backlogged(&self) -> bool {
        matches!(self, PressureLevel::High | PressureLevel::Critical)
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, PressureLevel::Critical)
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            PressureLevel::Low => 0.25,
            PressureLevel::Moderate => 0.50,
            PressureLevel::High => 0.75,
            PressureLevel::Critical => 1.0,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PressureLevel::Low => "low",
            PressureLevel::Moderate => "moderate",
            PressureLevel::High => "high",
            PressureLevel::Critical => "critical",
        }
    }
}

pub struct QueuePressure {
    pub depth: u32,
    pub worker_count: u32,
    pub ratio: f64,
    pub level: PressureLevel,
}

impl QueuePressure {
    pub fn new(queue_depth: u32, worker_count: u32) -> Self {
        let ratio = if worker_count == 0 {
            f64::MAX
        } else {
            queue_depth as f64 / worker_count as f64
        };

        let level = if ratio <= 0.5 {
            PressureLevel::Low
        } else if ratio <= 1.0 {
            PressureLevel::Moderate
        } else if ratio <= 2.0 {
            PressureLevel::High
        } else {
            PressureLevel::Critical
        };

        QueuePressure {
            depth: queue_depth,
            worker_count,
            ratio,
            level,
        }
    }

    pub fn compute_from_depth(queue_depth: u32, worker_count: u32) -> PressureLevel {
        Self::new(queue_depth, worker_count).level
    }

    pub fn is_idle(&self) -> bool {
        self.level == PressureLevel::Low
    }

    pub fn is_backlogged(&self) -> bool {
        self.level.is_backlogged()
    }

    pub fn is_critical(&self) -> bool {
        self.level.is_critical()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_low_pressure() {
        let p = QueuePressure::new(2, 8);
        assert_eq!(p.level, PressureLevel::Low);
        assert!(p.is_idle());
        assert!(!p.is_backlogged());
    }

    #[test]
    fn test_moderate_pressure() {
        let p = QueuePressure::new(6, 8);
        assert_eq!(p.level, PressureLevel::Moderate);
        assert!(!p.is_idle());
        assert!(!p.is_backlogged());
    }

    #[test]
    fn test_high_pressure() {
        let p = QueuePressure::new(12, 8);
        assert_eq!(p.level, PressureLevel::High);
        assert!(p.is_backlogged());
        assert!(!p.is_critical());
    }

    #[test]
    fn test_critical_pressure() {
        let p = QueuePressure::new(20, 8);
        assert_eq!(p.level, PressureLevel::Critical);
        assert!(p.is_backlogged());
        assert!(p.is_critical());
    }

    #[test]
    fn test_zero_workers() {
        let p = QueuePressure::new(5, 0);
        assert_eq!(p.level, PressureLevel::Critical);
    }

    #[test]
    fn test_empty_queue() {
        let p = QueuePressure::new(0, 4);
        assert_eq!(p.level, PressureLevel::Low);
        assert!(p.is_idle());
    }

    #[test]
    fn test_pressure_level_names() {
        assert_eq!(PressureLevel::Low.name(), "low");
        assert_eq!(PressureLevel::Moderate.name(), "moderate");
        assert_eq!(PressureLevel::High.name(), "high");
        assert_eq!(PressureLevel::Critical.name(), "critical");
    }

    #[test]
    fn test_compute_from_depth() {
        assert_eq!(QueuePressure::compute_from_depth(1, 4), PressureLevel::Low);
        assert_eq!(QueuePressure::compute_from_depth(3, 4), PressureLevel::Moderate);
        assert_eq!(QueuePressure::compute_from_depth(6, 4), PressureLevel::High);
        assert_eq!(QueuePressure::compute_from_depth(10, 4), PressureLevel::Critical);
    }
}
