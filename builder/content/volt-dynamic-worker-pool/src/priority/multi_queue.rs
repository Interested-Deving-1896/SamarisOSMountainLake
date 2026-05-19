use std::collections::VecDeque;
use std::sync::Mutex;

use crate::job::job::Job;
use crate::job::job_id::JobId;
use crate::priority::level::PriorityLevel;

pub struct MultiQueue {
    queues: [Mutex<VecDeque<Job>>; 5],
}

impl MultiQueue {
    pub fn new() -> Self {
        Self {
            queues: [
                Mutex::new(VecDeque::new()),
                Mutex::new(VecDeque::new()),
                Mutex::new(VecDeque::new()),
                Mutex::new(VecDeque::new()),
                Mutex::new(VecDeque::new()),
            ],
        }
    }

    fn queue_index(priority: PriorityLevel) -> usize {
        priority.as_u8() as usize
    }

    pub fn enqueue(&self, job: Job) {
        let idx = Self::queue_index(job.priority());
        let mut queue = self.queues[idx].lock().unwrap();
        queue.push_back(job);
    }

    pub fn dequeue(&self) -> Option<Job> {
        for i in (0..5).rev() {
            let mut queue = self.queues[i].lock().unwrap();
            if let Some(job) = queue.pop_front() {
                return Some(job);
            }
        }
        None
    }

    pub fn dequeue_for_priority(&self, priority: PriorityLevel) -> Option<Job> {
        let idx = Self::queue_index(priority);
        let mut queue = self.queues[idx].lock().unwrap();
        queue.pop_front()
    }

    pub fn cancel(&self, job_id: &JobId) -> bool {
        for i in 0..5 {
            let mut queue = self.queues[i].lock().unwrap();
            if let Some(pos) = queue.iter().position(|j| j.id() == job_id) {
                queue.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn queue_depth(&self) -> usize {
        let mut total = 0;
        for i in 0..5 {
            total += self.queues[i].lock().unwrap().len();
        }
        total
    }

    pub fn queue_depth_by_priority(&self, priority: PriorityLevel) -> usize {
        let idx = Self::queue_index(priority);
        self.queues[idx].lock().unwrap().len()
    }

    pub fn has_high_priority_jobs(&self) -> bool {
        for i in 2..5 {
            if !self.queues[i].lock().unwrap().is_empty() {
                return true;
            }
        }
        false
    }

    pub fn has_jobs_above(&self, priority: PriorityLevel) -> bool {
        let threshold = Self::queue_index(priority);
        for i in (threshold + 1)..5 {
            if !self.queues[i].lock().unwrap().is_empty() {
                return true;
            }
        }
        false
    }

    pub fn iter_priority(&self) -> impl Iterator<Item = PriorityLevel> + '_ {
        (0..5).rev().filter_map(|i| {
            let queue = self.queues[i].lock().unwrap();
            if queue.is_empty() {
                None
            } else {
                PriorityLevel::from_u8(i as u8)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::job::Job;
    use crate::job::job_id::JobId;
    use crate::priority::level::PriorityLevel;

    fn make_job(priority: PriorityLevel, data: &str) -> Job {
        let id = JobId::new();
        Job::new(id, format!("test_{}", data), priority, 1024)
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mq = MultiQueue::new();
        assert!(mq.dequeue().is_none());

        let job = make_job(PriorityLevel::Normal, "1");
        mq.enqueue(job);
        assert_eq!(mq.queue_depth(), 1);
        assert!(mq.dequeue().is_some());
        assert_eq!(mq.queue_depth(), 0);
    }

    #[test]
    fn test_priority_ordering() {
        let mq = MultiQueue::new();
        let low = make_job(PriorityLevel::Low, "low");
        let high = make_job(PriorityLevel::High, "high");
        mq.enqueue(low);
        mq.enqueue(high);

        let first = mq.dequeue().unwrap();
        assert_eq!(first.priority(), PriorityLevel::High);
        let second = mq.dequeue().unwrap();
        assert_eq!(second.priority(), PriorityLevel::Low);
    }

    #[test]
    fn test_cancel() {
        let mq = MultiQueue::new();
        let job = make_job(PriorityLevel::Normal, "cancel");
        let id = job.id().clone();
        mq.enqueue(job);
        assert_eq!(mq.queue_depth(), 1);
        assert!(mq.cancel(&id));
        assert_eq!(mq.queue_depth(), 0);
    }

    #[test]
    fn test_has_high_priority_jobs() {
        let mq = MultiQueue::new();
        assert!(!mq.has_high_priority_jobs());
        let job = make_job(PriorityLevel::High, "high");
        mq.enqueue(job);
        assert!(mq.has_high_priority_jobs());
    }
}
