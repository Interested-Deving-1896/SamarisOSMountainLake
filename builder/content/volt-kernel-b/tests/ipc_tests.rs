use std::sync::Arc;

use tesseract_engine::core::config::TesseractConfig;
use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::scheduler::Scheduler;
use tesseract_engine::security::SecurityManager;
use tesseract_engine::telemetry::Telemetry;

fn test_config() -> TesseractConfig {
    let mut config = TesseractConfig::default();
    config.socket_path = format!("/tmp/tesseract-test-{}.sock", std::process::id());
    config.max_workers = 2;
    config.scheduler_tick_ms = 1;
    config.debug_mode = false;
    config
}

#[test]
fn test_scheduler_submit_and_response() {
    let _config = test_config();
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(2, 1, telemetry));

    let header = SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0);
    let cmd = TesseractCommand::new(header, vec![]);

    let response = scheduler.submit(cmd);

    assert!(response.is_ok(), "scheduler should process heartbeat: {:?}", response);
}

#[test]
fn test_scheduler_priority_processing() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry));

    let low_cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 4, 0x01, 0),
        vec![],
    );

    let high_cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
        vec![],
    );

    let r1 = scheduler.submit(low_cmd);
    let r2 = scheduler.submit(high_cmd);

    assert!(r1.is_ok());
    assert!(r2.is_ok());
}

#[test]
fn test_security_authorization_valid() {
    let config = test_config();
    let security = SecurityManager::new(&config);

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 2, 0x01, 0),
        vec![],
    );

    assert!(security.authorize(&cmd).is_ok());
}

#[test]
fn test_security_rejects_zero_app_id() {
    let config = test_config();
    let security = SecurityManager::new(&config);

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 2, 0, 0),
        vec![],
    );

    assert!(security.authorize(&cmd).is_err());
}

#[test]
fn test_security_rejects_invalid_priority() {
    let config = test_config();
    let security = SecurityManager::new(&config);

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 255, 0x01, 0),
        vec![],
    );

    assert!(security.authorize(&cmd).is_err());
}

#[test]
fn test_security_rejects_large_payload() {
    let config = test_config();
    let security = SecurityManager::new(&config);

    let payload = vec![0u8; 11 * 1024 * 1024];
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 2, 0x01, payload.len() as u32),
        payload,
    );

    assert!(security.authorize(&cmd).is_err());
}

#[test]
fn test_quota_check_memory_exceeded() {
    use tesseract_engine::core::config::TesseractConfig;
    use tesseract_engine::security::quotas::ResourceQuotas;

    let config = TesseractConfig::default();
    let mut quotas = ResourceQuotas::new(&config);
    assert!(quotas.check_memory_quota(0x01, 1024 * 1024 * 1024).is_err());
}

#[test]
fn test_quota_release_memory() {
    use tesseract_engine::core::config::TesseractConfig;
    use tesseract_engine::security::quotas::ResourceQuotas;

    let config = TesseractConfig::default();
    let mut quotas = ResourceQuotas::new(&config);
    quotas.check_memory_quota(0x01, 1024 * 1024).unwrap();
    quotas.release_memory(0x01, 1024 * 1024);
}

#[test]
fn test_quota_set_and_get() {
    use tesseract_engine::core::config::TesseractConfig;
    use tesseract_engine::security::quotas::{ResourceQuotas, AppQuota};

    let config = TesseractConfig::default();
    let mut quotas = ResourceQuotas::new(&config);
    let q = AppQuota {
        max_memory_bytes: 1,
        max_concurrent_tasks: 1,
        max_commands_per_sec: 1,
        memory_used: 0,
        tasks_running: 0,
        commands_in_window: 0,
        window_start: std::time::Instant::now(),
    };
    quotas.set_quota(0xFF, q);
    let got = quotas.get_quota(0xFF);
    assert!(got.is_some());
    assert_eq!(got.unwrap().max_memory_bytes, 1);
}

#[test]
fn test_quota_reset() {
    use tesseract_engine::core::config::TesseractConfig;
    use tesseract_engine::security::quotas::ResourceQuotas;

    let config = TesseractConfig::default();
    let mut quotas = ResourceQuotas::new(&config);
    quotas.check_command_quota(0x01).unwrap();
    quotas.reset(0x01);
    assert!(quotas.get_quota(0x01).is_none());
}

#[test]
fn test_quota_reset_all() {
    use tesseract_engine::core::config::TesseractConfig;
    use tesseract_engine::security::quotas::ResourceQuotas;

    let config = TesseractConfig::default();
    let mut quotas = ResourceQuotas::new(&config);
    quotas.check_command_quota(0x01).unwrap();
    quotas.check_command_quota(0x02).unwrap();
    quotas.reset_all();
    assert!(quotas.get_quota(0x01).is_none());
    assert!(quotas.get_quota(0x02).is_none());
}

#[test]
fn test_audit_query_filtered() {
    use tesseract_engine::security::audit::{AuditLog, AuditEntry};
    let mut log = AuditLog::new(100);
    log.log(AuditEntry { timestamp: 1, app_id: 0x01, opcode: 0x0C, action: "test".into(), allowed: true, reason: String::new() });
    log.log(AuditEntry { timestamp: 2, app_id: 0x02, opcode: 0x0C, action: "test".into(), allowed: false, reason: "denied".into() });
    let filtered = log.query(Some(0x01));
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].app_id, 0x01);
}

#[test]
fn test_audit_recent() {
    use tesseract_engine::security::audit::{AuditLog, AuditEntry};
    let mut log = AuditLog::new(100);
    for i in 0..10 {
        log.log(AuditEntry { timestamp: i, app_id: 0x01, opcode: 0x0C, action: "test".into(), allowed: true, reason: String::new() });
    }
    let recent = log.recent(3);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].timestamp, 9);
}

#[test]
fn test_audit_export_json() {
    use tesseract_engine::security::audit::{AuditLog, AuditEntry};
    let mut log = AuditLog::new(100);
    log.log(AuditEntry { timestamp: 1, app_id: 0x01, opcode: 0x0C, action: "test".into(), allowed: true, reason: String::new() });
    let json = log.export_json();
    assert!(json.contains("0x00000001") || json.contains("app_id"));
    assert!(!json.is_empty());
}

#[test]
fn test_audit_len_clear() {
    use tesseract_engine::security::audit::{AuditLog, AuditEntry};
    let mut log = AuditLog::new(100);
    for _ in 0..5 {
        log.log(AuditEntry { timestamp: 1, app_id: 0x01, opcode: 0x0C, action: "test".into(), allowed: true, reason: String::new() });
    }
    assert_eq!(log.len(), 5);
    log.clear();
    assert_eq!(log.len(), 0);
}

#[test]
fn test_sandbox_mem_alloc_system_app_rejected() {
    use tesseract_engine::security::sandbox::CommandSandbox;
    let sandbox = CommandSandbox::new();
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::MemAlloc, 2, 0xFFFFFFFF, 0),
        vec![],
    );
    assert!(sandbox.validate(&cmd).is_err());
}

#[test]
fn test_sandbox_cpu_reserve_requires_critical() {
    use tesseract_engine::security::sandbox::CommandSandbox;
    let sandbox = CommandSandbox::new();
    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::CpuReserve, 2, 0x01, 0),
        vec![],
    );
    assert!(sandbox.validate(&cmd).is_err());
}

#[test]
fn test_sandbox_invalid_opcode_rejected() {
    use tesseract_engine::security::sandbox::CommandSandbox;
    let sandbox = CommandSandbox::new();
    let mut header = SbpHeader::new(Opcode::Heartbeat, 2, 0x01, 0);
    header.opcode = 0xFF;
    let cmd = TesseractCommand::new(header, vec![]);
    assert!(sandbox.validate(&cmd).is_err());
}

#[test]
fn test_profiler_enabled_disabled() {
    use tesseract_engine::telemetry::profiler::Profiler;
    let mut p = Profiler::new(true);
    assert!(p.is_enabled());
    p.set_enabled(false);
    assert!(!p.is_enabled());
}

#[test]
fn test_profiler_scope_elapsed_us() {
    use tesseract_engine::telemetry::profiler::Profiler;
    let p = Profiler::new(true);
    let scope = p.scope("test");
    std::thread::sleep(std::time::Duration::from_micros(10));
    let elapsed = scope.elapsed_us();
    assert!(elapsed.is_some());
    assert!(elapsed.unwrap() > 0);
}

#[test]
fn test_profiler_disabled_scope_returns_none() {
    use tesseract_engine::telemetry::profiler::Profiler;
    let p = Profiler::new(false);
    let scope = p.scope("test");
    assert!(scope.elapsed_us().is_none());
}
