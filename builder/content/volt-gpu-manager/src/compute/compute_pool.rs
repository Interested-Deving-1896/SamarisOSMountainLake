use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::{Sender, Receiver, unbounded};
use crate::compute::compute_job::GpuComputeJob;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct GpuComputePool {
    tx: Sender<GpuComputeJob>,
    rx: Receiver<GpuComputeJob>,
    max_concurrent: usize,
    active: AtomicUsize,
}

impl GpuComputePool {
    pub fn new(max_concurrent: usize) -> Self {
        let (tx, rx) = unbounded();
        Self {
            tx,
            rx,
            max_concurrent,
            active: AtomicUsize::new(0),
        }
    }

    pub fn submit(&self, job: GpuComputeJob) -> VgmResult<()> {
        if self.active.load(Ordering::Relaxed) >= self.max_concurrent {
            return Err(VgmError::GpuJobFailed(
                "Compute pool at max concurrent capacity".into(),
            ));
        }
        self.active.fetch_add(1, Ordering::Relaxed);
        self.tx.send(job).map_err(|_| {
            VgmError::GpuJobFailed("Failed to enqueue compute job".into())
        })?;
        Ok(())
    }

    pub fn dequeue(&self) -> Option<GpuComputeJob> {
        let job = self.rx.try_recv().ok()?;
        self.active.fetch_sub(1, Ordering::Relaxed);
        Some(job)
    }

    pub fn active_count(&self) -> usize {
        self.active.load(Ordering::Relaxed)
    }

    pub fn can_submit(&self) -> bool {
        self.active.load(Ordering::Relaxed) < self.max_concurrent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::priority::GpuPriority;

    #[test]
    fn test_new_pool_empty() {
        let pool = GpuComputePool::new(4);
        assert_eq!(pool.active_count(), 0);
        assert!(pool.can_submit());
    }

    #[test]
    fn test_submit_and_dequeue() {
        let pool = GpuComputePool::new(4);
        let job = GpuComputeJob::new(
            crate::compute::compute_job::GpuComputeJobKind::MatMul,
            GpuPriority::Normal,
            0,
        );
        pool.submit(job.clone()).unwrap();
        assert_eq!(pool.active_count(), 1);
        let dequeued = pool.dequeue();
        assert!(dequeued.is_some());
        assert_eq!(dequeued.unwrap().job_id, job.job_id);
        assert_eq!(pool.active_count(), 0);
    }

    #[test]
    fn test_cannot_submit_over_capacity() {
        let pool = GpuComputePool::new(1);
        let job = GpuComputeJob::new(
            crate::compute::compute_job::GpuComputeJobKind::Blur,
            GpuPriority::Idle,
            0,
        );
        pool.submit(job).unwrap();
        assert!(!pool.can_submit());
        let another = GpuComputeJob::new(
            crate::compute::compute_job::GpuComputeJobKind::Shadow,
            GpuPriority::Idle,
            0,
        );
        assert!(pool.submit(another).is_err());
    }

    #[test]
    fn test_dequeue_empty_returns_none() {
        let pool = GpuComputePool::new(4);
        assert!(pool.dequeue().is_none());
    }

    #[test]
    fn test_submit_multiple_and_drain() {
        let pool = GpuComputePool::new(10);
        for i in 0..5 {
            let job = GpuComputeJob::new(
                crate::compute::compute_job::GpuComputeJobKind::MatMul,
                GpuPriority::Normal,
                i,
            );
            pool.submit(job).unwrap();
        }
        assert_eq!(pool.active_count(), 5);
        for _ in 0..5 {
            assert!(pool.dequeue().is_some());
        }
        assert_eq!(pool.active_count(), 0);
    }
}
