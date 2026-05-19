pub mod metrics;
pub mod profiler;

pub use metrics::TelemetryMetrics;
pub use profiler::Profiler;

use std::sync::atomic::Ordering;
use std::time::Instant;

pub struct Telemetry {
    metrics: TelemetryMetrics,
    start_time: Instant,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            metrics: TelemetryMetrics::new(),
            start_time: Instant::now(),
        }
    }

    pub fn record_command(&self, opcode: u8, latency_us: u64, execution_us: u64) {
        self.metrics.commands_processed.fetch_add(1, Ordering::SeqCst);
        self.metrics.total_task_time_us.fetch_add(execution_us, Ordering::SeqCst);

        let latency_idx = match latency_us {
            0..=10 => 0,
            11..=50 => 1,
            51..=200 => 2,
            201..=1000 => 3,
            1001..=5000 => 4,
            5001..=10000 => 5,
            10001..=50000 => 6,
            50001..=100000 => 7,
            100001..=500000 => 8,
            _ => 9,
        };
        self.metrics.ipc_latency_histogram[latency_idx].fetch_add(1, Ordering::SeqCst);

        let op_idx = opcode as usize;
        if op_idx < 32 {
            self.metrics.commands_by_opcode[op_idx].fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn record_error(&self) {
        self.metrics.errors_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn snapshot(&self) -> metrics::MetricsSnapshot {
        self.metrics.snapshot(self.start_time.elapsed().as_secs())
    }

    pub fn reset(&self) {
        self.metrics.reset();
    }
}
