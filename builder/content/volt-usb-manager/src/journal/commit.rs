use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::result::VumResult;
use crate::journal::record::JournalRecord;
use crate::journal::record_type::RecordType;
use crate::journal::wal::WalWriter;

pub struct JournalCommit {
    pub writer: WalWriter,
    next_id: AtomicU64,
}

impl JournalCommit {
    pub fn new(writer: WalWriter) -> Self {
        Self {
            writer,
            next_id: AtomicU64::new(1),
        }
    }

    pub fn pending_id(&self) -> u64 {
        self.next_id.load(Ordering::Relaxed)
    }

    pub fn begin_write(&self, path: &str, data: Vec<u8>) -> VumResult<JournalRecord> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let record = JournalRecord::new(RecordType::BeginWrite, id, path, data);
        self.writer.append(&record)?;
        Ok(record)
    }

    pub fn commit_write(&self, record_id: u64) -> VumResult<()> {
        let record = JournalRecord::new(RecordType::CommitWrite, record_id, "", vec![]);
        self.writer.append(&record)?;
        Ok(())
    }

    pub fn abort_write(&self, record_id: u64) -> VumResult<()> {
        let record = JournalRecord::new(RecordType::AbortWrite, record_id, "", vec![]);
        self.writer.append(&record)?;
        Ok(())
    }

    pub fn begin_delete(&self, path: &str) -> VumResult<JournalRecord> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let record = JournalRecord::new(RecordType::BeginDelete, id, path, vec![]);
        self.writer.append(&record)?;
        Ok(record)
    }

    pub fn commit_delete(&self, record_id: u64) -> VumResult<()> {
        let record = JournalRecord::new(RecordType::CommitDelete, record_id, "", vec![]);
        self.writer.append(&record)?;
        Ok(())
    }

    pub fn checkpoint(&self) -> VumResult<()> {
        let record = JournalRecord::new(RecordType::Checkpoint, 0, "", vec![]);
        self.writer.append(&record)?;
        self.writer.fsync()?;
        Ok(())
    }

    pub fn clean_shutdown(&self) -> VumResult<()> {
        let record = JournalRecord::new(RecordType::CleanShutdown, 0, "", vec![]);
        self.writer.append(&record)?;
        self.writer.fsync()?;
        Ok(())
    }
}

impl std::fmt::Debug for JournalCommit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JournalCommit")
            .field("pending_id", &self.pending_id())
            .field("writer", &self.writer)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_commit() -> (JournalCommit, tempfile::TempDir) {
        let dir = tempdir().unwrap();
        let path = dir.path().join("wal.dat").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        let commit = JournalCommit::new(writer);
        (commit, dir)
    }

    #[test]
    fn test_begin_write_creates_record() {
        let (commit, _dir) = setup_commit();
        let record = commit.begin_write("/test/path", vec![1, 2, 3]).unwrap();
        assert_eq!(record.record_type, RecordType::BeginWrite);
        assert_eq!(record.record_id, 1);
        assert_eq!(record.path, "/test/path");
        assert_eq!(record.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_commit_write_appends() {
        let (commit, _dir) = setup_commit();
        let rec = commit.begin_write("/a", vec![0]).unwrap();
        commit.commit_write(rec.record_id).unwrap();
        assert_eq!(commit.writer.record_count(), 2);
    }

    #[test]
    fn test_abort_write_appends() {
        let (commit, _dir) = setup_commit();
        let rec = commit.begin_write("/a", vec![0]).unwrap();
        commit.abort_write(rec.record_id).unwrap();
        assert_eq!(commit.writer.record_count(), 2);
    }

    #[test]
    fn test_checkpoint_fsyncs() {
        let (commit, _dir) = setup_commit();
        commit.checkpoint().unwrap();
        assert_eq!(commit.writer.record_count(), 1);
    }

    #[test]
    fn test_clean_shutdown_fsyncs() {
        let (commit, _dir) = setup_commit();
        commit.clean_shutdown().unwrap();
        assert_eq!(commit.writer.record_count(), 1);
    }

    #[test]
    fn test_pending_id_increments() {
        let (commit, _dir) = setup_commit();
        let id1 = commit.pending_id();
        commit.begin_write("/a", vec![]).unwrap();
        let id2 = commit.pending_id();
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_begin_delete_creates_record() {
        let (commit, _dir) = setup_commit();
        let record = commit.begin_delete("/file/to/delete").unwrap();
        assert_eq!(record.record_type, RecordType::BeginDelete);
        assert_eq!(record.path, "/file/to/delete");
    }

    #[test]
    fn test_commit_delete_appends() {
        let (commit, _dir) = setup_commit();
        let rec = commit.begin_delete("/f").unwrap();
        commit.commit_delete(rec.record_id).unwrap();
        assert_eq!(commit.writer.record_count(), 2);
    }
}
