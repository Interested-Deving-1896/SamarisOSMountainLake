pub fn mount_point() -> String { "/mnt/volt_usb".to_string() }
pub fn backing_path() -> String { "/var/volt/usb_backing".to_string() }
pub fn runtime_dir() -> String { "/var/run/volt".to_string() }
pub fn state_dir() -> String { "/var/lib/volt".to_string() }
pub fn log_level() -> String { "info".to_string() }
pub fn safe_mode() -> bool { false }

pub fn fuse_enabled() -> bool { true }
pub fn fuse_allow_other() -> bool { false }
pub fn fuse_read_only() -> bool { false }
pub fn fuse_direct_io() -> bool { false }
pub fn fuse_kernel_cache() -> bool { true }
pub fn fuse_auto_unmount() -> bool { true }

pub fn require_removable() -> bool { true }
pub fn detect_interval_ms() -> u64 { 2000 }
pub fn read_only_fallback() -> bool { true }
pub fn handle_surprise_removal() -> bool { true }

pub fn read_cache_max_mb() -> u64 { 256 }
pub fn evict_at_percent() -> u8 { 90 }
pub fn cache_compression() -> bool { true }
pub fn zstd_level() -> i32 { 3 }
pub fn pin_boot_assets() -> bool { true }
pub fn pin_desktop_assets() -> bool { true }
pub fn lru_enabled() -> bool { true }

pub fn writeback_enabled() -> bool { true }
pub fn buffer_max_mb() -> u64 { 64 }
pub fn flush_interval_ms() -> u64 { 5000 }
pub fn flush_at_percent() -> u8 { 80 }
pub fn batch_size_kb() -> u64 { 512 }
pub fn durability_mode() -> String { "balanced".to_string() }
pub fn ack_mode() -> String { "lazy".to_string() }
pub fn metadata_fsync() -> bool { true }

pub fn journal_enabled() -> bool { true }
pub fn journal_path() -> String { "/var/volt/journal".to_string() }
pub fn journal_checksum() -> bool { true }
pub fn journal_fsync_on_record() -> bool { true }
pub fn journal_checkpoint_interval_ms() -> u64 { 30000 }
pub fn journal_replay_on_boot() -> bool { true }
pub fn journal_reject_corrupt_records() -> bool { true }

pub fn nand_block_kb() -> u64 { 128 }
pub fn max_concurrent_flushes() -> u32 { 4 }
pub fn prioritize_desktop() -> bool { true }
pub fn prioritize_metadata() -> bool { true }
pub fn background_throttle() -> u8 { 50 }
pub fn fairness_window_ms() -> u64 { 100 }

pub fn prefetch_enabled() -> bool { true }
pub fn prefetch_boot_assets() -> Vec<String> { vec!["/boot/".to_string()] }
pub fn prefetch_desktop_assets() -> Vec<String> { vec!["/usr/share/".to_string()] }
pub fn prefetch_max_prefetch_mb() -> u64 { 128 }

pub fn read_cache_algorithm() -> String { "zstd".to_string() }
pub fn write_cache_algorithm() -> String { "lz4".to_string() }
pub fn compression_zstd_level() -> i32 { 3 }
pub fn lz4_for_small_files() -> bool { true }
pub fn small_file_threshold_kb() -> u64 { 64 }

pub fn eject_require_clean_journal() -> bool { true }
pub fn eject_force_flush() -> bool { true }
pub fn eject_timeout_ms() -> u64 { 10000 }
pub fn eject_fail_if_dirty() -> bool { false }

pub fn ram_use_volt_ram_manager() -> bool { true }
pub fn ram_quota_mb() -> u64 { 512 }
pub fn ram_pressure_backoff() -> bool { true }

pub fn metrics_enabled() -> bool { true }
pub fn metrics_latency_histograms() -> bool { true }
pub fn metrics_throughput_counters() -> bool { true }
