use tesseract_engine::safety::SafetySupervisor;
use tesseract_engine::safety::limits::ResourceLimiter;
use tesseract_engine::safety::watchdog::{ThermalWatchdog, WatchdogAction};
use tesseract_engine::system::thermal::ThermalMetrics;

#[test]
fn test_safety_supervisor_defaults() {
    let sup = SafetySupervisor::new();
    assert_eq!(sup.current_throttle(), 100);
    assert!(!sup.is_emergency_stop());
}

#[test]
fn test_safety_thermal_throttle() {
    let sup = SafetySupervisor::new();
    sup.thermal_throttle();
    assert_eq!(sup.current_throttle(), 50);
}

#[test]
fn test_safety_emergency_throttle() {
    let sup = SafetySupervisor::new();
    sup.emergency_throttle();
    assert_eq!(sup.current_throttle(), 25);
}

#[test]
fn test_safety_emergency_shutdown() {
    let sup = SafetySupervisor::new();
    assert!(!sup.is_emergency_stop());
    sup.emergency_shutdown();
    assert!(sup.is_emergency_stop());
}

#[test]
fn test_safety_clear_emergency() {
    let sup = SafetySupervisor::new();
    sup.emergency_shutdown();
    sup.clear_emergency();
    assert!(!sup.is_emergency_stop());
    assert_eq!(sup.current_throttle(), 100);
}

#[test]
fn test_resource_limiter_memory_check_ok() {
    let limiter = ResourceLimiter::new();
    assert!(limiter.check_memory(1024).is_ok());
}

#[test]
fn test_resource_limiter_memory_check_exceeds() {
    let limiter = ResourceLimiter::new();
    assert!(limiter.check_memory(1024 * 1024 * 1024 + 1).is_err());
}

#[test]
fn test_resource_limiter_sockets() {
    let mut limiter = ResourceLimiter::new();
    for _ in 0..64 {
        assert!(limiter.reserve_socket().is_ok());
    }
    assert!(limiter.reserve_socket().is_err());
    limiter.release_socket();
    assert!(limiter.reserve_socket().is_ok());
}

#[test]
fn test_resource_limiter_tasks() {
    let mut limiter = ResourceLimiter::new();
    for _ in 0..16 {
        assert!(limiter.reserve_task().is_ok());
    }
    assert!(limiter.reserve_task().is_err());
    limiter.release_task();
    assert!(limiter.reserve_task().is_ok());
}

#[test]
fn test_resource_limiter_set_limits() {
    let mut limiter = ResourceLimiter::new();
    limiter.set_limits(512, 8, 4);
    assert!(limiter.check_memory(256).is_ok());
    assert!(limiter.check_memory(1024).is_err());
}

#[test]
fn test_thermal_watchdog_normal() {
    let mut wd = ThermalWatchdog::new();
    let metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 50.0)],
        max_temp: 50.0,
    };
    assert_eq!(wd.evaluate(&metrics), WatchdogAction::Normal);
    assert_eq!(wd.last_temp_c, 50.0);
}

#[test]
fn test_thermal_watchdog_throttle() {
    let mut wd = ThermalWatchdog::new();
    let metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 90.0)],
        max_temp: 90.0,
    };
    assert_eq!(wd.evaluate(&metrics), WatchdogAction::ThrottleTo50Percent);
}

#[test]
fn test_thermal_watchdog_emergency() {
    let mut wd = ThermalWatchdog::new();
    let metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 97.0)],
        max_temp: 97.0,
    };
    assert_eq!(wd.evaluate(&metrics), WatchdogAction::ReleaseCoresAndNotify);
}

#[test]
fn test_thermal_watchdog_critical() {
    let mut wd = ThermalWatchdog::new();
    let metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 101.0)],
        max_temp: 101.0,
    };
    assert_eq!(wd.evaluate(&metrics), WatchdogAction::EmergencyShutdown);
}

#[test]
fn test_thermal_watchdog_state() {
    let mut wd = ThermalWatchdog::new();
    let normal = ThermalMetrics { zones: vec![("cpu".into(), 50.0)], max_temp: 50.0 };
    wd.evaluate(&normal);
    // state not publicly readable directly, but last_temp_c should update
    wd.reset();
    assert_eq!(wd.last_temp_c, 0.0);
}

#[test]
fn test_safety_supervisor_watchdog_limiter_access() {
    let sup = SafetySupervisor::new();
    let _wd = sup.watchdog();
    let _lim = sup.limiter();
}
