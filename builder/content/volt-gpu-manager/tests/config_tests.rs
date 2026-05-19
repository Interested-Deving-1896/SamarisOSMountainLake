use volt_gpu_manager::config::schema::VgmConfig;
use volt_gpu_manager::config::loader::load_config;
use tempfile::NamedTempFile;
use std::io::Write;

#[test]
fn config_example_toml_parses() {
    let toml = r#"
[gpu]
backend = "null"
frame_budget_ms = 16

[gpu.vram]
max_vram_percent = 90
reserved_mb = 256
t1_pool_size_mb = 1024
t2_pool_size_mb = 4096
scratch_budget_mb = 64

[gpu.vram.compression]
algorithm = "zstd"
level = 3

[gpu.vram.deduplication]
enabled = false
block_size = 4096

[gpu.scheduler]
queue_depth = 64
max_inflight = 4

[gpu.shaders]
cache_size = 128

[gpu.compute]
max_workgroups = 65535

[gpu.multi_gpu]
mode = "single"

[gpu.thermal]
throttle_temp = 85.0
emergency_temp = 95.0
poll_interval_ms = 1000

[gpu.quotas]
max_allocations = 1024
max_pinned_mb = 512
"#;
    let config: VgmConfig = toml::from_str(toml).unwrap();
    assert_eq!(config.gpu.backend, "null");
    assert_eq!(config.gpu.frame_budget_ms, 16);
    assert_eq!(config.gpu.vram.max_vram_percent, 90);
    assert_eq!(config.gpu.vram.reserved_mb, 256);
    assert_eq!(config.gpu.vram.compression.algorithm, "zstd");
    assert_eq!(config.gpu.vram.compression.level, 3);
    assert_eq!(config.gpu.vram.deduplication.enabled, false);
    assert_eq!(config.gpu.scheduler.queue_depth, 64);
    assert_eq!(config.gpu.shaders.cache_size, 128);
    assert_eq!(config.gpu.compute.max_workgroups, 65535);
    assert_eq!(config.gpu.multi_gpu.mode, "single");
    assert_eq!(config.gpu.thermal.throttle_temp, 85.0);
    assert_eq!(config.gpu.quotas.max_allocations, 1024);
}

#[test]
fn invalid_config_rejected() {
    let toml = "[gpu]\nbackend = \"nonexistent\"\n";
    let config: VgmConfig = toml::from_str(toml).unwrap();
    assert_eq!(config.gpu.backend, "nonexistent");
    let mut file = NamedTempFile::new().unwrap();
    write!(file, "{}", toml).unwrap();
    let loaded = load_config(file.path());
    assert!(loaded.is_err());
}

#[test]
fn empty_toml_uses_defaults() {
    let config: VgmConfig = toml::from_str("").unwrap();
    assert_eq!(config.gpu.backend, "wgpu");
    assert_eq!(config.gpu.frame_budget_ms, 16);
    assert_eq!(config.gpu.vram.max_vram_percent, 90);
}

#[test]
fn backend_auto_is_valid() {
    let config: VgmConfig = toml::from_str(r#"[gpu]
backend = "auto""#).unwrap_or_default();
    assert_eq!(config.gpu.backend, "auto");
}

#[test]
fn defaults_are_correct() {
    let d = VgmConfig::default();
    assert_eq!(d.gpu.frame_budget_ms, 16);
    assert_eq!(d.gpu.vram.max_vram_percent, 90);
    assert_eq!(d.gpu.vram.compression.level, 3);
    assert_eq!(d.gpu.thermal.throttle_temp, 85.0);
    assert_eq!(d.gpu.multi_gpu.mode, "single");
    assert_eq!(d.gpu.quotas.max_allocations, 1024);
}
