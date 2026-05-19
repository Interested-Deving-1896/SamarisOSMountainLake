use crate::metrics::snapshot::GpuMetricsSnapshot;

pub struct GpuMetricsReport;

impl GpuMetricsReport {
    pub fn generate(snapshot: &GpuMetricsSnapshot) -> String {
        format!(
            "GpuMetricsReport:\n\
             enabled={} backend={} devices={}\n\
             vram: {} used / {} total\n\
             compression: {} ops, ratio={:.2}, saved={} bytes\n\
             shader cache: {} entries, {} hits, {} misses\n\
             frames: {} total, avg {:.1}ms, {} budget misses\n\
             compute: {} jobs queued, {} errors\n\
             thermal: {} backoffs={}\n\
             sbp: {} requests, {} errors\n\
             dedup: {} hits, {} misses, saved={} bytes\n\
             fallbacks={}",
            snapshot.gpu_enabled,
            snapshot.backend,
            snapshot.device_count,
            format_bytes(snapshot.vram_used_bytes),
            format_bytes(snapshot.vram_total_bytes),
            snapshot.compression_count,
            snapshot.average_compression_ratio,
            snapshot.compression_saved_bytes,
            snapshot.shader_cache_entries,
            snapshot.shader_cache_hit_count,
            snapshot.shader_cache_miss_count,
            snapshot.frame_count,
            snapshot.average_frame_time_ms,
            snapshot.frame_budget_miss_count,
            snapshot.compute_job_count,
            snapshot.gpu_job_error_count,
            snapshot.thermal_state,
            snapshot.thermal_backoff_events,
            snapshot.sbp_request_count,
            snapshot.sbp_error_count,
            snapshot.dedup_hit_count,
            snapshot.dedup_miss_count,
            snapshot.dedup_saved_bytes,
            snapshot.fallback_count,
        )
    }

    pub fn generate_json(snapshot: &GpuMetricsSnapshot) -> String {
        serde_json::to_string_pretty(snapshot).unwrap_or_default()
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes >= 1 << 30 {
        format!("{:.1} GiB", bytes as f64 / (1 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.1} MiB", bytes as f64 / (1 << 20) as f64)
    } else if bytes >= 1 << 10 {
        format!("{:.1} KiB", bytes as f64 / (1 << 10) as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_report() {
        let snap = GpuMetricsSnapshot::default();
        let report = GpuMetricsReport::generate(&snap);
        assert!(report.contains("GpuMetricsReport"));
        assert!(report.contains("enabled=false"));
    }

    #[test]
    fn test_generate_json() {
        let snap = GpuMetricsSnapshot::default();
        let json = GpuMetricsReport::generate_json(&snap);
        assert!(json.contains("gpu_enabled"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.0 KiB");
        assert_eq!(format_bytes(1048576), "1.0 MiB");
        assert_eq!(format_bytes(1073741824), "1.0 GiB");
    }

    #[test]
    fn test_report_with_values() {
        let snap = GpuMetricsSnapshot {
            gpu_enabled: true,
            backend: "vulkan".into(),
            frame_count: 500,
            ..Default::default()
        };
        let report = GpuMetricsReport::generate(&snap);
        assert!(report.contains("backend=vulkan"));
        assert!(report.contains("500"));
    }
}
