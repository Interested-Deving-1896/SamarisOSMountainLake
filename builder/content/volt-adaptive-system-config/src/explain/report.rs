use crate::budget::system_budget::SystemBudget;
use crate::explain::decision::ExplainDecision;
use crate::explain::reason::{
    budget_reason, cpu_worker_reason, orbit_burst_reason, ram_quota_reason,
    storage_cache_reason,
};
use crate::generator::generated_config::GeneratedConfig;
use crate::hardware::profile::HardwareProfile;

use crate::classify::machine_class::MachineClass;

#[derive(Debug, Clone)]
pub struct ExplainReport {
    pub generated_at: String,
    pub hardware: HardwareProfile,
    pub machine_classes: Vec<MachineClass>,
    pub decisions: Vec<ExplainDecision>,
    pub warnings: Vec<String>,
}

fn format_now() -> String {
    let now = time::OffsetDateTime::now_utc();
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        now.year(),
        now.month() as u8,
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
    )
}

impl ExplainReport {
    pub fn new(
        hw: &HardwareProfile,
        classes: &[MachineClass],
        budget: Option<&SystemBudget>,
        generated: Option<&GeneratedConfig>,
    ) -> Self {
        let generated_at = format_now();

        let mut decisions = Vec::new();
        let mut warnings = Vec::new();

        if let (Some(_budget), Some(generated)) = (budget, generated) {
            decisions.push(ExplainDecision::new(
                "Kernel B workers",
                generated.kernel_b.workers.to_string(),
                cpu_worker_reason(hw.cpu_cores, generated.kernel_b.workers),
            ));

            decisions.push(ExplainDecision::new(
                "Desktop quota",
                format!("{} MB", generated.vrm.desktop_quota_mb),
                ram_quota_reason(hw.ram_total_mb, generated.vrm.desktop_quota_mb, "desktop"),
            ));

            decisions.push(ExplainDecision::new(
                "Orbit quota",
                format!("{} MB", generated.vrm.orbit_quota_mb),
                ram_quota_reason(hw.ram_total_mb, generated.vrm.orbit_quota_mb, "orbit"),
            ));

            decisions.push(ExplainDecision::new(
                "Worker pool min",
                generated.worker_pool.min_workers.to_string(),
                format!(
                    "{} CPU cores → {} minimum workers",
                    hw.cpu_cores, generated.worker_pool.min_workers
                ),
            ));

            decisions.push(ExplainDecision::new(
                "Worker pool max",
                generated.worker_pool.max_workers.to_string(),
                format!(
                    "{} CPU cores → {} maximum workers",
                    hw.cpu_cores, generated.worker_pool.max_workers
                ),
            ));

            decisions.push(ExplainDecision::new(
                "VRM cache",
                format!("{} MB", generated.vrm.cache_mb),
                storage_cache_reason("system", generated.vrm.cache_mb),
            ));

            decisions.push(ExplainDecision::new(
                "VUM cache",
                format!("{} MB", generated.vum.cache_mb),
                storage_cache_reason("system", generated.vum.cache_mb),
            ));

            decisions.push(ExplainDecision::new(
                "VUM journal mode",
                generated.vum.journal_mode.clone(),
                format!("{} boot → journal mode: {}", hw.boot_medium.name(), generated.vum.journal_mode),
            ));

            decisions.push(ExplainDecision::new(
                "Orbit burst window",
                format!("{} ms", generated.worker_pool.orbit_burst_window_ms),
                orbit_burst_reason(hw.is_laptop, generated.worker_pool.orbit_burst_window_ms),
            ));

            decisions.push(ExplainDecision::new(
                "Budget allocation",
                format!("{} MB allocated / {} MB cap", generated.budget.allocated_total_mb, generated.budget.samaris_budget_cap_mb),
                budget_reason(generated.budget.allocated_total_mb, generated.budget.samaris_budget_cap_mb),
            ));

            if generated.vrm.desktop_quota_mb < 64 {
                warnings.push("Desktop quota is critically low (< 64 MB)".into());
            }
            if generated.worker_pool.min_workers == 0 {
                warnings.push("Minimum workers is zero — system may stall".into());
            }
            if generated.asc.safe_mode {
                warnings.push("System is running in SAFE MODE — some features limited".into());
            }
        } else {
            warnings.push("No budget or generated config available — showing hardware info only".into());
        }

        Self {
            generated_at,
            hardware: hw.clone(),
            machine_classes: classes.to_vec(),
            decisions,
            warnings,
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("# Volt ASC Explain Report\n\n");

        out.push_str("## Overview\n\n");
        out.push_str(&format!("- **Generated at:** {}\n", self.generated_at));
        out.push_str(&format!("- **Profile:** {}\n", self.decisions.first().map_or("unknown", |_| "configured")));
        out.push_str("\n");

        out.push_str("## Detected Hardware\n\n");
        out.push_str(&format!("- CPU cores: {}\n", self.hardware.cpu_cores));
        let ram_gb = self.hardware.ram_total_mb as f64 / 1024.0;
        out.push_str(&format!("- RAM: {} MB ({:.1} GB)\n", self.hardware.ram_total_mb, ram_gb));
        out.push_str(&format!("- Laptop: {}\n", self.hardware.is_laptop));
        out.push_str(&format!("- VM: {}\n", self.hardware.is_vm));
        out.push_str(&format!("- Boot medium: {}\n", self.hardware.boot_medium.name()));
        out.push_str(&format!("- Storage: {}\n", self.hardware.storage_type.name()));
        out.push_str(&format!("- GPU available: {}\n", self.hardware.gpu_available));
        out.push_str("\n");

        out.push_str("## Machine Classes\n\n");
        for mc in &self.machine_classes {
            out.push_str(&format!("- {}\n", mc.name()));
        }
        out.push_str("\n");

        out.push_str("## Generated Decisions\n\n");
        out.push_str("| Parameter | Value | Reason |\n");
        out.push_str("|-----------|-------|--------|\n");
        for d in &self.decisions {
            let value = if d.was_clamped {
                format!("~~{}~~ → {} (clamped)", d.original_value.as_deref().unwrap_or(""), d.value)
            } else {
                d.value.clone()
            };
            out.push_str(&format!("| {} | {} | {} |\n", d.parameter, value, d.reason));
        }
        out.push_str("\n");

        if !self.warnings.is_empty() {
            out.push_str("## Warnings\n\n");
            for w in &self.warnings {
                out.push_str(&format!("- ⚠ {}\n", w));
            }
            out.push_str("\n");
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::budget::system_budget::SystemBudget;
    use crate::generator::generated_config::{
        GeneratedAscSection, GeneratedBudgetSection, GeneratedConfig, GeneratedKernelBConfig,
        GeneratedVrmConfig, GeneratedVumConfig, GeneratedWorkerPoolConfig,
    };
    use crate::hardware::profile::{BootMedium, HardwareProfile, StorageType};

    fn test_hw() -> HardwareProfile {
        HardwareProfile {
            cpu_cores: 8,
            cpu_threads: 8,
            cpu_model: "test".into(),
            cpu_arch: "x86_64".into(),
            ram_total_mb: 16384,
            ram_available_mb: 8192,
            swap_total_mb: 4096,
            is_laptop: true,
            is_vm: false,
            boot_medium: BootMedium::InternalDisk,
            storage_type: StorageType::Nvme,
            usb_speed: None,
            gpu_available: true,
            gpu_vendor: Some("test".into()),
            gpu_model: Some("test".into()),
            gpu_memory_mb: Some(2048),
            battery_present: true,
            thermal_available: true,
            confidence: crate::hardware::confidence::DetectionConfidence::new(true),
        }
    }

    fn test_config() -> GeneratedConfig {
        GeneratedConfig {
            kernel_b: GeneratedKernelBConfig { workers: 6 },
            worker_pool: GeneratedWorkerPoolConfig {
                min_workers: 2,
                max_workers: 16,
                desktop_min: 2,
                system_min: 1,
                orbit_default_max: 4,
                orbit_burst_max: 8,
                orbit_burst_window_ms: 600,
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
                cache_mb: 256,
                buffer_mb: 128,
                flush_interval_ms: 1000,
                batch_size_kb: 64,
                journal_mode: "wal".into(),
                prefetch_boot_assets: false,
            },
            budget: GeneratedBudgetSection {
                samaris_budget_cap_mb: 8192,
                allocated_total_mb: 4096,
                safety_margin_mb: 512,
            },
            asc: GeneratedAscSection {
                profile: "balanced".into(),
                machine_classes: vec!["general".into()],
                generated_at: "now".into(),
                safe_mode: false,
            },
        }
    }

    #[test]
    fn test_report_renders_markdown() {
        let hw = test_hw();
        let budget = SystemBudget::default();
        let config = test_config();
        let classes = crate::classify::machine_class::classify(&hw);
        let report = ExplainReport::new(&hw, &classes, Some(&budget), Some(&config));
        let rendered = report.render();
        assert!(rendered.contains("# Volt ASC Explain Report"));
        assert!(rendered.contains("## Detected Hardware"));
        assert!(rendered.contains("## Generated Decisions"));
        assert!(rendered.contains("Kernel B workers"));
    }
}
