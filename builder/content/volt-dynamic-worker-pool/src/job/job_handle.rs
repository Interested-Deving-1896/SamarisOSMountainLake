use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::job::job_id::JobId;

#[derive(Debug, Clone)]
pub struct JobHandle {
    id: JobId,
    cancelled: Arc<AtomicBool>,
    completed: Arc<AtomicBool>,
    name: String,
}

impl JobHandle {
    pub fn new(id: JobId, name: String) -> Self {
        Self {
            id,
            cancelled: Arc::new(AtomicBool::new(false)),
            completed: Arc::new(AtomicBool::new(false)),
            name,
        }
    }

    pub fn id(&self) -> &JobId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn mark_completed(&self) {
        self.completed.store(true, Ordering::SeqCst);
    }

    pub fn is_completed(&self) -> bool {
        self.completed.load(Ordering::SeqCst)
    }

    pub fn cancelled_flag(&self) -> Arc<AtomicBool> {
        self.cancelled.clone()
    }

    pub fn completed_flag(&self) -> Arc<AtomicBool> {
        self.completed.clone()
    }
}

impl PartialEq for JobHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for JobHandle {}

impl std::hash::Hash for JobHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_cancel() {
        let handle = JobHandle::new(JobId::new(), "test".into());
        assert!(!handle.is_cancelled());
        handle.cancel();
        assert!(handle.is_cancelled());
    }

    #[test]
    fn test_handle_completion() {
        let handle = JobHandle::new(JobId::new(), "test".into());
        assert!(!handle.is_completed());
        handle.mark_completed();
        assert!(handle.is_completed());
    }
}
