use volt_usb_manager::core::state::VumState;
use volt_usb_manager::safety::invariants::InvariantChecker;

#[test]
fn test_no_dirty_writes_after_clean_shutdown() {
    assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::Shutdown).is_ok());
    assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::Unmounted).is_ok());
    assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::JournalDirty).is_err());
    assert!(InvariantChecker::check_no_dirty_writes_after_clean_shutdown(&VumState::CorruptionDetected).is_err());
}

#[test]
fn test_no_ack_durable_before_commit() {
    assert!(InvariantChecker::check_no_ack_durable_before_commit().is_ok());
}

#[test]
fn test_no_mount_if_unrecoverable_journal() {
    assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::RecoveryRequired).is_err());
    assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::CorruptionDetected).is_err());
    assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::FatalError).is_err());
    assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::Running).is_ok());
    assert!(InvariantChecker::check_no_mount_if_unrecoverable(&VumState::ConfigLoaded).is_ok());
}

#[test]
fn test_cache_is_not_sole_source_of_user_data() {
    use volt_usb_manager::cache::cache_key::CacheKey;
    use volt_usb_manager::cache::cache_entry::CacheEntry;
    use volt_usb_manager::cache::read_cache::ReadCache;
    let mut cache = ReadCache::new(10);
    let key = CacheKey::new("/data/file", 100, 100);
    let entry = CacheEntry::new(key.clone(), "/data/file", vec![1, 2, 3]);
    cache.insert(key, entry).unwrap();
    cache.evict_all().unwrap();
    assert_eq!(cache.entry_count(), 0);
}

#[test]
fn test_no_path_escapes_backing_store() {
    let dir = tempfile::tempdir().unwrap();
    let sub = dir.path().join("valid_subdir");
    std::fs::create_dir(&sub).unwrap();
    assert!(InvariantChecker::check_no_path_escape(
        sub.to_string_lossy().as_ref(),
        dir.path().to_string_lossy().as_ref(),
    ).is_ok());
    let traversal = dir.path().join("..").join("etc");
    assert!(InvariantChecker::check_no_path_escape(
        traversal.to_string_lossy().as_ref(),
        dir.path().to_string_lossy().as_ref(),
    ).is_err());
}

#[test]
fn test_state_transitions_are_valid() {
    assert!(VumState::Uninitialized.can_transition_to(&VumState::ConfigLoaded));
    assert!(VumState::ConfigLoaded.can_transition_to(&VumState::DeviceDetected));
    assert!(VumState::DeviceDetected.can_transition_to(&VumState::BackingMounted));
    assert!(VumState::Running.can_transition_to(&VumState::Flushing));
    assert!(VumState::Ejecting.can_transition_to(&VumState::Unmounted));
    assert!(VumState::Unmounted.can_transition_to(&VumState::Shutdown));
    assert!(VumState::Running.can_transition_to(&VumState::FatalError));
}

#[test]
fn test_state_transitions_are_invalid() {
    assert!(!VumState::Uninitialized.can_transition_to(&VumState::Running));
    assert!(!VumState::ConfigLoaded.can_transition_to(&VumState::FuseMounted));
    assert!(!VumState::Shutdown.can_transition_to(&VumState::Running));
    assert!(!VumState::Running.can_transition_to(&VumState::Uninitialized));
}

#[test]
fn test_no_dirty_journal_after_checkpoint() {
    use volt_usb_manager::journal::journal::{Journal, JournalConfig};
    let dir = tempfile::tempdir().unwrap();
    let config = JournalConfig {
        path: dir.path().to_str().unwrap().to_string(),
        fsync_on_record: false,
        checkpoint_interval_ms: 5000,
    };
    let journal = Journal::open(config).unwrap();
    let id = journal.begin_write("/invariant", vec![1]).unwrap();
    journal.commit_write(id).unwrap();
    assert!(journal.is_dirty());
    journal.checkpoint().unwrap();
    assert!(!journal.is_dirty());
}

#[test]
fn test_manager_respects_state_invariants() {
    use volt_usb_manager::core::manager::VoltUsbManager;
    use volt_usb_manager::config::schema::VumConfig;
    let config = VumConfig::default();
    let mut mgr = VoltUsbManager::new(config);
    assert_eq!(mgr.state(), VumState::Uninitialized);
    mgr.init().unwrap();
    assert_eq!(mgr.state(), VumState::ConfigLoaded);
    mgr.shutdown().unwrap();
    assert_eq!(mgr.state(), VumState::Shutdown);
}
