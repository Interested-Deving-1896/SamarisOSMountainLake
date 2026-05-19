use std::collections::HashMap;

use serde::Deserialize;

use crate::config::defaults;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;

#[derive(Debug, Clone, Deserialize)]
pub struct VrmConfig {
    #[serde(default)]
    pub manager: ManagerConfig,
    #[serde(default)]
    pub pressure: PressureConfig,
    #[serde(default)]
    pub compression: CompressionConfig,
    #[serde(default)]
    pub deduplication: DedupConfig,
    #[serde(default)]
    pub pools: PoolsConfig,
    #[serde(default)]
    pub gc: GcConfig,
    #[serde(default)]
    pub apps: HashMap<String, AppConfig>,
}

impl VrmConfig {
    pub fn load(path: &str) -> VrmResult<Self> {
        let content = std::fs::read_to_string(path).map_err(|e| VrmError::InvalidConfig(format!("failed to read config file '{}': {}", path, e)))?;
        let config: Self = toml::from_str(&content).map_err(|e| VrmError::InvalidConfig(format!("failed to parse config '{}': {}", path, e)))?;
        config.validate()?;
        Ok(config)
    }

    pub fn load_or_default(path: &str) -> Self {
        match Self::load(path) {
            Ok(config) => config,
            Err(e) => {
                tracing::warn!("Failed to load config from '{}': {}; using defaults", path, e);
                Self::default()
            }
        }
    }
}

impl Default for VrmConfig {
    fn default() -> Self {
        Self {
            manager: ManagerConfig::default(),
            pressure: PressureConfig::default(),
            compression: CompressionConfig::default(),
            deduplication: DedupConfig::default(),
            pools: PoolsConfig::default(),
            gc: GcConfig::default(),
            apps: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManagerConfig {
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default = "default_shm_size_mb")]
    pub shm_size_mb: u64,
    #[serde(default = "default_page_size_kb")]
    pub page_size_kb: u64,
    #[serde(default = "default_numa_enabled")]
    pub numa_enabled: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_compression_enabled")]
    pub enable_compression: bool,
    #[serde(default = "default_dedup_enabled")]
    pub enable_deduplication: bool,
}

fn default_workers() -> usize { defaults::DEFAULT_WORKERS }
fn default_shm_size_mb() -> u64 { defaults::DEFAULT_SHM_SIZE_MB }
fn default_page_size_kb() -> u64 { defaults::DEFAULT_PAGE_SIZE_KB }
fn default_numa_enabled() -> bool { defaults::DEFAULT_NUMA_ENABLED }
fn default_log_level() -> String { defaults::DEFAULT_LOG_LEVEL.to_string() }
fn default_compression_enabled() -> bool { true }
fn default_dedup_enabled() -> bool { true }

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            workers: defaults::DEFAULT_WORKERS,
            shm_size_mb: defaults::DEFAULT_SHM_SIZE_MB,
            page_size_kb: defaults::DEFAULT_PAGE_SIZE_KB,
            numa_enabled: defaults::DEFAULT_NUMA_ENABLED,
            log_level: defaults::DEFAULT_LOG_LEVEL.to_string(),
            enable_compression: true,
            enable_deduplication: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PressureConfig {
    #[serde(default = "default_green_max")]
    pub green_max: f64,
    #[serde(default = "default_yellow_enter")]
    pub yellow_enter: f64,
    #[serde(default = "default_yellow_exit")]
    pub yellow_exit: f64,
    #[serde(default = "default_orange_enter")]
    pub orange_enter: f64,
    #[serde(default = "default_orange_exit")]
    pub orange_exit: f64,
    #[serde(default = "default_red_enter")]
    pub red_enter: f64,
    #[serde(default = "default_red_exit")]
    pub red_exit: f64,
    #[serde(default = "default_sample_interval_ms")]
    pub sample_interval_ms: u64,
    #[serde(default = "default_history_window_secs")]
    pub history_window_secs: u64,
}

fn default_green_max() -> f64 { defaults::DEFAULT_GREEN_MAX }
fn default_yellow_enter() -> f64 { defaults::DEFAULT_YELLOW_ENTER }
fn default_yellow_exit() -> f64 { defaults::DEFAULT_YELLOW_EXIT }
fn default_orange_enter() -> f64 { defaults::DEFAULT_ORANGE_ENTER }
fn default_orange_exit() -> f64 { defaults::DEFAULT_ORANGE_EXIT }
fn default_red_enter() -> f64 { defaults::DEFAULT_RED_ENTER }
fn default_red_exit() -> f64 { defaults::DEFAULT_RED_EXIT }
fn default_sample_interval_ms() -> u64 { defaults::DEFAULT_SAMPLE_INTERVAL_MS }
fn default_history_window_secs() -> u64 { defaults::DEFAULT_HISTORY_WINDOW_SECS }

impl Default for PressureConfig {
    fn default() -> Self {
        Self {
            green_max: defaults::DEFAULT_GREEN_MAX,
            yellow_enter: defaults::DEFAULT_YELLOW_ENTER,
            yellow_exit: defaults::DEFAULT_YELLOW_EXIT,
            orange_enter: defaults::DEFAULT_ORANGE_ENTER,
            orange_exit: defaults::DEFAULT_ORANGE_EXIT,
            red_enter: defaults::DEFAULT_RED_ENTER,
            red_exit: defaults::DEFAULT_RED_EXIT,
            sample_interval_ms: defaults::DEFAULT_SAMPLE_INTERVAL_MS,
            history_window_secs: defaults::DEFAULT_HISTORY_WINDOW_SECS,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompressionConfig {
    #[serde(default = "default_compression_algorithm")]
    pub algorithm: String,
    #[serde(default = "default_compression_level")]
    pub level: i32,
    #[serde(default = "default_compression_min_size")]
    pub min_size_bytes: u64,
    #[serde(default = "default_compression_max_size")]
    pub max_size_bytes: u64,
    #[serde(default = "default_lz4_enabled")]
    pub enable_lz4: bool,
}

fn default_compression_algorithm() -> String { defaults::DEFAULT_COMPRESSION_ALGORITHM.to_string() }
fn default_compression_level() -> i32 { defaults::DEFAULT_COMPRESSION_LEVEL }
fn default_compression_min_size() -> u64 { defaults::DEFAULT_COMPRESSION_MIN_SIZE }
fn default_compression_max_size() -> u64 { defaults::DEFAULT_COMPRESSION_MAX_SIZE }
fn default_lz4_enabled() -> bool { true }

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: defaults::DEFAULT_COMPRESSION_ALGORITHM.to_string(),
            level: defaults::DEFAULT_COMPRESSION_LEVEL,
            min_size_bytes: defaults::DEFAULT_COMPRESSION_MIN_SIZE,
            max_size_bytes: defaults::DEFAULT_COMPRESSION_MAX_SIZE,
            enable_lz4: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DedupConfig {
    #[serde(default = "default_dedup_enabled")]
    pub enabled: bool,
    #[serde(default = "default_dedup_hash")]
    pub hash_algorithm: String,
    #[serde(default = "default_dedup_min_block")]
    pub min_block_size: u64,
    #[serde(default = "default_dedup_max_block")]
    pub max_block_size: u64,
    #[serde(default = "default_dedup_threshold")]
    pub dedup_threshold: f64,
}

fn default_dedup_hash() -> String { defaults::DEFAULT_DEDUP_HASH.to_string() }
fn default_dedup_min_block() -> u64 { defaults::DEFAULT_DEDUP_MIN_BLOCK }
fn default_dedup_max_block() -> u64 { defaults::DEFAULT_DEDUP_MAX_BLOCK }
fn default_dedup_threshold() -> f64 { defaults::DEFAULT_DEDUP_THRESHOLD }

impl Default for DedupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hash_algorithm: defaults::DEFAULT_DEDUP_HASH.to_string(),
            min_block_size: defaults::DEFAULT_DEDUP_MIN_BLOCK,
            max_block_size: defaults::DEFAULT_DEDUP_MAX_BLOCK,
            dedup_threshold: defaults::DEFAULT_DEDUP_THRESHOLD,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoolsConfig {
    #[serde(default = "default_pool_count")]
    pub pool_count: usize,
    #[serde(default)]
    pub size_classes: Vec<String>,
}

fn default_pool_count() -> usize { defaults::DEFAULT_POOL_COUNT }

impl Default for PoolsConfig {
    fn default() -> Self {
        Self {
            pool_count: defaults::DEFAULT_POOL_COUNT,
            size_classes: vec![
                "16B".into(), "64B".into(), "256B".into(),
                "1KB".into(), "4KB".into(), "16KB".into(), "64KB".into(),
            ],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GcConfig {
    #[serde(default = "default_gc_interval_ms")]
    pub interval_ms: u64,
    #[serde(default = "default_gc_threshold_pct")]
    pub threshold_percent: f64,
    #[serde(default = "default_gc_aggressive_pct")]
    pub aggressive_threshold: f64,
    #[serde(default = "default_gc_max_pages")]
    pub max_pages_per_cycle: u32,
}

fn default_gc_interval_ms() -> u64 { defaults::DEFAULT_GC_INTERVAL_MS }
fn default_gc_threshold_pct() -> f64 { defaults::DEFAULT_GC_THRESHOLD_PCT }
fn default_gc_aggressive_pct() -> f64 { defaults::DEFAULT_GC_AGGRESSIVE_PCT }
fn default_gc_max_pages() -> u32 { defaults::DEFAULT_GC_MAX_PAGES }

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            interval_ms: defaults::DEFAULT_GC_INTERVAL_MS,
            threshold_percent: defaults::DEFAULT_GC_THRESHOLD_PCT,
            aggressive_threshold: defaults::DEFAULT_GC_AGGRESSIVE_PCT,
            max_pages_per_cycle: defaults::DEFAULT_GC_MAX_PAGES,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_app_max_memory_mb")]
    pub max_memory_mb: u64,
    #[serde(default = "default_app_priority")]
    pub priority: String,
    #[serde(default = "default_app_compression_allowed")]
    pub compression_allowed: bool,
    #[serde(default = "default_app_inactive_after_ms")]
    pub inactive_after_ms: u64,
    #[serde(default = "default_app_preferred_tier")]
    pub preferred_tier: String,
}

fn default_app_max_memory_mb() -> u64 { defaults::DEFAULT_APP_MAX_MEMORY_MB }
fn default_app_priority() -> String { defaults::DEFAULT_APP_PRIORITY.to_string() }
fn default_app_compression_allowed() -> bool { defaults::DEFAULT_APP_COMPRESSION_ALLOWED }
fn default_app_inactive_after_ms() -> u64 { defaults::DEFAULT_APP_INACTIVE_AFTER_MS }
fn default_app_preferred_tier() -> String { defaults::DEFAULT_APP_PREFERRED_TIER.to_string() }

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_memory_mb: defaults::DEFAULT_APP_MAX_MEMORY_MB,
            priority: defaults::DEFAULT_APP_PRIORITY.to_string(),
            compression_allowed: defaults::DEFAULT_APP_COMPRESSION_ALLOWED,
            inactive_after_ms: defaults::DEFAULT_APP_INACTIVE_AFTER_MS,
            preferred_tier: defaults::DEFAULT_APP_PREFERRED_TIER.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VrmConfig::default();
        assert_eq!(config.manager.workers, 8);
        assert_eq!(config.manager.shm_size_mb, 64);
        assert!(config.manager.enable_compression);
        assert!(config.manager.enable_deduplication);
        assert_eq!(config.pressure.green_max, 70.0);
        assert_eq!(config.pressure.yellow_enter, 70.0);
        assert_eq!(config.pressure.yellow_exit, 65.0);
        assert_eq!(config.pressure.orange_enter, 80.0);
        assert_eq!(config.pressure.red_enter, 90.0);
        assert_eq!(config.compression.algorithm, "zstd");
        assert!(config.deduplication.enabled);
        assert_eq!(config.gc.interval_ms, 5000);
        assert!(config.apps.is_empty());
    }

    #[test]
    fn test_config_roundtrip() {
        let toml_str = r#"
[manager]
workers = 4
shm_size_mb = 128

[pressure]
green_max = 60.0
yellow_enter = 65.0
yellow_exit = 60.0

[compression]
algorithm = "lz4"
level = 1

[deduplication]
enabled = true

[pools]
pool_count = 2

[gc]
interval_ms = 10000

[apps.test_app]
max_memory_mb = 1024
priority = "critical"
"#;
        let config: VrmConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.manager.workers, 4);
        assert_eq!(config.manager.shm_size_mb, 128);
        assert_eq!(config.pressure.green_max, 60.0);
        assert_eq!(config.compression.algorithm, "lz4");
        assert_eq!(config.apps.len(), 1);
        let app = config.apps.get("test_app").unwrap();
        assert_eq!(app.max_memory_mb, 1024);
        assert_eq!(app.priority, "critical");
    }
}
