use crossbeam::channel::{unbounded, Sender, Receiver};
use crate::compression::algorithm::CompressionAlgorithm;
use crate::pages::page_id::PageId;
use crate::core::result::VrmResult;

#[derive(Debug, Clone)]
pub struct DecompressionJob {
    pub page_id: PageId,
    pub compressed_data: Vec<u8>,
    pub algorithm: CompressionAlgorithm,
}

pub struct DecompressionQueue {
    tx: Sender<DecompressionJob>,
    rx: Receiver<DecompressionJob>,
}

impl DecompressionQueue {
    pub fn new() -> Self {
        let (tx, rx) = unbounded();
        DecompressionQueue { tx, rx }
    }

    pub fn enqueue(&self, job: DecompressionJob) -> VrmResult<()> {
        self.tx
            .send(job)
            .map_err(|e| crate::core::error::VrmError::Other(format!("enqueue decompression: {}", e)))
    }

    pub fn dequeue(&self) -> Option<DecompressionJob> {
        self.rx.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::page_id::PageId;

    #[test]
    fn test_enqueue_dequeue() {
        let queue = DecompressionQueue::new();
        let job = DecompressionJob {
            page_id: PageId::new(),
            compressed_data: vec![4, 5, 6],
            algorithm: CompressionAlgorithm::None,
        };
        queue.enqueue(job.clone()).unwrap();
        let popped = queue.dequeue().unwrap();
        assert_eq!(popped.page_id, job.page_id);
        assert_eq!(popped.compressed_data, job.compressed_data);
    }

    #[test]
    fn test_empty_dequeue() {
        let queue = DecompressionQueue::new();
        assert!(queue.dequeue().is_none());
    }
}
