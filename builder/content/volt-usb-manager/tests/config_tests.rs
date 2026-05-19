use volt_usb_manager::config::loader::load_config;
use volt_usb_manager::config::schema::VumConfig;
use std::io::Write;

#[test]
fn test_config_example_parses_correctly() {
    let toml_str = r#"
[manager]
mount_point = "/mnt/volt_usb"
backing_path = "/var/volt/usb_backing"
runtime_dir = "/var/run/volt"
state_dir = "/var/lib/volt"
log_level = "info"
safe_mode = false

[fuse]
enabled = true
allow_other = false
read_only = false
direct_io = false
kernel_cache = true
auto_unmount = true

[device]
require_removable = true
detect_interval_ms = 2000
read_only_fallback = true
handle_surprise_removal = true

[cache]
read_cache_max_mb = 256
evict_at_percent = 90
compression = true
zstd_level = 3
pin_boot_assets = true
pin_desktop_assets = true
lru_enabled = true

[writeback]
enabled = true
buffer_max_mb = 64
flush_interval_ms = 5000
flush_at_percent = 80
batch_size_kb = 512
durability_mode = "balanced"
ack_mode = "lazy"
metadata_fsync = true

[journal]
enabled = true
path = "/var/volt/journal"
checksum = true
fsync_on_record = true
checkpoint_interval_ms = 30000
replay_on_boot = true
reject_corrupt_records = true

[scheduler]
nand_block_kb = 128
max_concurrent_flushes = 4
prioritize_desktop = true
prioritize_metadata = true
background_throttle = 50
fairness_window_ms = 100

[prefetch]
enabled = true
boot_assets = ["/boot/"]
desktop_assets = ["/usr/share/"]
max_prefetch_mb = 128

[compression]
read_cache_algorithm = "zstd"
write_cache_algorithm = "lz4"
zstd_level = 3
lz4_for_small_files = true
small_file_threshold_kb = 64

[eject]
require_clean_journal = true
force_flush = true
timeout_ms = 10000
fail_if_dirty = false

[ram]
use_volt_ram_manager = true
quota_mb = 512
pressure_backoff = true

[metrics]
enabled = true
latency_histograms = true
throughput_counters = true
"#;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(tmp, "{}", toml_str).unwrap();
    let config = load_config(tmp.path().to_str().unwrap()).unwrap();
    assert_eq!(config.manager.mount_point, "/mnt/volt_usb");
    assert_eq!(config.manager.backing_path, "/var/volt/usb_backing");
    assert_eq!(config.cache.read_cache_max_mb, 256);
    assert_eq!(config.writeback.buffer_max_mb, 64);
    assert_eq!(config.journal.path, "/var/volt/journal");
    assert_eq!(config.scheduler.nand_block_kb, 128);
    assert_eq!(config.compression.read_cache_algorithm, "zstd");
    assert_eq!(config.eject.timeout_ms, 10000);
    assert_eq!(config.ram.quota_mb, 512);
    assert!(config.metrics.enabled);
}

#[test]
fn test_invalid_config_rejected() {
    let toml_str = r#"
[manager]
mount_point = ""
backing_path = ""
"#;
    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(tmp, "{}", toml_str).unwrap();
    let result = load_config(tmp.path().to_str().unwrap());
    assert!(result.is_err());
}

#[test]
fn test_defaults_are_correct() {
    let config = VumConfig::default();
    assert_eq!(config.manager.mount_point, "/mnt/volt_usb");
    assert_eq!(config.manager.backing_path, "/var/volt/usb_backing");
    assert_eq!(config.manager.log_level, "info");
    assert!(!config.manager.safe_mode);
    assert!(config.fuse.enabled);
    assert!(!config.fuse.allow_other);
    assert!(config.device.require_removable);
    assert_eq!(config.device.detect_interval_ms, 2000);
    assert_eq!(config.cache.read_cache_max_mb, 256);
    assert_eq!(config.cache.evict_at_percent, 90);
    assert!(config.cache.compression);
    assert!(config.writeback.enabled);
    assert_eq!(config.writeback.buffer_max_mb, 64);
    assert_eq!(config.writeback.flush_interval_ms, 5000);
    assert_eq!(config.writeback.durability_mode, "balanced");
    assert!(config.journal.enabled);
    assert_eq!(config.journal.path, "/var/volt/journal");
    assert!(config.journal.checksum);
    assert_eq!(config.scheduler.nand_block_kb, 128);
    assert_eq!(config.scheduler.max_concurrent_flushes, 4);
    assert!(config.prefetch.enabled);
    assert_eq!(config.prefetch.max_prefetch_mb, 128);
    assert_eq!(config.compression.read_cache_algorithm, "zstd");
    assert!(config.compression.lz4_for_small_files);
    assert!(config.eject.require_clean_journal);
    assert_eq!(config.eject.timeout_ms, 10000);
    assert!(config.ram.use_volt_ram_manager);
    assert_eq!(config.ram.quota_mb, 512);
    assert!(config.metrics.enabled);
}

#[test]
fn test_validate_rejects_empty_mount_point() {
    let mut config = VumConfig::default();
    config.manager.mount_point = String::new();
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_empty_backing_path() {
    let mut config = VumConfig::default();
    config.manager.backing_path = String::new();
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_zero_cache_size() {
    let mut config = VumConfig::default();
    config.cache.read_cache_max_mb = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_invalid_evict_percent() {
    let mut config = VumConfig::default();
    config.cache.evict_at_percent = 150;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_zero_buffer_with_writeback_enabled() {
    let mut config = VumConfig::default();
    config.writeback.buffer_max_mb = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_zero_eject_timeout() {
    let mut config = VumConfig::default();
    config.eject.timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_invalid_background_throttle() {
    let mut config = VumConfig::default();
    config.scheduler.background_throttle = 101;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_invalid_flush_percent() {
    let mut config = VumConfig::default();
    config.writeback.flush_at_percent = 101;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_prefetch_enabled_with_zero_max() {
    let mut config = VumConfig::default();
    config.prefetch.enabled = true;
    config.prefetch.max_prefetch_mb = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_validate_rejects_invalid_zstd_level() {
    let mut config = VumConfig::default();
    config.compression.zstd_level = 0;
    assert!(config.validate().is_err());
    config.compression.zstd_level = 23;
    assert!(config.validate().is_err());
}
