use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_worker_count_invariant_enforces_min() {
    let result = safety::invariants::InvariantChecker::check_worker_count(1, 2, 48);
    assert!(result.is_err());
}

#[test]
fn test_worker_count_invariant_enforces_max() {
    let result = safety::invariants::InvariantChecker::check_worker_count(49, 2, 48);
    assert!(result.is_err());
}

#[test]
fn test_worker_count_invariant_passes() {
    let result = safety::invariants::InvariantChecker::check_worker_count(8, 2, 48);
    assert!(result.is_ok());
}

#[test]
fn test_no_busy_kill_invariant() {
    let result = safety::invariants::InvariantChecker::check_no_busy_kill(WorkerState::Busy);
    assert!(result.is_err());

    let ok = safety::invariants::InvariantChecker::check_no_busy_kill(WorkerState::Idle);
    assert!(ok.is_ok());
}

#[test]
fn test_valid_state_transitions() {
    use volt_dynamic_worker_pool::core::state::WorkerPoolState;
    assert!(WorkerPoolState::Uninitialized.can_transition_to(&WorkerPoolState::Starting));
    assert!(WorkerPoolState::Starting.can_transition_to(&WorkerPoolState::Running));
    assert!(!WorkerPoolState::Uninitialized.can_transition_to(&WorkerPoolState::Running));
    assert!(!WorkerPoolState::Shutdown.can_transition_to(&WorkerPoolState::Running));
}

#[test]
fn test_metrics_non_negative_check() {
    let snap = MetricsSnapshot::new();
    let result = safety::invariants::InvariantChecker::check_metrics_non_negative(&snap);
    assert!(result.is_ok());
}
