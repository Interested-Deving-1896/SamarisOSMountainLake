use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AscConfig {
    #[serde(default)]
    pub adaptive: AdaptiveConfig,
    #[serde(default)]
    pub hardware: HardwareOverrideConfig,
    #[serde(default)]
    pub budget: BudgetConfig,
    #[serde(default)]
    pub profiles: ProfilesConfig,
    #[serde(default)]
    pub kernel_b: KernelBAdaptiveConfig,
    #[serde(default)]
    pub worker_pool: WorkerPoolAdaptiveConfig,
    #[serde(default)]
    pub vrm: VrmAdaptiveConfig,
    #[serde(default)]
    pub vum: VumAdaptiveConfig,
    #[serde(default)]
    pub explain: ExplainConfig,
    #[serde(default)]
    pub output: OutputConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AdaptiveConfig {
    pub enabled: bool,
    pub mode: String,
    pub profile: String,
    pub safe_mode: bool,
    pub explain: bool,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: "auto".into(),
            profile: "balanced".into(),
            safe_mode: true,
            explain: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HardwareOverrideConfig {
    pub cpu_cores: String,
    pub ram_total_mb: String,
    pub gpu_available: String,
    pub usb_speed: String,
    pub storage_type: String,
    pub boot_medium: String,
    pub is_vm: String,
    pub is_laptop: String,
}

impl Default for HardwareOverrideConfig {
    fn default() -> Self {
        Self {
            cpu_cores: "auto".into(),
            ram_total_mb: "auto".into(),
            gpu_available: "auto".into(),
            usb_speed: "auto".into(),
            storage_type: "auto".into(),
            boot_medium: "auto".into(),
            is_vm: "auto".into(),
            is_laptop: "auto".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BudgetConfig {
    pub enabled: bool,
    pub safety_margin_percent: String,
    pub max_samaris_ram_percent: String,
    pub protect_desktop: bool,
    pub reduce_orbit_before_desktop: bool,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            safety_margin_percent: "10".into(),
            max_samaris_ram_percent: "50".into(),
            protect_desktop: true,
            reduce_orbit_before_desktop: true,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProfilesConfig {
    pub available: Vec<String>,
}

impl Default for ProfilesConfig {
    fn default() -> Self {
        Self {
            available: vec![
                "balanced".into(),
                "performance".into(),
                "conservative".into(),
                "laptop".into(),
                "desktop".into(),
                "minimal".into(),
            ],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KernelBAdaptiveConfig {
    pub enabled: bool,
    pub workers: String,
}

impl Default for KernelBAdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            workers: "auto".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorkerPoolAdaptiveConfig {
    pub min_workers: String,
    pub max_workers: String,
    pub desktop_min: String,
    pub system_min: String,
    pub orbit_default_max: String,
    pub orbit_burst_max: String,
    pub orbit_burst_window_ms: String,
}

impl Default for WorkerPoolAdaptiveConfig {
    fn default() -> Self {
        Self {
            min_workers: "auto".into(),
            max_workers: "auto".into(),
            desktop_min: "auto".into(),
            system_min: "auto".into(),
            orbit_default_max: "auto".into(),
            orbit_burst_max: "auto".into(),
            orbit_burst_window_ms: "auto".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VrmAdaptiveConfig {
    pub enabled: bool,
    pub desktop_quota_mb: String,
    pub orbit_quota_mb: String,
    pub cache_mb: String,
    pub pressure_policy: String,
    pub compression_level: String,
}

impl Default for VrmAdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            desktop_quota_mb: "auto".into(),
            orbit_quota_mb: "auto".into(),
            cache_mb: "256".into(),
            pressure_policy: "balanced".into(),
            compression_level: "3".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VumAdaptiveConfig {
    pub enabled: bool,
    pub cache_mb: String,
    pub buffer_mb: String,
    pub flush_interval_ms: String,
    pub batch_size_kb: String,
    pub journal_mode: String,
    pub prefetch_boot_assets: String,
}

impl Default for VumAdaptiveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_mb: "128".into(),
            buffer_mb: "64".into(),
            flush_interval_ms: "5000".into(),
            batch_size_kb: "64".into(),
            journal_mode: "wal".into(),
            prefetch_boot_assets: "true".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExplainConfig {
    pub write_report: bool,
    pub report_path: String,
}

impl Default for ExplainConfig {
    fn default() -> Self {
        Self {
            write_report: false,
            report_path: "/var/lib/samaris/asc/last-explain-report.md".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OutputConfig {
    pub generated_config_path: String,
    pub hardware_profile_path: String,
    pub last_generated_config_path: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            generated_config_path: "/run/samaris/adaptive.generated.toml".into(),
            hardware_profile_path: "/var/lib/samaris/asc/last-hardware-profile.json".into(),
            last_generated_config_path: "/var/lib/samaris/asc/last-generated-config.toml".into(),
        }
    }
}
