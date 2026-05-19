use std::fmt;

use serde::{Deserialize, Serialize};

use crate::metrics::histogram::HistogramSnapshot;
use crate::metrics::queue_metrics::QueueMetricsSnapshot;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::metrics::utilization::UtilizationSnapshot;
use crate::metrics::worker_metrics::WorkerMetricsSnapshot;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsReport {
    pub snapshot: MetricsSnapshot,
    pub histograms: HistogramSnapshot,
    pub utilization: UtilizationSnapshot,
    pub queue: QueueMetricsSnapshot,
    pub workers: WorkerMetricsSnapshot,
}

impl MetricsReport {
    pub fn new(
        snapshot: MetricsSnapshot,
        histograms: HistogramSnapshot,
        utilization: UtilizationSnapshot,
        queue: QueueMetricsSnapshot,
        workers: WorkerMetricsSnapshot,
    ) -> Self {
        Self {
            snapshot,
            histograms,
            utilization,
            queue,
            workers,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

impl fmt::Display for MetricsReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "=== Volt Dynamic Worker Pool Metrics Report ===")?;
        writeln!(f)?;
        writeln!(f, "--- Pool State ---")?;
        writeln!(f, "  State:               {}", self.snapshot.worker_pool_state)?;
        writeln!(f, "  Uptime:              {} ms", self.snapshot.uptime_ms)?;
        writeln!(f, "  Active Workers:      {}", self.snapshot.active_workers)?;
        writeln!(f, "  Idle Workers:        {}", self.snapshot.idle_workers)?;
        writeln!(f, "  Queue Depth:         {}", self.snapshot.queue_depth)?;
        writeln!(
            f,
            "  High Pri Queue:      {}",
            self.snapshot.high_priority_queue_depth
        )?;
        writeln!(f)?;
        writeln!(f, "--- Job Counters ---")?;
        writeln!(
            f,
            "  Submitted:           {}",
            self.snapshot.total_jobs_submitted
        )?;
        writeln!(
            f,
            "  Completed:           {}",
            self.snapshot.total_jobs_completed
        )?;
        writeln!(
            f,
            "  Failed:              {}",
            self.snapshot.total_jobs_failed
        )?;
        writeln!(
            f,
            "  Cancelled:           {}",
            self.snapshot.total_jobs_cancelled
        )?;
        writeln!(
            f,
            "  Timed Out:           {}",
            self.snapshot.total_jobs_timed_out
        )?;
        writeln!(
            f,
            "  Throughput:          {:.2} jobs/s",
            self.snapshot.throughput_jobs_per_sec
        )?;
        writeln!(
            f,
            "  Avg Completion:      {:.2} ms",
            self.snapshot.avg_completion_time_ms
        )?;
        writeln!(
            f,
            "  Avg Queue Wait:      {:.2} ms",
            self.snapshot.avg_queue_time_ms
        )?;
        writeln!(f)?;
        writeln!(f, "--- System Events ---")?;
        writeln!(f, "  Yields:              {}", self.snapshot.yield_count)?;
        writeln!(
            f,
            "  Preemptions:         {}",
            self.snapshot.preemption_count
        )?;
        writeln!(
            f,
            "  Orbit Bursts:        {}",
            self.snapshot.orbit_burst_count
        )?;
        writeln!(
            f,
            "  Scaling Events:      {}",
            self.snapshot.scaling_events
        )?;
        writeln!(
            f,
            "  Thermal Throttles:   {}",
            self.snapshot.thermal_throttle_count
        )?;
        writeln!(
            f,
            "  Desktop Pressure:    {:.4}",
            self.snapshot.desktop_pressure
        )?;
        writeln!(f)?;
        writeln!(f, "--- Latency Percentiles (ns) ---")?;
        writeln!(
            f,
            "  Queue Wait   P50: {}  P90: {}  P99: {}",
            self.histograms.p50_queue_wait,
            self.histograms.p90_queue_wait,
            self.histograms.p99_queue_wait
        )?;
        writeln!(
            f,
            "  Execution    P50: {}  P90: {}  P99: {}",
            self.histograms.p50_execution,
            self.histograms.p90_execution,
            self.histograms.p99_execution
        )?;
        writeln!(
            f,
            "  Preemption   P50: {}  P90: {}  P99: {}",
            self.histograms.p50_preemption,
            self.histograms.p90_preemption,
            self.histograms.p99_preemption
        )?;
        writeln!(
            f,
            "  Yield        P50: {}  P90: {}  P99: {}",
            self.histograms.p50_yield,
            self.histograms.p90_yield,
            self.histograms.p99_yield
        )?;
        writeln!(f)?;
        writeln!(f, "--- Utilization ---")?;
        writeln!(
            f,
            "  Total Worker Time:  {} us",
            self.utilization.total_worker_time_us
        )?;
        writeln!(
            f,
            "  Busy Worker Time:   {} us",
            self.utilization.busy_worker_time_us
        )?;
        writeln!(
            f,
            "  Utilization:        {:.2}%",
            self.utilization.utilization * 100.0
        )?;
        writeln!(f)?;
        writeln!(f, "--- Queue Metrics ---")?;
        writeln!(
            f,
            "  Enqueues:           {}",
            self.queue.enqueue_count
        )?;
        writeln!(
            f,
            "  Dequeues:           {}",
            self.queue.dequeue_count
        )?;
        writeln!(f, "  Cancels:            {}", self.queue.cancel_count)?;
        writeln!(
            f,
            "  Peak Depth:         {}",
            self.queue.peak_depth
        )?;
        writeln!(f)?;
        writeln!(f, "--- Worker Metrics ---")?;
        writeln!(
            f,
            "  Spawns:             {}",
            self.workers.spawn_count
        )?;
        writeln!(
            f,
            "  Retires:            {}",
            self.workers.retire_count
        )?;
        writeln!(
            f,
            "  Max Concurrent:     {}",
            self.workers.max_concurrent
        )?;
        writeln!(
            f,
            "  Crashes:            {}",
            self.workers.crash_count
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::queue_metrics::QueueMetrics;
    use crate::metrics::utilization::UtilizationTracker;
    use crate::metrics::worker_metrics::WorkerMetrics;
    use crate::metrics::latency::LatencyTracker;

    fn make_snapshot() -> MetricsSnapshot {
        MetricsSnapshot {
            total_jobs_submitted: 100,
            total_jobs_completed: 85,
            total_jobs_failed: 5,
            total_jobs_cancelled: 5,
            total_jobs_timed_out: 5,
            active_workers: 8,
            idle_workers: 2,
            queue_depth: 15,
            high_priority_queue_depth: 3,
            avg_completion_time_ms: 12.5,
            avg_queue_time_ms: 2.3,
            throughput_jobs_per_sec: 42.0,
            uptime_ms: 60000,
            yield_count: 10,
            preemption_count: 3,
            orbit_burst_count: 1,
            scaling_events: 2,
            desktop_pressure: 0.25,
            thermal_throttle_count: 0,
            worker_pool_state: "running".into(),
        }
    }

    fn make_histogram_snapshot() -> HistogramSnapshot {
        let lt = LatencyTracker::new();
        for _ in 0..100 {
            lt.record_queue_wait(500);
            lt.record_execution(5000);
            lt.record_preemption(200);
            lt.record_yield(300);
        }
        lt.snapshot()
    }

    fn make_utilization_snapshot() -> UtilizationSnapshot {
        let ut = UtilizationTracker::new();
        ut.record_busy(75000);
        ut.record_idle(25000);
        ut.snapshot()
    }

    fn make_queue_snapshot() -> QueueMetricsSnapshot {
        let qm = QueueMetrics::new();
        qm.record_enqueue(1);
        qm.record_enqueue(10);
        qm.record_enqueue(5);
        qm.record_dequeue();
        qm.record_cancel();
        qm.snapshot()
    }

    fn make_worker_snapshot() -> WorkerMetricsSnapshot {
        let wm = WorkerMetrics::new();
        wm.record_spawn(1);
        wm.record_spawn(8);
        wm.record_spawn(5);
        wm.record_retire();
        wm.record_crash();
        wm.snapshot()
    }

    #[test]
    fn test_metrics_report_creation() {
        let report = MetricsReport::new(
            make_snapshot(),
            make_histogram_snapshot(),
            make_utilization_snapshot(),
            make_queue_snapshot(),
            make_worker_snapshot(),
        );
        assert_eq!(report.snapshot.total_jobs_submitted, 100);
        assert_eq!(report.workers.spawn_count, 3);
    }

    #[test]
    fn test_metrics_report_display() {
        let report = MetricsReport::new(
            make_snapshot(),
            make_histogram_snapshot(),
            make_utilization_snapshot(),
            make_queue_snapshot(),
            make_worker_snapshot(),
        );
        let display = format!("{}", report);
        assert!(display.contains("Volt Dynamic Worker Pool Metrics Report"));
        assert!(display.contains("Job Counters"));
        assert!(display.contains("Latency Percentiles"));
        assert!(display.contains("Utilization"));
    }

    #[test]
    fn test_metrics_report_json() {
        let report = MetricsReport::new(
            make_snapshot(),
            make_histogram_snapshot(),
            make_utilization_snapshot(),
            make_queue_snapshot(),
            make_worker_snapshot(),
        );
        let json = report.to_json().unwrap();
        assert!(json.contains("total_jobs_submitted"));
        assert!(json.contains("p50_queue_wait"));
        assert!(json.contains("utilization"));
    }

    #[test]
    fn test_metrics_report_json_compact() {
        let report = MetricsReport::new(
            make_snapshot(),
            make_histogram_snapshot(),
            make_utilization_snapshot(),
            make_queue_snapshot(),
            make_worker_snapshot(),
        );
        let json = report.to_json_compact().unwrap();
        assert!(json.contains("total_jobs_submitted"));
    }
}
