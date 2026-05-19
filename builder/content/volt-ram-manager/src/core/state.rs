use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug)]
pub struct VrmState {
    pub start_time: Instant,
    pub boot_complete: AtomicBool,
    pub shutdown_requested: AtomicBool,
    pub total_allocated_bytes: AtomicU64,
    pub total_freed_bytes: AtomicU64,
    pub allocation_count: AtomicU64,
    pub free_count: AtomicU64,
}

impl VrmState {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            boot_complete: AtomicBool::new(false),
            shutdown_requested: AtomicBool::new(false),
            total_allocated_bytes: AtomicU64::new(0),
            total_freed_bytes: AtomicU64::new(0),
            allocation_count: AtomicU64::new(0),
            free_count: AtomicU64::new(0),
        }
    }

    pub fn uptime_ms(&self) -> u64 {
        self.start_time.elapsed().as_millis() as u64
    }

    pub fn mark_boot_complete(&self) {
        self.boot_complete.store(true, Ordering::SeqCst);
    }

    pub fn request_shutdown(&self) {
        self.shutdown_requested.store(true, Ordering::SeqCst);
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }

    pub fn record_alloc(&self, bytes: u64) {
        self.total_allocated_bytes.fetch_add(bytes, Ordering::SeqCst);
        self.allocation_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_free(&self, bytes: u64) {
        self.total_freed_bytes.fetch_add(bytes, Ordering::SeqCst);
        self.free_count.fetch_add(1, Ordering::SeqCst);
    }
}
