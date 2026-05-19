use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TesseractConfig {
    pub socket_path: String,
    pub websocket_port: u16,
    pub debug_mode: bool,
    pub max_workers: usize,
    pub thermal_throttle_celsius: f64,
    pub thermal_emergency_celsius: f64,
    pub thermal_critical_celsius: f64,
    pub metrics_interval_ms: u64,
    pub watchdog_interval_ms: u64,
    pub scheduler_tick_ms: u64,
    pub max_total_memory_mb: u64,
    pub max_open_sockets: u32,
    pub max_concurrent_tasks: u32,
    pub audit_max_entries: usize,
    pub quota_default_max_memory_mb: u64,
    pub quota_default_max_tasks: u32,
    pub quota_default_max_commands_per_sec: u32,
}

impl Default for TesseractConfig {
    fn default() -> Self {
        Self {
            socket_path: "/run/samaris/volt-kernel-b.sock".into(),
            websocket_port: 9998,
            debug_mode: false,
            max_workers: 4,
            thermal_throttle_celsius: 85.0,
            thermal_emergency_celsius: 95.0,
            thermal_critical_celsius: 100.0,
            metrics_interval_ms: 1000,
            watchdog_interval_ms: 500,
            scheduler_tick_ms: 1,
            max_total_memory_mb: 1024,
            max_open_sockets: 64,
            max_concurrent_tasks: 16,
            audit_max_entries: 10_000,
            quota_default_max_memory_mb: 256,
            quota_default_max_tasks: 4,
            quota_default_max_commands_per_sec: 100,
        }
    }
}

impl TesseractConfig {
    pub fn load(path: &str) -> Result<Self, TesseractConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| TesseractConfigError::Io(format!("cannot read {path}: {e}")))?;
        let config: TesseractConfig = toml::from_str(&content)
            .map_err(|e| TesseractConfigError::Parse(e.to_string()))?;
        Ok(config)
    }

    pub fn load_or_default(path: &str) -> Self {
        Self::load(path).unwrap_or_else(|e| {
            tracing::warn!("config load failed ({e}), using defaults");
            Self::default()
        })
    }
}

#[derive(Debug)]
pub enum TesseractConfigError {
    Io(String),
    Parse(String),
}

impl std::fmt::Display for TesseractConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "config io: {msg}"),
            Self::Parse(msg) => write!(f, "config parse: {msg}"),
        }
    }
}

impl std::error::Error for TesseractConfigError {}
