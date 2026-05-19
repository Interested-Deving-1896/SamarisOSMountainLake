use std::sync::Arc;

use tesseract_engine::boot::{BootMode, BootSequence};
use tesseract_engine::core::config::TesseractConfig;
use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::protocol::flatbuffer;
use tesseract_engine::protocol::command::CommandPayload;
use tesseract_engine::scheduler::Scheduler;
use tesseract_engine::security::SecurityManager;
use tesseract_engine::telemetry::Telemetry;

fn test_config() -> TesseractConfig {
    let mut config = TesseractConfig::default();
    config.socket_path = format!("/tmp/tesseract-full-{}.sock", std::process::id());
    config.max_workers = 4;
    config.scheduler_tick_ms = 1;
    config.debug_mode = false;
    config
}

#[test]
fn test_end_to_end_sbp_lifecycle() {
    let config = test_config();
    let security = SecurityManager::new(&config);
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    let header = SbpHeader::new(Opcode::Heartbeat, 0, 0x42, 0);
    let cmd = TesseractCommand::new(header, vec![]);
    assert!(security.authorize(&cmd).is_ok());

    let bytes = cmd.to_bytes();
    let decoded = TesseractCommand::from_bytes(&bytes).unwrap();
    let response = scheduler.submit(decoded);
    assert!(response.is_ok(), "full SBP lifecycle failed");

    let snap = telemetry.snapshot();
    assert!(snap.commands_processed > 0);
}

#[test]
fn test_boot_then_full_workload() {
    let boot = BootSequence::new(BootMode::Fast).execute().unwrap();
    let scheduler = boot.scheduler.clone();

    for i in 0..100 {
        let cmd = TesseractCommand::new(
            SbpHeader::new(Opcode::QueryCores, 0, 0x01, 0),
            vec![],
        );
        let resp = scheduler.submit(cmd);
        assert!(resp.is_ok(), "boot workload cmd {i} failed");
    }

    assert!(boot.timing.total_us > 0, "boot should have timing info");
    assert!(!boot.timing.phases.is_empty(), "boot should have phase timestamps");
}

#[test]
fn test_end_to_end_priority_ordering() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry.clone()));

    for _ in 0..4 {
        let low = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, 4, 0x01, 0),
            vec![],
        );
        let high = TesseractCommand::new(
            SbpHeader::new(Opcode::Heartbeat, 0, 0x01, 0),
            vec![],
        );
        let _ = scheduler.submit(low);
        let _ = scheduler.submit(high);
    }

    let snap = telemetry.snapshot();
    assert!(snap.commands_processed > 0);
}

#[test]
fn test_boot_timing_all_phases_recorded() {
    let result = BootSequence::new(BootMode::Fast).execute().unwrap();
    assert!(result.timing.scheduler_init_us > 0, "scheduler init time missing");
    assert!(result.timing.total_us > 0, "total time missing");
    assert!(!result.timing.phases.is_empty(), "should have phase entries");
}

#[test]
fn test_boot_with_asset_root() {
    let seq = BootSequence::new(BootMode::Fast).with_asset_root("/tmp");
    let result = seq.execute().unwrap();
    assert!(result.elapsed.as_nanos() > 0);
}

#[test]
fn test_end_to_end_flatbuffer_pipeline() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(4, 1, telemetry));

    let payload = CommandPayload::RenderRect {
        x: 0, y: 0, w: 100, h: 100,
        border_radius: 0.0, shadow_blur: 0.0,
        shadow_offset_x: 0.0, shadow_offset_y: 0.0,
        fill_r: 255, fill_g: 0, fill_b: 0, fill_a: 255,
        border_r: 0, border_g: 0, border_b: 0, border_width: 0.0,
    };
    let fb_data = flatbuffer::command_to_payload(&payload, Opcode::GpuRender);
    let header = SbpHeader::new(Opcode::GpuRender, 2, 0x01, fb_data.len() as u32);
    let cmd = TesseractCommand::new(header, fb_data);
    let response = scheduler.submit(cmd);
    assert!(response.is_ok(), "FlatBuffer pipeline failed");
}

#[test]
fn test_end_to_end_compute_task_throughput() {
    let telemetry = Arc::new(Telemetry::new());
    let scheduler = Arc::new(Scheduler::new(8, 1, telemetry.clone()));
    let n = 50;

    for i in 0..n {
        let header = SbpHeader::new(Opcode::Heartbeat, (i % 5) as u8, 0x01, 0);
        let cmd = TesseractCommand::new(header, vec![]);
        let resp = scheduler.submit(cmd);
        assert!(resp.is_ok(), "task {i} failed");
    }

    let snap = telemetry.snapshot();
    assert_eq!(snap.commands_processed, n as u64);
}
