use std::collections::VecDeque;

use crate::scheduler::priority::IoPriority;

#[derive(Debug, Clone)]
pub struct IoJob {
    pub id: u64,
    pub priority: IoPriority,
    pub path: String,
    pub offset: u64,
    pub data_len: u64,
    pub is_metadata: bool,
    pub submitted_at: u64,
}

pub struct IoQueue {
    queues: [VecDeque<IoJob>; 5],
    total_items: usize,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

impl IoQueue {
    pub fn new() -> Self {
        IoQueue {
            queues: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            total_items: 0,
        }
    }

    pub fn enqueue(&mut self, job: IoJob) {
        let idx = job.priority as usize;
        self.queues[idx].push_back(job);
        self.total_items += 1;
    }

    pub fn dequeue(&mut self) -> Option<IoJob> {
        for q in self.queues.iter_mut() {
            if let Some(job) = q.pop_front() {
                self.total_items = self.total_items.saturating_sub(1);
                return Some(job);
            }
        }
        None
    }

    pub fn dequeue_many(&mut self, max: usize) -> Vec<IoJob> {
        let mut result = Vec::with_capacity(max);
        for q in self.queues.iter_mut() {
            while result.len() < max {
                match q.pop_front() {
                    Some(job) => {
                        self.total_items = self.total_items.saturating_sub(1);
                        result.push(job);
                    }
                    None => break,
                }
            }
            if result.len() >= max {
                break;
            }
        }
        result
    }

    pub fn len(&self) -> usize {
        self.total_items
    }

    pub fn is_empty(&self) -> bool {
        self.total_items == 0
    }
}

impl Default for IoQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl IoJob {
    pub fn new(id: u64, path: &str, offset: u64, data_len: u64, priority: IoPriority) -> Self {
        IoJob {
            id,
            priority,
            path: path.to_string(),
            offset,
            data_len,
            is_metadata: priority == IoPriority::CriticalMetadata,
            submitted_at: now_ms(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn job(id: u64, priority: IoPriority) -> IoJob {
        IoJob::new(id, "/path", 0, 4096, priority)
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut q = IoQueue::new();
        q.enqueue(job(1, IoPriority::Desktop));
        let j = q.dequeue().unwrap();
        assert_eq!(j.id, 1);
        assert!(q.is_empty());
    }

    #[test]
    fn test_dequeue_respects_priority() {
        let mut q = IoQueue::new();
        q.enqueue(job(1, IoPriority::Background));
        q.enqueue(job(2, IoPriority::CriticalMetadata));
        let j = q.dequeue().unwrap();
        assert_eq!(j.id, 2);
        assert_eq!(j.priority, IoPriority::CriticalMetadata);
    }

    #[test]
    fn test_dequeue_many() {
        let mut q = IoQueue::new();
        q.enqueue(job(1, IoPriority::Desktop));
        q.enqueue(job(2, IoPriority::Desktop));
        q.enqueue(job(3, IoPriority::Background));
        let batch = q.dequeue_many(2);
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].id, 1);
        assert_eq!(batch[1].id, 2);
    }

    #[test]
    fn test_dequeue_from_empty() {
        let mut q = IoQueue::new();
        assert!(q.dequeue().is_none());
    }

    #[test]
    fn test_len() {
        let mut q = IoQueue::new();
        assert_eq!(q.len(), 0);
        q.enqueue(job(1, IoPriority::Cache));
        assert_eq!(q.len(), 1);
        q.enqueue(job(2, IoPriority::Cache));
        assert_eq!(q.len(), 2);
        q.dequeue();
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_is_empty() {
        let mut q = IoQueue::new();
        assert!(q.is_empty());
        q.enqueue(job(1, IoPriority::UserVisible));
        assert!(!q.is_empty());
    }

    #[test]
    fn test_io_job_new_sets_metadata_flag() {
        let meta = IoJob::new(1, "/meta", 0, 64, IoPriority::CriticalMetadata);
        assert!(meta.is_metadata);
        let data = IoJob::new(2, "/data", 0, 4096, IoPriority::UserVisible);
        assert!(!data.is_metadata);
    }
}
