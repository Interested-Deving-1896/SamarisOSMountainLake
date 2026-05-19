use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

pub struct DeadlockDetector {
    pub check_interval_ms: u64,
    pub last_check: Mutex<Instant>,
    pub warning_count: AtomicU64,
}

impl DeadlockDetector {
    pub fn new(check_interval_ms: u64) -> Self {
        Self {
            check_interval_ms,
            last_check: Mutex::new(Instant::now()),
            warning_count: AtomicU64::new(0),
        }
    }

    pub fn check(&self, all_workers_busy: bool, queue_not_empty: bool, progress_made: bool) -> bool {
        let mut last = self.last_check.lock().unwrap();
        let elapsed = last.elapsed().as_millis() as u64;
        if elapsed < self.check_interval_ms {
            return false;
        }
        *last = Instant::now();

        if all_workers_busy && queue_not_empty && !progress_made {
            self.record_warning();
            return true;
        }
        false
    }

    pub fn record_warning(&self) {
        self.warning_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn warning_count(&self) -> u64 {
        self.warning_count.load(Ordering::SeqCst)
    }
}
