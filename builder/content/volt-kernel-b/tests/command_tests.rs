use tesseract_engine::protocol::header::{Flags, SbpHeader};
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::protocol::flatbuffer;
use tesseract_engine::protocol::command::{CommandPayload, ComputeKind, StreamType, QueryKind};
use tesseract_engine::compute_bridge::task::ComputeKind as TaskComputeKind;

#[test]
fn test_cmd_to_bytes_from_bytes_roundtrip() {
    let header = SbpHeader::new(Opcode::GpuRender, 2, 0xDEADBEEF, 42);
    let payload = vec![0xAB; 42];
    let cmd = TesseractCommand::new(header, payload.clone());

    let bytes = cmd.to_bytes();
    let decoded = TesseractCommand::from_bytes(&bytes).unwrap();

    assert_eq!(decoded.header.opcode, cmd.header.opcode);
    assert_eq!(decoded.header.priority, cmd.header.priority);
    assert_eq!(decoded.header.app_id, cmd.header.app_id);
    assert_eq!(decoded.header.payload_len, cmd.header.payload_len);
    assert_eq!(decoded.payload, payload);
}

#[test]
fn test_cmd_is_response() {
    let header = SbpHeader::new(Opcode::Heartbeat, 0, 1, 0)
        .with_flags(Flags::RESPONSE);
    let cmd = TesseractCommand::new(header, vec![]);
    assert!(cmd.is_response());
    assert!(!cmd.is_error());
}

#[test]
fn test_cmd_is_error() {
    let header = SbpHeader::new(Opcode::Heartbeat, 0, 1, 0)
        .with_flags(Flags::RESPONSE | Flags::ERROR);
    let cmd = TesseractCommand::new(header, vec![]);
    assert!(cmd.is_error());
    assert!(cmd.is_response());
}

#[test]
fn test_cmd_opcode_appid_priority() {
    let header = SbpHeader::new(Opcode::QueryCores, 0, 0x42, 0);
    let cmd = TesseractCommand::new(header, vec![]);
    assert_eq!(cmd.opcode().unwrap(), Opcode::QueryCores);
    assert_eq!(cmd.app_id(), 0x42);
    assert_eq!(cmd.priority(), 0);
}

#[test]
fn test_cmd_payload_len() {
    let payload = vec![1, 2, 3, 4, 5];
    let header = SbpHeader::new(Opcode::MemAlloc, 2, 1, 5);
    let cmd = TesseractCommand::new(header, payload);
    assert_eq!(cmd.payload_len(), 5);
}

#[test]
fn test_cmd_from_bytes_truncated_header() {
    let result = TesseractCommand::from_bytes(&[0u8; 10]);
    assert!(result.is_err());
}

#[test]
fn test_command_payload_render_rect_roundtrip() {
    let payload = CommandPayload::RenderRect {
        x: 10, y: 20, w: 800, h: 600,
        border_radius: 8.0, shadow_blur: 4.0,
        shadow_offset_x: 2.0, shadow_offset_y: 2.0,
        fill_r: 255, fill_g: 0, fill_b: 0, fill_a: 255,
        border_r: 0, border_g: 0, border_b: 0, border_width: 1.0,
    };
    let data = flatbuffer::command_to_payload(&payload, Opcode::GpuRender);
    let decoded = flatbuffer::payload_to_command(Opcode::GpuRender, &data).unwrap();
    match decoded {
        CommandPayload::RenderRect { x, y, w, h, .. } => {
            assert_eq!(x, 10);
            assert_eq!(y, 20);
            assert_eq!(w, 800);
            assert_eq!(h, 600);
        }
        _ => panic!("expected RenderRect"),
    }
}

#[test]
fn test_command_payload_compute_task_roundtrip() {
    let input = b"test data".to_vec();
    let payload = CommandPayload::ComputeTask {
        kind: ComputeKind::HashSha256,
        data: input.clone(),
    };
    let data = flatbuffer::command_to_payload(&payload, Opcode::CpuExec);
    let decoded = flatbuffer::payload_to_command(Opcode::CpuExec, &data).unwrap();
    match decoded {
        CommandPayload::ComputeTask { kind, data } => {
            assert_eq!(kind, ComputeKind::HashSha256);
            assert_eq!(data, input);
        }
        _ => panic!("expected ComputeTask"),
    }
}

#[test]
fn test_command_payload_mem_alloc_roundtrip() {
    let payload = CommandPayload::MemAlloc { size: 65536 };
    let data = flatbuffer::command_to_payload(&payload, Opcode::MemAlloc);
    let decoded = flatbuffer::payload_to_command(Opcode::MemAlloc, &data).unwrap();
    match decoded {
        CommandPayload::MemAlloc { size } => assert_eq!(size, 65536),
        _ => panic!("expected MemAlloc"),
    }
}

#[test]
fn test_command_payload_heartbeat_roundtrip() {
    let payload = CommandPayload::Heartbeat;
    let data = flatbuffer::command_to_payload(&payload, Opcode::Heartbeat);
    let decoded = flatbuffer::payload_to_command(Opcode::Heartbeat, &data).unwrap();
    match decoded {
        CommandPayload::Heartbeat => {}
        _ => panic!("expected Heartbeat"),
    }
}

#[test]
fn test_command_payload_context_create_roundtrip() {
    let payload = CommandPayload::ContextCreate { name: "test-ctx".into() };
    let data = flatbuffer::command_to_payload(&payload, Opcode::ContextCreate);
    let decoded = flatbuffer::payload_to_command(Opcode::ContextCreate, &data).unwrap();
    match decoded {
        CommandPayload::ContextCreate { name } => assert_eq!(name, "test-ctx"),
        _ => panic!("expected ContextCreate"),
    }
}

#[test]
fn test_command_payload_query_roundtrip() {
    let payload = CommandPayload::Query { kind: QueryKind::Cores };
    let data = flatbuffer::command_to_payload(&payload, Opcode::QueryCores);
    let decoded = flatbuffer::payload_to_command(Opcode::QueryCores, &data).unwrap();
    match decoded {
        CommandPayload::Query { kind } => assert_eq!(kind, QueryKind::Cores),
        _ => panic!("expected Query"),
    }
}

#[test]
fn test_parse_packet_all_fields() {
    let data = flatbuffer::command_to_payload(
        &CommandPayload::Empty,
        Opcode::Heartbeat,
    );
    let packet = flatbuffer::parse_packet(&data).unwrap();
    assert!(packet.int32_values.is_empty());
    assert!(packet.uint32_values.is_empty());
    assert!(packet.byte_data.is_empty());
    assert!(packet.string_value.is_empty());
}

#[test]
fn test_compute_kind_from_to_byte() {
    let kinds = [
        TaskComputeKind::Compress,
        TaskComputeKind::Decompress,
        TaskComputeKind::HashSha256,
        TaskComputeKind::EncryptAes256Gcm,
        TaskComputeKind::DecryptAes256Gcm,
        TaskComputeKind::ImageBlur,
        TaskComputeKind::ImageResize,
        TaskComputeKind::ImageFilter,
        TaskComputeKind::Custom(42),
    ];
    for kind in &kinds {
        assert_eq!(TaskComputeKind::from_byte(kind.to_byte()), *kind);
    }
}

#[test]
fn test_stream_type_variants() {
    let v = StreamType::Video { codec: 1 };
    let a = StreamType::Audio { sample_rate: 44100, channels: 2 };
    match v { StreamType::Video { codec } => assert_eq!(codec, 1), _ => panic!() }
    match a { StreamType::Audio { sample_rate, channels } => {
        assert_eq!(sample_rate, 44100);
        assert_eq!(channels, 2);
    } _ => panic!() }
}

#[test]
fn test_query_kind_variants() {
    assert_eq!(QueryKind::Cores as u8, 0);
    assert_eq!(QueryKind::Gpu as u8, 1);
    assert_eq!(QueryKind::Thermal as u8, 2);
}
