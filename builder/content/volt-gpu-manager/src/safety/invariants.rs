use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct InvariantChecker;

impl InvariantChecker {
    pub fn check_t2_is_vram_not_ram(tier: &str) -> VgmResult<()> {
        if tier == "ram" {
            return Err(VgmError::InternalInvariantViolation(
                "T2 compressed pool must be in VRAM, not RAM".into(),
            ));
        }
        Ok(())
    }

    pub fn check_no_restore_without_scratch(scratch_available: bool) -> VgmResult<()> {
        if !scratch_available {
            return Err(VgmError::InternalInvariantViolation(
                "Cannot restore resource without scratch space".into(),
            ));
        }
        Ok(())
    }

    pub fn check_no_desktop_compression(is_desktop_frame: bool) -> VgmResult<()> {
        if is_desktop_frame {
            return Err(VgmError::InternalInvariantViolation(
                "Cannot compress resources in the current desktop frame".into(),
            ));
        }
        Ok(())
    }

    pub fn check_no_current_frame_compression(is_current_frame: bool) -> VgmResult<()> {
        if is_current_frame {
            return Err(VgmError::InternalInvariantViolation(
                "Cannot compress resource used in the current frame".into(),
            ));
        }
        Ok(())
    }

    pub fn check_no_fake_compression_ratio(reported_ratio: f64, actual_ratio: f64) -> VgmResult<()> {
        if (reported_ratio - actual_ratio).abs() > 0.01 {
            return Err(VgmError::InternalInvariantViolation(
                "Compression ratio mismatch — possible fake reporting".into(),
            ));
        }
        Ok(())
    }

    pub fn check_no_panic_on_missing_gpu(gpu_available: bool) -> VgmResult<()> {
        if !gpu_available {
            return Err(VgmError::GpuUnavailable(
                "GPU is not available — graceful handling required".into(),
            ));
        }
        Ok(())
    }
}

pub fn verify_pointer(ptr: *const u8, size: usize) -> VgmResult<()> {
    if ptr.is_null() {
        return Err(VgmError::InternalInvariantViolation(
            "Null pointer dereference prevented".into(),
        ));
    }
    if size > 1 << 30 {
        return Err(VgmError::InternalInvariantViolation(format!(
            "Suspiciously large memory region: {} bytes",
            size
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t2_is_vram_not_ram() {
        assert!(InvariantChecker::check_t2_is_vram_not_ram("vram").is_ok());
        assert!(InvariantChecker::check_t2_is_vram_not_ram("ram").is_err());
    }

    #[test]
    fn test_no_restore_without_scratch() {
        assert!(InvariantChecker::check_no_restore_without_scratch(true).is_ok());
        assert!(InvariantChecker::check_no_restore_without_scratch(false).is_err());
    }

    #[test]
    fn test_no_desktop_compression() {
        assert!(InvariantChecker::check_no_desktop_compression(false).is_ok());
        assert!(InvariantChecker::check_no_desktop_compression(true).is_err());
    }

    #[test]
    fn test_no_current_frame_compression() {
        assert!(InvariantChecker::check_no_current_frame_compression(false).is_ok());
        assert!(InvariantChecker::check_no_current_frame_compression(true).is_err());
    }

    #[test]
    fn test_no_fake_compression_ratio() {
        assert!(InvariantChecker::check_no_fake_compression_ratio(0.5, 0.5).is_ok());
        assert!(InvariantChecker::check_no_fake_compression_ratio(0.9, 0.5).is_err());
    }

    #[test]
    fn test_no_panic_on_missing_gpu() {
        assert!(InvariantChecker::check_no_panic_on_missing_gpu(true).is_ok());
        assert!(InvariantChecker::check_no_panic_on_missing_gpu(false).is_err());
    }

    #[test]
    fn test_verify_pointer_null() {
        assert!(verify_pointer(std::ptr::null(), 10).is_err());
    }

    #[test]
    fn test_verify_pointer_valid() {
        let data = [0u8; 10];
        assert!(verify_pointer(data.as_ptr(), 10).is_ok());
    }
}
