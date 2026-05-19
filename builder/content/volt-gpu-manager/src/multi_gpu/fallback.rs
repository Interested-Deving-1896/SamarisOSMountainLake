use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackReason {
    NoGpuAvailable,
    MultiGpuUnsupported,
    BackendFailed,
    ThermalEmergency,
    DriverError,
}

impl FallbackReason {
    pub fn description(&self) -> &'static str {
        match self {
            FallbackReason::NoGpuAvailable => "No GPU available",
            FallbackReason::MultiGpuUnsupported => "Multi-GPU not supported",
            FallbackReason::BackendFailed => "GPU backend failed",
            FallbackReason::ThermalEmergency => "Thermal emergency",
            FallbackReason::DriverError => "Driver error",
        }
    }
}

pub struct MultiGpuFallback {
    pub active: bool,
    pub reason: Option<FallbackReason>,
    fallback_count: std::sync::atomic::AtomicU64,
}

impl MultiGpuFallback {
    pub fn new() -> Self {
        Self {
            active: false,
            reason: None,
            fallback_count: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn activate(&mut self, reason: FallbackReason) -> VgmResult<()> {
        self.active = true;
        self.reason = Some(reason);
        self.fallback_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        tracing::warn!("Multi-GPU fallback activated: {}", reason.description());
        Err(VgmError::MultiGpuUnsupported(reason.description().into()))
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.reason = None;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn fallback_count(&self) -> u64 {
        self.fallback_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for MultiGpuFallback {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fallback_inactive() {
        let fb = MultiGpuFallback::new();
        assert!(!fb.is_active());
        assert!(fb.reason.is_none());
    }

    #[test]
    fn test_activate() {
        let mut fb = MultiGpuFallback::new();
        let result = fb.activate(FallbackReason::NoGpuAvailable);
        assert!(result.is_err());
        assert!(fb.is_active());
        assert_eq!(fb.reason, Some(FallbackReason::NoGpuAvailable));
    }

    #[test]
    fn test_deactivate() {
        let mut fb = MultiGpuFallback::new();
        let _ = fb.activate(FallbackReason::BackendFailed);
        fb.deactivate();
        assert!(!fb.is_active());
        assert!(fb.reason.is_none());
    }

    #[test]
    fn test_fallback_count() {
        let mut fb = MultiGpuFallback::new();
        assert_eq!(fb.fallback_count(), 0);
        let _ = fb.activate(FallbackReason::DriverError);
        let _ = fb.activate(FallbackReason::DriverError);
        assert_eq!(fb.fallback_count(), 2);
    }

    #[test]
    fn test_reason_descriptions() {
        assert_eq!(FallbackReason::NoGpuAvailable.description(), "No GPU available");
        assert_eq!(FallbackReason::ThermalEmergency.description(), "Thermal emergency");
    }
}
