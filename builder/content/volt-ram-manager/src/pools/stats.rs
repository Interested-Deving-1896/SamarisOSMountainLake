use std::sync::atomic::{AtomicU64, Ordering};

pub struct PoolStats {
    pub allocations: AtomicU64,
    pub deallocations: AtomicU64,
    pub allocated_bytes: AtomicU64,
    pub freed_bytes: AtomicU64,
}

impl PoolStats {
    pub fn new() -> Self {
        PoolStats {
            allocations: AtomicU64::new(0),
            deallocations: AtomicU64::new(0),
            allocated_bytes: AtomicU64::new(0),
            freed_bytes: AtomicU64::new(0),
        }
    }

    pub fn record_alloc(&self, bytes: u64) {
        self.allocations.fetch_add(1, Ordering::Relaxed);
        self.allocated_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn record_free(&self, bytes: u64) {
        self.deallocations.fetch_add(1, Ordering::Relaxed);
        self.freed_bytes.fetch_add(bytes, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_new() {
        let stats = PoolStats::new();
        assert_eq!(stats.allocations.load(Ordering::Relaxed), 0);
        assert_eq!(stats.deallocations.load(Ordering::Relaxed), 0);
        assert_eq!(stats.allocated_bytes.load(Ordering::Relaxed), 0);
        assert_eq!(stats.freed_bytes.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_record_alloc() {
        let stats = PoolStats::new();
        stats.record_alloc(1024);
        assert_eq!(stats.allocations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.allocated_bytes.load(Ordering::Relaxed), 1024);
    }

    #[test]
    fn test_record_free() {
        let stats = PoolStats::new();
        stats.record_alloc(512);
        stats.record_free(512);
        assert_eq!(stats.deallocations.load(Ordering::Relaxed), 1);
        assert_eq!(stats.freed_bytes.load(Ordering::Relaxed), 512);
    }

    #[test]
    fn test_multiple_records() {
        let stats = PoolStats::new();
        stats.record_alloc(100);
        stats.record_alloc(200);
        stats.record_alloc(300);
        assert_eq!(stats.allocations.load(Ordering::Relaxed), 3);
        assert_eq!(stats.allocated_bytes.load(Ordering::Relaxed), 600);
    }
}
