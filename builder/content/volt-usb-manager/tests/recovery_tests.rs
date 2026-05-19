use std::path::Path;

use volt_usb_manager::journal::record::JournalRecord;
use volt_usb_manager::journal::record_type::RecordType;
use volt_usb_manager::journal::recovery::RecoveryEngine;
use volt_usb_manager::journal::wal::WalWriter;

fn write_wal(path: &str, records: &[JournalRecord]) {
    let writer = WalWriter::open(path).unwrap();
    for r in records {
        writer.append(r).unwrap();
    }
    writer.fsync().unwrap();
}

#[test]
fn test_dirty_journal_replay() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("dirty.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    std::fs::create_dir_all(&backing).unwrap();
    let rec1 = JournalRecord::new(RecordType::BeginWrite, 1, "file.txt", b"hello".to_vec());
    let rec2 = JournalRecord::new(RecordType::CommitWrite, 1, "", vec![]);
    write_wal(&wal, &[rec1, rec2]);
    let replay = RecoveryEngine::run(&wal, &backing).unwrap();
    assert_eq!(replay.writes_applied, 1);
    assert_eq!(replay.records_replayed, 2);
    let dest = Path::new(&backing).join("file.txt");
    assert!(dest.exists());
    assert_eq!(std::fs::read_to_string(dest).unwrap(), "hello");
}

#[test]
fn test_incomplete_trailing_record_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("partial.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    std::fs::create_dir_all(&backing).unwrap();
    let rec1 = JournalRecord::new(RecordType::BeginWrite, 1, "good.txt", b"good".to_vec());
    let rec2 = JournalRecord::new(RecordType::CommitWrite, 1, "", vec![]);
    write_wal(&wal, &[rec1, rec2]);
    let mut partial = std::fs::read(&wal).unwrap();
    partial.extend_from_slice(&[0xFF, 0xFE, 0xFD]);
    std::fs::write(&wal, &partial).unwrap();
    let replay = RecoveryEngine::run(&wal, &backing).unwrap();
    assert_eq!(replay.writes_applied, 1);
    assert_eq!(replay.records_replayed, 2);
}

#[test]
fn test_committed_write_survives_recovery() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("survive.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    std::fs::create_dir_all(&backing).unwrap();
    let content = b"durable content";
    let rec1 = JournalRecord::new(RecordType::BeginWrite, 1, "survive.txt", content.to_vec());
    let rec2 = JournalRecord::new(RecordType::CommitWrite, 1, "", vec![]);
    write_wal(&wal, &[rec1, rec2]);
    let replay = RecoveryEngine::run(&wal, &backing).unwrap();
    assert_eq!(replay.writes_applied, 1);
    let dest = Path::new(&backing).join("survive.txt");
    assert_eq!(std::fs::read(dest).unwrap(), content);
}

#[test]
fn test_uncommitted_write_not_applied() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("uncommitted.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    std::fs::create_dir_all(&backing).unwrap();
    let rec = JournalRecord::new(RecordType::BeginWrite, 1, "secret.txt", b"not committed".to_vec());
    write_wal(&wal, &[rec]);
    let replay = RecoveryEngine::run(&wal, &backing).unwrap();
    assert_eq!(replay.writes_applied, 0);
    assert!(!Path::new(&backing).join("secret.txt").exists());
}

#[test]
fn test_recovery_skips_corrupt_record() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("bad.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    std::fs::create_dir_all(&backing).unwrap();
    let mut rec = JournalRecord::new(RecordType::BeginWrite, 1, "bad.txt", b"bad".to_vec());
    rec.checksum ^= 1;
    let mut bytes = rec.to_bytes();
    bytes[rec.size() - 4..].copy_from_slice(&rec.checksum.to_le_bytes());
    std::fs::write(&wal, &bytes).unwrap();
    let result = RecoveryEngine::run(&wal, &backing);
    assert!(result.is_ok());
    let replay = result.unwrap();
    assert_eq!(replay.records_replayed, 0);
}

#[test]
fn test_clean_journal_no_recovery_needed() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("clean.wal").to_str().unwrap().to_string();
    let rec = JournalRecord::new(RecordType::CleanShutdown, 0, "", vec![]);
    write_wal(&wal, &[rec]);
    let needed = RecoveryEngine::check_needed(&wal).unwrap();
    assert!(!needed);
}

#[test]
fn test_empty_journal_no_replay() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("empty.wal").to_str().unwrap().to_string();
    let backing = dir.path().join("backing").to_str().unwrap().to_string();
    let _writer = WalWriter::open(&wal).unwrap();
    let replay = RecoveryEngine::run(&wal, &backing).unwrap();
    assert_eq!(replay.records_replayed, 0);
    assert_eq!(replay.writes_applied, 0);
}

#[test]
fn test_needs_recovery_before_mount() {
    let dir = tempfile::tempdir().unwrap();
    let wal = dir.path().join("needs_recovery.wal").to_str().unwrap().to_string();
    let rec = JournalRecord::new(RecordType::BeginWrite, 1, "/a", vec![1]);
    write_wal(&wal, &[rec]);
    let needs = RecoveryEngine::needs_recovery_before_mount(&wal).unwrap();
    assert!(needs);
}
