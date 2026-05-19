#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct MetricsSnapshot {
    pub uptime_ms: u64,
    pub total_allocated: u64,
    pub total_freed: u64,
    pub allocation_count: u64,
    pub free_count: u64,
    pub active_apps: u32,
}
