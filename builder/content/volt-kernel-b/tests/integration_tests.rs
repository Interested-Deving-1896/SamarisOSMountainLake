use std::sync::Arc;

use tesseract_engine::core::config::TesseractConfig;
use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::scheduler::Scheduler;
use tesseract_engine::security::SecurityManager;
use tesseract_engine::system::SystemMonitor;
use tesseract_engine::telemetry::Telemetry;
fn test_config() -> TesseractConfig {
    let mut config = TesseractConfig::default();
    config.socket_path = format!("/tmp/tesseract-integration-{}.sock", std::process::id());
    config.max_workers = 4;
    config.scheduler_tick_ms = 1;
    config.debug_mode = false;
    config
}

#[test]
fn test_telemetry_record_error() {
    let telemetry = Arc::new(Telemetry::new());
    telemetry.record_error();
    let snap = telemetry.snapshot();
    assert_eq!(snap.errors_count, 1);
}

#[test]
fn test_telemetry_reset() {
    let telemetry = Arc::new(Telemetry::new());
    telemetry.record_command(0x01, 100, 500);
    telemetry.reset();
    let snap = telemetry.snapshot();
    assert_eq!(snap.commands_processed, 0);
}

#[test]
fn test_full_command_lifecycle() {
    let _config = test_config();
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 0x42, 0),
        vec![],
    );

    let response = scheduler.submit(cmd);
    assert!(response.is_ok(), "heartbeat should succeed: {:?}", response);

    let snapshot = telemetry.snapshot();
    assert!(snapshot.commands_processed > 0, "telemetry should record commands");
}

#[test]
fn test_query_cores_execution() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry));

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::QueryCores, 0, 0x01, 0),
        vec![],
    );

    let response = scheduler.submit(cmd);
    assert!(response.is_ok(), "query cores should succeed: {:?}", response);
}

#[test]
fn test_multiple_commands_processed() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    for i in 0..10 {
        let cmd = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, (i % 5) as u8, 0x01, 0),
            vec![],
        );
        let response = scheduler.submit(cmd);
        assert!(response.is_ok(), "cmd {i} failed: {:?}", response);
    }

    let snapshot = telemetry.snapshot();
    assert_eq!(snapshot.commands_processed, 10);
}

#[test]
fn test_security_audit_log() {
    let config = test_config();
    let security = SecurityManager::new(&config);

    let valid_cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 2, 0x01, 0),
        vec![],
    );
    assert!(security.authorize(&valid_cmd).is_ok());

    let audit = security.audit().read();
    let entries = audit.query(None);
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].app_id, 0x01);
    assert!(entries[0].allowed);
}

#[test]
fn test_system_metrics_collection() {
    let monitor = SystemMonitor::new();

    let snapshot = monitor.collect_all();
    assert!(snapshot.is_ok(), "metrics should collect: {:?}", snapshot);

    let snapshot = snapshot.unwrap();
    // CPU count should be at least 1
    assert!(snapshot.cpu.count > 0, "should detect at least 1 cpu core");
}

#[test]
fn test_multiple_priority_levels() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry));

    let priorities = [0, 1, 2, 3, 4];

    for prio in &priorities {
        let cmd = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, *prio, 0x01, 0),
            vec![],
        );
        let response = scheduler.submit(cmd);
        assert!(response.is_ok(), "priority {prio} failed");
    }
}

#[test]
fn test_thermal_watchdog_thresholds() {
    use tesseract_engine::safety::watchdog::{ThermalWatchdog, WatchdogAction};
    use tesseract_engine::system::thermal::ThermalMetrics;

    let mut watchdog = ThermalWatchdog::new();

    let normal_metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 50.0)],
        max_temp: 50.0,
    };
    assert_eq!(watchdog.evaluate(&normal_metrics), WatchdogAction::Normal);

    let throttle_metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 90.0)],
        max_temp: 90.0,
    };
    assert_eq!(watchdog.evaluate(&throttle_metrics), WatchdogAction::ThrottleTo50Percent);

    let emergency_metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 97.0)],
        max_temp: 97.0,
    };
    assert_eq!(watchdog.evaluate(&emergency_metrics), WatchdogAction::ReleaseCoresAndNotify);

    let critical_metrics = ThermalMetrics {
        zones: vec![("cpu".into(), 101.0)],
        max_temp: 101.0,
    };
    assert_eq!(watchdog.evaluate(&critical_metrics), WatchdogAction::EmergencyShutdown);
}
