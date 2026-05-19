use volt_gpu_manager::thermal::{ThermalState, ThermalLevel, ThermalPolicy};
use volt_gpu_manager::scheduler::GpuPriority;

#[test]
fn temperature_maps_to_state() {
    let s = ThermalState::new(ThermalLevel::Hot, 72.0);
    assert_eq!(s.level, ThermalLevel::Hot);
    assert!((s.temperature_c - 72.0).abs() < 0.001);
}

#[test]
fn hot_reduces_normal() {
    let policy = ThermalPolicy::default();
    let state = ThermalState::new(ThermalLevel::Hot, 72.0);
    assert!(policy.should_block_priority(&state, GpuPriority::Idle));
    assert!(!policy.should_block_priority(&state, GpuPriority::Critical));
}

#[test]
fn throttle_disables_burst() {
    let policy = ThermalPolicy::default();
    let state = ThermalState::new(ThermalLevel::Throttle, 85.0);
    assert!(policy.should_block_priority(&state, GpuPriority::Idle));
    assert!(!policy.should_block_priority(&state, GpuPriority::Normal));
}

#[test]
fn emergency_stops_non_critical_compute() {
    let policy = ThermalPolicy::default();
    let state = ThermalState::new(ThermalLevel::Emergency, 95.0);
    assert!(policy.should_block_priority(&state, GpuPriority::Critical));
    assert!(policy.should_block_priority(&state, GpuPriority::Normal));
    assert!(policy.should_cpu_fallback(&state));
}

#[test]
fn unknown_uses_conservative() {
    let state = ThermalState::new(ThermalLevel::Unknown, 0.0);
    assert!(!state.should_reduce_compute());
    assert!(!state.should_stop_compute());
    assert_eq!(state.name(), "unknown");
}
