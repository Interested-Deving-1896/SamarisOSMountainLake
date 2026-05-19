use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

#[derive(Debug)]
pub struct GpuLatencyMetrics {
    frame_times_us: AtomicU64,
    frame_count: AtomicU64,
    compile_times_us: AtomicU64,
    compile_count: AtomicU64,
    compression_times_us: AtomicU64,
    compression_count: AtomicU64,
    decompression_times_us: AtomicU64,
    decompression_count: AtomicU64,
}

impl GpuLatencyMetrics {
    pub fn new() -> Self {
        Self {
            frame_times_us: AtomicU64::new(0),
            frame_count: AtomicU64::new(0),
            compile_times_us: AtomicU64::new(0),
            compile_count: AtomicU64::new(0),
            compression_times_us: AtomicU64::new(0),
            compression_count: AtomicU64::new(0),
            decompression_times_us: AtomicU64::new(0),
            decompression_count: AtomicU64::new(0),
        }
    }

    pub fn record_frame(&self, start: Instant) {
        let elapsed = start.elapsed().as_micros() as u64;
        self.frame_times_us.fetch_add(elapsed, Ordering::Relaxed);
        self.frame_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_compile(&self, elapsed_us: u64) {
        self.compile_times_us.fetch_add(elapsed_us, Ordering::Relaxed);
        self.compile_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_compression(&self, elapsed_us: u64) {
        self.compression_times_us.fetch_add(elapsed_us, Ordering::Relaxed);
        self.compression_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_decompression(&self, elapsed_us: u64) {
        self.decompression_times_us.fetch_add(elapsed_us, Ordering::Relaxed);
        self.decompression_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn average_frame_time_us(&self) -> u64 {
        let count = self.frame_count.load(Ordering::Relaxed);
        if count == 0 { return 0; }
        self.frame_times_us.load(Ordering::Relaxed) / count
    }

    pub fn average_compile_time_us(&self) -> u64 {
        let count = self.compile_count.load(Ordering::Relaxed);
        if count == 0 { return 0; }
        self.compile_times_us.load(Ordering::Relaxed) / count
    }

    pub fn average_compression_time_us(&self) -> u64 {
        let count = self.compression_count.load(Ordering::Relaxed);
        if count == 0 { return 0; }
        self.compression_times_us.load(Ordering::Relaxed) / count
    }

    pub fn average_decompression_time_us(&self) -> u64 {
        let count = self.decompression_count.load(Ordering::Relaxed);
        if count == 0 { return 0; }
        self.decompression_times_us.load(Ordering::Relaxed) / count
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count.load(Ordering::Relaxed)
    }
}

impl Default for GpuLatencyMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_metrics() {
        let m = GpuLatencyMetrics::new();
        assert_eq!(m.frame_count(), 0);
    }

    #[test]
    fn test_record_frame() {
        let m = GpuLatencyMetrics::new();
        m.record_frame(Instant::now());
        assert_eq!(m.frame_count(), 1);
    }

    #[test]
    fn test_average_compile_time() {
        let m = GpuLatencyMetrics::new();
        m.record_compile(100);
        m.record_compile(200);
        assert_eq!(m.average_compile_time_us(), 150);
    }

    #[test]
    fn test_zero_division() {
        let m = GpuLatencyMetrics::new();
        assert_eq!(m.average_frame_time_us(), 0);
    }

    #[test]
    fn test_compression_average() {
        let m = GpuLatencyMetrics::new();
        m.record_compression(500);
        m.record_decompression(300);
        assert_eq!(m.average_compression_time_us(), 500);
        assert_eq!(m.average_decompression_time_us(), 300);
    }
}
