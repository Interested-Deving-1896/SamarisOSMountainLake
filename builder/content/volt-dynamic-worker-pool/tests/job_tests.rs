use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_job_creation() {
    use priority::level::PriorityLevel;
    let id = JobId::new();
    let job = Job::new(id.clone(), "test-job".into(), PriorityLevel::High, 4096);
    assert_eq!(job.id(), &id);
    assert_eq!(job.name(), "test-job");
    assert_eq!(job.priority(), PriorityLevel::High);
    assert_eq!(job.payload_size_bytes(), 4096);
}

#[test]
fn test_job_equality_by_id() {
    let id = JobId::new();
    let a = Job::new(id.clone(), "a".into(), priority::level::PriorityLevel::Normal, 1024);
    let b = Job::new(id, "b".into(), priority::level::PriorityLevel::Low, 2048);
    assert_eq!(a, b);
}

#[test]
fn test_job_handle_cancel_and_complete() {
    let handle = JobHandle::new(JobId::new(), "test".into());
    assert!(!handle.is_cancelled());
    handle.cancel();
    assert!(handle.is_cancelled());

    let handle2 = JobHandle::new(JobId::new(), "test2".into());
    handle2.mark_completed();
    assert!(handle2.is_completed());
}

#[test]
fn test_job_context_yield_budget() {
    use job::job_context::JobContext;
    let job = Job::new(JobId::new(), "ctx".into(), priority::level::PriorityLevel::Normal, 512);
    let handle = JobHandle::new(job.id().clone(), job.name().into());
    let mut ctx = JobContext::new(job, handle, 500, 0);
    assert_eq!(ctx.yield_budget_remaining(), 500);
    ctx.consume_yield_budget(100);
    assert_eq!(ctx.yield_budget_remaining(), 400);
    ctx.reset_yield_budget();
    assert_eq!(ctx.yield_budget_remaining(), 500);
}

#[test]
fn test_job_priority_scoring() {
    use job::job_id::JobPriority;
    use priority::level::PriorityLevel;
    let low = JobPriority::new(PriorityLevel::Low, 0);
    let high = JobPriority::new(PriorityLevel::High, 0);
    assert!(low.score() < high.score());
}
