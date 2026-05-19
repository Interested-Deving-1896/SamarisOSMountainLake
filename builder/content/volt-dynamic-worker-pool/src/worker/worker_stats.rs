use crate::job::job_id::JobId;

#[derive(Debug, Clone)]
pub struct WorkerStats {
    pub jobs_completed: u64,
    pub jobs_failed: u64,
    pub busy_time_us: u64,
    pub idle_time_us: u64,
    pub yield_count: u64,
    pub preemption_count: u64,
    pub last_job_id: Option<JobId>,
}

impl WorkerStats {
    pub fn new() -> Self {
        Self {
            jobs_completed: 0,
            jobs_failed: 0,
            busy_time_us: 0,
            idle_time_us: 0,
            yield_count: 0,
            preemption_count: 0,
            last_job_id: None,
        }
    }

    pub fn record_job_start(&mut self, job_id: JobId) {
        self.last_job_id = Some(job_id);
    }

    pub fn record_job_complete(&mut self, success: bool, elapsed_us: u64) {
        if success {
            self.jobs_completed += 1;
        } else {
            self.jobs_failed += 1;
        }
        self.busy_time_us = self.busy_time_us.saturating_add(elapsed_us);
    }

    pub fn record_idle(&mut self, elapsed_us: u64) {
        self.idle_time_us = self.idle_time_us.saturating_add(elapsed_us);
    }

    pub fn record_yield(&mut self) {
        self.yield_count += 1;
    }

    pub fn record_preemption(&mut self) {
        self.preemption_count += 1;
    }

    pub fn snapshot(&self) -> Self {
        self.clone()
    }

    pub fn total_jobs(&self) -> u64 {
        self.jobs_completed + self.jobs_failed
    }

    pub fn total_time_us(&self) -> u64 {
        self.busy_time_us.saturating_add(self.idle_time_us)
    }

    pub fn utilization_pct(&self) -> f64 {
        let total = self.total_time_us();
        if total == 0 {
            return 0.0;
        }
        (self.busy_time_us as f64 / total as f64) * 100.0
    }
}

impl Default for WorkerStats {
    fn default() -> Self {
        Self::new()
    }
}
