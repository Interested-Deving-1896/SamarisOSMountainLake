use crate::compute::compute_job::{GpuComputeJob, GpuComputeJobKind};
use crate::core::result::VgmResult;

pub struct CpuComputeFallback {
    pub enabled: bool,
    fallback_count: std::sync::atomic::AtomicU64,
}

impl CpuComputeFallback {
    pub fn new() -> Self {
        Self {
            enabled: true,
            fallback_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn with_enabled(enabled: bool) -> Self {
        Self {
            enabled,
            fallback_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn execute(&self, job: &GpuComputeJob) -> VgmResult<()> {
        if !self.enabled {
            return Err(crate::core::error::VgmError::GpuJobFailed(
                "CPU fallback is disabled".into(),
            ));
        }
        self.fallback_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if job.input_size > 1024 * 1024 * 1024 {
            return Err(crate::core::error::VgmError::GpuJobFailed(
                "Input too large for CPU fallback".into(),
            ));
        }
        Ok(())
    }

    pub fn can_handle(&self, kind: &GpuComputeJobKind) -> bool {
        matches!(
            kind,
            GpuComputeJobKind::Blur
                | GpuComputeJobKind::Shadow
                | GpuComputeJobKind::Composite
                | GpuComputeJobKind::Transform2D
        )
    }

    pub fn fallback_count(&self) -> u64 {
        self.fallback_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for CpuComputeFallback {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::priority::GpuPriority;

    fn make_job(kind: GpuComputeJobKind, size: u64) -> GpuComputeJob {
        GpuComputeJob {
            job_id: uuid::Uuid::new_v4(),
            kind,
            priority: GpuPriority::Normal,
            input_size: size,
            app_id: 0,
        }
    }

    #[test]
    fn test_execute_works() {
        let fb = CpuComputeFallback::new();
        let job = make_job(GpuComputeJobKind::Blur, 1024);
        assert!(fb.execute(&job).is_ok());
    }

    #[test]
    fn test_disabled_fallback_fails() {
        let fb = CpuComputeFallback::with_enabled(false);
        let job = make_job(GpuComputeJobKind::Blur, 1024);
        assert!(fb.execute(&job).is_err());
    }

    #[test]
    fn test_can_handle_supported() {
        let fb = CpuComputeFallback::new();
        assert!(fb.can_handle(&GpuComputeJobKind::Blur));
        assert!(!fb.can_handle(&GpuComputeJobKind::MatMul));
    }

    #[test]
    fn test_fallback_count() {
        let fb = CpuComputeFallback::new();
        assert_eq!(fb.fallback_count(), 0);
        let job = make_job(GpuComputeJobKind::Shadow, 512);
        fb.execute(&job).unwrap();
        assert_eq!(fb.fallback_count(), 1);
    }

    #[test]
    fn test_too_large_input_fails() {
        let fb = CpuComputeFallback::new();
        let job = make_job(GpuComputeJobKind::Composite, 2 * 1024 * 1024 * 1024);
        assert!(fb.execute(&job).is_err());
    }
}
