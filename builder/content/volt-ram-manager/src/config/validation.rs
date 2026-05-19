use crate::config::schema::VrmConfig;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;

impl VrmConfig {
    pub fn validate(&self) -> VrmResult<()> {
        // Manager validation
        if self.manager.workers == 0 {
            return Err(VrmError::InvalidConfig("manager.workers must be > 0".into()));
        }
        if self.manager.shm_size_mb == 0 {
            return Err(VrmError::InvalidConfig("manager.shm_size_mb must be > 0".into()));
        }
        if self.manager.page_size_kb == 0 {
            return Err(VrmError::InvalidConfig("manager.page_size_kb must be > 0".into()));
        }

        // Pressure threshold validation: green <= yellow <= orange <= red
        let p = &self.pressure;
        if p.green_max > p.yellow_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.green_max ({}) must be <= pressure.yellow_enter ({})", p.green_max, p.yellow_enter),
            ));
        }
        if p.yellow_enter < p.yellow_exit {
            return Err(VrmError::InvalidConfig(
                format!("pressure.yellow_enter ({}) must be >= pressure.yellow_exit ({})", p.yellow_enter, p.yellow_exit),
            ));
        }
        if p.yellow_enter > p.orange_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.yellow_enter ({}) must be <= pressure.orange_enter ({})", p.yellow_enter, p.orange_enter),
            ));
        }
        if p.orange_enter < p.orange_exit {
            return Err(VrmError::InvalidConfig(
                format!("pressure.orange_enter ({}) must be >= pressure.orange_exit ({})", p.orange_enter, p.orange_exit),
            ));
        }
        if p.orange_enter > p.red_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.orange_enter ({}) must be <= pressure.red_enter ({})", p.orange_enter, p.red_enter),
            ));
        }
        if p.red_enter < p.red_exit {
            return Err(VrmError::InvalidConfig(
                format!("pressure.red_enter ({}) must be >= pressure.red_exit ({})", p.red_enter, p.red_exit),
            ));
        }

        // Exit thresholds must be below enter thresholds for hysteresis
        if p.yellow_exit >= p.yellow_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.yellow_exit ({}) must be < pressure.yellow_enter ({})", p.yellow_exit, p.yellow_enter),
            ));
        }
        if p.orange_exit >= p.orange_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.orange_exit ({}) must be < pressure.orange_enter ({})", p.orange_exit, p.orange_enter),
            ));
        }
        if p.red_exit >= p.red_enter {
            return Err(VrmError::InvalidConfig(
                format!("pressure.red_exit ({}) must be < pressure.red_enter ({})", p.red_exit, p.red_enter),
            ));
        }

        // GC validation
        if self.gc.interval_ms == 0 {
            return Err(VrmError::InvalidConfig("gc.interval_ms must be > 0".into()));
        }
        if self.gc.threshold_percent <= 0.0 || self.gc.threshold_percent > 100.0 {
            return Err(VrmError::InvalidConfig(
                format!("gc.threshold_percent ({}) must be in (0, 100]", self.gc.threshold_percent),
            ));
        }
        if self.gc.aggressive_threshold <= 0.0 || self.gc.aggressive_threshold > 100.0 {
            return Err(VrmError::InvalidConfig(
                format!("gc.aggressive_threshold ({}) must be in (0, 100]", self.gc.aggressive_threshold),
            ));
        }

        // Pool validation
        if self.pools.pool_count == 0 {
            return Err(VrmError::InvalidConfig("pools.pool_count must be > 0".into()));
        }

        // Sample interval
        if p.sample_interval_ms == 0 {
            return Err(VrmError::InvalidConfig("pressure.sample_interval_ms must be > 0".into()));
        }

        // Compression validation
        if self.compression.min_size_bytes >= self.compression.max_size_bytes {
            return Err(VrmError::InvalidConfig(
                format!(
                    "compression.min_size_bytes ({}) must be < compression.max_size_bytes ({})",
                    self.compression.min_size_bytes, self.compression.max_size_bytes
                ),
            ));
        }

        // Dedup validation
        if self.deduplication.min_block_size >= self.deduplication.max_block_size {
            return Err(VrmError::InvalidConfig(
                format!(
                    "deduplication.min_block_size ({}) must be < deduplication.max_block_size ({})",
                    self.deduplication.min_block_size, self.deduplication.max_block_size
                ),
            ));
        }
        if self.deduplication.dedup_threshold < 1.0 {
            return Err(VrmError::InvalidConfig(
                format!("deduplication.dedup_threshold ({}) must be >= 1.0", self.deduplication.dedup_threshold),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::schema::*;

    fn valid_config() -> VrmConfig {
        VrmConfig::default()
    }

    #[test]
    fn test_valid_config_passes() {
        assert!(valid_config().validate().is_ok());
    }

    #[test]
    fn test_zero_workers_fails() {
        let mut c = valid_config();
        c.manager.workers = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_zero_shm_fails() {
        let mut c = valid_config();
        c.manager.shm_size_mb = 0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_green_exceeds_yellow_fails() {
        let mut c = valid_config();
        c.pressure.green_max = 80.0;
        c.pressure.yellow_enter = 70.0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_yellow_enter_below_exit_fails() {
        let mut c = valid_config();
        c.pressure.yellow_enter = 60.0;
        c.pressure.yellow_exit = 65.0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_exit_not_below_enter_fails() {
        let mut c = valid_config();
        c.pressure.yellow_exit = 70.0;
        c.pressure.yellow_enter = 70.0;
        assert!(c.validate().is_err());
    }

    #[test]
    fn test_hysteresis_chain() {
        let mut c = valid_config();
        c.pressure.orange_enter = 75.0;
        c.pressure.orange_exit = 75.0;
        assert!(c.validate().is_err());
    }
}
