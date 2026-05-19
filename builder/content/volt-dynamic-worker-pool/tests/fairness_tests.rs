use volt_dynamic_worker_pool::*;

#[test]
fn test_aging_policy_defaults() {
    use priority::aging::AgingPolicy;
    let policy = AgingPolicy::default();
    assert!(policy.enabled);
    assert_eq!(policy.aging_after_ms, 5_000);
    assert_eq!(policy.starvation_limit_ms, 30_000);
}

#[test]
fn test_aging_policy_starvation_detection() {
    use priority::aging::AgingPolicy;
    let policy = AgingPolicy::new();
    assert!(!policy.is_starved(10_000));
    assert!(policy.is_starved(30_000));
}

#[test]
fn test_aging_policy_boost_at_starvation() {
    use priority::aging::AgingPolicy;
    use priority::level::PriorityLevel;
    let policy = AgingPolicy::new();
    assert_eq!(policy.compute_boost(30_000), Some(PriorityLevel::High));
    assert_eq!(policy.compute_boost(1_000), None);
}

#[test]
fn test_fairness_policy_tracks_starvation() {
    use priority::fairness::FairnessPolicy;
    let mut policy = FairnessPolicy::new();
    assert_eq!(policy.check_fairness(1_000), None);
    assert_eq!(policy.starvation_count, 0);
}

#[test]
fn test_fairness_snapshot() {
    use priority::fairness::FairnessPolicy;
    let mut policy = FairnessPolicy::new();
    policy.record_starvation();
    policy.record_boost();
    let snap = policy.snapshot();
    assert_eq!(snap.starvation_count, 1);
    assert_eq!(snap.boost_count, 1);
}
