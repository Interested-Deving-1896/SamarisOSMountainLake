use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use parking_lot::Mutex;

use crate::scheduler::priority::IoPriority;
use crate::scheduler::queue::{IoQueue, IoJob};
use crate::scheduler::batcher::IoBatcher;

pub struct IoScheduler {
    queue: Mutex<IoQueue>,
    #[allow(dead_code)]
    batcher: IoBatcher,
    max_concurrent: usize,
    active_flushes: AtomicUsize,
    next_id: AtomicU64,
}

impl IoScheduler {
    pub fn new(max_concurrent: usize, batch_kb: u64) -> Self {
        IoScheduler {
            queue: Mutex::new(IoQueue::new()),
            batcher: IoBatcher::new(batch_kb),
            max_concurrent,
            active_flushes: AtomicUsize::new(0),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn submit(
        &self,
        path: &str,
        offset: u64,
        len: u64,
        priority: IoPriority,
        is_metadata: bool,
    ) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut job = IoJob::new(id, path, offset, len, priority);
        job.is_metadata = is_metadata;
        self.queue.lock().enqueue(job);
        self.active_flushes.fetch_add(1, Ordering::Relaxed);
        id
    }

    pub fn dequeue_batch(&self, max: usize) -> Vec<IoJob> {
        let jobs = self.queue.lock().dequeue_many(max);
        jobs
    }

    pub fn complete(&self, _job_id: u64) {
        self.active_flushes.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn pending_count(&self) -> usize {
        self.queue.lock().len()
    }

    pub fn can_submit(&self) -> bool {
        let active = self.active_flushes.load(Ordering::Relaxed);
        active < self.max_concurrent
    }
}

impl Default for IoScheduler {
    fn default() -> Self {
        Self::new(4, 64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_increases_count() {
        let sched = IoScheduler::new(4, 64);
        sched.submit("/path", 0, 4096, IoPriority::Desktop, false);
        assert_eq!(sched.pending_count(), 1);
    }

    #[test]
    fn test_submit_multiple() {
        let sched = IoScheduler::new(4, 64);
        sched.submit("/a", 0, 100, IoPriority::CriticalMetadata, true);
        sched.submit("/b", 0, 200, IoPriority::Desktop, false);
        assert_eq!(sched.pending_count(), 2);
    }

    #[test]
    fn test_dequeue_batch() {
        let sched = IoScheduler::new(4, 64);
        sched.submit("/a", 0, 100, IoPriority::Desktop, false);
        sched.submit("/b", 0, 200, IoPriority::Desktop, false);
        let batch = sched.dequeue_batch(10);
        assert_eq!(batch.len(), 2);
        assert_eq!(sched.pending_count(), 0);
    }

    #[test]
    fn test_can_submit_initially() {
        let sched = IoScheduler::new(4, 64);
        assert!(sched.can_submit());
    }

    #[test]
    fn test_complete_decrements() {
        let sched = IoScheduler::new(4, 64);
        let id = sched.submit("/x", 0, 512, IoPriority::UserVisible, false);
        let _ = sched.dequeue_batch(1);
        sched.complete(id);
        assert!(sched.can_submit());
    }

    #[test]
    fn test_submit_returns_unique_ids() {
        let sched = IoScheduler::new(4, 64);
        let id1 = sched.submit("/a", 0, 1, IoPriority::Cache, false);
        let id2 = sched.submit("/b", 0, 1, IoPriority::Cache, false);
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_submit_sets_is_metadata() {
        let sched = IoScheduler::new(4, 64);
        sched.submit("/meta", 0, 64, IoPriority::CriticalMetadata, true);
        sched.submit("/data", 0, 64, IoPriority::Desktop, false);
        let batch = sched.dequeue_batch(10);
        assert!(batch[0].is_metadata);
        assert!(!batch[1].is_metadata);
    }
}
