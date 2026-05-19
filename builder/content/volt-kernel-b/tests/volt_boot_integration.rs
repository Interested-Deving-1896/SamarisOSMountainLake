use std::sync::Arc;

use tesseract_engine::boot::{BootMode, BootSequence};
use tesseract_engine::core::config::TesseractConfig;
use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::scheduler::Scheduler;
use tesseract_engine::telemetry::Telemetry;

fn test_config() -> TesseractConfig {
    let mut config = TesseractConfig::default();
    config.socket_path = format!("/tmp/tesseract-vb-test-{}.sock", std::process::id());
    config.max_workers = 8;
    config.scheduler_tick_ms = 1;
    config.debug_mode = false;
    config
}

#[test]
fn test_fast_boot_executes_all_phases() {
    let result = BootSequence::new(BootMode::Fast)
        .with_workers(8)
        .execute();

    assert!(result.is_ok(), "fast boot should succeed");
    let result = result.unwrap();

    // Scheduler running
    assert!(result.scheduler.worker_count() >= 8);

    // Timing measured
    assert!(result.elapsed.as_micros() > 0, "boot should have non-zero duration");

    println!(
        "VOLT BOOT integration: {}ms, {} workers",
        result.elapsed.as_millis(),
        result.scheduler.worker_count()
    );
}

#[test]
fn test_boot_then_normal_init_is_safe() {
    // Simulate: run boot sequence, then start normal engine init
    let fast = BootSequence::new(BootMode::Fast).execute();
    assert!(fast.is_ok());

    let _config = test_config();
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    let cmd = TesseractCommand::new(
        SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
        vec![],
    );

    let response = scheduler.submit(cmd);
    assert!(response.is_ok(), "scheduler should work after boot sequence");
}

#[test]
fn test_multiple_fast_boot_calls() {
    // Boot sequence should be idempotent
    for i in 0..5 {
        let result = BootSequence::new(BootMode::Fast).execute();
        assert!(result.is_ok(), "fast boot iteration {i} failed");
    }
}

#[test]
fn test_boot_scheduler_can_process_commands() {
    let result = BootSequence::new(BootMode::Fast)
        .with_workers(8)
        .execute()
        .unwrap();

    for i in 0..20 {
        let cmd = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, (i % 5) as u8, 0x01, 0),
            vec![],
        );
        let response = result.scheduler.submit(cmd);
        assert!(response.is_ok(), "cmd {i} through boot scheduler failed");
    }
}

#[test]
fn test_boot_mode_with_telemetry() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(8, 1, telemetry.clone()));

    for _i in 0..10 {
        let cmd = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
            vec![],
        );
        let _ = scheduler.submit(cmd);
    }

    let snap = telemetry.snapshot();
    assert_eq!(snap.commands_processed, 10);
}
