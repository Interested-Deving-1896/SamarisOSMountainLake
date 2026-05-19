use crate::writeback::pending_write::PendingWrite;

#[derive(Debug, Clone)]
pub struct WriteBatch {
    pub writes: Vec<PendingWrite>,
    pub total_bytes: u64,
    pub is_metadata: bool,
}

impl WriteBatch {
    pub fn new() -> Self {
        Self {
            writes: Vec::new(),
            total_bytes: 0,
            is_metadata: false,
        }
    }

    pub fn add(&mut self, write: PendingWrite) {
        if write.data.is_empty() && write.path.is_empty() {
            self.is_metadata = true;
        }
        self.total_bytes += write.size() as u64;
        self.writes.push(write);
    }

    pub fn can_add(&self, write: &PendingWrite, max_batch_kb: u64) -> bool {
        let max_bytes = (max_batch_kb * 1024) as u64;
        self.total_bytes + write.size() as u64 <= max_bytes
    }

    pub fn is_empty(&self) -> bool {
        self.writes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.writes.len()
    }
}

impl Default for WriteBatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_write(id: u64, data: Vec<u8>) -> PendingWrite {
        PendingWrite::new(id, "/f", 0, data, 0)
    }

    #[test]
    fn test_batch_new() {
        let batch = WriteBatch::new();
        assert!(batch.is_empty());
        assert_eq!(batch.total_bytes, 0);
        assert!(!batch.is_metadata);
    }

    #[test]
    fn test_batch_add() {
        let mut batch = WriteBatch::new();
        batch.add(make_write(1, vec![0u8; 100]));
        assert!(!batch.is_empty());
        assert_eq!(batch.total_bytes, 100);
        assert_eq!(batch.len(), 1);
    }

    #[test]
    fn test_batch_can_add() {
        let mut batch = WriteBatch::new();
        batch.add(make_write(1, vec![0u8; 500]));
        let new_write = make_write(2, vec![0u8; 600]);
        assert!(batch.can_add(&new_write, 2)); // 2KB = 2048 bytes, 500+600=1100 <= 2048
        assert!(!batch.can_add(&new_write, 1)); // 1KB = 1024 bytes, 500+600=1100 > 1024
    }

    #[test]
    fn test_batch_can_add_empty_batch() {
        let batch = WriteBatch::new();
        let write = make_write(1, vec![0u8; 4096]);
        assert!(batch.can_add(&write, 4));
        assert!(!batch.can_add(&write, 3));
    }

    #[test]
    fn test_batch_is_metadata_flag() {
        let mut batch = WriteBatch::new();
        let empty_write = PendingWrite::new(1, "", 0, vec![], 0);
        batch.add(empty_write);
        assert!(batch.is_metadata);
    }

    #[test]
    fn test_batch_multiple_adds() {
        let mut batch = WriteBatch::new();
        for i in 0..5 {
            batch.add(make_write(i, vec![i as u8; 10]));
        }
        assert_eq!(batch.len(), 5);
        assert_eq!(batch.total_bytes, 50);
    }

    #[test]
    fn test_batch_clone() {
        let mut batch = WriteBatch::new();
        batch.add(make_write(1, vec![1, 2, 3]));
        let cloned = batch.clone();
        assert_eq!(cloned.len(), 1);
        assert_eq!(cloned.total_bytes, 3);
    }
}
