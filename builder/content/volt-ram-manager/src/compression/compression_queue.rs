use std::sync::Arc;
use crossbeam::channel::{unbounded, Sender, Receiver};
use crate::compression::algorithm::CompressionAlgorithm;
use crate::pages::page_id::PageId;
use crate::core::result::VrmResult;

#[derive(Debug, Clone)]
pub struct CompressionJob {
    pub page_id: PageId,
    pub data: Vec<u8>,
    pub algorithm: CompressionAlgorithm,
}

pub struct CompressionQueue {
    tx: Sender<CompressionJob>,
    rx: Receiver<CompressionJob>,
}

impl CompressionQueue {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        CompressionQueue { tx, rx }
    }

    pub fn enqueue(&self, job: CompressionJob) -> VrmResult<()> {
        self.tx
            .send(job)
            .map_err(|e| crate::core::error::VrmError::Other(format!("enqueue compression: {}", e)))
    }

    pub fn dequeue(&self) -> Option<CompressionJob> {
        self.rx.try_recv().ok()
    }

    pub fn len(&self) -> usize {
        self.rx.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::page_id::PageId;

    #[test]
    fn test_enqueue_dequeue() {
        let queue = CompressionQueue::new();
        let job = CompressionJob {
            page_id: PageId::new(),
            data: vec![1, 2, 3],
            algorithm: CompressionAlgorithm::None,
        };
        queue.enqueue(job.clone()).unwrap();
        let popped = queue.dequeue().unwrap();
        assert_eq!(popped.page_id, job.page_id);
        assert_eq!(popped.data, job.data);
    }

    #[test]
    fn test_empty_dequeue() {
        let queue = CompressionQueue::new();
        assert!(queue.dequeue().is_none());
    }

    #[test]
    fn test_len() {
        let queue = CompressionQueue::new();
        assert_eq!(queue.len(), 0);
        queue
            .enqueue(CompressionJob {
                page_id: PageId::new(),
                data: vec![],
                algorithm: CompressionAlgorithm::None,
            })
            .unwrap();
        assert!(queue.len() > 0);
    }
}
