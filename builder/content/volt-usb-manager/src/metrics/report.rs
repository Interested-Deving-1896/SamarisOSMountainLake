use crate::metrics::snapshot::MetricsSnapshot;

pub struct MetricsReport;

impl MetricsReport {
    pub fn generate(snapshot: &MetricsSnapshot) -> String {
        format!(
            "=== Volt USB Manager Metrics Report ===\n\
             Uptime: {} ms\n\
             State: {:?}\n\
             Device present: {} | Read-only: {} | Mounted: {} | Journal dirty: {}\n\
             Cache: {}/{} bytes ({} entries, hit={}, miss={}, evict={}, ratio={:.2}%)\n\
             Write buffer: {}/{} bytes ({} pending, {} dirty bytes)\n\
             Flush: {} ok, {} errors (last: {} us)\n\
             Read: {} logical / {} physical bytes\n\
             Write: {} logical / {} physical bytes\n\
             Compression: {} ops, decompression: {} ops, saved {} bytes\n\
             Journal: {} records, {} bytes, {} replays\n\
             ACKs: {} buffered, {} durable\n\
             Eject: {} total, {} unsafe blocked, {} removals\n\
             SBP requests: {}, errors: {}\n\
             Avg latency: read {} us, write {} us, flush {} us",
            snapshot.uptime_ms,
            snapshot.state,
            snapshot.device_present,
            snapshot.read_only,
            snapshot.mounted,
            snapshot.journal_dirty,
            snapshot.cache_used_bytes,
            snapshot.cache_max_bytes,
            snapshot.cache_entries,
            snapshot.cache_hit_count,
            snapshot.cache_miss_count,
            snapshot.cache_eviction_count,
            snapshot.cache_hit_ratio * 100.0,
            snapshot.write_buffer_used_bytes,
            snapshot.write_buffer_max_bytes,
            snapshot.pending_write_count,
            snapshot.dirty_bytes,
            snapshot.flush_count,
            snapshot.flush_error_count,
            snapshot.last_flush_duration_us,
            snapshot.bytes_read_logical,
            snapshot.bytes_read_physical,
            snapshot.bytes_written_logical,
            snapshot.bytes_written_physical,
            snapshot.compression_count,
            snapshot.decompression_count,
            snapshot.compression_saved_bytes,
            snapshot.journal_records,
            snapshot.journal_bytes,
            snapshot.journal_replay_count,
            snapshot.ack_buffered_count,
            snapshot.ack_durable_count,
            snapshot.eject_count,
            snapshot.unsafe_eject_blocked_count,
            snapshot.device_removed_count,
            snapshot.sbp_request_count,
            snapshot.sbp_error_count,
            snapshot.avg_read_latency_us,
            snapshot.avg_write_latency_us,
            snapshot.avg_flush_latency_us,
        )
    }

    pub fn generate_short(snapshot: &MetricsSnapshot) -> String {
        format!(
            "[{:?}] cache={:.1}% pending={}B fl={}/{} lat={}/{}/{}us jrnl={} eject={}",
            snapshot.state,
            snapshot.cache_hit_ratio * 100.0,
            snapshot.dirty_bytes,
            snapshot.flush_count,
            snapshot.flush_error_count,
            snapshot.avg_read_latency_us,
            snapshot.avg_write_latency_us,
            snapshot.avg_flush_latency_us,
            snapshot.journal_records,
            snapshot.eject_count,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::VumState;

    fn sample_snapshot() -> MetricsSnapshot {
        MetricsSnapshot {
            uptime_ms: 5000,
            state: VumState::Running,
            device_present: true,
            cache_hit_count: 100,
            cache_miss_count: 20,
            cache_hit_ratio: 100.0 / 120.0,
            ..Default::default()
        }
    }

    #[test]
    fn test_generate_contains_state() {
        let s = sample_snapshot();
        let report = MetricsReport::generate(&s);
        assert!(report.contains("Running"));
    }

    #[test]
    fn test_generate_contains_uptime() {
        let s = sample_snapshot();
        let report = MetricsReport::generate(&s);
        assert!(report.contains("5000 ms"));
    }

    #[test]
    fn test_generate_short_is_compact() {
        let s = sample_snapshot();
        let short = MetricsReport::generate_short(&s);
        assert!(short.len() < 120);
    }

    #[test]
    fn test_generate_short_contains_hit_ratio() {
        let s = sample_snapshot();
        let short = MetricsReport::generate_short(&s);
        assert!(short.contains("cache="));
    }

    #[test]
    fn test_generate_empty_snapshot() {
        let s = MetricsSnapshot::default();
        let report = MetricsReport::generate(&s);
        assert!(!report.is_empty());
    }
}
