use crate::metrics::snapshot::MetricsSnapshot;

pub fn format_status(snapshot: &MetricsSnapshot) -> String {
    format!(
        "Volt USB Manager Status\n\
         Uptime: {}ms\n\
         State: {:?}\n\
         Device Present: {}\n\
         Read Only: {}\n\
         Mounted: {}\n\
         Journal Dirty: {}\n\
         Recovery Required: {}\n\
         Cache: {}/{} bytes ({} entries, hit ratio: {:.2}%)\n\
         Write Buffer: {}/{} bytes ({} pending writes, {} dirty)\n\
         Flush: {} ok, {} errors (last: {}us)\n\
         IO: {}R/{}W logical, {}R/{}W physical\n\
         Compress: {} ops / {} ops, saved {} bytes\n\
         Journal: {} records, {} bytes, {} replays\n\
         ACKs: {} buffered, {} durable\n\
         Eject: {} total, {} blocked unsafe, {} removals\n\
         SBP: {} requests, {} errors\n\
         Latency: avg read {}us, avg write {}us, avg flush {}us",
        snapshot.uptime_ms,
        snapshot.state,
        snapshot.device_present,
        snapshot.read_only,
        snapshot.mounted,
        snapshot.journal_dirty,
        snapshot.recovery_required,
        snapshot.cache_used_bytes,
        snapshot.cache_max_bytes,
        snapshot.cache_entries,
        snapshot.cache_hit_ratio * 100.0,
        snapshot.write_buffer_used_bytes,
        snapshot.write_buffer_max_bytes,
        snapshot.pending_write_count,
        snapshot.dirty_bytes,
        snapshot.flush_count,
        snapshot.flush_error_count,
        snapshot.last_flush_duration_us,
        snapshot.bytes_read_logical,
        snapshot.bytes_written_logical,
        snapshot.bytes_read_physical,
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

pub fn format_short(snapshot: &MetricsSnapshot) -> String {
    format!(
        "State: {:?} | Cache: {:.1}% hit ({}/{}) | Pending: {} writes/{}B | \
         Flush: {}/{} err | Lat: {}r/{}w/{}f us | Journal: {} rec | Eject: {}",
        snapshot.state,
        snapshot.cache_hit_ratio * 100.0,
        snapshot.cache_hit_count,
        snapshot.cache_miss_count,
        snapshot.pending_write_count,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::state::VumState;
    use crate::metrics::snapshot::MetricsSnapshot;

    fn test_snapshot() -> MetricsSnapshot {
        MetricsSnapshot {
            uptime_ms: 12345,
            state: VumState::Running,
            device_present: true,
            read_only: false,
            mounted: true,
            journal_dirty: false,
            recovery_required: false,
            cache_max_bytes: 10_000_000,
            cache_used_bytes: 4_000_000,
            cache_entries: 142,
            cache_hit_count: 900,
            cache_miss_count: 100,
            cache_eviction_count: 23,
            cache_hit_ratio: 0.9,
            write_buffer_max_bytes: 5_000_000,
            write_buffer_used_bytes: 1_000_000,
            pending_write_count: 7,
            dirty_bytes: 4096,
            flush_count: 50,
            flush_error_count: 2,
            last_flush_duration_us: 1500,
            bytes_read_logical: 1_000_000,
            bytes_read_physical: 1_200_000,
            bytes_written_logical: 500_000,
            bytes_written_physical: 550_000,
            compression_count: 100,
            decompression_count: 80,
            compression_saved_bytes: 200_000,
            journal_records: 3000,
            journal_bytes: 1_000_000,
            journal_replay_count: 1,
            ack_buffered_count: 450,
            ack_durable_count: 440,
            eject_count: 12,
            unsafe_eject_blocked_count: 2,
            device_removed_count: 3,
            sbp_request_count: 5000,
            sbp_error_count: 8,
            avg_read_latency_us: 120,
            avg_write_latency_us: 350,
            avg_flush_latency_us: 2200,
        }
    }

    #[test]
    fn test_format_status_contains_state() {
        let s = test_snapshot();
        let output = format_status(&s);
        assert!(output.contains("Running"));
        assert!(output.contains("12345ms"));
        assert!(output.contains("Cache"));
        assert!(output.contains("Flush"));
    }

    #[test]
    fn test_format_status_contains_cache_hit_rate() {
        let s = test_snapshot();
        let output = format_status(&s);
        assert!(output.contains("90.00%"));
    }

    #[test]
    fn test_format_short_contains_state() {
        let s = test_snapshot();
        let output = format_short(&s);
        assert!(output.contains("Running"));
        assert!(output.contains("90.0%"));
    }

    #[test]
    fn test_format_short_is_shorter_than_full() {
        let s = test_snapshot();
        let full = format_status(&s);
        let short = format_short(&s);
        assert!(short.len() < full.len());
    }

    #[test]
    fn test_format_short_contains_latency() {
        let s = test_snapshot();
        let output = format_short(&s);
        assert!(output.contains("120r"));
        assert!(output.contains("350w"));
    }

    #[test]
    fn test_format_status_zero_values() {
        let s = MetricsSnapshot::default();
        let output = format_status(&s);
        assert!(!output.is_empty());
    }

    #[test]
    fn test_format_short_zero_hit_ratio() {
        let s = MetricsSnapshot::default();
        let output = format_short(&s);
        assert!(output.contains("0.0%"));
    }
}
