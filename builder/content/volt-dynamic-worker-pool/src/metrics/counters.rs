use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::RwLock;

use crate::metrics::snapshot::MetricsSnapshot;

#[derive(Debug)]
pub struct MetricsCounters {
    pub total_jobs_submitted: AtomicU64,
    pub total_jobs_completed: AtomicU64,
    pub total_jobs_failed: AtomicU64,
    pub total_jobs_cancelled: AtomicU64,
    pub total_jobs_timed_out: AtomicU64,
    pub yield_count: AtomicU64,
    pub preemption_count: AtomicU64,
    pub orbit_burst_count: AtomicU64,
    pub scaling_events: AtomicU64,
    pub thermal_throttle_count: AtomicU64,
    pub avg_completion_time_ns: AtomicU64,
    pub avg_queue_time_ns: AtomicU64,
    pub completion_samples: AtomicU64,
    pub queue_samples: AtomicU64,
    pub desktop_pressure: RwLock<f64>,
    pub throughput_window: RwLock<ThroughputWindow>,
}

#[derive(Debug, Clone)]
pub struct ThroughputWindow {
    pub start_time: std::time::Instant,
    pub jobs_in_window: u64,
}

impl ThroughputWindow {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            jobs_in_window: 0,
        }
    }
}

impl Default for ThroughputWindow {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCounters {
    pub fn new() -> Self {
        Self {
            total_jobs_submitted: AtomicU64::new(0),
            total_jobs_completed: AtomicU64::new(0),
            total_jobs_failed: AtomicU64::new(0),
            total_jobs_cancelled: AtomicU64::new(0),
            total_jobs_timed_out: AtomicU64::new(0),
            yield_count: AtomicU64::new(0),
            preemption_count: AtomicU64::new(0),
            orbit_burst_count: AtomicU64::new(0),
            scaling_events: AtomicU64::new(0),
            thermal_throttle_count: AtomicU64::new(0),
            avg_completion_time_ns: AtomicU64::new(0),
            avg_queue_time_ns: AtomicU64::new(0),
            completion_samples: AtomicU64::new(0),
            queue_samples: AtomicU64::new(0),
            desktop_pressure: RwLock::new(0.0),
            throughput_window: RwLock::new(ThroughputWindow::new()),
        }
    }

    pub fn snapshot(
        &self,
        active_workers: u32,
        idle_workers: u32,
        queue_depth: usize,
        high_priority_queue_depth: usize,
        uptime_ms: u64,
        worker_pool_state: String,
    ) -> MetricsSnapshot {
        let submitted = self.total_jobs_submitted.load(Ordering::Relaxed);
        let completed = self.total_jobs_completed.load(Ordering::Relaxed);
        let throughput = {
            let window = self.throughput_window.read();
            let elapsed = window.start_time.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                window.jobs_in_window as f64 / elapsed
            } else {
                0.0
            }
        };

        let completion_samples = self.completion_samples.load(Ordering::Relaxed);
        let avg_completion = if completion_samples > 0 {
            self.avg_completion_time_ns.load(Ordering::Relaxed) as f64
                / completion_samples as f64
                / 1_000_000.0
        } else {
            0.0
        };

        let queue_samples = self.queue_samples.load(Ordering::Relaxed);
        let avg_queue = if queue_samples > 0 {
            self.avg_queue_time_ns.load(Ordering::Relaxed) as f64
                / queue_samples as f64
                / 1_000_000.0
        } else {
            0.0
        };

        MetricsSnapshot {
            total_jobs_submitted: submitted,
            total_jobs_completed: completed,
            total_jobs_failed: self.total_jobs_failed.load(Ordering::Relaxed),
            total_jobs_cancelled: self.total_jobs_cancelled.load(Ordering::Relaxed),
            total_jobs_timed_out: self.total_jobs_timed_out.load(Ordering::Relaxed),
            active_workers,
            idle_workers,
            queue_depth,
            high_priority_queue_depth,
            avg_completion_time_ms: avg_completion,
            avg_queue_time_ms: avg_queue,
            throughput_jobs_per_sec: throughput,
            uptime_ms,
            yield_count: self.yield_count.load(Ordering::Relaxed),
            preemption_count: self.preemption_count.load(Ordering::Relaxed),
            orbit_burst_count: self.orbit_burst_count.load(Ordering::Relaxed),
            scaling_events: self.scaling_events.load(Ordering::Relaxed),
            desktop_pressure: *self.desktop_pressure.read(),
            thermal_throttle_count: self.thermal_throttle_count.load(Ordering::Relaxed),
            worker_pool_state,
        }
    }

    pub fn record_submission(&self) {
        self.total_jobs_submitted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_completion(&self, elapsed_ns: u64) {
        self.total_jobs_completed.fetch_add(1, Ordering::Relaxed);
        self.update_running_avg(&self.avg_completion_time_ns, elapsed_ns, &self.completion_samples);
        {
            let mut window = self.throughput_window.write();
            window.jobs_in_window += 1;
        }
    }

    pub fn record_failure(&self) {
        self.total_jobs_failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cancellation(&self) {
        self.total_jobs_cancelled.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_timeout(&self) {
        self.total_jobs_timed_out.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_yield(&self) {
        self.yield_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_preemption(&self) {
        self.preemption_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_orbit_burst(&self) {
        self.orbit_burst_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_scaling_event(&self) {
        self.scaling_events.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_thermal_throttle(&self) {
        self.thermal_throttle_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_desktop_pressure(&self, pressure: f64) {
        *self.desktop_pressure.write() = pressure;
    }

    fn update_running_avg(&self, avg: &AtomicU64, sample: u64, count: &AtomicU64) {
        let n = count.fetch_add(1, Ordering::Relaxed) + 1;
        let current = avg.load(Ordering::Relaxed);
        let new_avg = current + (sample.saturating_sub(current)) / n;
        avg.store(new_avg, Ordering::Relaxed);
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
    fn test_counters() {
        let c = MetricsCounters::new();
        assert_eq!(c.total_jobs_submitted.load(Ordering::Relaxed), 0);

        c.record_submission();
        assert_eq!(c.total_jobs_submitted.load(Ordering::Relaxed), 1);

        c.record_completion(1_000_000);
        assert_eq!(c.total_jobs_completed.load(Ordering::Relaxed), 1);

        c.record_failure();
        assert_eq!(c.total_jobs_failed.load(Ordering::Relaxed), 1);

        c.record_yield();
        assert_eq!(c.yield_count.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_desktop_pressure() {
        let c = MetricsCounters::new();
        c.set_desktop_pressure(0.75);
        assert!((*c.desktop_pressure.read() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_snapshot() {
        let c = MetricsCounters::new();
        c.record_submission();
        c.record_completion(500_000);
        let snap = c.snapshot(4, 2, 10, 3, 1000, "running".into());
        assert_eq!(snap.total_jobs_submitted, 1);
        assert_eq!(snap.total_jobs_completed, 1);
        assert_eq!(snap.active_workers, 4);
        assert_eq!(snap.idle_workers, 2);
        assert_eq!(snap.queue_depth, 10);
    }
}
