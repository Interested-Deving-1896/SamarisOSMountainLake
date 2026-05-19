use crate::config::schema::VumConfig;
use crate::core::error::VumError;
use crate::core::result::VumResult;

impl VumConfig {
    pub fn validate(&self) -> VumResult<()> {
        if self.manager.mount_point.is_empty() {
            return Err(VumError::InvalidConfig(
                "manager.mount_point must not be empty".into(),
            ));
        }
        if self.manager.backing_path.is_empty() {
            return Err(VumError::InvalidConfig(
                "manager.backing_path must not be empty".into(),
            ));
        }
        if self.cache.read_cache_max_mb == 0 {
            return Err(VumError::InvalidConfig(
                "cache.read_cache_max_mb must be greater than 0".into(),
            ));
        }
        if self.cache.evict_at_percent > 100 {
            return Err(VumError::InvalidConfig(
                "cache.evict_at_percent must be between 0 and 100".into(),
            ));
        }
        if self.writeback.enabled && self.writeback.buffer_max_mb == 0 {
            return Err(VumError::InvalidConfig(
                "writeback.buffer_max_mb must be greater than 0 when writeback is enabled".into(),
            ));
        }
        if self.writeback.flush_at_percent > 100 {
            return Err(VumError::InvalidConfig(
                "writeback.flush_at_percent must be between 0 and 100".into(),
            ));
        }
        if self.scheduler.background_throttle > 100 {
            return Err(VumError::InvalidConfig(
                "scheduler.background_throttle must be between 0 and 100".into(),
            ));
        }
        if self.eject.timeout_ms == 0 {
            return Err(VumError::InvalidConfig(
                "eject.timeout_ms must be greater than 0".into(),
            ));
        }
        if self.prefetch.max_prefetch_mb == 0 && self.prefetch.enabled {
            return Err(VumError::InvalidConfig(
                "prefetch.max_prefetch_mb must be greater than 0 when prefetch is enabled".into(),
            ));
        }
        if self.compression.zstd_level < 1 || self.compression.zstd_level > 22 {
            return Err(VumError::InvalidConfig(
                "compression.zstd_level must be between 1 and 22".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = VumConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_empty_mount_point() {
        let mut config = VumConfig::default();
        config.manager.mount_point = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_empty_backing_path() {
        let mut config = VumConfig::default();
        config.manager.backing_path = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_cache_size() {
        let mut config = VumConfig::default();
        config.cache.read_cache_max_mb = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_evict_percent() {
        let mut config = VumConfig::default();
        config.cache.evict_at_percent = 150;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_buffer_with_writeback_enabled() {
        let mut config = VumConfig::default();
        config.writeback.buffer_max_mb = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_zero_timeout() {
        let mut config = VumConfig::default();
        config.eject.timeout_ms = 0;
        assert!(config.validate().is_err());
    }
}
