use std::sync::atomic::{AtomicU64, Ordering};

use crate::metrics::snapshot::MetricsSnapshot;

#[derive(Debug)]
pub struct MetricsCounters {
    pub bytes_read_logical: AtomicU64,
    pub bytes_read_physical: AtomicU64,
    pub bytes_written_logical: AtomicU64,
    pub bytes_written_physical: AtomicU64,
    pub cache_hits: AtomicU64,
    pub cache_misses: AtomicU64,
    pub cache_evictions: AtomicU64,
    pub flush_count: AtomicU64,
    pub flush_errors: AtomicU64,
    pub ack_buffered: AtomicU64,
    pub ack_durable: AtomicU64,
    pub eject_count: AtomicU64,
    pub unsafe_eject_blocked: AtomicU64,
    pub device_removals: AtomicU64,
    pub sbp_requests: AtomicU64,
    pub sbp_errors: AtomicU64,
    pub compression_ops: AtomicU64,
    pub decompression_ops: AtomicU64,
}

impl MetricsCounters {
    pub fn new() -> Self {
        MetricsCounters {
            bytes_read_logical: AtomicU64::new(0),
            bytes_read_physical: AtomicU64::new(0),
            bytes_written_logical: AtomicU64::new(0),
            bytes_written_physical: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
            cache_evictions: AtomicU64::new(0),
            flush_count: AtomicU64::new(0),
            flush_errors: AtomicU64::new(0),
            ack_buffered: AtomicU64::new(0),
            ack_durable: AtomicU64::new(0),
            eject_count: AtomicU64::new(0),
            unsafe_eject_blocked: AtomicU64::new(0),
            device_removals: AtomicU64::new(0),
            sbp_requests: AtomicU64::new(0),
            sbp_errors: AtomicU64::new(0),
            compression_ops: AtomicU64::new(0),
            decompression_ops: AtomicU64::new(0),
        }
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let hit_ratio = if total == 0 { 0.0 } else { hits as f64 / total as f64 };

        MetricsSnapshot {
            cache_hit_count: hits,
            cache_miss_count: misses,
            cache_eviction_count: self.cache_evictions.load(Ordering::Relaxed),
            cache_hit_ratio: hit_ratio,
            bytes_read_logical: self.bytes_read_logical.load(Ordering::Relaxed),
            bytes_read_physical: self.bytes_read_physical.load(Ordering::Relaxed),
            bytes_written_logical: self.bytes_written_logical.load(Ordering::Relaxed),
            bytes_written_physical: self.bytes_written_physical.load(Ordering::Relaxed),
            flush_count: self.flush_count.load(Ordering::Relaxed),
            flush_error_count: self.flush_errors.load(Ordering::Relaxed),
            ack_buffered_count: self.ack_buffered.load(Ordering::Relaxed),
            ack_durable_count: self.ack_durable.load(Ordering::Relaxed),
            eject_count: self.eject_count.load(Ordering::Relaxed),
            unsafe_eject_blocked_count: self.unsafe_eject_blocked.load(Ordering::Relaxed),
            device_removed_count: self.device_removals.load(Ordering::Relaxed),
            sbp_request_count: self.sbp_requests.load(Ordering::Relaxed),
            sbp_error_count: self.sbp_errors.load(Ordering::Relaxed),
            compression_count: self.compression_ops.load(Ordering::Relaxed),
            decompression_count: self.decompression_ops.load(Ordering::Relaxed),
            ..Default::default()
        }
    }
}

impl Default for MetricsCounters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counters_all_zero() {
        let c = MetricsCounters::new();
        let snap = c.snapshot();
        assert_eq!(snap.cache_hit_count, 0);
        assert_eq!(snap.cache_miss_count, 0);
        assert_eq!(snap.flush_count, 0);
        assert_eq!(snap.sbp_request_count, 0);
    }

    #[test]
    fn test_snapshot_reflects_atomic_increments() {
        let c = MetricsCounters::new();
        c.cache_hits.fetch_add(10, Ordering::Relaxed);
        c.cache_misses.fetch_add(5, Ordering::Relaxed);
        c.flush_count.fetch_add(3, Ordering::Relaxed);
        c.eject_count.fetch_add(1, Ordering::Relaxed);
        c.sbp_requests.fetch_add(100, Ordering::Relaxed);

        let snap = c.snapshot();
        assert_eq!(snap.cache_hit_count, 10);
        assert_eq!(snap.cache_miss_count, 5);
        assert_eq!(snap.flush_count, 3);
        assert_eq!(snap.eject_count, 1);
        assert_eq!(snap.sbp_request_count, 100);
    }

    #[test]
    fn test_hit_ratio_calculation() {
        let c = MetricsCounters::new();
        c.cache_hits.fetch_add(7, Ordering::Relaxed);
        c.cache_misses.fetch_add(3, Ordering::Relaxed);
        let snap = c.snapshot();
        assert!((snap.cache_hit_ratio - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_zero_hit_ratio_when_no_requests() {
        let c = MetricsCounters::new();
        let snap = c.snapshot();
        assert!((snap.cache_hit_ratio - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_snapshot_does_not_consume_counters() {
        let c = MetricsCounters::new();
        c.cache_hits.fetch_add(5, Ordering::Relaxed);
        let _ = c.snapshot();
        assert_eq!(c.cache_hits.load(Ordering::Relaxed), 5);
    }
}
