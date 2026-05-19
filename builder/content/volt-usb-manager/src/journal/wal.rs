use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::journal::record::JournalRecord;

pub struct WalWriter {
    file: Mutex<File>,
    path: String,
    bytes_written: AtomicU64,
    record_count: AtomicU64,
}

impl WalWriter {
    pub fn open(path: &str) -> VumResult<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)
            .map_err(|e| VumError::JournalOpenFailed(format!("Cannot open WAL: {}", e)))?;
        let bytes = file.metadata().map(|m| m.len()).unwrap_or(0);
        Ok(Self {
            file: Mutex::new(file),
            path: path.to_string(),
            bytes_written: AtomicU64::new(bytes),
            record_count: AtomicU64::new(0),
        })
    }

    pub fn create(path: &str) -> VumResult<Self> {
        let parent = Path::new(path).parent();
        if let Some(p) = parent {
            if !p.as_os_str().is_empty() {
                std::fs::create_dir_all(p).map_err(|e| {
                    VumError::JournalOpenFailed(format!("Cannot create WAL dir: {}", e))
                })?;
            }
        }
        Self::open(path)
    }

    pub fn append(&self, record: &JournalRecord) -> VumResult<()> {
        let bytes = record.to_bytes();
        let mut file = self.file.lock();
        file.write_all(&bytes).map_err(|e| {
            VumError::JournalWriteFailed(format!("WAL append: {}", e))
        })?;
        self.bytes_written.fetch_add(bytes.len() as u64, Ordering::Relaxed);
        self.record_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    pub fn fsync(&self) -> VumResult<()> {
        let file = self.file.lock();
        file.sync_all().map_err(|e| {
            VumError::FsyncFailed(format!("WAL fsync: {}", e))
        })
    }

    pub fn bytes_written(&self) -> u64 {
        self.bytes_written.load(Ordering::Relaxed)
    }

    pub fn record_count(&self) -> u64 {
        self.record_count.load(Ordering::Relaxed)
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

pub struct WalReader {
    data: Vec<u8>,
    pos: usize,
}

impl WalReader {
    pub fn open(path: &str) -> VumResult<Self> {
        let data = match std::fs::read(path) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                return Err(VumError::ReadFailed(format!("Cannot read WAL: {}", e)));
            }
        };
        Ok(Self { data, pos: 0 })
    }

    pub fn read_record(&mut self) -> Option<JournalRecord> {
        if self.pos >= self.data.len() {
            return None;
        }
        if self.data.len() - self.pos < 4 {
            return None;
        }
        let magic = u32::from_le_bytes(
            self.data[self.pos..self.pos + 4].try_into().unwrap(),
        );
        if magic != crate::journal::record::RECORD_MAGIC {
            return None;
        }
        match JournalRecord::from_bytes(&self.data[self.pos..]) {
            Ok(record) => {
                self.pos += record.size();
                Some(record)
            }
            Err(_) => None,
        }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn total_bytes(&self) -> usize {
        self.data.len()
    }
}

impl std::fmt::Debug for WalWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalWriter")
            .field("path", &self.path)
            .field("bytes_written", &self.bytes_written.load(Ordering::Relaxed))
            .field("record_count", &self.record_count.load(Ordering::Relaxed))
            .finish()
    }
}

impl std::fmt::Debug for WalReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalReader")
            .field("pos", &self.pos)
            .field("total_bytes", &self.data.len())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::record_type::RecordType;
    use tempfile::tempdir;

    fn make_record(id: u64, path: &str, data: &[u8]) -> JournalRecord {
        JournalRecord::new(RecordType::BeginWrite, id, path, data.to_vec())
    }

    #[test]
    fn test_wal_writer_open_and_append() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_wal").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        let rec = make_record(1, "/a", b"hello");
        writer.append(&rec).unwrap();
        assert_eq!(writer.record_count(), 1);
        assert!(writer.bytes_written() > 0);
    }

    #[test]
    fn test_wal_writer_fsync() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("fsync_test").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        writer.append(&make_record(1, "/x", b"data")).unwrap();
        writer.fsync().unwrap();
    }

    #[test]
    fn test_wal_reader_open_empty() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty_wal").to_str().unwrap().to_string();
        let _writer = WalWriter::open(&path).unwrap();
        let mut reader = WalReader::open(&path).unwrap();
        assert!(reader.read_record().is_none());
        assert_eq!(reader.remaining(), 0);
    }

    #[test]
    fn test_wal_write_then_read() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("wal_rw").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        writer.append(&make_record(10, "/file.bin", &[1, 2, 3])).unwrap();
        writer.append(&make_record(11, "/dir/file2.bin", &[4, 5])).unwrap();
        writer.fsync().unwrap();
        drop(writer);

        let mut reader = WalReader::open(&path).unwrap();
        let r1 = reader.read_record().unwrap();
        assert_eq!(r1.record_id, 10);
        assert_eq!(r1.path, "/file.bin");
        assert_eq!(r1.data, vec![1, 2, 3]);
        let r2 = reader.read_record().unwrap();
        assert_eq!(r2.record_id, 11);
        assert_eq!(r2.path, "/dir/file2.bin");
        assert_eq!(r2.data, vec![4, 5]);
        assert!(reader.read_record().is_none());
    }

    #[test]
    fn test_wal_reader_multiple_types() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("wal_types").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        writer.append(&JournalRecord::new(RecordType::BeginWrite, 1, "/a", vec![0])).unwrap();
        writer.append(&JournalRecord::new(RecordType::CommitWrite, 1, "/a", vec![])).unwrap();
        writer.append(&JournalRecord::new(RecordType::Checkpoint, 0, "", vec![])).unwrap();
        writer.append(&JournalRecord::new(RecordType::CleanShutdown, 0, "", vec![])).unwrap();
        writer.fsync().unwrap();
        drop(writer);

        let mut reader = WalReader::open(&path).unwrap();
        assert_eq!(reader.read_record().unwrap().record_type, RecordType::BeginWrite);
        assert_eq!(reader.read_record().unwrap().record_type, RecordType::CommitWrite);
        assert_eq!(reader.read_record().unwrap().record_type, RecordType::Checkpoint);
        assert_eq!(reader.read_record().unwrap().record_type, RecordType::CleanShutdown);
        assert!(reader.read_record().is_none());
    }

    #[test]
    fn test_wal_writer_path() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("path_test").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        assert_eq!(writer.path(), &path);
    }

    #[test]
    fn test_wal_reader_total_bytes() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("total").to_str().unwrap().to_string();
        let writer = WalWriter::open(&path).unwrap();
        writer.append(&make_record(1, "/a", b"12345")).unwrap();
        writer.fsync().unwrap();
        let written = writer.bytes_written();
        drop(writer);
        let reader = WalReader::open(&path).unwrap();
        assert_eq!(reader.total_bytes(), written as usize);
    }

    #[test]
    fn test_wal_reader_file_not_found() {
        let result = WalReader::open("/tmp/does_not_exist_wal_12345");
        assert!(result.is_ok());
        let mut reader = result.unwrap();
        assert_eq!(reader.total_bytes(), 0);
        assert!(reader.read_record().is_none());
    }
}
