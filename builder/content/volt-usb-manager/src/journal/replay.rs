use std::collections::{HashMap, HashSet};

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::journal::record::JournalRecord;
use crate::journal::record_type::RecordType;

pub struct JournalReplay {
    pub records_replayed: u64,
    pub writes_applied: u64,
    pub writes_rolled_back: u64,
    pub last_record_id: u64,
}

impl JournalReplay {
    pub fn new() -> Self {
        Self {
            records_replayed: 0,
            writes_applied: 0,
            writes_rolled_back: 0,
            last_record_id: 0,
        }
    }

    pub fn replay(&mut self, records: &[JournalRecord], backing_path: &str) -> VumResult<()> {
        self.records_replayed = records.len() as u64;

        let incomplete_ids: HashSet<u64> = self
            .find_incomplete(records)
            .iter()
            .map(|r| r.record_id)
            .collect();

        let begins: HashMap<u64, &JournalRecord> = records
            .iter()
            .filter(|r| r.record_type.is_begin())
            .map(|r| (r.record_id, r))
            .collect();

        for record in records {
            if !record.verify_checksum() {
                return Err(VumError::JournalChecksumFailed);
            }
            if record.record_id > self.last_record_id {
                self.last_record_id = record.record_id;
            }
            match record.record_type {
                RecordType::CommitWrite | RecordType::CommitRename => {
                    if incomplete_ids.contains(&record.record_id) {
                        continue;
                    }
                    if let Some(begin) = begins.get(&record.record_id) {
                        let rel_path = begin.path.trim_start_matches('/');
                        let dest = std::path::Path::new(backing_path).join(rel_path);
                        if let Some(parent) = dest.parent() {
                            std::fs::create_dir_all(parent).map_err(|e| {
                                VumError::WriteFailed(format!("Replay mkdir: {}", e))
                            })?;
                        }
                        std::fs::write(&dest, &begin.data).map_err(|e| {
                            VumError::WriteFailed(format!("Replay write: {}", e))
                        })?;
                        self.writes_applied += 1;
                    }
                }
                RecordType::CommitDelete => {
                    if incomplete_ids.contains(&record.record_id) {
                        continue;
                    }
                    if let Some(begin) = begins.get(&record.record_id) {
                        let rel_path = begin.path.trim_start_matches('/');
                        let dest = std::path::Path::new(backing_path).join(rel_path);
                        let _ = std::fs::remove_file(&dest);
                        self.writes_applied += 1;
                    }
                }
                RecordType::AbortWrite => {
                    if let Some(begin) = begins.get(&record.record_id) {
                        let rel_path = begin.path.trim_start_matches('/');
                        let dest = std::path::Path::new(backing_path).join(rel_path);
                        let _ = std::fs::remove_file(&dest);
                        self.writes_rolled_back += 1;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn find_incomplete<'a>(&self, records: &'a [JournalRecord]) -> Vec<&'a JournalRecord> {
        let mut completed: HashMap<u64, bool> = HashMap::new();
        for record in records {
            if record.record_type.is_commit() || record.record_type == RecordType::AbortWrite {
                completed.insert(record.record_id, true);
            }
        }
        records
            .iter()
            .filter(|r| r.record_type.is_begin())
            .filter(|r| !completed.contains_key(&r.record_id))
            .collect()
    }
}

impl Default for JournalReplay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::record::JournalRecord;
    use crate::journal::record_type::RecordType;
    use tempfile::tempdir;

    fn make_record(ty: RecordType, id: u64, path: &str, data: &[u8]) -> JournalRecord {
        JournalRecord::new(ty, id, path, data.to_vec())
    }

    #[test]
    fn test_new_replay_counts() {
        let r = JournalReplay::new();
        assert_eq!(r.records_replayed, 0);
        assert_eq!(r.writes_applied, 0);
        assert_eq!(r.writes_rolled_back, 0);
        assert_eq!(r.last_record_id, 0);
    }

    #[test]
    fn test_find_incomplete_no_commit() {
        let records = vec![
            make_record(RecordType::BeginWrite, 1, "/a", b"data"),
            make_record(RecordType::BeginWrite, 2, "/b", b"data2"),
            make_record(RecordType::CommitWrite, 1, "", b""),
        ];
        let replay = JournalReplay::new();
        let incomplete = replay.find_incomplete(&records);
        assert_eq!(incomplete.len(), 1);
        assert_eq!(incomplete[0].record_id, 2);
    }

    #[test]
    fn test_find_incomplete_all_complete() {
        let records = vec![
            make_record(RecordType::BeginWrite, 1, "/a", b"data"),
            make_record(RecordType::CommitWrite, 1, "", b""),
            make_record(RecordType::BeginDelete, 2, "/b", b""),
            make_record(RecordType::CommitDelete, 2, "", b""),
        ];
        let replay = JournalReplay::new();
        let incomplete = replay.find_incomplete(&records);
        assert_eq!(incomplete.len(), 0);
    }

    #[test]
    fn test_find_incomplete_with_abort() {
        let records = vec![
            make_record(RecordType::BeginWrite, 1, "/a", b"data"),
            make_record(RecordType::AbortWrite, 1, "", b""),
        ];
        let replay = JournalReplay::new();
        let incomplete = replay.find_incomplete(&records);
        assert_eq!(incomplete.len(), 0);
    }

    #[test]
    fn test_replay_applies_committed_write() {
        let dir = tempdir().unwrap();
        let backing = dir.path().join("backing");
        std::fs::create_dir_all(&backing).unwrap();
        let records = vec![
            make_record(RecordType::BeginWrite, 1, "test.txt", b"hello world"),
            make_record(RecordType::CommitWrite, 1, "", b""),
        ];
        let mut replay = JournalReplay::new();
        replay
            .replay(&records, backing.to_str().unwrap())
            .unwrap();
        let dest = backing.join("test.txt");
        assert!(dest.exists());
        assert_eq!(std::fs::read_to_string(dest).unwrap(), "hello world");
        assert_eq!(replay.writes_applied, 1);
        assert_eq!(replay.records_replayed, 2);
    }

    #[test]
    fn test_replay_skips_incomplete_write() {
        let dir = tempdir().unwrap();
        let backing = dir.path().join("backing");
        std::fs::create_dir_all(&backing).unwrap();
        let records = vec![make_record(
            RecordType::BeginWrite,
            1,
            "secret.txt",
            b"not applied",
        )];
        let mut replay = JournalReplay::new();
        replay
            .replay(&records, backing.to_str().unwrap())
            .unwrap();
        let dest = backing.join("secret.txt");
        assert!(!dest.exists());
        assert_eq!(replay.writes_applied, 0);
    }

    #[test]
    fn test_replay_rolls_back_aborted_write() {
        let dir = tempdir().unwrap();
        let backing = dir.path().join("backing");
        std::fs::create_dir_all(&backing).unwrap();
        std::fs::write(backing.join("aborted.txt"), b"old data").unwrap();

        let records = vec![
            make_record(RecordType::BeginWrite, 1, "aborted.txt", b"new data"),
            make_record(RecordType::AbortWrite, 1, "", b""),
        ];
        let mut replay = JournalReplay::new();
        replay
            .replay(&records, backing.to_str().unwrap())
            .unwrap();
        assert_eq!(replay.writes_rolled_back, 1);
    }

    #[test]
    fn test_replay_tracks_last_record_id() {
        let records = vec![
            make_record(RecordType::BeginWrite, 5, "a", b"x"),
            make_record(RecordType::CommitWrite, 5, "", b""),
            make_record(RecordType::BeginWrite, 10, "b", b"y"),
            make_record(RecordType::CommitWrite, 10, "", b""),
        ];
        let dir = tempdir().unwrap();
        let backing = dir.path().join("b");
        std::fs::create_dir_all(&backing).unwrap();
        let mut replay = JournalReplay::new();
        replay
            .replay(&records, backing.to_str().unwrap())
            .unwrap();
        assert_eq!(replay.last_record_id, 10);
    }

    #[test]
    fn test_replay_checksum_failure() {
        let mut records = vec![
            make_record(RecordType::BeginWrite, 1, "a", b"data"),
            make_record(RecordType::CommitWrite, 1, "", b""),
        ];
        records[0].checksum ^= 1;
        let dir = tempdir().unwrap();
        let backing = dir.path().join("b");
        std::fs::create_dir_all(&backing).unwrap();
        let mut replay = JournalReplay::new();
        let result = replay.replay(&records, backing.to_str().unwrap());
        assert!(result.is_err());
    }
}
