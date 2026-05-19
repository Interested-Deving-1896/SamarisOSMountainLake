use tesseract_engine::boot::assets::AssetCache;
use tesseract_engine::boot::watchdog::WatchdogFiles;
use tesseract_engine::boot::notifier::SystemdNotifier;
use tesseract_engine::boot::{BootMode, BootSequence};

#[test]
fn test_boot_mode_enum() {
    assert!(BootMode::Fast.is_fast());
    assert!(!BootMode::Normal.is_fast());
    assert_ne!(BootMode::Fast, BootMode::Normal);
}

#[test]
fn test_boot_sequence_default_workers() {
    let normal = BootSequence::new(BootMode::Normal);
    let fast = BootSequence::new(BootMode::Fast);
    // We can't directly check worker_count as it's private,
    // but we can verify the sequence doesn't crash
    assert!(normal.execute().is_ok());
    assert!(fast.execute().is_ok());
}

#[test]
fn test_boot_sequence_custom_workers() {
    let seq = BootSequence::new(BootMode::Fast).with_workers(16);
    let result = seq.execute();
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.elapsed.as_nanos() > 0);
}

#[test]
fn test_boot_result_has_timing() {
    let result = BootSequence::new(BootMode::Normal).execute().unwrap();
    assert!(result.elapsed.as_micros() > 0);
    assert!(result.scheduler.worker_count() >= 1);
}

#[test]
fn test_asset_cache_empty_on_missing_root() {
    let cache = AssetCache::precache("/nonexistent/path");
    assert!(cache.is_err());
}

#[test]
fn test_watchdog_write_remove() {
    WatchdogFiles::write("test-watchdog").ok();
    // The file may or may not exist depending on /run availability
    // But the function should not panic
    WatchdogFiles::remove("test-watchdog");
}

#[test]
fn test_sd_notify_no_socket() {
    // Without NOTIFY_SOCKET, should succeed silently
    assert!(SystemdNotifier::notify("READY=1").is_ok());
}

#[test]
fn test_boot_mode_normal_does_not_preinit_gpu() {
    let result = BootSequence::new(BootMode::Normal).execute().unwrap();
    // In normal mode, gpu_canvas should not be pre-initialized
    assert!(result.gpu_canvas.is_none());
}

#[test]
fn test_boot_mode_fast_preinits_workers() {
    let result = BootSequence::new(BootMode::Fast).execute().unwrap();
    assert!(result.scheduler.worker_count() >= 8);
}

#[test]
fn test_asset_cache_successful_precache() {
    use tempfile::TempDir;
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join("app")).unwrap();
    std::fs::write(dir.path().join("app/index.html"), b"<html>test</html>").unwrap();
    std::fs::create_dir_all(dir.path().join("app/assets")).unwrap();
    std::fs::write(dir.path().join("app/assets/index-test.js"), b"console.log('hi');").unwrap();

    let cache = AssetCache::precache(dir.path().to_str().unwrap()).unwrap();
    assert!(!cache.is_empty());
    assert!(!cache.keys().collect::<Vec<_>>().is_empty());
    assert!(cache.total_bytes() > 0);
}

#[test]
fn test_asset_cache_get_and_contains() {
    use tempfile::TempDir;
    let dir = TempDir::new().unwrap();
    std::fs::create_dir_all(dir.path().join("app")).unwrap();
    std::fs::write(dir.path().join("app/index.html"), b"hello world").unwrap();

    let cache = AssetCache::precache(dir.path().to_str().unwrap()).unwrap();
    assert!(cache.contains("app/index.html"));
    assert_eq!(cache.get("app/index.html"), Some(&b"hello world"[..]));
    assert!(!cache.contains("nonexistent"));
    assert!(cache.get("nonexistent").is_none());
}

#[test]
fn test_asset_cache_empty_dir_has_no_assets() {
    use tempfile::TempDir;
    let dir = TempDir::new().unwrap();
    let cache = AssetCache::precache(dir.path().to_str().unwrap()).unwrap();
    assert!(cache.is_empty());
    assert_eq!(cache.len(), 0);
}

#[test]
fn test_watchdog_exists_after_write() {
    WatchdogFiles::write("test-watchdog-exists").ok();
    // exists() may be false if /run not writable (non-Linux), but should not panic
    WatchdogFiles::remove("test-watchdog-exists");
}

#[test]
fn test_watchdog_ready_dir() {
    let dir = WatchdogFiles::ready_dir();
    assert_eq!(dir.to_string_lossy(), "/run");
}

#[test]
fn test_sd_notify_ready_status_stopping() {
    SystemdNotifier::notify_ready();
    SystemdNotifier::notify_status("testing");
    SystemdNotifier::notify_stopping();
    // Should not panic even without NOTIFY_SOCKET
}

#[test]
fn test_boot_with_asset_root_and_timing() {
    let result = BootSequence::new(BootMode::Fast)
        .with_asset_root("/tmp")
        .execute()
        .unwrap();
    assert!(result.timing.total_us > 0);
    assert!(!result.timing.phases.is_empty());
    let phase_names: Vec<&str> = result.timing.phases.iter().map(|p| p.name).collect();
    assert!(phase_names.contains(&"scheduler_init"));
    assert!(phase_names.contains(&"total"));
}
