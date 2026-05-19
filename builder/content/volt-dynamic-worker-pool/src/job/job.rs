use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::job::job_id::JobId;
use crate::priority::level::PriorityLevel;

static NEXT_JOB_SEQ: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    id: JobId,
    name: String,
    priority: PriorityLevel,
    payload_size_bytes: u64,
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    created_at: Instant,
    sequence: u64,
    metadata: std::collections::HashMap<String, String>,
}

impl Job {
    pub fn new(id: JobId, name: String, priority: PriorityLevel, payload_size_bytes: u64) -> Self {
        Self {
            id,
            name,
            priority,
            payload_size_bytes,
            created_at: Instant::now(),
            sequence: NEXT_JOB_SEQ.fetch_add(1, Ordering::Relaxed),
            metadata: std::collections::HashMap::new(),
        }
    }

    pub fn id(&self) -> &JobId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn priority(&self) -> PriorityLevel {
        self.priority
    }

    pub fn set_priority(&mut self, priority: PriorityLevel) {
        self.priority = priority;
    }

    pub fn payload_size_bytes(&self) -> u64 {
        self.payload_size_bytes
    }

    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn elapsed_ms(&self) -> u64 {
        self.created_at.elapsed().as_millis() as u64
    }

    pub fn metadata(&self) -> &std::collections::HashMap<String, String> {
        &self.metadata
    }

    pub fn metadata_mut(&mut self) -> &mut std::collections::HashMap<String, String> {
        &mut self.metadata
    }

    pub fn insert_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Job {}

impl std::hash::Hash for Job {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_creation() {
        let id = JobId::new();
        let job = Job::new(id.clone(), "test".into(), PriorityLevel::High, 4096);
        assert_eq!(job.id(), &id);
        assert_eq!(job.name(), "test");
        assert_eq!(job.priority(), PriorityLevel::High);
        assert_eq!(job.payload_size_bytes(), 4096);
    }

    #[test]
    fn test_job_equality() {
        let id = JobId::new();
        let a = Job::new(id.clone(), "a".into(), PriorityLevel::Normal, 1024);
        let b = Job::new(id, "b".into(), PriorityLevel::Low, 2048);
        assert_eq!(a, b);
    }

    #[test]
    fn test_sequence_increases() {
        let a = Job::new(JobId::new(), "a".into(), PriorityLevel::Normal, 0);
        let b = Job::new(JobId::new(), "b".into(), PriorityLevel::Normal, 0);
        assert!(b.sequence() > a.sequence());
    }

    #[test]
    fn test_metadata() {
        let mut job = Job::new(JobId::new(), "meta".into(), PriorityLevel::Normal, 0);
        job.insert_metadata("key".into(), "value".into());
        assert_eq!(job.metadata().get("key").unwrap(), "value");
    }

    #[test]
    fn test_elapsed_ms() {
        let job = Job::new(JobId::new(), "time".into(), PriorityLevel::Normal, 0);
        assert!(job.elapsed_ms() < 1000);
    }
}
