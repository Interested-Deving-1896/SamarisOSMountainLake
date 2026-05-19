use serde::Deserialize;

use crate::config::defaults;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct VgmConfig {
    pub gpu: GpuConfig,
}

impl Default for VgmConfig {
    fn default() -> Self {
        Self {
            gpu: GpuConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct GpuConfig {
    pub backend: String,
    pub frame_budget_ms: u64,
    pub vram: VramConfig,
    pub scheduler: SchedulerConfig,
    pub shaders: ShaderConfig,
    pub compute: ComputeConfig,
    pub multi_gpu: MultiGpuConfig,
    pub thermal: ThermalConfig,
    pub quotas: QuotaConfig,
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            backend: defaults::DEFAULT_BACKEND_TYPE.to_string(),
            frame_budget_ms: defaults::DEFAULT_FRAME_BUDGET_MS,
            vram: VramConfig::default(),
            scheduler: SchedulerConfig::default(),
            shaders: ShaderConfig::default(),
            compute: ComputeConfig::default(),
            multi_gpu: MultiGpuConfig::default(),
            thermal: ThermalConfig::default(),
            quotas: QuotaConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct VramConfig {
    pub max_vram_percent: u8,
    pub reserved_mb: u64,
    pub compression: CompressionConfig,
    pub deduplication: DedupConfig,
    pub t1_pool_size_mb: u64,
    pub t2_pool_size_mb: u64,
    pub scratch_budget_mb: u64,
}

impl Default for VramConfig {
    fn default() -> Self {
        Self {
            max_vram_percent: defaults::DEFAULT_MAX_VRAM_PERCENT,
            reserved_mb: defaults::DEFAULT_VRAM_RESERVED_MB,
            compression: CompressionConfig::default(),
            deduplication: DedupConfig::default(),
            t1_pool_size_mb: defaults::DEFAULT_VRAM_T1_POOL_SIZE_MB,
            t2_pool_size_mb: defaults::DEFAULT_VRAM_T2_POOL_SIZE_MB,
            scratch_budget_mb: defaults::DEFAULT_SCRATCH_BUDGET_MB,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct CompressionConfig {
    pub algorithm: String,
    pub level: u8,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            algorithm: defaults::DEFAULT_COMPRESSION_ALGORITHM.to_string(),
            level: defaults::DEFAULT_COMPRESSION_LEVEL,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DedupConfig {
    pub enabled: bool,
    pub block_size: u64,
}

impl Default for DedupConfig {
    fn default() -> Self {
        Self {
            enabled: defaults::DEFAULT_DEDUP_ENABLED,
            block_size: defaults::DEFAULT_DEDUP_BLOCK_SIZE,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SchedulerConfig {
    pub queue_depth: usize,
    pub max_inflight: usize,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            queue_depth: defaults::DEFAULT_SCHEDULER_QUEUE_DEPTH,
            max_inflight: defaults::DEFAULT_SCHEDULER_MAX_INFLIGHT,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ShaderConfig {
    pub cache_size: usize,
}

impl Default for ShaderConfig {
    fn default() -> Self {
        Self {
            cache_size: defaults::DEFAULT_SHADER_CACHE_SIZE,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ComputeConfig {
    pub max_workgroups: u32,
}

impl Default for ComputeConfig {
    fn default() -> Self {
        Self {
            max_workgroups: defaults::DEFAULT_COMPUTE_MAX_WORKGROUPS,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MultiGpuConfig {
    pub mode: String,
}

impl Default for MultiGpuConfig {
    fn default() -> Self {
        Self {
            mode: defaults::DEFAULT_MULTI_GPU_MODE.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ThermalConfig {
    pub throttle_temp: f32,
    pub emergency_temp: f32,
    pub poll_interval_ms: u64,
}

impl Default for ThermalConfig {
    fn default() -> Self {
        Self {
            throttle_temp: defaults::DEFAULT_THERMAL_THROTTLE_TEMP,
            emergency_temp: defaults::DEFAULT_THERMAL_EMERGENCY_TEMP,
            poll_interval_ms: defaults::DEFAULT_THERMAL_POLL_INTERVAL_MS,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct QuotaConfig {
    pub max_allocations: usize,
    pub max_pinned_mb: u64,
}

impl Default for QuotaConfig {
    fn default() -> Self {
        Self {
            max_allocations: defaults::DEFAULT_QUOTA_MAX_ALLOCATIONS,
            max_pinned_mb: defaults::DEFAULT_QUOTA_MAX_PINNED_MB,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VgmConfig::default();
        assert_eq!(config.gpu.backend, "wgpu");
        assert_eq!(config.gpu.frame_budget_ms, 16);
        assert_eq!(config.gpu.vram.max_vram_percent, 90);
        assert_eq!(config.gpu.vram.compression.algorithm, "zstd");
        assert_eq!(config.gpu.thermal.throttle_temp, 85.0);
        assert_eq!(config.gpu.multi_gpu.mode, "single");
    }

    #[test]
    fn test_deserialize_toml() {
        let toml_str = r#"
[gpu]
backend = "null"
frame_budget_ms = 32

[gpu.vram]
max_vram_percent = 75

[gpu.vram.compression]
algorithm = "lz4"
level = 5

[gpu.thermal]
throttle_temp = 80.0
emergency_temp = 90.0
"#;
        let config: VgmConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.gpu.backend, "null");
        assert_eq!(config.gpu.frame_budget_ms, 32);
        assert_eq!(config.gpu.vram.max_vram_percent, 75);
        assert_eq!(config.gpu.vram.compression.algorithm, "lz4");
        assert_eq!(config.gpu.vram.compression.level, 5);
        assert_eq!(config.gpu.thermal.throttle_temp, 80.0);
        assert_eq!(config.gpu.thermal.emergency_temp, 90.0);
    }

    #[test]
    fn test_deserialize_empty_uses_defaults() {
        let toml_str = "";
        let config: VgmConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.gpu.backend, "wgpu");
        assert_eq!(config.gpu.frame_budget_ms, 16);
    }
}
