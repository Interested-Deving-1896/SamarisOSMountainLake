use volt_usb_manager::journal::checkpoint::Checkpoint;
use volt_usb_manager::journal::journal::{Journal, JournalConfig};
use volt_usb_manager::journal::record::JournalRecord;
use volt_usb_manager::journal::record_type::RecordType;
use volt_usb_manager::journal::wal::WalReader;

fn make_journal_config(dir: &tempfile::TempDir) -> JournalConfig {
    JournalConfig {
        path: dir.path().to_str().unwrap().to_string(),
        fsync_on_record: false,
        checkpoint_interval_ms: 5000,
    }
}

#[test]
fn test_append_record() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    let id = journal.begin_write("/test", vec![1, 2, 3]).unwrap();
    assert_eq!(id, 1);
    assert!(journal.is_dirty());
    assert!(journal.record_count() >= 1);
}

#[test]
fn test_checksum_validation() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("wal.dat");
    let wal_str = wal_path.to_str().unwrap();
    let record = JournalRecord::new(RecordType::BeginWrite, 1, "/test", vec![0x42]);
    let bytes = record.to_bytes();
    std::fs::write(wal_str, &bytes).unwrap();
    let mut reader = WalReader::open(wal_str).unwrap();
    let result = reader.read_record();
    assert!(result.is_some());
    assert!(result.unwrap().verify_checksum());
}

#[test]
fn test_corrupt_record_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let wal_path = dir.path().join("corrupt.wal");
    let wal_str = wal_path.to_str().unwrap();
    let record = JournalRecord::new(RecordType::BeginWrite, 1, "/test", vec![0x42]);
    let mut bytes = record.to_bytes();
    bytes[record.size() - 5] ^= 0xFF;
    std::fs::write(wal_str, &bytes).unwrap();
    let mut reader = WalReader::open(wal_str).unwrap();
    assert!(reader.read_record().is_none());
}

#[test]
fn test_begin_commit_lifecycle() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    let id = journal.begin_write("/lifecycle", vec![99]).unwrap();
    assert!(journal.is_dirty());
    journal.commit_write(id).unwrap();
    assert!(journal.record_count() >= 2);
}

#[test]
fn test_checkpoint_creation() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config.clone()).unwrap();
    let id = journal.begin_write("/cp_test", vec![1]).unwrap();
    journal.commit_write(id).unwrap();
    journal.checkpoint().unwrap();
    assert!(!journal.is_dirty());
    let cp = Checkpoint::read(&config.path).unwrap();
    assert!(cp.last_record_id >= 1);
    assert!(cp.timestamp > 0);
}

#[test]
fn test_clean_shutdown_marker() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    journal.clean_shutdown().unwrap();
    assert!(!journal.is_dirty());
    let wal_path = dir.path().join("wal.dat");
    let data = std::fs::read(wal_path).unwrap();
    assert!(!data.is_empty());
}

#[test]
fn test_abort_write() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    let id = journal.begin_write("/abort", vec![0]).unwrap();
    journal.abort_write(id).unwrap();
    assert_eq!(journal.record_count(), 2);
}

#[test]
fn test_checkpoint_clears_dirty_flag() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    journal.checkpoint().unwrap();
    assert!(!journal.is_dirty());
}

#[test]
fn test_multiple_writes_increment_count() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config).unwrap();
    for i in 0..10 {
        let id = journal.begin_write(&format!("/f{}", i), vec![i]).unwrap();
        journal.commit_write(id).unwrap();
    }
    assert_eq!(journal.record_count(), 20);
    assert!(journal.bytes_written() > 0);
}

#[test]
fn test_reopen_journal_preserves_dirty_state() {
    let dir = tempfile::tempdir().unwrap();
    let config = make_journal_config(&dir);
    let journal = Journal::open(config.clone()).unwrap();
    let id = journal.begin_write("/reopen", vec![1]).unwrap();
    journal.commit_write(id).unwrap();
    drop(journal);
    let journal2 = Journal::open(config).unwrap();
    assert!(journal2.is_dirty());
}
