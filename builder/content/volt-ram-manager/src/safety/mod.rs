use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use crate::core::error::VrmError;
use crate::core::result::VrmResult;

pub struct SafetyGuard {
    enabled: AtomicBool,
    violation_count: AtomicU64,
}

impl SafetyGuard {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            violation_count: AtomicU64::new(0),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }

    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    pub fn check_safe(&self, ptr: *const u8, size: usize) -> VrmResult<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        if ptr.is_null() {
            self.violation_count.fetch_add(1, Ordering::SeqCst);
            return Err(VrmError::InvalidAllocation("null pointer detected".into()));
        }
        if size == 0 {
            self.violation_count.fetch_add(1, Ordering::SeqCst);
            return Err(VrmError::InvalidAllocation("zero-size access detected".into()));
        }
        if size > 1024 * 1024 * 1024 {
            self.violation_count.fetch_add(1, Ordering::SeqCst);
            return Err(VrmError::InvalidAllocation(format!(
                "suspiciously large access: {} bytes",
                size
            )));
        }
        Ok(())
    }

    pub fn validate_range(&self, offset: u64, len: u64, bound: u64) -> VrmResult<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        if offset >= bound {
            return Err(VrmError::InvalidAllocation(format!(
                "offset {} out of bounds (bound: {})",
                offset, bound
            )));
        }
        let end = offset.saturating_add(len);
        if end > bound {
            return Err(VrmError::InvalidAllocation(format!(
                "range [{}, {}) out of bounds (bound: {})",
                offset, end, bound
            )));
        }
        Ok(())
    }

    pub fn violation_count(&self) -> u64 {
        self.violation_count.load(Ordering::SeqCst)
    }

    pub fn reset_violations(&self) {
        self.violation_count.store(0, Ordering::SeqCst);
    }
}

impl Default for SafetyGuard {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Canary {
    value: u64,
}

impl Canary {
    pub fn new() -> Self {
        Self { value: 0xDEADBEEF_CAFEBABE }
    }

    pub fn with_value(value: u64) -> Self {
        Self { value }
    }

    pub fn verify(&self) -> bool {
        self.value == 0xDEADBEEF_CAFEBABE
    }

    pub fn corrupt(&mut self) {
        self.value = 0;
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Canary {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MemoryBarrier;

impl MemoryBarrier {
    pub fn new() -> Self {
        Self
    }

    pub fn full_fence() {
        std::sync::atomic::fence(Ordering::SeqCst);
    }

    pub fn acquire_fence() {
        std::sync::atomic::fence(Ordering::Acquire);
    }

    pub fn release_fence() {
        std::sync::atomic::fence(Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_guard_default_enabled() {
        let guard = SafetyGuard::new();
        assert!(guard.is_enabled());
    }

    #[test]
    fn test_safety_guard_enable_disable() {
        let guard = SafetyGuard::new();
        guard.disable();
        assert!(!guard.is_enabled());
        guard.enable();
        assert!(guard.is_enabled());
    }

    #[test]
    fn test_check_safe_null() {
        let guard = SafetyGuard::new();
        let result = guard.check_safe(std::ptr::null(), 10);
        assert!(result.is_err());
        assert_eq!(guard.violation_count(), 1);
    }

    #[test]
    fn test_check_safe_zero_size() {
        let guard = SafetyGuard::new();
        let data = [1u8, 2, 3];
        let result = guard.check_safe(data.as_ptr(), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_safe_ok() {
        let guard = SafetyGuard::new();
        let data = [1u8, 2, 3];
        let result = guard.check_safe(data.as_ptr(), 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_check_safe_disabled_bypass() {
        let guard = SafetyGuard::new();
        guard.disable();
        let result = guard.check_safe(std::ptr::null(), 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_range() {
        let guard = SafetyGuard::new();
        assert!(guard.validate_range(0, 100, 200).is_ok());
        assert!(guard.validate_range(0, 100, 50).is_err());
        assert!(guard.validate_range(150, 100, 200).is_err());
        assert!(guard.validate_range(200, 0, 200).is_err());
    }

    #[test]
    fn test_canary_verify() {
        let canary = Canary::new();
        assert!(canary.verify());
    }

    #[test]
    fn test_canary_corrupt() {
        let mut canary = Canary::new();
        canary.corrupt();
        assert!(!canary.verify());
    }

    #[test]
    fn test_canary_with_value() {
        let canary = Canary::with_value(42);
        assert_eq!(canary.value(), 42);
        assert!(!canary.verify());
    }

    #[test]
    fn test_memory_barrier() {
        MemoryBarrier::full_fence();
        MemoryBarrier::acquire_fence();
        MemoryBarrier::release_fence();
        let _barrier = MemoryBarrier::new();
    }

    #[test]
    fn test_reset_violations() {
        let guard = SafetyGuard::new();
        let _ = guard.check_safe(std::ptr::null(), 10);
        assert_eq!(guard.violation_count(), 1);
        guard.reset_violations();
        assert_eq!(guard.violation_count(), 0);
    }
}
