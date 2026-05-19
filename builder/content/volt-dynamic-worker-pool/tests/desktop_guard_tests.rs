use volt_dynamic_worker_pool::*;

#[test]
fn test_frame_pressure_ordering() {
    use desktop_guard::frame_pressure::FramePressure;
    assert!(FramePressure::None < FramePressure::Low);
    assert!(FramePressure::Low < FramePressure::Medium);
    assert!(FramePressure::Medium < FramePressure::High);
    assert!(FramePressure::High < FramePressure::Critical);
}

#[test]
fn test_frame_pressure_from_f64() {
    use desktop_guard::frame_pressure::FramePressure;
    assert_eq!(FramePressure::from_f64(0.0), FramePressure::None);
    assert_eq!(FramePressure::from_f64(0.2), FramePressure::Low);
    assert_eq!(FramePressure::from_f64(0.5), FramePressure::Medium);
    assert_eq!(FramePressure::from_f64(0.7), FramePressure::High);
    assert_eq!(FramePressure::from_f64(0.9), FramePressure::Critical);
}

#[test]
fn test_latency_guard_frame_tracking() {
    use desktop_guard::latency_guard::LatencyGuard;
    let guard = LatencyGuard::new(16, 4, 2);
    guard.record_frame_time(10);
    guard.record_frame_time(20);
    guard.record_frame_time(30);
    assert_eq!(guard.avg_frame_time_ms(), 20);
    assert!(guard.is_exceeding_budget());
}

#[test]
fn test_desktop_protection_blocks_orbit_burst_on_critical() {
    use desktop_guard::protection::DesktopProtection;
    let dp = DesktopProtection::new(16, 4, 2, true);
    assert!(!dp.block_orbit_burst());
    dp.set_pressure_from_frame_time(40);
    assert!(dp.block_orbit_burst());
}

#[test]
fn test_desktop_guard_throttling() {
    use desktop_guard::frame_pressure::{DesktopGuard, FramePressure};
    let guard = DesktopGuard::new(true, FramePressure::High, 2.0);
    assert!(!guard.should_throttle());
    guard.set_pressure(FramePressure::High);
    assert!(guard.should_throttle());
    assert!((guard.yield_multiplier() - 2.0).abs() < 0.001);
}
