use std::sync::atomic::{AtomicU64, Ordering};
use parking_lot::Mutex;
use crate::core::result::VgmResult;
use crate::scheduler::batch::GpuBatch;
use crate::scheduler::command::GpuCommand;
use crate::scheduler::command_queue::GpuCommandQueue;
use crate::scheduler::desktop_guard::DesktopFrameGuard;
use crate::scheduler::priority::GpuPriority;

pub struct GpuScheduler {
    queue: Mutex<GpuCommandQueue>,
    frame_guard: Mutex<DesktopFrameGuard>,
    next_id: AtomicU64,
    queued_count: AtomicU64,
}

impl GpuScheduler {
    pub fn new(frame_budget_ms: u64) -> Self {
        GpuScheduler {
            queue: Mutex::new(GpuCommandQueue::new()),
            frame_guard: Mutex::new(DesktopFrameGuard::new(frame_budget_ms)),
            next_id: AtomicU64::new(1),
            queued_count: AtomicU64::new(0),
        }
    }

    pub fn submit(&self, mut cmd: GpuCommand) {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        cmd.id = id;
        {
            let guard = self.frame_guard.lock();
            if guard.should_pause_priority(cmd.priority) {
                cmd.priority = GpuPriority::Idle;
            }
        }
        self.queue.lock().enqueue(cmd);
        self.queued_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn submit_batch(&self, batch: GpuBatch) -> VgmResult<()> {
        for cmd in batch.commands {
            self.submit(cmd);
        }
        Ok(())
    }

    pub fn dequeue(&self) -> Option<Vec<GpuCommand>> {
        let mut queue = self.queue.lock();
        if queue.is_empty() {
            return None;
        }

        let mut batch = Vec::new();

        let priorities = [
            GpuPriority::Critical,
            GpuPriority::High,
            GpuPriority::Normal,
            GpuPriority::Idle,
        ];

        for priority in &priorities {
            let mut cmds = queue.dequeue_batch(*priority);
            let count = cmds.len() as u64;
            self.queued_count.fetch_sub(count, Ordering::Relaxed);
            batch.append(&mut cmds);
        }

        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }

    pub fn begin_frame(&self) {
        self.frame_guard.lock().begin_frame();
    }

    pub fn end_frame(&self, elapsed_ms: u64) {
        self.frame_guard.lock().end_frame(elapsed_ms);
    }

    pub fn queued_count(&self) -> u64 {
        self.queued_count.load(Ordering::Relaxed)
    }

    pub fn is_frame_pressure(&self) -> bool {
        self.frame_guard.lock().is_frame_pressure()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::command::GpuCommandKind;

    fn make_cmd(kind: GpuCommandKind, priority: GpuPriority) -> GpuCommand {
        GpuCommand::new(kind, priority, "sched_test")
    }

    #[test]
    fn test_new_scheduler() {
        let sched = GpuScheduler::new(16);
        assert_eq!(sched.queued_count(), 0);
        assert!(!sched.is_frame_pressure());
    }

    #[test]
    fn test_submit_and_dequeue() {
        let sched = GpuScheduler::new(16);
        sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Normal));
        assert_eq!(sched.queued_count(), 1);
        let batch = sched.dequeue().unwrap();
        assert_eq!(batch.len(), 1);
        assert_eq!(sched.queued_count(), 0);
    }

    #[test]
    fn test_dequeue_empty() {
        let sched = GpuScheduler::new(16);
        assert!(sched.dequeue().is_none());
    }

    #[test]
    fn test_submit_batch() {
        let sched = GpuScheduler::new(16);
        let mut batch = GpuBatch::new(GpuPriority::Normal);
        batch.add(make_cmd(GpuCommandKind::Render, GpuPriority::Normal));
        batch.add(make_cmd(GpuCommandKind::Transfer, GpuPriority::Normal));
        sched.submit_batch(batch).unwrap();
        assert_eq!(sched.queued_count(), 2);
    }

    #[test]
    fn test_frame_cycle() {
        let sched = GpuScheduler::new(16);
        sched.begin_frame();
        assert!(!sched.is_frame_pressure());
        sched.end_frame(10);
        assert!(!sched.is_frame_pressure());
        sched.end_frame(20);
        assert!(sched.is_frame_pressure());
    }

    #[test]
    fn test_priority_assignment() {
        let sched = GpuScheduler::new(16);
        sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Critical));
        sched.submit(make_cmd(GpuCommandKind::Compute, GpuPriority::Idle));
        let batch = sched.dequeue().unwrap();
        assert_eq!(batch.len(), 2);
    }

    #[test]
    fn test_queued_count_consistency() {
        let sched = GpuScheduler::new(16);
        sched.submit(make_cmd(GpuCommandKind::Barrier, GpuPriority::High));
        sched.submit(make_cmd(GpuCommandKind::Barrier, GpuPriority::High));
        assert_eq!(sched.queued_count(), 2);
        sched.dequeue();
        assert_eq!(sched.queued_count(), 0);
    }
}
