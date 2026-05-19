use crate::config::schema::VgmConfig;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

impl VgmConfig {
    pub fn validate(&self) -> VgmResult<()> {
        if self.gpu.frame_budget_ms == 0 {
            return Err(VgmError::InvalidConfig(
                "frame_budget_ms must be > 0".into(),
            ));
        }

        if self.gpu.vram.max_vram_percent > 100 {
            return Err(VgmError::InvalidConfig(
                "max_vram_percent must be <= 100".into(),
            ));
        }

        if self.gpu.vram.max_vram_percent == 0 {
            return Err(VgmError::InvalidConfig(
                "max_vram_percent must be > 0".into(),
            ));
        }

        match self.gpu.vram.compression.algorithm.as_str() {
            "none" | "zstd" | "lz4" | "crc" => {}
            other => {
                return Err(VgmError::InvalidConfig(format!(
                    "Unknown compression algorithm '{}'",
                    other
                )))
            }
        }

        if self.gpu.vram.compression.level == 0 {
            return Err(VgmError::InvalidConfig(
                "compression level must be > 0".into(),
            ));
        }

        if self.gpu.vram.compression.level > 22 {
            return Err(VgmError::InvalidConfig(
                "compression level must be <= 22".into(),
            ));
        }

        match self.gpu.multi_gpu.mode.as_str() {
            "single" | "alternate" | "split" | "mirror" => {}
            other => {
                return Err(VgmError::InvalidConfig(format!(
                    "Unknown multi-gpu mode '{}'",
                    other
                )))
            }
        }

        match self.gpu.backend.as_str() {
            "wgpu" | "vulkan" | "metal" | "null" => {}
            other => {
                return Err(VgmError::InvalidConfig(format!(
                    "Unknown backend '{}'",
                    other
                )))
            }
        }

        if self.gpu.thermal.throttle_temp >= self.gpu.thermal.emergency_temp {
            return Err(VgmError::InvalidConfig(
                "throttle_temp must be less than emergency_temp".into(),
            ));
        }

        if self.gpu.vram.deduplication.block_size == 0
            || self.gpu.vram.deduplication.block_size & (self.gpu.vram.deduplication.block_size - 1) != 0
        {
            return Err(VgmError::InvalidConfig(
                "dedup block_size must be a non-zero power of two".into(),
            ));
        }

        if self.gpu.vram.t1_pool_size_mb == 0 {
            return Err(VgmError::InvalidConfig(
                "t1_pool_size_mb must be > 0".into(),
            ));
        }

        if self.gpu.quotas.max_allocations == 0 {
            return Err(VgmError::InvalidConfig(
                "max_allocations must be > 0".into(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VgmConfig;

    #[test]
    fn test_valid_default_config() {
        let config = VgmConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_frame_budget() {
        let mut config = VgmConfig::default();
        config.gpu.frame_budget_ms = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_vram_percent() {
        let mut config = VgmConfig::default();
        config.gpu.vram.max_vram_percent = 101;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_compression_algorithm() {
        let mut config = VgmConfig::default();
        config.gpu.vram.compression.algorithm = "invalid".into();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_backend() {
        let mut config = VgmConfig::default();
        config.gpu.backend = "dx12".into();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_thermal_thresholds() {
        let mut config = VgmConfig::default();
        config.gpu.thermal.throttle_temp = 95.0;
        config.gpu.thermal.emergency_temp = 85.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_dedup_block_size() {
        let mut config = VgmConfig::default();
        config.gpu.vram.deduplication.block_size = 0;
        assert!(config.validate().is_err());
    }
}
