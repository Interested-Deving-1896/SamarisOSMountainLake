use crate::config::schema::HardwareConfig;

pub struct HardwareProbe {
    pub cpu_cores: u32,
    pub available_parallelism: u32,
    pub ram_bytes: u64,
    pub min_workers: u32,
    pub max_workers: u32,
}

pub struct HardwareProfile {
    pub cpu_cores: u32,
    pub min_workers: u32,
    pub max_workers: u32,
    pub ram_bytes: u64,
}

impl HardwareProbe {
    pub fn new(config: HardwareConfig) -> Self {
        let cpu_cores = match std::thread::available_parallelism() {
            Ok(n) => n.get() as u32,
            Err(_) => config.default_cpu_cores,
        };

        let has_min_override = config.min_workers_override > 0;
        let has_max_override = config.max_workers_override > 0;

        let (mut min_workers, max_workers) = if has_min_override || has_max_override {
            let min = if has_min_override {
                config.min_workers_override
            } else {
                2
            };
            let max = if has_max_override {
                config.max_workers_override
            } else {
                48
            };
            (min, max)
        } else {
            let min = std::cmp::max(2, cpu_cores / 3).min(12);
            let max = std::cmp::max(min, cpu_cores * 3 / 4).min(48);
            (min, max)
        };

        if min_workers > max_workers {
            min_workers = max_workers;
        }

        HardwareProbe {
            cpu_cores,
            available_parallelism: cpu_cores,
            ram_bytes: config.ram_bytes,
            min_workers,
            max_workers,
        }
    }

    pub fn profile(&self) -> HardwareProfile {
        HardwareProfile {
            cpu_cores: self.cpu_cores,
            min_workers: self.min_workers,
            max_workers: self.max_workers,
            ram_bytes: self.ram_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_probe_defaults() {
        let config = HardwareConfig::default();
        let probe = HardwareProbe::new(config);
        assert!(probe.min_workers >= 2);
        assert!(probe.max_workers <= 48);
        assert!(probe.min_workers <= probe.max_workers);
    }

    #[test]
    fn test_hardware_probe_overrides() {
        let config = HardwareConfig {
            min_workers_override: 8,
            max_workers_override: 24,
            ..HardwareConfig::default()
        };
        let probe = HardwareProbe::new(config);
        assert_eq!(probe.min_workers, 8);
        assert_eq!(probe.max_workers, 24);
    }

    #[test]
    fn test_hardware_probe_min_not_exceed_max() {
        let config = HardwareConfig {
            min_workers_override: 20,
            max_workers_override: 10,
            ..HardwareConfig::default()
        };
        let probe = HardwareProbe::new(config);
        assert!(probe.min_workers <= probe.max_workers);
    }

    #[test]
    fn test_hardware_profile() {
        let config = HardwareConfig::default();
        let probe = HardwareProbe::new(config);
        let profile = probe.profile();
        assert_eq!(profile.cpu_cores, probe.cpu_cores);
        assert_eq!(profile.min_workers, probe.min_workers);
        assert_eq!(profile.max_workers, probe.max_workers);
        assert_eq!(profile.ram_bytes, probe.ram_bytes);
    }
}
