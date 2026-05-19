use serde::Serialize;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct GpuMetricsSnapshot {
    pub gpu_enabled: bool,
    pub backend: String,
    pub device_count: u32,
    pub active_device: u32,
    pub vram_total_bytes: u64,
    pub vram_used_bytes: u64,
    pub t1_active_bytes: u64,
    pub t2_compressed_bytes: u64,
    pub t2_original_bytes: u64,
    pub t3_fallback_bytes: u64,
    pub compression_count: u64,
    pub decompression_count: u64,
    pub compression_saved_bytes: u64,
    pub average_compression_ratio: f64,
    pub compression_latency_avg_us: u64,
    pub restore_latency_avg_us: u64,
    pub resource_count: u64,
    pub dedup_hit_count: u64,
    pub dedup_miss_count: u64,
    pub dedup_saved_bytes: u64,
    pub eviction_count: u64,
    pub restore_count: u64,
    pub shader_cache_entries: u64,
    pub shader_cache_hit_count: u64,
    pub shader_cache_miss_count: u64,
    pub shader_compile_count: u64,
    pub shader_compile_error_count: u64,
    pub frame_count: u64,
    pub average_frame_time_ms: f64,
    pub frame_budget_miss_count: u64,
    pub compute_job_count: u64,
    pub queued_gpu_jobs: u64,
    pub gpu_job_error_count: u64,
    pub thermal_state: String,
    pub thermal_backoff_events: u64,
    pub fallback_count: u64,
    pub sbp_request_count: u64,
    pub sbp_error_count: u64,
    #[serde(skip)]
    pub timestamp: Instant,
}

impl Default for GpuMetricsSnapshot {
    fn default() -> Self {
        Self {
            gpu_enabled: false,
            backend: String::new(),
            device_count: 0,
            active_device: 0,
            vram_total_bytes: 0,
            vram_used_bytes: 0,
            t1_active_bytes: 0,
            t2_compressed_bytes: 0,
            t2_original_bytes: 0,
            t3_fallback_bytes: 0,
            compression_count: 0,
            decompression_count: 0,
            compression_saved_bytes: 0,
            average_compression_ratio: 0.0,
            compression_latency_avg_us: 0,
            restore_latency_avg_us: 0,
            resource_count: 0,
            dedup_hit_count: 0,
            dedup_miss_count: 0,
            dedup_saved_bytes: 0,
            eviction_count: 0,
            restore_count: 0,
            shader_cache_entries: 0,
            shader_cache_hit_count: 0,
            shader_cache_miss_count: 0,
            shader_compile_count: 0,
            shader_compile_error_count: 0,
            frame_count: 0,
            average_frame_time_ms: 0.0,
            frame_budget_miss_count: 0,
            compute_job_count: 0,
            queued_gpu_jobs: 0,
            gpu_job_error_count: 0,
            thermal_state: "unknown".into(),
            thermal_backoff_events: 0,
            fallback_count: 0,
            sbp_request_count: 0,
            sbp_error_count: 0,
            timestamp: Instant::now(),
        }
    }
}

impl GpuMetricsSnapshot {
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn into_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_snapshot() {
        let snap = GpuMetricsSnapshot::default();
        assert!(!snap.gpu_enabled);
        assert_eq!(snap.vram_total_bytes, 0);
    }

    #[test]
    fn test_into_json() {
        let snap = GpuMetricsSnapshot::default();
        let json = snap.into_json();
        assert!(json.is_object());
        assert_eq!(json["gpu_enabled"], serde_json::Value::Bool(false));
    }

    #[test]
    fn test_timestamp() {
        let snap = GpuMetricsSnapshot::default();
        let _ts = snap.timestamp();
    }

    #[test]
    fn test_serialize_all_fields() {
        let snap = GpuMetricsSnapshot {
            gpu_enabled: true,
            backend: "wgpu".into(),
            device_count: 2,
            vram_total_bytes: 8 << 30,
            vram_used_bytes: 1 << 30,
            compression_count: 42,
            dedup_hit_count: 99,
            frame_count: 1000,
            average_frame_time_ms: 8.3,
            thermal_state: "normal".into(),
            ..Default::default()
        };
        let json = serde_json::to_value(&snap).unwrap();
        assert_eq!(json["backend"], "wgpu");
        assert_eq!(json["device_count"], 2);
        assert_eq!(json["compression_count"], 42);
        assert_eq!(json["frame_count"], 1000);
        assert_eq!(json["average_frame_time_ms"], 8.3);
    }

}
