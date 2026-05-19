use std::time::Instant;

use crate::job::job::Job;
use crate::job::job_handle::JobHandle;
use crate::job::job_id::JobId;
use crate::priority::level::PriorityLevel;

#[derive(Clone)]
pub struct JobContext {
    job: Job,
    handle: JobHandle,
    yield_budget_us: u64,
    yield_consumed: u64,
    started_at: Instant,
    worker_id: u32,
}

impl JobContext {
    pub fn new(job: Job, handle: JobHandle, yield_budget_us: u64, worker_id: u32) -> Self {
        Self {
            started_at: Instant::now(),
            yield_budget_us,
            yield_consumed: 0,
            job,
            handle,
            worker_id,
        }
    }

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn job_mut(&mut self) -> &mut Job {
        &mut self.job
    }

    pub fn handle(&self) -> &JobHandle {
        &self.handle
    }

    pub fn id(&self) -> &JobId {
        self.job.id()
    }

    pub fn priority(&self) -> PriorityLevel {
        self.job.priority()
    }

    pub fn worker_id(&self) -> u32 {
        self.worker_id
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.started_at.elapsed().as_millis() as u64
    }

    pub fn yield_budget_remaining(&self) -> u64 {
        self.yield_budget_us.saturating_sub(self.yield_consumed)
    }

    pub fn consume_yield_budget(&mut self, amount: u64) {
        self.yield_consumed = self.yield_consumed.saturating_add(amount);
    }

    pub fn reset_yield_budget(&mut self) {
        self.yield_consumed = 0;
    }

    pub fn is_cancelled(&self) -> bool {
        self.handle.is_cancelled()
    }

    pub fn is_completed(&self) -> bool {
        self.handle.is_completed()
    }

    pub fn mark_completed(&self) {
        self.handle.mark_completed();
    }

    pub fn cancel(&self) {
        self.handle.cancel();
    }
}

impl std::fmt::Debug for JobContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobContext")
            .field("job", &self.job)
            .field("handle", &self.handle)
            .field("yield_budget_us", &self.yield_budget_us)
            .field("yield_consumed", &self.yield_consumed)
            .field("worker_id", &self.worker_id)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::job::Job;

    #[test]
    fn test_context_creation() {
        let job = Job::new(JobId::new(), "ctx".into(), PriorityLevel::High, 1024);
        let handle = JobHandle::new(job.id().clone(), job.name().into());
        let ctx = JobContext::new(job, handle, 1000, 1);
        assert_eq!(ctx.worker_id(), 1);
        assert_eq!(ctx.yield_budget_remaining(), 1000);
    }

    #[test]
    fn test_yield_budget() {
        let job = Job::new(JobId::new(), "budget".into(), PriorityLevel::Normal, 512);
        let handle = JobHandle::new(job.id().clone(), job.name().into());
        let mut ctx = JobContext::new(job, handle, 500, 0);
        ctx.consume_yield_budget(100);
        assert_eq!(ctx.yield_budget_remaining(), 400);
        ctx.reset_yield_budget();
        assert_eq!(ctx.yield_budget_remaining(), 500);
    }

    #[test]
    fn test_cancellation() {
        let job = Job::new(JobId::new(), "cancel".into(), PriorityLevel::Low, 256);
        let handle = JobHandle::new(job.id().clone(), job.name().into());
        let ctx = JobContext::new(job, handle, 100, 2);
        assert!(!ctx.is_cancelled());
        ctx.cancel();
        assert!(ctx.is_cancelled());
    }
}
