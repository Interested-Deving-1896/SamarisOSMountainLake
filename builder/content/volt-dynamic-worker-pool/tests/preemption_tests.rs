use volt_dynamic_worker_pool::*;

#[test]
fn test_yield_result_semantics() {
    use preemption::cooperative::YieldResult;
    assert!(YieldResult::Yielded.should_continue());
    assert!(YieldResult::Resumed.should_continue());
    assert!(!YieldResult::Cancelled.should_continue());
    assert!(!YieldResult::Completed.should_continue());
    assert!(!YieldResult::Preempted.should_continue());
    assert!(!YieldResult::BudgetExhausted.should_continue());
}

#[test]
fn test_cooperative_scheduler_preempt() {
    use preemption::cooperative::CooperativeScheduler;
    let sched = CooperativeScheduler::new(true, 1000, 5000);
    assert!(sched.preemption_enabled());
    assert!(sched.should_preempt(5000));
    assert!(!sched.should_preempt(4999));
}

#[test]
fn test_cooperative_scheduler_preempt_disabled() {
    use preemption::cooperative::CooperativeScheduler;
    let sched = CooperativeScheduler::new(false, 1000, 5000);
    assert!(!sched.preemption_enabled());
    assert!(!sched.should_preempt(u64::MAX));
}

#[test]
fn test_cooperative_scheduler_yield_budget() {
    use preemption::cooperative::{CooperativeScheduler, YieldResult};
    let sched = CooperativeScheduler::new(true, 100, 1000);
    assert_eq!(sched.should_yield(0), YieldResult::BudgetExhausted);
    assert_eq!(sched.should_yield(50), YieldResult::Yielded);
}
