#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YieldResult {
    Yielded,
    Resumed,
    Cancelled,
    Preempted,
    Completed,
    BudgetExhausted,
}

impl YieldResult {
    pub fn should_continue(&self) -> bool {
        matches!(self, Self::Yielded | Self::Resumed)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Yielded => "yielded",
            Self::Resumed => "resumed",
            Self::Cancelled => "cancelled",
            Self::Preempted => "preempted",
            Self::Completed => "completed",
            Self::BudgetExhausted => "budget_exhausted",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CooperativeScheduler {
    preemption_enabled: bool,
    default_yield_budget_us: u64,
    force_preempt_after_us: u64,
}

impl CooperativeScheduler {
    pub fn new(preemption_enabled: bool, default_yield_budget_us: u64, force_preempt_after_us: u64) -> Self {
        Self {
            preemption_enabled,
            default_yield_budget_us,
            force_preempt_after_us,
        }
    }

    pub fn preemption_enabled(&self) -> bool {
        self.preemption_enabled
    }

    pub fn default_yield_budget_us(&self) -> u64 {
        self.default_yield_budget_us
    }

    pub fn force_preempt_after_us(&self) -> u64 {
        self.force_preempt_after_us
    }

    pub fn should_preempt(&self, elapsed_us: u64) -> bool {
        self.preemption_enabled && elapsed_us >= self.force_preempt_after_us
    }

    pub fn should_yield(&self, budget_remaining: u64) -> YieldResult {
        if budget_remaining == 0 {
            YieldResult::BudgetExhausted
        } else {
            YieldResult::Yielded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yield_result() {
        assert!(YieldResult::Yielded.should_continue());
        assert!(!YieldResult::Cancelled.should_continue());
        assert!(!YieldResult::Completed.should_continue());
    }

    #[test]
    fn test_cooperative_scheduler() {
        let sched = CooperativeScheduler::new(true, 1000, 5000);
        assert!(sched.preemption_enabled());
        assert!(sched.should_preempt(5000));
        assert!(!sched.should_preempt(4999));
    }

    #[test]
    fn test_preempt_disabled() {
        let sched = CooperativeScheduler::new(false, 1000, 5000);
        assert!(!sched.should_preempt(99999));
    }

    #[test]
    fn test_yield_budget_exhausted() {
        let sched = CooperativeScheduler::new(true, 100, 1000);
        assert_eq!(sched.should_yield(0), YieldResult::BudgetExhausted);
        assert_eq!(sched.should_yield(50), YieldResult::Yielded);
    }
}
