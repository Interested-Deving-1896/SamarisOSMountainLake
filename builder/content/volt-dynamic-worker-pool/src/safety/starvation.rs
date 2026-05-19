use std::sync::atomic::{AtomicU64, Ordering};

pub struct StarvationDetector {
    pub limit_ms: u64,
    pub starvation_count: AtomicU64,
    pub warning_count: AtomicU64,
}

impl StarvationDetector {
    pub fn new(limit_ms: u64) -> Self {
        Self {
            limit_ms,
            starvation_count: AtomicU64::new(0),
            warning_count: AtomicU64::new(0),
        }
    }

    pub fn check(&self, job_wait_ms: u64) -> bool {
        if job_wait_ms > self.limit_ms {
            self.record_starvation();
            return true;
        }
        false
    }

    pub fn record_starvation(&self) {
        self.starvation_count.fetch_add(1, Ordering::SeqCst);
        self.warning_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn starvation_count(&self) -> u64 {
        self.starvation_count.load(Ordering::SeqCst)
    }

    pub fn warning_count(&self) -> u64 {
        self.warning_count.load(Ordering::SeqCst)
    }
}
