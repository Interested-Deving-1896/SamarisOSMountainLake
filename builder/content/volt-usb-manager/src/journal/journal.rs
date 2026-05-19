use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use once_cell::sync::OnceCell;

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::journal::checkpoint::Checkpoint;
use crate::journal::commit::JournalCommit;
use crate::journal::wal::WalWriter;

#[derive(Debug, Clone)]
pub struct JournalConfig {
    pub path: String,
    pub fsync_on_record: bool,
    pub checkpoint_interval_ms: u64,
}

impl Default for JournalConfig {
    fn default() -> Self {
        Self {
            path: "journal".to_string(),
            fsync_on_record: false,
            checkpoint_interval_ms: 5000,
        }
    }
}

pub struct Journal {
    commit: OnceCell<JournalCommit>,
    config: JournalConfig,
    records_since_checkpoint: AtomicU64,
    dirty: AtomicBool,
}

impl Journal {
    fn get_commit(&self) -> &JournalCommit {
        self.commit.get_or_init(|| {
            let wal_path = Path::new(&self.config.path).join("wal.dat");
            let wal_str = wal_path.to_str().expect("Journal: invalid WAL path");
            std::fs::create_dir_all(Path::new(&self.config.path))
                .expect("Journal: cannot create journal directory");
            let writer =
                WalWriter::create(wal_str).expect("Journal: cannot create WAL writer");
            JournalCommit::new(writer)
        })
    }

    pub fn new(path: &str) -> Self {
        let was_dirty = Path::new(path).exists();
        Self {
            commit: OnceCell::new(),
            config: JournalConfig {
                path: path.to_string(),
                fsync_on_record: false,
                checkpoint_interval_ms: 5000,
            },
            records_since_checkpoint: AtomicU64::new(0),
            dirty: AtomicBool::new(was_dirty),
        }
    }

    pub fn open(config: JournalConfig) -> VumResult<Self> {
        let wal_dir = Path::new(&config.path);
        let wal_path = wal_dir.join("wal.dat");
        let was_dirty = wal_path.exists();
        std::fs::create_dir_all(wal_dir).map_err(|e| {
            VumError::JournalOpenFailed(format!("Cannot create journal dir: {}", e))
        })?;
        Ok(Self {
            commit: OnceCell::new(),
            config,
            records_since_checkpoint: AtomicU64::new(0),
            dirty: AtomicBool::new(was_dirty),
        })
    }

    pub fn begin_write(&self, path: &str, data: Vec<u8>) -> VumResult<u64> {
        let commit = self.get_commit();
        let record = commit.begin_write(path, data)?;
        self.dirty.store(true, Ordering::Relaxed);
        self.records_since_checkpoint.fetch_add(1, Ordering::Relaxed);
        if self.config.fsync_on_record {
            commit.writer.fsync()?;
        }
        Ok(record.record_id)
    }

    pub fn commit_write(&self, record_id: u64) -> VumResult<()> {
        let commit = self.get_commit();
        commit.commit_write(record_id)?;
        if self.config.fsync_on_record {
            commit.writer.fsync()?;
        }
        let count = self.records_since_checkpoint.load(Ordering::Relaxed);
        if count >= self.config.checkpoint_interval_ms / 100 {
            let _ = commit;
            self.checkpoint()?;
        }
        Ok(())
    }

    pub fn abort_write(&self, record_id: u64) -> VumResult<()> {
        let commit = self.get_commit();
        commit.abort_write(record_id)?;
        if self.config.fsync_on_record {
            commit.writer.fsync()?;
        }
        Ok(())
    }

    pub fn checkpoint(&self) -> VumResult<()> {
        {
            let commit = self.get_commit();
            commit.checkpoint()?;
        }
        let last_id = self.get_commit().pending_id().saturating_sub(1);
        Checkpoint::write(&self.config.path, last_id)?;
        self.records_since_checkpoint.store(0, Ordering::Relaxed);
        self.dirty.store(false, Ordering::Relaxed);
        Ok(())
    }

    pub fn clean_shutdown(&self) -> VumResult<()> {
        {
            let commit = self.get_commit();
            commit.clean_shutdown()?;
        }
        Checkpoint::clear(&self.config.path)?;
        self.dirty.store(false, Ordering::Relaxed);
        self.records_since_checkpoint.store(0, Ordering::Relaxed);
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    pub fn record_count(&self) -> u64 {
        self.commit
            .get()
            .map(|c| c.writer.record_count())
            .unwrap_or(0)
    }

    pub fn bytes_written(&self) -> u64 {
        self.commit
            .get()
            .map(|c| c.writer.bytes_written())
            .unwrap_or(0)
    }

    pub fn config(&self) -> &JournalConfig {
        &self.config
    }
}

impl std::fmt::Debug for Journal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Journal")
            .field("config", &self.config)
            .field("dirty", &self.is_dirty())
            .field(
                "records_since_checkpoint",
                &self.records_since_checkpoint.load(Ordering::Relaxed),
            )
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_config(dir: &tempfile::TempDir) -> JournalConfig {
        JournalConfig {
            path: dir.path().to_str().unwrap().to_string(),
            fsync_on_record: false,
            checkpoint_interval_ms: 5000,
        }
    }

    #[test]
    fn test_journal_open() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        assert!(!journal.is_dirty());
        assert_eq!(journal.record_count(), 0);
    }

    #[test]
    fn test_journal_begin_and_commit() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        let id = journal.begin_write("/test/file", vec![1, 2, 3]).unwrap();
        assert_eq!(id, 1);
        assert!(journal.is_dirty());
        journal.commit_write(id).unwrap();
        assert!(journal.is_dirty());
        assert!(journal.record_count() >= 2);
    }

    #[test]
    fn test_journal_abort() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        let id = journal.begin_write("/abort/file", vec![0]).unwrap();
        journal.abort_write(id).unwrap();
        assert_eq!(journal.record_count(), 2);
    }

    #[test]
    fn test_journal_checkpoint() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        let id = journal.begin_write("/cp", vec![1]).unwrap();
        journal.commit_write(id).unwrap();
        journal.checkpoint().unwrap();
        assert!(!journal.is_dirty());
        assert_eq!(journal.records_since_checkpoint.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_journal_clean_shutdown() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        journal.clean_shutdown().unwrap();
        assert!(!journal.is_dirty());
    }

    #[test]
    fn test_journal_bytes_written() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        let id = journal.begin_write("/bytes", b"1234567890".to_vec()).unwrap();
        journal.commit_write(id).unwrap();
        assert!(journal.bytes_written() > 0);
    }

    #[test]
    fn test_journal_config_default() {
        let config = JournalConfig::default();
        assert_eq!(config.path, "journal");
        assert!(!config.fsync_on_record);
        assert_eq!(config.checkpoint_interval_ms, 5000);
    }

    #[test]
    fn test_journal_fsync_on_record() {
        let dir = tempdir().unwrap();
        let config = JournalConfig {
            path: dir.path().to_str().unwrap().to_string(),
            fsync_on_record: true,
            checkpoint_interval_ms: 999999,
        };
        let journal = Journal::open(config).unwrap();
        let id = journal.begin_write("/fsync", vec![42]).unwrap();
        journal.commit_write(id).unwrap();
        assert!(journal.record_count() >= 2);
    }

    #[test]
    fn test_journal_multiple_writes() {
        let dir = tempdir().unwrap();
        let config = make_config(&dir);
        let journal = Journal::open(config).unwrap();
        for i in 0..5 {
            let id = journal.begin_write(&format!("/file{}", i), vec![i]).unwrap();
            journal.commit_write(id).unwrap();
        }
        assert_eq!(journal.record_count(), 10);
    }

    #[test]
    fn test_journal_new_never_fails() {
        let _journal = Journal::new("/nonexistent/path/that/cant/be/created");
    }
}
