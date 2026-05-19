use std::time::{Duration, Instant};

use crate::job::job_id::JobId;

pub struct StarvationGuard {
    pub starvation_limit_ms: u64,
    pub check_interval_ms: u64,
    job_wait_times: Vec<(JobId, Instant)>,
}

impl StarvationGuard {
    pub fn new() -> Self {
        Self {
            starvation_limit_ms: 30_000,
            check_interval_ms: 1_000,
            job_wait_times: Vec::new(),
        }
    }

    pub fn track_job(&mut self, job_id: JobId) {
        self.job_wait_times.push((job_id, Instant::now()));
    }

    pub fn is_starved(&self, job_id: &JobId) -> bool {
        self.job_wait_times
            .iter()
            .find(|(id, _)| id == job_id)
            .map(|(_, enqueued_at)| {
                let elapsed = enqueued_at.elapsed();
                elapsed >= Duration::from_millis(self.starvation_limit_ms)
            })
            .unwrap_or(false)
    }

    pub fn starved_jobs(&self) -> Vec<JobId> {
        let limit = Duration::from_millis(self.starvation_limit_ms);
        self.job_wait_times
            .iter()
            .filter(|(_, enqueued_at)| enqueued_at.elapsed() >= limit)
            .map(|(id, _)| id.clone())
            .collect()
    }

    pub fn prune_expired(&mut self) {
        self.job_wait_times.retain(|(_, enqueued_at)| {
            enqueued_at.elapsed() < Duration::from_millis(self.starvation_limit_ms * 2)
        });
    }
}

impl Default for StarvationGuard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_guard() {
        let guard = StarvationGuard::new();
        assert_eq!(guard.starvation_limit_ms, 30_000);
        assert_eq!(guard.check_interval_ms, 1_000);
        assert!(guard.job_wait_times.is_empty());
    }

    #[test]
    fn test_track_and_not_starved() {
        let guard = StarvationGuard::new();
        let job_id = JobId::new();
        let mut guard = guard;
        guard.track_job(job_id.clone());
        assert!(!guard.is_starved(&job_id));
    }

    #[test]
    fn test_starved_jobs_empty_initially() {
        let guard = StarvationGuard::new();
        assert!(guard.starved_jobs().is_empty());
    }

    #[test]
    fn test_prune_does_not_remove_recent() {
        let mut guard = StarvationGuard::new();
        guard.track_job(JobId::new());
        guard.prune_expired();
        assert_eq!(guard.job_wait_times.len(), 1);
    }

    #[test]
    fn test_is_starved_untracked_job() {
        let guard = StarvationGuard::new();
        assert!(!guard.is_starved(&JobId::new()));
    }

    #[test]
    fn test_default() {
        let guard = StarvationGuard::default();
        assert_eq!(guard.starvation_limit_ms, 30_000);
    }
}
