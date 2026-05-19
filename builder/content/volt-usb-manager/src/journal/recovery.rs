use std::path::Path;

use crate::core::result::VumResult;
use crate::journal::replay::JournalReplay;
use crate::journal::wal::WalReader;

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn check_needed(path: &str) -> VumResult<bool> {
        let wal_path = Path::new(path);
        if !wal_path.exists() {
            return Ok(false);
        }
        let meta = std::fs::metadata(wal_path)?;
        if meta.len() == 0 {
            return Ok(false);
        }
        let mut reader = WalReader::open(path)?;
        let mut count = 0u64;
        while reader.read_record().is_some() {
            count += 1;
        }
        if count == 0 {
            return Ok(false);
        }
        let mut last_is_clean = false;
        let mut reader2 = WalReader::open(path)?;
        let mut rec;
        loop {
            rec = reader2.read_record();
            match rec {
                Some(ref r) => {
                    if r.record_type == crate::journal::record_type::RecordType::CleanShutdown {
                        last_is_clean = true;
                    } else {
                        last_is_clean = false;
                    }
                }
                None => break,
            }
        }
        Ok(!last_is_clean)
    }

    pub fn run(journal_path: &str, backing_path: &str) -> VumResult<JournalReplay> {
        let mut reader = WalReader::open(journal_path)?;
        let mut records = Vec::new();
        while let Some(record) = reader.read_record() {
            records.push(record);
        }
        if records.is_empty() {
            return Ok(JournalReplay::new());
        }
        let mut replay = JournalReplay::new();
        replay.replay(&records, backing_path)?;
        Ok(replay)
    }

    pub fn needs_recovery_before_mount(journal_path: &str) -> VumResult<bool> {
        Self::check_needed(journal_path)
    }
}

impl Default for RecoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::record::JournalRecord;
    use crate::journal::record_type::RecordType;
    use crate::journal::wal::WalWriter;
    use tempfile::tempdir;

    fn write_wal(path: &str, records: &[JournalRecord]) {
        let writer = WalWriter::open(path).unwrap();
        for r in records {
            writer.append(r).unwrap();
        }
        writer.fsync().unwrap();
    }

    #[test]
    fn test_check_needed_no_file() {
        let result = RecoveryEngine::check_needed("/tmp/nonexistent_wal_XXXX").unwrap();
        assert!(!result);
    }

    #[test]
    fn test_check_needed_empty_wal() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("empty.wal").to_str().unwrap().to_string();
        let _writer = WalWriter::open(&path).unwrap();
        let result = RecoveryEngine::check_needed(&path).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_check_needed_with_records() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("data.wal").to_str().unwrap().to_string();
        let rec = JournalRecord::new(RecordType::BeginWrite, 1, "/a", vec![1]);
        write_wal(&path, &[rec]);
        let result = RecoveryEngine::check_needed(&path).unwrap();
        assert!(result);
    }

    #[test]
    fn test_run_empty_wal() {
        let dir = tempdir().unwrap();
        let wal = dir.path().join("empty.wal").to_str().unwrap().to_string();
        let backing = dir.path().join("backing").to_str().unwrap().to_string();
        let replay = RecoveryEngine::run(&wal, &backing).unwrap();
        assert_eq!(replay.records_replayed, 0);
    }

    #[test]
    fn test_run_applies_committed() {
        let dir = tempdir().unwrap();
        let wal = dir.path().join("run.wal").to_str().unwrap().to_string();
        let backing = dir.path().join("backing").to_str().unwrap().to_string();
        std::fs::create_dir_all(&backing).unwrap();
        let rec1 = JournalRecord::new(RecordType::BeginWrite, 1, "out.txt", b"hello".to_vec());
        let rec2 = JournalRecord::new(RecordType::CommitWrite, 1, "", vec![]);
        write_wal(&wal, &[rec1, rec2]);
        let replay = RecoveryEngine::run(&wal, &backing).unwrap();
        assert_eq!(replay.writes_applied, 1);
        let dest = Path::new(&backing).join("out.txt");
        assert!(dest.exists());
    }

    #[test]
    fn test_needs_recovery_before_mount() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("mount.wal").to_str().unwrap().to_string();
        let rec = JournalRecord::new(RecordType::BeginWrite, 1, "/a", vec![1]);
        write_wal(&path, &[rec]);
        let result = RecoveryEngine::needs_recovery_before_mount(&path).unwrap();
        assert!(result);
    }

    #[test]
    fn test_clean_shutdown_no_recovery_needed() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("clean.wal").to_str().unwrap().to_string();
        let rec = JournalRecord::new(RecordType::CleanShutdown, 0, "", vec![]);
        write_wal(&path, &[rec]);
        let result = RecoveryEngine::check_needed(&path).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_engine_new() {
        let engine = RecoveryEngine::new();
        let _ = engine;
    }
}
