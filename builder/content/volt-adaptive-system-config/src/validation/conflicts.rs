use crate::core::error::AscError;
use crate::core::result::AscResult;
use crate::generator::generated_config::GeneratedConfig;

#[derive(Debug, Clone)]
pub enum ConflictSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub module: String,
    pub field: String,
    pub a: String,
    pub b: String,
    pub severity: ConflictSeverity,
}

pub struct ConflictResolver;

impl ConflictResolver {
    pub fn detect_conflicts(generated: &GeneratedConfig) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        if generated.worker_pool.desktop_min > generated.worker_pool.min_workers {
            conflicts.push(Conflict {
                module: "worker_pool".into(),
                field: "desktop_min".into(),
                a: format!("desktop_min={}", generated.worker_pool.desktop_min),
                b: format!("min_workers={}", generated.worker_pool.min_workers),
                severity: ConflictSeverity::Error,
            });
        }

        if generated.worker_pool.orbit_default_max > generated.worker_pool.max_workers {
            conflicts.push(Conflict {
                module: "worker_pool".into(),
                field: "orbit_default_max".into(),
                a: format!("orbit_default_max={}", generated.worker_pool.orbit_default_max),
                b: format!("max_workers={}", generated.worker_pool.max_workers),
                severity: ConflictSeverity::Error,
            });
        }

        if generated.vrm.desktop_quota_mb + generated.vrm.orbit_quota_mb + generated.vrm.cache_mb
            > generated.budget.samaris_budget_cap_mb
        {
            conflicts.push(Conflict {
                module: "vrm".into(),
                field: "combined_quota".into(),
                a: format!(
                    "vrm_total={}",
                    generated.vrm.desktop_quota_mb + generated.vrm.orbit_quota_mb + generated.vrm.cache_mb
                ),
                b: format!("budget_cap={}", generated.budget.samaris_budget_cap_mb),
                severity: ConflictSeverity::Warning,
            });
        }

        if generated.kernel_b.workers > generated.worker_pool.max_workers {
            conflicts.push(Conflict {
                module: "kernel_b".into(),
                field: "workers".into(),
                a: format!("kernel_b_workers={}", generated.kernel_b.workers),
                b: format!("max_workers={}", generated.worker_pool.max_workers),
                severity: ConflictSeverity::Warning,
            });
        }

        conflicts
    }

    pub fn resolve(conflicts: Vec<Conflict>, safe_mode: bool) -> AscResult<GeneratedConfig> {
        if !safe_mode {
            let errors: Vec<_> = conflicts
                .iter()
                .filter(|c| matches!(c.severity, ConflictSeverity::Error))
                .collect();
            if !errors.is_empty() {
                let details: Vec<String> = errors
                    .iter()
                    .map(|c| format!("{}: {} vs {}", c.field, c.a, c.b))
                    .collect();
                return Err(AscError::PolicyConflict(details.join("; ")));
            }
        }
        Err(AscError::PolicyConflict(
            "resolve() requires a mutable config reference to apply resolutions — use resolve_in_place instead".into(),
        ))
    }

    pub fn resolve_in_place(
        conflicts: &[Conflict],
        generated: &mut GeneratedConfig,
        safe_mode: bool,
    ) -> AscResult<Vec<String>> {
        let mut resolutions = Vec::new();

        for conflict in conflicts {
            match conflict.severity {
                ConflictSeverity::Error => {
                    if safe_mode {
                        match conflict.field.as_str() {
                            "desktop_min" => {
                                generated.worker_pool.desktop_min = generated.worker_pool.min_workers;
                                resolutions.push(format!("clamped desktop_min to min_workers ({})", generated.worker_pool.min_workers));
                            }
                            "orbit_default_max" => {
                                generated.worker_pool.orbit_default_max = generated.worker_pool.max_workers;
                                resolutions.push(format!("clamped orbit_default_max to max_workers ({})", generated.worker_pool.max_workers));
                            }
                            _ => {
                                resolutions.push(format!("unresolved error conflict in field '{}': {} vs {}", conflict.field, conflict.a, conflict.b));
                            }
                        }
                    } else {
                        return Err(AscError::PolicyConflict(format!(
                            "{}: {} vs {}",
                            conflict.field, conflict.a, conflict.b
                        )));
                    }
                }
                ConflictSeverity::Warning => {
                    resolutions.push(format!(
                        "warning suppressed for field '{}': {} vs {}",
                        conflict.field, conflict.a, conflict.b
                    ));
                }
            }
        }

        Ok(resolutions)
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
            kernel_b: GeneratedKernelBConfig { workers: 16 },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: 2,
                max_workers: 8,
                desktop_min: 4,
                system_min: 1,
                orbit_default_max: 12,
                orbit_burst_max: 16,
                orbit_burst_window_ms: 200,
            },
            vrm: GeneratedVrmConfig {
                desktop_quota_mb: 4096,
                orbit_quota_mb: 2048,
                cache_mb: 1024,
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
                cache_mb: 128,
                buffer_mb: 64,
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
    fn test_detect_conflicts_finds_issues() {
        let config = make_config();
        let conflicts = ConflictResolver::detect_conflicts(&config);
        assert!(conflicts.len() >= 2);
    }

    #[test]
    fn test_resolve_in_place_safe_mode() {
        let mut config = make_config();
        let conflicts = ConflictResolver::detect_conflicts(&config);
        let result = ConflictResolver::resolve_in_place(&conflicts, &mut config, true);
        assert!(result.is_ok());
    }
}
