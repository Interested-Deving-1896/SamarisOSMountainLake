use crate::generator::generated_config::GeneratedConfig;

#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub section: String,
    pub field: String,
    pub before: String,
    pub after: String,
    pub changed: bool,
}

#[derive(Debug, Clone)]
pub struct ConfigDiff {
    pub entries: Vec<DiffEntry>,
}

impl ConfigDiff {
    pub fn between(a: &GeneratedConfig, b: &GeneratedConfig) -> Self {
        let mut entries = Vec::new();

        Self::compare_scalar(
            &mut entries, "kernel_b", "workers",
            &a.kernel_b.workers.to_string(),
            &b.kernel_b.workers.to_string(),
        );

        Self::compare_scalar(
            &mut entries, "worker_pool", "min_workers",
            &a.worker_pool.min_workers.to_string(),
            &b.worker_pool.min_workers.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "max_workers",
            &a.worker_pool.max_workers.to_string(),
            &b.worker_pool.max_workers.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "desktop_min",
            &a.worker_pool.desktop_min.to_string(),
            &b.worker_pool.desktop_min.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "system_min",
            &a.worker_pool.system_min.to_string(),
            &b.worker_pool.system_min.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "orbit_default_max",
            &a.worker_pool.orbit_default_max.to_string(),
            &b.worker_pool.orbit_default_max.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "orbit_burst_max",
            &a.worker_pool.orbit_burst_max.to_string(),
            &b.worker_pool.orbit_burst_max.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "worker_pool", "orbit_burst_window_ms",
            &a.worker_pool.orbit_burst_window_ms.to_string(),
            &b.worker_pool.orbit_burst_window_ms.to_string(),
        );

        Self::compare_scalar(
            &mut entries, "vrm", "desktop_quota_mb",
            &a.vrm.desktop_quota_mb.to_string(),
            &b.vrm.desktop_quota_mb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vrm", "orbit_quota_mb",
            &a.vrm.orbit_quota_mb.to_string(),
            &b.vrm.orbit_quota_mb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vrm", "cache_mb",
            &a.vrm.cache_mb.to_string(),
            &b.vrm.cache_mb.to_string(),
        );

        Self::compare_scalar(
            &mut entries, "vum", "cache_mb",
            &a.vum.cache_mb.to_string(),
            &b.vum.cache_mb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vum", "buffer_mb",
            &a.vum.buffer_mb.to_string(),
            &b.vum.buffer_mb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vum", "flush_interval_ms",
            &a.vum.flush_interval_ms.to_string(),
            &b.vum.flush_interval_ms.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vum", "batch_size_kb",
            &a.vum.batch_size_kb.to_string(),
            &b.vum.batch_size_kb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "vum", "journal_mode",
            &a.vum.journal_mode,
            &b.vum.journal_mode,
        );
        Self::compare_scalar(
            &mut entries, "vum", "prefetch_boot_assets",
            &a.vum.prefetch_boot_assets.to_string(),
            &b.vum.prefetch_boot_assets.to_string(),
        );

        Self::compare_scalar(
            &mut entries, "budget", "samaris_budget_cap_mb",
            &a.budget.samaris_budget_cap_mb.to_string(),
            &b.budget.samaris_budget_cap_mb.to_string(),
        );
        Self::compare_scalar(
            &mut entries, "budget", "allocated_total_mb",
            &a.budget.allocated_total_mb.to_string(),
            &b.budget.allocated_total_mb.to_string(),
        );

        Self { entries }
    }

    fn compare_scalar(
        entries: &mut Vec<DiffEntry>,
        section: &str,
        field: &str,
        before: &str,
        after: &str,
    ) {
        entries.push(DiffEntry {
            section: section.to_string(),
            field: field.to_string(),
            before: before.to_string(),
            after: after.to_string(),
            changed: before != after,
        });
    }

    pub fn has_changes(&self) -> bool {
        self.entries.iter().any(|e| e.changed)
    }

    pub fn changed_entries(&self) -> Vec<&DiffEntry> {
        self.entries.iter().filter(|e| e.changed).collect()
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# Config Diff\n\n");
        out.push_str("| Section | Field | Before | After |\n");
        out.push_str("|---------|-------|--------|-------|\n");

        for entry in &self.entries {
            if entry.changed {
                out.push_str(&format!(
                    "| {} | {} | {} | **{}** |\n",
                    entry.section, entry.field, entry.before, entry.after
                ));
            }
        }

        if !self.has_changes() {
            out.push_str("_No changes detected._\n");
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::generated_config::{
        GeneratedAscSection, GeneratedBudgetSection, GeneratedConfig, GeneratedKernelBConfig,
        GeneratedVrmConfig, GeneratedVumConfig, GeneratedWorkerPoolConfig,
    };

    fn base_config() -> GeneratedConfig {
        GeneratedConfig {
            kernel_b: GeneratedKernelBConfig { workers: 4 },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: 2, max_workers: 8, desktop_min: 1, system_min: 1,
                orbit_default_max: 4, orbit_burst_max: 8, orbit_burst_window_ms: 200,
            },
            vrm: GeneratedVrmConfig {
                desktop_quota_mb: 2048, orbit_quota_mb: 1024, cache_mb: 512,
                pressure_green_max_percent: 50, pressure_yellow_enter_percent: 50,
                pressure_yellow_exit_percent: 45, pressure_orange_enter_percent: 70,
                pressure_orange_exit_percent: 60, pressure_red_enter_percent: 85,
                pressure_red_exit_percent: 75, min_free_mb_yellow: 1024,
                min_free_mb_orange: 512, min_free_mb_red: 256,
            },
            vum: GeneratedVumConfig {
                cache_mb: 128, buffer_mb: 64, flush_interval_ms: 1000, batch_size_kb: 32,
                journal_mode: "delete".into(), prefetch_boot_assets: false,
            },
            budget: GeneratedBudgetSection {
                samaris_budget_cap_mb: 8192, allocated_total_mb: 4096, safety_margin_mb: 512,
            },
            asc: GeneratedAscSection {
                profile: "balanced".into(), machine_classes: vec![], generated_at: "now".into(), safe_mode: false,
            },
        }
    }

    #[test]
    fn test_diff_no_changes() {
        let a = base_config();
        let b = base_config();
        let diff = ConfigDiff::between(&a, &b);
        assert!(!diff.has_changes());
    }

    #[test]
    fn test_diff_detects_change() {
        let mut a = base_config();
        a.kernel_b.workers = 6;
        let b = base_config();
        let diff = ConfigDiff::between(&a, &b);
        assert!(diff.has_changes());
        assert_eq!(diff.changed_entries().len(), 1);
    }
}
