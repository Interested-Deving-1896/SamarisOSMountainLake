#[derive(Debug, Clone, PartialEq)]
pub enum ScalingDecision {
    ScaleUp { new_count: u32, reason: String },
    ScaleDown { new_count: u32, reason: String },
    NoChange { reason: String },
}

impl ScalingDecision {
    pub fn is_scale_up(&self) -> bool {
        matches!(self, ScalingDecision::ScaleUp { .. })
    }

    pub fn is_scale_down(&self) -> bool {
        matches!(self, ScalingDecision::ScaleDown { .. })
    }

    pub fn is_no_change(&self) -> bool {
        matches!(self, ScalingDecision::NoChange { .. })
    }

    pub fn new_count(&self) -> Option<u32> {
        match self {
            ScalingDecision::ScaleUp { new_count, .. } => Some(*new_count),
            ScalingDecision::ScaleDown { new_count, .. } => Some(*new_count),
            ScalingDecision::NoChange { .. } => None,
        }
    }

    pub fn reason(&self) -> &str {
        match self {
            ScalingDecision::ScaleUp { reason, .. } => reason,
            ScalingDecision::ScaleDown { reason, .. } => reason,
            ScalingDecision::NoChange { reason, .. } => reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_up_decision() {
        let d = ScalingDecision::ScaleUp {
            new_count: 8,
            reason: "high load".to_string(),
        };
        assert!(d.is_scale_up());
        assert!(!d.is_scale_down());
        assert!(!d.is_no_change());
        assert_eq!(d.new_count(), Some(8));
        assert_eq!(d.reason(), "high load");
    }

    #[test]
    fn test_scale_down_decision() {
        let d = ScalingDecision::ScaleDown {
            new_count: 2,
            reason: "low load".to_string(),
        };
        assert!(d.is_scale_down());
        assert_eq!(d.new_count(), Some(2));
    }

    #[test]
    fn test_no_change_decision() {
        let d = ScalingDecision::NoChange {
            reason: "stable".to_string(),
        };
        assert!(d.is_no_change());
        assert_eq!(d.new_count(), None);
        assert_eq!(d.reason(), "stable");
    }
}
