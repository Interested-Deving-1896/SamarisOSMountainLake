use serde::{Deserialize, Serialize};

use crate::core::state::VumState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub uptime_ms: u64,
    pub state: VumState,
    pub device_present: bool,
    pub read_only: bool,
    pub mounted: bool,
    pub journal_dirty: bool,
    pub recovery_required: bool,
    pub cache_max_bytes: u64,
    pub cache_used_bytes: u64,
    pub cache_entries: u64,
    pub cache_hit_count: u64,
    pub cache_miss_count: u64,
    pub cache_eviction_count: u64,
    pub cache_hit_ratio: f64,
    pub write_buffer_max_bytes: u64,
    pub write_buffer_used_bytes: u64,
    pub pending_write_count: u64,
    pub dirty_bytes: u64,
    pub flush_count: u64,
    pub flush_error_count: u64,
    pub last_flush_duration_us: u64,
    pub bytes_read_logical: u64,
    pub bytes_read_physical: u64,
    pub bytes_written_logical: u64,
    pub bytes_written_physical: u64,
    pub compression_count: u64,
    pub decompression_count: u64,
    pub compression_saved_bytes: u64,
    pub journal_records: u64,
    pub journal_bytes: u64,
    pub journal_replay_count: u64,
    pub ack_buffered_count: u64,
    pub ack_durable_count: u64,
    pub eject_count: u64,
    pub unsafe_eject_blocked_count: u64,
    pub device_removed_count: u64,
    pub sbp_request_count: u64,
    pub sbp_error_count: u64,
    pub avg_read_latency_us: u64,
    pub avg_write_latency_us: u64,
    pub avg_flush_latency_us: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_snapshot() {
        let s = MetricsSnapshot::default();
        assert_eq!(s.uptime_ms, 0);
        assert_eq!(s.cache_hit_count, 0);
        assert_eq!(s.cache_hit_ratio, 0.0);
    }

    #[test]
    fn test_json_serialize_roundtrip() {
        let s = MetricsSnapshot::default();
        let json = serde_json::to_string(&s).unwrap();
        let parsed: MetricsSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.uptime_ms, s.uptime_ms);
    }

    #[test]
    fn test_clone_snapshot() {
        let s = MetricsSnapshot::default();
        let c = s.clone();
        assert_eq!(s.uptime_ms, c.uptime_ms);
    }

    #[test]
    fn test_debug_format() {
        let s = MetricsSnapshot::default();
        let debug = format!("{:?}", s);
        assert!(!debug.is_empty());
    }

    #[test]
    fn test_json_contains_all_fields() {
        let s = MetricsSnapshot::default();
        let json = serde_json::to_value(&s).unwrap();
        assert!(json.get("uptime_ms").is_some());
        assert!(json.get("state").is_some());
        assert!(json.get("cache_hit_ratio").is_some());
        assert!(json.get("journal_records").is_some());
        assert!(json.get("avg_read_latency_us").is_some());
    }
}
