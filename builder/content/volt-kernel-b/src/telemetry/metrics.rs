use std::sync::atomic::{AtomicU64, Ordering};

pub struct TelemetryMetrics {
    pub commands_processed: AtomicU64,
    pub errors_count: AtomicU64,
    pub total_task_time_us: AtomicU64,
    pub ipc_latency_histogram: [AtomicU64; 10],
    pub commands_by_opcode: [AtomicU64; 32],
}

impl TelemetryMetrics {
    pub fn new() -> Self {
        Self {
            commands_processed: AtomicU64::new(0),
            errors_count: AtomicU64::new(0),
            total_task_time_us: AtomicU64::new(0),
            ipc_latency_histogram: [
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0),
            ],
            commands_by_opcode: [
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0), AtomicU64::new(0),
            ],
        }
    }

    pub fn snapshot(&self, uptime_secs: u64) -> MetricsSnapshot {
        let processed = self.commands_processed.load(Ordering::SeqCst);
        let errors = self.errors_count.load(Ordering::SeqCst);
        let total_time = self.total_task_time_us.load(Ordering::SeqCst);

        let avg_exec_us = if processed > 0 {
            total_time / processed
        } else {
            0
        };

        let commands_per_sec = if uptime_secs > 0 {
            processed / uptime_secs
        } else {
            0
        };

        let mut latency_buckets = [0u64; 10];
        for (i, bucket) in latency_buckets.iter_mut().enumerate() {
            *bucket = self.ipc_latency_histogram[i].load(Ordering::SeqCst);
        }

        let mut opcode_counts = [0u64; 32];
        for (i, count) in opcode_counts.iter_mut().enumerate() {
            *count = self.commands_by_opcode[i].load(Ordering::SeqCst);
        }

        MetricsSnapshot {
            commands_processed: processed,
            errors_count: errors,
            avg_execution_time_us: avg_exec_us,
            commands_per_second: commands_per_sec,
            uptime_secs,
            latency_histogram_buckets_us: latency_buckets,
            opcode_counts,
        }
    }

    pub fn reset(&self) {
        self.commands_processed.store(0, Ordering::SeqCst);
        self.errors_count.store(0, Ordering::SeqCst);
        self.total_task_time_us.store(0, Ordering::SeqCst);
        for bucket in &self.ipc_latency_histogram {
            bucket.store(0, Ordering::SeqCst);
        }
        for count in &self.commands_by_opcode {
            count.store(0, Ordering::SeqCst);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub commands_processed: u64,
    pub errors_count: u64,
    pub avg_execution_time_us: u64,
    pub commands_per_second: u64,
    pub uptime_secs: u64,
    pub latency_histogram_buckets_us: [u64; 10],
    pub opcode_counts: [u64; 32],
}
