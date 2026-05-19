#[derive(Debug, Clone, Copy)]
pub struct YieldBudget {
    pub max_budget_us: u64,
    pub force_preempt_after_us: u64,
}

impl YieldBudget {
    pub fn new(max_budget_us: u64, force_preempt_after_us: u64) -> Self {
        Self {
            max_budget_us,
            force_preempt_after_us,
        }
    }

    pub fn exhausted(&self, consumed: u64) -> bool {
        consumed >= self.max_budget_us
    }

    pub fn should_force_preempt(&self, elapsed_us: u64) -> bool {
        elapsed_us >= self.force_preempt_after_us
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let budget = YieldBudget::new(1000, 5000);
        assert_eq!(budget.max_budget_us, 1000);
        assert_eq!(budget.force_preempt_after_us, 5000);
    }

    #[test]
    fn test_exhausted() {
        let budget = YieldBudget::new(100, 500);
        assert!(!budget.exhausted(50));
        assert!(budget.exhausted(100));
        assert!(budget.exhausted(150));
    }

    #[test]
    fn test_should_force_preempt() {
        let budget = YieldBudget::new(100, 500);
        assert!(!budget.should_force_preempt(499));
        assert!(budget.should_force_preempt(500));
        assert!(budget.should_force_preempt(501));
    }
}
