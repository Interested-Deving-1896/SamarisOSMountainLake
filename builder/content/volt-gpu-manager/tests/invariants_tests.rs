use volt_gpu_manager::safety::invariants::InvariantChecker;
use volt_gpu_manager::core::VgmError;

#[test]
fn t2_is_vram_not_ram() {
    assert!(InvariantChecker::check_t2_is_vram_not_ram("vram").is_ok());
    let err = InvariantChecker::check_t2_is_vram_not_ram("ram").unwrap_err();
    assert!(matches!(err, VgmError::InternalInvariantViolation(_)));
}

#[test]
fn no_restore_without_scratch_budget() {
    assert!(InvariantChecker::check_no_restore_without_scratch(true).is_ok());
    let err = InvariantChecker::check_no_restore_without_scratch(false).unwrap_err();
    assert!(matches!(err, VgmError::InternalInvariantViolation(_)));
}

#[test]
fn no_desktop_compression() {
    assert!(InvariantChecker::check_no_desktop_compression(false).is_ok());
    let err = InvariantChecker::check_no_desktop_compression(true).unwrap_err();
    assert!(matches!(err, VgmError::InternalInvariantViolation(_)));
}

#[test]
fn no_current_frame_compression() {
    assert!(InvariantChecker::check_no_current_frame_compression(false).is_ok());
    let err = InvariantChecker::check_no_current_frame_compression(true).unwrap_err();
    assert!(matches!(err, VgmError::InternalInvariantViolation(_)));
}

#[test]
fn no_fake_ratio() {
    assert!(InvariantChecker::check_no_fake_compression_ratio(0.5, 0.5).is_ok());
    let err = InvariantChecker::check_no_fake_compression_ratio(0.9, 0.5).unwrap_err();
    assert!(matches!(err, VgmError::InternalInvariantViolation(_)));
}

#[test]
fn no_invalid_state_transition() {
    use volt_gpu_manager::core::VgmState;
    let state = VgmState::Shutdown;
    assert!(!state.can_transition_to(&VgmState::Running));
    assert!(state.can_transition_to(&VgmState::Uninitialized));
}

#[test]
fn no_panic_on_missing_gpu() {
    let err = InvariantChecker::check_no_panic_on_missing_gpu(false).unwrap_err();
    assert!(matches!(err, VgmError::GpuUnavailable(_)));
    assert!(InvariantChecker::check_no_panic_on_missing_gpu(true).is_ok());
}
