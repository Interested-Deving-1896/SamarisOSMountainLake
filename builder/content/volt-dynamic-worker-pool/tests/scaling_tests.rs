use volt_dynamic_worker_pool::*;
use volt_dynamic_worker_pool::prelude::*;

#[test]
fn test_scaling_decision_constructors() {
    let up = ScalingDecision::ScaleUp { new_count: 8, reason: "load".into() };
    assert!(up.is_scale_up());
    assert_eq!(up.new_count(), Some(8));

    let down = ScalingDecision::ScaleDown { new_count: 2, reason: "idle".into() };
    assert!(down.is_scale_down());
    assert_eq!(down.new_count(), Some(2));

    let none = ScalingDecision::NoChange { reason: "stable".into() };
    assert!(none.is_no_change());
    assert_eq!(none.new_count(), None);
}

#[test]
fn test_scaling_policy_scale_up_decision() {
    let config = config::schema::ScalingSection::default();
    let policy = scaling::policy::ScalingPolicy::new(&config);
    let thermal = scaling::thermal::ThermalState::Normal;
    let decision = policy.should_scale_up(10, 4, 0.50, &thermal);
    assert!(decision.is_scale_up());
}

#[test]
fn test_scaling_policy_no_scale_up_at_max() {
    let config = config::schema::ScalingSection::default();
    let policy = scaling::policy::ScalingPolicy::new(&config);
    let thermal = scaling::thermal::ThermalState::Normal;
    let decision = policy.should_scale_up(10, 48, 0.50, &thermal);
    assert!(decision.is_no_change());
}

#[test]
fn test_scaling_policy_no_scale_down_at_min() {
    let config = config::schema::ScalingSection::default();
    let policy = scaling::policy::ScalingPolicy::new(&config);
    let thermal = scaling::thermal::ThermalState::Normal;
    let decision = policy.should_scale_down(1, 2, 0.10, &thermal, false);
    assert!(decision.is_no_change());
}
