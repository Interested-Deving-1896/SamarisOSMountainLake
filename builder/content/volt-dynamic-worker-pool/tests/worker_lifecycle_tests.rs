use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_worker_id_creation() {
    let wid = WorkerId::new(42);
    assert_eq!(wid.as_u32(), 42);
    assert_eq!(wid.to_string(), "worker-42");
}

#[test]
fn test_worker_state_transitions() {
    use worker::lifecycle::WorkerLifecycle;
    let mut lc = WorkerLifecycle::new(WorkerId::new(1));
    assert_eq!(lc.state(), WorkerState::Idle);

    lc.mark_busy().unwrap();
    assert_eq!(lc.state(), WorkerState::Busy);

    lc.stop().unwrap();
    assert_eq!(lc.state(), WorkerState::Stopped);
}

#[test]
fn test_worker_state_invalid_transition() {
    use worker::lifecycle::WorkerLifecycle;
    let mut lc = WorkerLifecycle::new(WorkerId::new(1));
    lc.mark_busy().unwrap();
    lc.stop().unwrap();
    assert!(lc.mark_busy().is_err());
}

#[test]
fn test_worker_state_is_active() {
    assert!(WorkerState::Idle.is_active());
    assert!(WorkerState::Busy.is_active());
    assert!(!WorkerState::Stopped.is_active());
    assert!(!WorkerState::Error.is_active());
}

#[test]
fn test_worker_retire_from_idle() {
    use worker::lifecycle::WorkerLifecycle;
    let mut lc = WorkerLifecycle::new(WorkerId::new(1));
    lc.retire().unwrap();
    assert_eq!(lc.state(), WorkerState::Stopped);
}
