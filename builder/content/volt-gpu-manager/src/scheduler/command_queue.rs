use std::collections::VecDeque;
use crate::scheduler::command::GpuCommand;
use crate::scheduler::priority::GpuPriority;

pub struct GpuCommandQueue {
    queues: [VecDeque<GpuCommand>; 4],
    total: usize,
}

impl GpuCommandQueue {
    pub fn new() -> Self {
        GpuCommandQueue {
            queues: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            total: 0,
        }
    }

    pub fn enqueue(&mut self, cmd: GpuCommand) {
        let idx = cmd.priority as usize;
        self.queues[idx].push_back(cmd);
        self.total += 1;
    }

    pub fn dequeue(&mut self) -> Option<GpuCommand> {
        for queue in &mut self.queues {
            if let Some(cmd) = queue.pop_front() {
                self.total -= 1;
                return Some(cmd);
            }
        }
        None
    }

    pub fn dequeue_batch(&mut self, priority: GpuPriority) -> Vec<GpuCommand> {
        let idx = priority as usize;
        let batch_size = priority.batch_size();
        let mut batch = Vec::with_capacity(batch_size);
        let queue = &mut self.queues[idx];
        while batch.len() < batch_size {
            match queue.pop_front() {
                Some(cmd) => {
                    batch.push(cmd);
                    self.total -= 1;
                }
                None => break,
            }
        }
        batch
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }

    pub fn len(&self) -> usize {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::command::GpuCommandKind;

    fn make_cmd(priority: GpuPriority) -> GpuCommand {
        GpuCommand::new(GpuCommandKind::Compute, priority, "test")
    }

    #[test]
    fn test_new_queue_empty() {
        let q = GpuCommandQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut q = GpuCommandQueue::new();
        q.enqueue(make_cmd(GpuPriority::Normal));
        assert_eq!(q.len(), 1);
        assert!(!q.is_empty());
        let cmd = q.dequeue().unwrap();
        assert_eq!(cmd.priority, GpuPriority::Normal);
        assert!(q.is_empty());
    }

    #[test]
    fn test_priority_order() {
        let mut q = GpuCommandQueue::new();
        q.enqueue(make_cmd(GpuPriority::Idle));
        q.enqueue(make_cmd(GpuPriority::Critical));
        q.enqueue(make_cmd(GpuPriority::Normal));
        q.enqueue(make_cmd(GpuPriority::High));
        assert_eq!(q.dequeue().unwrap().priority, GpuPriority::Critical);
        assert_eq!(q.dequeue().unwrap().priority, GpuPriority::High);
        assert_eq!(q.dequeue().unwrap().priority, GpuPriority::Normal);
        assert_eq!(q.dequeue().unwrap().priority, GpuPriority::Idle);
    }

    #[test]
    fn test_dequeue_empty() {
        let mut q = GpuCommandQueue::new();
        assert!(q.dequeue().is_none());
    }

    #[test]
    fn test_dequeue_batch() {
        let mut q = GpuCommandQueue::new();
        for _ in 0..10 {
            q.enqueue(make_cmd(GpuPriority::Normal));
        }
        let batch = q.dequeue_batch(GpuPriority::Normal);
        assert_eq!(batch.len(), 8);
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn test_dequeue_batch_partial() {
        let mut q = GpuCommandQueue::new();
        q.enqueue(make_cmd(GpuPriority::High));
        q.enqueue(make_cmd(GpuPriority::High));
        let batch = q.dequeue_batch(GpuPriority::High);
        assert_eq!(batch.len(), 2);
        assert!(q.is_empty());
    }

    #[test]
    fn test_len_consistency() {
        let mut q = GpuCommandQueue::new();
        q.enqueue(make_cmd(GpuPriority::Critical));
        q.enqueue(make_cmd(GpuPriority::Normal));
        q.enqueue(make_cmd(GpuPriority::Idle));
        assert_eq!(q.len(), 3);
        q.dequeue();
        assert_eq!(q.len(), 2);
        q.dequeue_batch(GpuPriority::Idle);
        assert_eq!(q.len(), 1);
    }
}
