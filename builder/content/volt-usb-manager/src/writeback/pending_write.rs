use std::time::Instant;

use crate::writeback::write_status::WriteStatus;

#[derive(Debug, Clone)]
pub struct PendingWrite {
    pub write_id: u64,
    pub path: String,
    pub offset: u64,
    pub data: Vec<u8>,
    pub submitted_at: Instant,
    pub priority: u8,
    pub requires_fsync: bool,
    pub journal_record_id: u64,
    pub status: WriteStatus,
}

impl PendingWrite {
    pub fn new(id: u64, path: &str, offset: u64, data: Vec<u8>, priority: u8) -> Self {
        Self {
            write_id: id,
            path: path.to_string(),
            offset,
            data,
            submitted_at: Instant::now(),
            priority,
            requires_fsync: true,
            journal_record_id: 0,
            status: WriteStatus::Pending,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_flushable(&self) -> bool {
        matches!(self.status, WriteStatus::Pending)
    }

    pub fn elapsed_ms(&self) -> u128 {
        self.submitted_at.elapsed().as_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pending_write_new() {
        let pw = PendingWrite::new(1, "/path/to/file", 0, vec![1, 2, 3], 5);
        assert_eq!(pw.write_id, 1);
        assert_eq!(pw.path, "/path/to/file");
        assert_eq!(pw.offset, 0);
        assert_eq!(pw.data, vec![1, 2, 3]);
        assert_eq!(pw.priority, 5);
        assert_eq!(pw.status, WriteStatus::Pending);
    }

    #[test]
    fn test_pending_write_size() {
        let pw = PendingWrite::new(2, "/f", 0, vec![0u8; 1024], 1);
        assert_eq!(pw.size(), 1024);
    }

    #[test]
    fn test_is_flushable_pending() {
        let pw = PendingWrite::new(3, "/f", 0, vec![], 0);
        assert!(pw.is_flushable());
    }

    #[test]
    fn test_is_flushable_not_pending() {
        let mut pw = PendingWrite::new(4, "/f", 0, vec![], 0);
        pw.status = WriteStatus::Flushing;
        assert!(!pw.is_flushable());
        pw.status = WriteStatus::Durable;
        assert!(!pw.is_flushable());
    }

    #[test]
    fn test_requires_fsync_default() {
        let pw = PendingWrite::new(5, "/f", 0, vec![], 0);
        assert!(pw.requires_fsync);
    }

    #[test]
    fn test_journal_record_id_default() {
        let pw = PendingWrite::new(6, "/f", 0, vec![], 0);
        assert_eq!(pw.journal_record_id, 0);
    }

    #[test]
    fn test_elapsed_ms_increasing() {
        let pw = PendingWrite::new(7, "/f", 0, vec![], 0);
        let t1 = pw.elapsed_ms();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let t2 = pw.elapsed_ms();
        assert!(t2 >= t1);
    }

    #[test]
    fn test_clone() {
        let a = PendingWrite::new(8, "/clone", 10, vec![99], 3);
        let b = a.clone();
        assert_eq!(a.write_id, b.write_id);
        assert_eq!(a.path, b.path);
        assert_eq!(a.data, b.data);
    }
}
