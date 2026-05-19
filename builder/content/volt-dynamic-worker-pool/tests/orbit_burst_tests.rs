use volt_dynamic_worker_pool::*;

#[test]
fn test_orbit_burst_request_creation() {
    use orbit::burst_controller::OrbitBurstRequest;
    let req = OrbitBurstRequest::new("job-1".into(), 5, 500, "test burst".into());
    assert_eq!(req.job_id, "job-1");
    assert_eq!(req.priority_boost, 5);
    assert_eq!(req.reason, "test burst");
}

#[test]
fn test_orbit_burst_accepted() {
    use orbit::burst_controller::{OrbitBurstController, OrbitBurstRequest};
    let ctrl = OrbitBurstController::new(10, 1000, 5);
    let req = OrbitBurstRequest::new("job-1".into(), 2, 500, "test".into());
    let decision = ctrl.request_burst(&req);
    assert!(decision.is_accepted());
    assert_eq!(ctrl.active_bursts(), 1);
}

#[test]
fn test_orbit_burst_release() {
    use orbit::burst_controller::{OrbitBurstController, OrbitBurstRequest};
    let ctrl = OrbitBurstController::new(10, 0, 5);
    let req = OrbitBurstRequest::new("job-1".into(), 1, 100, "test".into());
    assert!(ctrl.request_burst(&req).is_accepted());
    assert_eq!(ctrl.active_bursts(), 1);
    ctrl.release_burst();
    assert_eq!(ctrl.active_bursts(), 0);
}

#[test]
fn test_orbit_burst_limit_reached() {
    use orbit::burst_controller::{OrbitBurstController, OrbitBurstRequest, OrbitBurstDecision};
    let ctrl = OrbitBurstController::new(2, 0, 5);
    let req = OrbitBurstRequest::new("job".into(), 1, 100, "test".into());
    assert!(ctrl.request_burst(&req).is_accepted());
    assert!(ctrl.request_burst(&req).is_accepted());
    assert_eq!(
        ctrl.request_burst(&req),
        OrbitBurstDecision::RejectedLimitReached
    );
}

#[test]
fn test_orbit_burst_priority_boost_capped() {
    use orbit::burst_controller::OrbitBurstRequest;
    let req = OrbitBurstRequest::new("j".into(), 99, 100, "t".into());
    assert_eq!(req.priority_boost, 10);
}
