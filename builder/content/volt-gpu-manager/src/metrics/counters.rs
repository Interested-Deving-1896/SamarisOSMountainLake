use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct GpuMetricsCounters {
    pub vram_allocations: AtomicU64,
    pub vram_frees: AtomicU64,
    pub compression_ops: AtomicU64,
    pub decompression_ops: AtomicU64,
    pub shader_compiles: AtomicU64,
    pub shader_errors: AtomicU64,
    pub frame_submissions: AtomicU64,
    pub frame_budget_misses: AtomicU64,
    pub compute_jobs_submitted: AtomicU64,
    pub compute_jobs_completed: AtomicU64,
    pub compute_job_errors: AtomicU64,
    pub sbp_requests: AtomicU64,
    pub sbp_errors: AtomicU64,
    pub thermal_backoff_events: AtomicU64,
    pub fallback_activations: AtomicU64,
    pub dedup_hits: AtomicU64,
    pub dedup_misses: AtomicU64,
    pub evictions: AtomicU64,
    pub restores: AtomicU64,
}

impl GpuMetricsCounters {
    pub fn new() -> Self {
        Self {
            vram_allocations: AtomicU64::new(0),
            vram_frees: AtomicU64::new(0),
            compression_ops: AtomicU64::new(0),
            decompression_ops: AtomicU64::new(0),
            shader_compiles: AtomicU64::new(0),
            shader_errors: AtomicU64::new(0),
            frame_submissions: AtomicU64::new(0),
            frame_budget_misses: AtomicU64::new(0),
            compute_jobs_submitted: AtomicU64::new(0),
            compute_jobs_completed: AtomicU64::new(0),
            compute_job_errors: AtomicU64::new(0),
            sbp_requests: AtomicU64::new(0),
            sbp_errors: AtomicU64::new(0),
            thermal_backoff_events: AtomicU64::new(0),
            fallback_activations: AtomicU64::new(0),
            dedup_hits: AtomicU64::new(0),
            dedup_misses: AtomicU64::new(0),
            evictions: AtomicU64::new(0),
            restores: AtomicU64::new(0),
        }
    }

    pub fn increment(&self, counter: CounterKind) {
        let target: &AtomicU64 = match counter {
            CounterKind::VramAllocations => &self.vram_allocations,
            CounterKind::VramFrees => &self.vram_frees,
            CounterKind::CompressionOps => &self.compression_ops,
            CounterKind::DecompressionOps => &self.decompression_ops,
            CounterKind::ShaderCompiles => &self.shader_compiles,
            CounterKind::ShaderErrors => &self.shader_errors,
            CounterKind::FrameSubmissions => &self.frame_submissions,
            CounterKind::FrameBudgetMisses => &self.frame_budget_misses,
            CounterKind::ComputeJobsSubmitted => &self.compute_jobs_submitted,
            CounterKind::ComputeJobsCompleted => &self.compute_jobs_completed,
            CounterKind::ComputeJobErrors => &self.compute_job_errors,
            CounterKind::SbpRequests => &self.sbp_requests,
            CounterKind::SbpErrors => &self.sbp_errors,
            CounterKind::ThermalBackoffEvents => &self.thermal_backoff_events,
            CounterKind::FallbackActivations => &self.fallback_activations,
            CounterKind::DedupHits => &self.dedup_hits,
            CounterKind::DedupMisses => &self.dedup_misses,
            CounterKind::Evictions => &self.evictions,
            CounterKind::Restores => &self.restores,
        };
        target.fetch_add(1, Ordering::Relaxed);
    }

    pub fn read(&self, counter: CounterKind) -> u64 {
        let target: &AtomicU64 = match counter {
            CounterKind::VramAllocations => &self.vram_allocations,
            CounterKind::VramFrees => &self.vram_frees,
            CounterKind::CompressionOps => &self.compression_ops,
            CounterKind::DecompressionOps => &self.decompression_ops,
            CounterKind::ShaderCompiles => &self.shader_compiles,
            CounterKind::ShaderErrors => &self.shader_errors,
            CounterKind::FrameSubmissions => &self.frame_submissions,
            CounterKind::FrameBudgetMisses => &self.frame_budget_misses,
            CounterKind::ComputeJobsSubmitted => &self.compute_jobs_submitted,
            CounterKind::ComputeJobsCompleted => &self.compute_jobs_completed,
            CounterKind::ComputeJobErrors => &self.compute_job_errors,
            CounterKind::SbpRequests => &self.sbp_requests,
            CounterKind::SbpErrors => &self.sbp_errors,
            CounterKind::ThermalBackoffEvents => &self.thermal_backoff_events,
            CounterKind::FallbackActivations => &self.fallback_activations,
            CounterKind::DedupHits => &self.dedup_hits,
            CounterKind::DedupMisses => &self.dedup_misses,
            CounterKind::Evictions => &self.evictions,
            CounterKind::Restores => &self.restores,
        };
        target.load(Ordering::Relaxed)
    }
}

impl Default for GpuMetricsCounters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CounterKind {
    VramAllocations,
    VramFrees,
    CompressionOps,
    DecompressionOps,
    ShaderCompiles,
    ShaderErrors,
    FrameSubmissions,
    FrameBudgetMisses,
    ComputeJobsSubmitted,
    ComputeJobsCompleted,
    ComputeJobErrors,
    SbpRequests,
    SbpErrors,
    ThermalBackoffEvents,
    FallbackActivations,
    DedupHits,
    DedupMisses,
    Evictions,
    Restores,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_counters_all_zero() {
        let c = GpuMetricsCounters::new();
        assert_eq!(c.read(CounterKind::VramAllocations), 0);
        assert_eq!(c.read(CounterKind::SbpRequests), 0);
    }

    #[test]
    fn test_increment() {
        let c = GpuMetricsCounters::new();
        c.increment(CounterKind::CompressionOps);
        c.increment(CounterKind::CompressionOps);
        assert_eq!(c.read(CounterKind::CompressionOps), 2);
    }

    #[test]
    fn test_increment_all() {
        let c = GpuMetricsCounters::new();
        let kinds = vec![
            CounterKind::VramAllocations,
            CounterKind::VramFrees,
            CounterKind::CompressionOps,
            CounterKind::DecompressionOps,
            CounterKind::ShaderCompiles,
            CounterKind::ShaderErrors,
            CounterKind::FrameSubmissions,
            CounterKind::FrameBudgetMisses,
            CounterKind::ComputeJobsSubmitted,
            CounterKind::ComputeJobsCompleted,
            CounterKind::ComputeJobErrors,
            CounterKind::SbpRequests,
            CounterKind::SbpErrors,
            CounterKind::ThermalBackoffEvents,
            CounterKind::FallbackActivations,
            CounterKind::DedupHits,
            CounterKind::DedupMisses,
            CounterKind::Evictions,
            CounterKind::Restores,
        ];
        for kind in &kinds {
            c.increment(*kind);
        }
        for kind in &kinds {
            assert_eq!(c.read(*kind), 1, "Counter {:?} should be 1", kind);
        }
    }

    #[test]
    fn test_independent_counters() {
        let c = GpuMetricsCounters::new();
        c.increment(CounterKind::DedupHits);
        assert_eq!(c.read(CounterKind::DedupHits), 1);
        assert_eq!(c.read(CounterKind::DedupMisses), 0);
    }

    #[test]
    fn test_default_impl() {
        let c = GpuMetricsCounters::default();
        assert_eq!(c.read(CounterKind::Evictions), 0);
    }
}
