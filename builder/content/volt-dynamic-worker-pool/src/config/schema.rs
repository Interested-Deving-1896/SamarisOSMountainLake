use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct WorkerPoolConfig {
    pub worker_pool: WorkerPoolSection,
    pub adapters: AdaptersSection,
    pub metrics: MetricsSection,
}

impl Default for WorkerPoolConfig {
    fn default() -> Self {
        crate::config::defaults::default_config()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct WorkerPoolSection {
    pub scheduler: String,
    pub preemption_enabled: bool,
    pub yield_budget_us: u64,
    pub idle_timeout_ms: u64,
    pub max_workers_cap: u64,
    pub safe_mode: bool,
    pub integration_mode: String,
    pub scaling: ScalingSection,
    pub hardware: HardwareSection,
    pub reservations: ReservationsSection,
    pub desktop_guard: DesktopGuardSection,
    pub priorities: PrioritiesSection,
    pub fairness: FairnessSection,
    pub thermal: ThermalSection,
}

impl Default for WorkerPoolSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ScalingSection {
    pub mode: String,
    pub scale_up_queue_factor: f64,
    pub scale_down_queue_factor: f64,
    pub scale_up_cpu_threshold: f64,
    pub scale_down_cpu_threshold: f64,
    pub scale_cooldown_ms: u64,
    pub min_workers_override: Option<u64>,
    pub max_workers_override: Option<u64>,
}

impl Default for ScalingSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.scaling
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HardwareSection {
    pub detect_cpu_cores: bool,
    pub detect_ram: bool,
    pub detect_thermal: bool,
    pub default_cpu_cores: u64,
}

#[derive(Debug, Clone)]
pub struct HardwareConfig {
    pub default_cpu_cores: u32,
    pub min_workers_override: u32,
    pub max_workers_override: u32,
    pub ram_bytes: u64,
}

impl Default for HardwareConfig {
    fn default() -> Self {
        HardwareConfig {
            default_cpu_cores: 4,
            min_workers_override: 0,
            max_workers_override: 0,
            ram_bytes: 8_589_934_592,
        }
    }
}

impl From<&HardwareSection> for HardwareConfig {
    fn from(section: &HardwareSection) -> Self {
        HardwareConfig {
            default_cpu_cores: section.default_cpu_cores as u32,
            min_workers_override: 0,
            max_workers_override: 0,
            ram_bytes: if section.detect_ram { 0 } else { 0 },
        }
    }
}

impl Default for HardwareSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.hardware
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReservationsSection {
    pub desktop_min_workers: u64,
    pub system_min_workers: u64,
    pub orbit_default_fraction: f64,
    pub orbit_burst_window_ms: u64,
    pub orbit_burst_cooldown_ms: u64,
    pub orbit_max_consecutive_bursts: u64,
}

impl Default for ReservationsSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.reservations
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DesktopGuardSection {
    pub enabled: bool,
    pub frame_budget_ms: u64,
    pub latency_guard_ms: u64,
    pub reduce_orbit_on_frame_pressure: bool,
    pub reduce_background_on_frame_pressure: bool,
}

impl Default for DesktopGuardSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.desktop_guard
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PrioritiesSection {
    pub orbit: String,
    pub desktop: String,
    pub electron: String,
    pub kernel_a: String,
    pub kernel_b: String,
    pub vrm: String,
    pub vum: String,
    pub vgm: String,
    pub background: String,
}

impl Default for PrioritiesSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.priorities
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FairnessSection {
    pub aging_enabled: bool,
    pub aging_after_ms: u64,
    pub starvation_limit_ms: u64,
    pub priority_boost_on_starvation: bool,
}

impl Default for FairnessSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.fairness
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ThermalSection {
    pub thermal_backoff_enabled: bool,
    pub scale_down_on_thermal_pressure: bool,
    pub disable_orbit_burst_on_thermal_pressure: bool,
}

impl Default for ThermalSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.worker_pool.thermal
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AdaptersSection {
    pub enabled: bool,
    pub kernel_b: String,
    pub kernel_a: String,
    pub desktop: String,
    pub orbit: String,
    pub vrm: String,
    pub vum: String,
    pub vgm: String,
}

impl Default for AdaptersSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.adapters
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MetricsSection {
    pub enabled: bool,
    pub latency_histograms: bool,
    pub utilization_tracking: bool,
    pub queue_tracking: bool,
}

impl Default for MetricsSection {
    fn default() -> Self {
        let d = crate::config::defaults::default_config();
        d.metrics
    }
}
