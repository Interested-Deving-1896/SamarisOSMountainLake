use serde::{Deserialize, Serialize};

use crate::config::defaults;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VumConfig {
    #[serde(default)]
    pub manager: ManagerConfig,
    #[serde(default)]
    pub fuse: FuseConfig,
    #[serde(default)]
    pub device: DeviceConfig,
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub writeback: WritebackConfig,
    #[serde(default)]
    pub journal: JournalConfig,
    #[serde(default)]
    pub scheduler: SchedulerConfig,
    #[serde(default)]
    pub prefetch: PrefetchConfig,
    #[serde(default)]
    pub compression: CompressionConfig,
    #[serde(default)]
    pub eject: EjectConfig,
    #[serde(default)]
    pub ram: RamConfig,
    #[serde(default)]
    pub metrics: MetricsConfig,
}

impl Default for VumConfig {
    fn default() -> Self {
        VumConfig {
            manager: ManagerConfig::default(),
            fuse: FuseConfig::default(),
            device: DeviceConfig::default(),
            cache: CacheConfig::default(),
            writeback: WritebackConfig::default(),
            journal: JournalConfig::default(),
            scheduler: SchedulerConfig::default(),
            prefetch: PrefetchConfig::default(),
            compression: CompressionConfig::default(),
            eject: EjectConfig::default(),
            ram: RamConfig::default(),
            metrics: MetricsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerConfig {
    #[serde(default = "defaults::mount_point")]
    pub mount_point: String,
    #[serde(default = "defaults::backing_path")]
    pub backing_path: String,
    #[serde(default = "defaults::runtime_dir")]
    pub runtime_dir: String,
    #[serde(default = "defaults::state_dir")]
    pub state_dir: String,
    #[serde(default = "defaults::log_level")]
    pub log_level: String,
    #[serde(default = "defaults::safe_mode")]
    pub safe_mode: bool,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        ManagerConfig {
            mount_point: defaults::mount_point(),
            backing_path: defaults::backing_path(),
            runtime_dir: defaults::runtime_dir(),
            state_dir: defaults::state_dir(),
            log_level: defaults::log_level(),
            safe_mode: defaults::safe_mode(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseConfig {
    #[serde(default = "defaults::fuse_enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::fuse_allow_other")]
    pub allow_other: bool,
    #[serde(default = "defaults::fuse_read_only")]
    pub read_only: bool,
    #[serde(default = "defaults::fuse_direct_io")]
    pub direct_io: bool,
    #[serde(default = "defaults::fuse_kernel_cache")]
    pub kernel_cache: bool,
    #[serde(default = "defaults::fuse_auto_unmount")]
    pub auto_unmount: bool,
}

impl Default for FuseConfig {
    fn default() -> Self {
        FuseConfig {
            enabled: defaults::fuse_enabled(),
            allow_other: defaults::fuse_allow_other(),
            read_only: defaults::fuse_read_only(),
            direct_io: defaults::fuse_direct_io(),
            kernel_cache: defaults::fuse_kernel_cache(),
            auto_unmount: defaults::fuse_auto_unmount(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfig {
    #[serde(default = "defaults::require_removable")]
    pub require_removable: bool,
    #[serde(default = "defaults::detect_interval_ms")]
    pub detect_interval_ms: u64,
    #[serde(default = "defaults::read_only_fallback")]
    pub read_only_fallback: bool,
    #[serde(default = "defaults::handle_surprise_removal")]
    pub handle_surprise_removal: bool,
}

impl Default for DeviceConfig {
    fn default() -> Self {
        DeviceConfig {
            require_removable: defaults::require_removable(),
            detect_interval_ms: defaults::detect_interval_ms(),
            read_only_fallback: defaults::read_only_fallback(),
            handle_surprise_removal: defaults::handle_surprise_removal(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "defaults::read_cache_max_mb")]
    pub read_cache_max_mb: u64,
    #[serde(default = "defaults::evict_at_percent")]
    pub evict_at_percent: u8,
    #[serde(default = "defaults::cache_compression")]
    pub compression: bool,
    #[serde(default = "defaults::zstd_level")]
    pub zstd_level: i32,
    #[serde(default = "defaults::pin_boot_assets")]
    pub pin_boot_assets: bool,
    #[serde(default = "defaults::pin_desktop_assets")]
    pub pin_desktop_assets: bool,
    #[serde(default = "defaults::lru_enabled")]
    pub lru_enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            read_cache_max_mb: defaults::read_cache_max_mb(),
            evict_at_percent: defaults::evict_at_percent(),
            compression: defaults::cache_compression(),
            zstd_level: defaults::zstd_level(),
            pin_boot_assets: defaults::pin_boot_assets(),
            pin_desktop_assets: defaults::pin_desktop_assets(),
            lru_enabled: defaults::lru_enabled(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritebackConfig {
    #[serde(default = "defaults::writeback_enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::buffer_max_mb")]
    pub buffer_max_mb: u64,
    #[serde(default = "defaults::flush_interval_ms")]
    pub flush_interval_ms: u64,
    #[serde(default = "defaults::flush_at_percent")]
    pub flush_at_percent: u8,
    #[serde(default = "defaults::batch_size_kb")]
    pub batch_size_kb: u64,
    #[serde(default = "defaults::durability_mode")]
    pub durability_mode: String,
    #[serde(default = "defaults::ack_mode")]
    pub ack_mode: String,
    #[serde(default = "defaults::metadata_fsync")]
    pub metadata_fsync: bool,
}

impl Default for WritebackConfig {
    fn default() -> Self {
        WritebackConfig {
            enabled: defaults::writeback_enabled(),
            buffer_max_mb: defaults::buffer_max_mb(),
            flush_interval_ms: defaults::flush_interval_ms(),
            flush_at_percent: defaults::flush_at_percent(),
            batch_size_kb: defaults::batch_size_kb(),
            durability_mode: defaults::durability_mode(),
            ack_mode: defaults::ack_mode(),
            metadata_fsync: defaults::metadata_fsync(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalConfig {
    #[serde(default = "defaults::journal_enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::journal_path")]
    pub path: String,
    #[serde(default = "defaults::journal_checksum")]
    pub checksum: bool,
    #[serde(default = "defaults::journal_fsync_on_record")]
    pub fsync_on_record: bool,
    #[serde(default = "defaults::journal_checkpoint_interval_ms")]
    pub checkpoint_interval_ms: u64,
    #[serde(default = "defaults::journal_replay_on_boot")]
    pub replay_on_boot: bool,
    #[serde(default = "defaults::journal_reject_corrupt_records")]
    pub reject_corrupt_records: bool,
}

impl Default for JournalConfig {
    fn default() -> Self {
        JournalConfig {
            enabled: defaults::journal_enabled(),
            path: defaults::journal_path(),
            checksum: defaults::journal_checksum(),
            fsync_on_record: defaults::journal_fsync_on_record(),
            checkpoint_interval_ms: defaults::journal_checkpoint_interval_ms(),
            replay_on_boot: defaults::journal_replay_on_boot(),
            reject_corrupt_records: defaults::journal_reject_corrupt_records(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    #[serde(default = "defaults::nand_block_kb")]
    pub nand_block_kb: u64,
    #[serde(default = "defaults::max_concurrent_flushes")]
    pub max_concurrent_flushes: u32,
    #[serde(default = "defaults::prioritize_desktop")]
    pub prioritize_desktop: bool,
    #[serde(default = "defaults::prioritize_metadata")]
    pub prioritize_metadata: bool,
    #[serde(default = "defaults::background_throttle")]
    pub background_throttle: u8,
    #[serde(default = "defaults::fairness_window_ms")]
    pub fairness_window_ms: u64,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        SchedulerConfig {
            nand_block_kb: defaults::nand_block_kb(),
            max_concurrent_flushes: defaults::max_concurrent_flushes(),
            prioritize_desktop: defaults::prioritize_desktop(),
            prioritize_metadata: defaults::prioritize_metadata(),
            background_throttle: defaults::background_throttle(),
            fairness_window_ms: defaults::fairness_window_ms(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrefetchConfig {
    #[serde(default = "defaults::prefetch_enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::prefetch_boot_assets")]
    pub boot_assets: Vec<String>,
    #[serde(default = "defaults::prefetch_desktop_assets")]
    pub desktop_assets: Vec<String>,
    #[serde(default = "defaults::prefetch_max_prefetch_mb")]
    pub max_prefetch_mb: u64,
}

impl Default for PrefetchConfig {
    fn default() -> Self {
        PrefetchConfig {
            enabled: defaults::prefetch_enabled(),
            boot_assets: defaults::prefetch_boot_assets(),
            desktop_assets: defaults::prefetch_desktop_assets(),
            max_prefetch_mb: defaults::prefetch_max_prefetch_mb(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    #[serde(default = "defaults::read_cache_algorithm")]
    pub read_cache_algorithm: String,
    #[serde(default = "defaults::write_cache_algorithm")]
    pub write_cache_algorithm: String,
    #[serde(default = "defaults::compression_zstd_level")]
    pub zstd_level: i32,
    #[serde(default = "defaults::lz4_for_small_files")]
    pub lz4_for_small_files: bool,
    #[serde(default = "defaults::small_file_threshold_kb")]
    pub small_file_threshold_kb: u64,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig {
            read_cache_algorithm: defaults::read_cache_algorithm(),
            write_cache_algorithm: defaults::write_cache_algorithm(),
            zstd_level: defaults::compression_zstd_level(),
            lz4_for_small_files: defaults::lz4_for_small_files(),
            small_file_threshold_kb: defaults::small_file_threshold_kb(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EjectConfig {
    #[serde(default = "defaults::eject_require_clean_journal")]
    pub require_clean_journal: bool,
    #[serde(default = "defaults::eject_force_flush")]
    pub force_flush: bool,
    #[serde(default = "defaults::eject_timeout_ms")]
    pub timeout_ms: u64,
    #[serde(default = "defaults::eject_fail_if_dirty")]
    pub fail_if_dirty: bool,
}

impl Default for EjectConfig {
    fn default() -> Self {
        EjectConfig {
            require_clean_journal: defaults::eject_require_clean_journal(),
            force_flush: defaults::eject_force_flush(),
            timeout_ms: defaults::eject_timeout_ms(),
            fail_if_dirty: defaults::eject_fail_if_dirty(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RamConfig {
    #[serde(default = "defaults::ram_use_volt_ram_manager")]
    pub use_volt_ram_manager: bool,
    #[serde(default = "defaults::ram_quota_mb")]
    pub quota_mb: u64,
    #[serde(default = "defaults::ram_pressure_backoff")]
    pub pressure_backoff: bool,
}

impl Default for RamConfig {
    fn default() -> Self {
        RamConfig {
            use_volt_ram_manager: defaults::ram_use_volt_ram_manager(),
            quota_mb: defaults::ram_quota_mb(),
            pressure_backoff: defaults::ram_pressure_backoff(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    #[serde(default = "defaults::metrics_enabled")]
    pub enabled: bool,
    #[serde(default = "defaults::metrics_latency_histograms")]
    pub latency_histograms: bool,
    #[serde(default = "defaults::metrics_throughput_counters")]
    pub throughput_counters: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig {
            enabled: defaults::metrics_enabled(),
            latency_histograms: defaults::metrics_latency_histograms(),
            throughput_counters: defaults::metrics_throughput_counters(),
        }
    }
}
