use crate::generator::generated_config::GeneratedConfig;

#[derive(Debug, Clone)]
pub struct SafetyCaps {
    pub max_workers: usize,
    pub max_orbit_quota_mb: u64,
    pub max_desktop_quota_mb: u64,
    pub max_vrm_cache_mb: u64,
    pub max_vum_cache_mb: u64,
    pub min_desktop_workers: usize,
}

#[derive(Debug, Clone)]
pub struct ClampedValue {
    pub field: String,
    pub original: String,
    pub clamped: String,
}

impl SafetyCaps {
    pub fn default_caps() -> Self {
        Self {
            max_workers: 16,
            max_orbit_quota_mb: 4096,
            max_desktop_quota_mb: 8192,
            max_vrm_cache_mb: 2048,
            max_vum_cache_mb: 1024,
            min_desktop_workers: 1,
        }
    }

    pub fn apply(&self, generated: &mut GeneratedConfig) -> Vec<ClampedValue> {
        let mut clamped = Vec::new();

        if generated.worker_pool.max_workers > self.max_workers {
            clamped.push(ClampedValue {
                field: "worker_pool.max_workers".into(),
                original: generated.worker_pool.max_workers.to_string(),
                clamped: self.max_workers.to_string(),
            });
            generated.worker_pool.max_workers = self.max_workers;
        }

        if generated.worker_pool.min_workers > generated.worker_pool.max_workers {
            clamped.push(ClampedValue {
                field: "worker_pool.min_workers".into(),
                original: generated.worker_pool.min_workers.to_string(),
                clamped: generated.worker_pool.max_workers.to_string(),
            });
            generated.worker_pool.min_workers = generated.worker_pool.max_workers;
        }

        if generated.worker_pool.desktop_min < self.min_desktop_workers {
            clamped.push(ClampedValue {
                field: "worker_pool.desktop_min".into(),
                original: generated.worker_pool.desktop_min.to_string(),
                clamped: self.min_desktop_workers.to_string(),
            });
            generated.worker_pool.desktop_min = self.min_desktop_workers;
        }

        if generated.vrm.orbit_quota_mb > self.max_orbit_quota_mb {
            clamped.push(ClampedValue {
                field: "vrm.orbit_quota_mb".into(),
                original: generated.vrm.orbit_quota_mb.to_string(),
                clamped: self.max_orbit_quota_mb.to_string(),
            });
            generated.vrm.orbit_quota_mb = self.max_orbit_quota_mb;
        }

        if generated.vrm.desktop_quota_mb > self.max_desktop_quota_mb {
            clamped.push(ClampedValue {
                field: "vrm.desktop_quota_mb".into(),
                original: generated.vrm.desktop_quota_mb.to_string(),
                clamped: self.max_desktop_quota_mb.to_string(),
            });
            generated.vrm.desktop_quota_mb = self.max_desktop_quota_mb;
        }

        if generated.vrm.cache_mb > self.max_vrm_cache_mb {
            clamped.push(ClampedValue {
                field: "vrm.cache_mb".into(),
                original: generated.vrm.cache_mb.to_string(),
                clamped: self.max_vrm_cache_mb.to_string(),
            });
            generated.vrm.cache_mb = self.max_vrm_cache_mb;
        }

        if generated.vum.cache_mb > self.max_vum_cache_mb {
            clamped.push(ClampedValue {
                field: "vum.cache_mb".into(),
                original: generated.vum.cache_mb.to_string(),
                clamped: self.max_vum_cache_mb.to_string(),
            });
            generated.vum.cache_mb = self.max_vum_cache_mb;
        }

        clamped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::generated_config::{
        GeneratedAscSection, GeneratedBudgetSection, GeneratedConfig, GeneratedKernelBConfig,
        GeneratedVrmConfig, GeneratedVumConfig, GeneratedWorkerPoolConfig,
    };

    fn make_config() -> GeneratedConfig {
        GeneratedConfig {
            kernel_b: GeneratedKernelBConfig { workers: 4 },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: 2,
                max_workers: 32,
                desktop_min: 0,
                system_min: 1,
                orbit_default_max: 4,
                orbit_burst_max: 8,
                orbit_burst_window_ms: 200,
            },
            vrm: GeneratedVrmConfig {
                desktop_quota_mb: 16384,
                orbit_quota_mb: 8192,
                cache_mb: 4096,
                pressure_green_max_percent: 50,
                pressure_yellow_enter_percent: 50,
                pressure_yellow_exit_percent: 45,
                pressure_orange_enter_percent: 70,
                pressure_orange_exit_percent: 60,
                pressure_red_enter_percent: 85,
                pressure_red_exit_percent: 75,
                min_free_mb_yellow: 1024,
                min_free_mb_orange: 512,
                min_free_mb_red: 256,
            },
            vum: GeneratedVumConfig {
                cache_mb: 2048,
                buffer_mb: 512,
                flush_interval_ms: 1000,
                batch_size_kb: 32,
                journal_mode: "wal".into(),
                prefetch_boot_assets: false,
            },
            budget: GeneratedBudgetSection {
                samaris_budget_cap_mb: 4096,
                allocated_total_mb: 2048,
                safety_margin_mb: 512,
            },
            asc: GeneratedAscSection {
                profile: "balanced".into(),
                machine_classes: vec![],
                generated_at: "now".into(),
                safe_mode: false,
            },
        }
    }

    #[test]
    fn test_apply_caps_clamps_excessive_values() {
        let caps = SafetyCaps::default_caps();
        let mut config = make_config();
        let clamped = caps.apply(&mut config);

        assert_eq!(config.worker_pool.max_workers, 16);
        assert_eq!(config.vrm.desktop_quota_mb, 8192);
        assert_eq!(config.vrm.orbit_quota_mb, 4096);
        assert_eq!(config.vrm.cache_mb, 2048);
        assert_eq!(config.vum.cache_mb, 1024);
        assert!(config.worker_pool.desktop_min >= 1);
        assert!(!clamped.is_empty());
    }
}
