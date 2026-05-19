use std::collections::VecDeque;

use crate::scheduler::priority::Priority;
use crate::scheduler::ScheduledTask;

pub struct PriorityQueues {
    queues: [VecDeque<ScheduledTask>; 5],
}

impl PriorityQueues {
    pub fn new() -> Self {
        Self {
            queues: [
                VecDeque::new(), // Critical
                VecDeque::new(), // High
                VecDeque::new(), // Normal
                VecDeque::new(), // Low
                VecDeque::new(), // Idle
            ],
        }
    }

    pub fn enqueue(&mut self, task: ScheduledTask) {
        let priority = task.command.priority().min(4) as usize;
        self.queues[priority].push_back(task);
    }

    /// Dequeue one cycle of tasks respecting priority caps.
    /// Returns tasks in priority order, round-robin within each level.
    ///
    /// Algorithm:
    /// 1. CRITICAL: all tasks execute immediately
    /// 2. HIGH: up to 8 tasks
    /// 3. NORMAL: up to 4 tasks
    /// 4. LOW: up to 2 tasks
    /// 5. IDLE: up to 1 task
    pub fn dequeue_cycle(&mut self) -> Vec<ScheduledTask> {
        let mut output = Vec::new();

        let priorities = [
            (0, None),          // Critical — all
            (1, Some(8)),       // High — max 8
            (2, Some(4)),       // Normal — max 4
            (3, Some(2)),       // Low — max 2
            (4, Some(1)),       // Idle — max 1
        ];

        for (idx, max) in &priorities {
            let max_tasks = max.unwrap_or(usize::MAX);
            let queue = &mut self.queues[*idx];
            let count = queue.len().min(max_tasks);
            for _ in 0..count {
                if let Some(task) = queue.pop_front() {
                    output.push(task);
                }
            }
        }

        output
    }

    pub fn len(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }

    /// Peek at the next task without removing it.
    pub fn peek(&self) -> Option<&ScheduledTask> {
        for queue in &self.queues {
            if let Some(task) = queue.front() {
                return Some(task);
            }
        }
        None
    }

    pub fn queue_len(&self, priority: Priority) -> usize {
        self.queues[priority as usize].len()
    }

    pub fn clear(&mut self) {
        for queue in &mut self.queues {
            queue.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use crate::protocol::header::SbpHeader;
    use crate::protocol::opcodes::Opcode;
    use crate::protocol::TesseractCommand;

    fn make_cmd(priority: u8) -> ScheduledTask {
        let header = SbpHeader::new(Opcode::Heartbeat, priority, 0, 0);
        let cmd = TesseractCommand::new(header, vec![]);
        let (tx, _) = crossbeam::channel::unbounded();
        ScheduledTask {
            command: cmd,
            received_at: Instant::now(),
            response_tx: tx,
        }
    }

    #[test]
    fn test_priority_ordering() {
        let mut pq = PriorityQueues::new();
        pq.enqueue(make_cmd(4)); // IDLE
        pq.enqueue(make_cmd(0)); // CRITICAL
        pq.enqueue(make_cmd(2)); // NORMAL
        pq.enqueue(make_cmd(1)); // HIGH
        pq.enqueue(make_cmd(3)); // LOW

        let tasks = pq.dequeue_cycle();
        assert_eq!(tasks.len(), 5);
        assert_eq!(tasks[0].command.priority(), 0);
        assert_eq!(tasks[1].command.priority(), 1);
        assert_eq!(tasks[2].command.priority(), 2);
        assert_eq!(tasks[3].command.priority(), 3);
        assert_eq!(tasks[4].command.priority(), 4);
    }

    #[test]
    fn test_priority_caps() {
        let mut pq = PriorityQueues::new();
        for _ in 0..20 {
            pq.enqueue(make_cmd(2)); // NORMAL — max 4 per cycle
        }
        let tasks = pq.dequeue_cycle();
        assert_eq!(tasks.len(), 4);
    }

    #[test]
    fn test_critical_unbounded() {
        let mut pq = PriorityQueues::new();
        for _ in 0..100 {
            pq.enqueue(make_cmd(0));
        }
        let tasks = pq.dequeue_cycle();
        assert_eq!(tasks.len(), 100);
    }

    #[test]
    fn test_round_robin() {
        let mut pq = PriorityQueues::new();
        pq.enqueue(make_cmd(1));
        pq.enqueue(make_cmd(1));
        pq.enqueue(make_cmd(1));
        let tasks = pq.dequeue_cycle();
        assert_eq!(tasks.len(), 3);
    }

    #[test]
    fn test_empty() {
        let mut pq = PriorityQueues::new();
        assert!(pq.is_empty());
        let tasks = pq.dequeue_cycle();
        assert!(tasks.is_empty());
    }
}
