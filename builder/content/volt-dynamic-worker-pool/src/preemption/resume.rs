use crate::job::job_id::JobId;
use crate::preemption::checkpoint::JobCheckpoint;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ResumeToken {
    pub job_id: JobId,
    pub checkpoint: Option<JobCheckpoint>,
    pub resumed_at: Instant,
}

impl ResumeToken {
    pub fn new(job_id: JobId) -> Self {
        Self {
            job_id,
            checkpoint: None,
            resumed_at: Instant::now(),
        }
    }

    pub fn with_checkpoint(job_id: JobId, checkpoint: JobCheckpoint) -> Self {
        Self {
            job_id,
            checkpoint: Some(checkpoint),
            resumed_at: Instant::now(),
        }
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.resumed_at.elapsed().as_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let id = JobId::new();
        let token = ResumeToken::new(id.clone());
        assert_eq!(token.job_id, id);
        assert!(token.checkpoint.is_none());
    }

    #[test]
    fn test_with_checkpoint() {
        let id = JobId::new();
        let cp = JobCheckpoint::new(vec![1, 2, 3], 0.5);
        let token = ResumeToken::with_checkpoint(id.clone(), cp.clone());
        assert_eq!(token.job_id, id);
        assert_eq!(token.checkpoint, Some(cp));
    }

    #[test]
    fn test_elapsed_ms() {
        let id = JobId::new();
        let token = ResumeToken::new(id);
        assert!(token.elapsed_ms() < 1000);
    }
}
