use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

const MAX_FRAME_SAMPLES: usize = 60;

pub struct LatencyGuard {
    pub frame_budget_ms: u64,
    pub latency_guard_ms: u64,
    pub desktop_min_workers: u32,
    last_frame_time: AtomicU64,
    frame_times: Mutex<Vec<u64>>,
}

impl LatencyGuard {
    pub fn new(frame_budget_ms: u64, latency_guard_ms: u64, desktop_min_workers: u32) -> Self {
        Self {
            frame_budget_ms,
            latency_guard_ms,
            desktop_min_workers,
            last_frame_time: AtomicU64::new(0),
            frame_times: Mutex::new(Vec::with_capacity(MAX_FRAME_SAMPLES)),
        }
    }

    pub fn record_frame_time(&self, time_ms: u64) {
        self.last_frame_time.store(time_ms, Ordering::SeqCst);
        let mut times = self.frame_times.lock().unwrap();
        times.push(time_ms);
        if times.len() > MAX_FRAME_SAMPLES {
            times.remove(0);
        }
    }

    pub fn is_exceeding_budget(&self) -> bool {
        self.last_frame_time.load(Ordering::SeqCst) > self.frame_budget_ms
    }

    pub fn avg_frame_time_ms(&self) -> u64 {
        let times = self.frame_times.lock().unwrap();
        if times.is_empty() {
            return 0;
        }
        let sum: u64 = times.iter().sum();
        sum / times.len() as u64
    }

    pub fn should_protect(&self) -> bool {
        let avg = self.avg_frame_time_ms();
        avg > self.frame_budget_ms.saturating_sub(self.latency_guard_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_latency_guard() {
        let guard = LatencyGuard::new(16, 4, 2);
        assert_eq!(guard.frame_budget_ms, 16);
        assert_eq!(guard.latency_guard_ms, 4);
        assert_eq!(guard.desktop_min_workers, 2);
        assert_eq!(guard.avg_frame_time_ms(), 0);
    }

    #[test]
    fn test_record_frame_time() {
        let guard = LatencyGuard::new(16, 4, 2);
        guard.record_frame_time(10);
        assert!(!guard.is_exceeding_budget());
        guard.record_frame_time(20);
        assert!(guard.is_exceeding_budget());
    }

    #[test]
    fn test_avg_frame_time_ms() {
        let guard = LatencyGuard::new(16, 4, 2);
        guard.record_frame_time(10);
        guard.record_frame_time(20);
        guard.record_frame_time(30);
        assert_eq!(guard.avg_frame_time_ms(), 20);
    }

    #[test]
    fn test_should_protect() {
        let guard = LatencyGuard::new(16, 4, 2);
        guard.record_frame_time(10);
        assert!(!guard.should_protect());

        let guard2 = LatencyGuard::new(16, 4, 2);
        guard2.record_frame_time(15);
        assert!(guard2.should_protect());
    }

    #[test]
    fn test_max_samples_capped() {
        let guard = LatencyGuard::new(16, 4, 2);
        for i in 1..=100 {
            guard.record_frame_time(i);
        }
        assert_eq!(guard.avg_frame_time_ms(), (41 + 100) / 2);
    }
}
