use tesseract_engine::protocol::header::{Flags, SbpHeader, SBP_MAGIC, SBP_VERSION};
use tesseract_engine::protocol::opcodes::Opcode;

#[test]
fn test_header_encode_decode_roundtrip() {
    let header = SbpHeader::new(Opcode::Heartbeat, 2, 0x12345678, 100);
    let encoded = header.encode();
    let decoded = SbpHeader::decode(&encoded).unwrap();

    assert_eq!(decoded.magic, SBP_MAGIC);
    assert_eq!(decoded.version, SBP_VERSION);
    assert_eq!(decoded.opcode, Opcode::Heartbeat as u8);
    assert_eq!(decoded.priority, 2);
    assert_eq!(decoded.app_id, 0x12345678);
    assert_eq!(decoded.payload_len, 100);
}

#[test]
fn test_header_checksum() {
    let header = SbpHeader::new(Opcode::GpuRender, 0, 1, 0);
    let encoded = header.encode();
    let checksum = encoded[15];
    let computed: u8 = encoded[..15].iter().fold(0, |a, b| a ^ b);
    assert_eq!(checksum, computed);
}

#[test]
fn test_header_invalid_checksum() {
    let mut encoded = SbpHeader::new(Opcode::Heartbeat, 2, 0, 0).encode();
    encoded[15] ^= 0xFF;
    assert!(SbpHeader::decode(&encoded).is_err());
}

#[test]
fn test_header_invalid_magic() {
    let mut encoded = SbpHeader::new(Opcode::Heartbeat, 2, 0, 0).encode();
    encoded[0] = 0x00;
    assert!(SbpHeader::decode(&encoded).is_err());
}

#[test]
fn test_all_opcodes() {
    for (byte, expected) in &[
        (0x01, Opcode::GpuRender),
        (0x02, Opcode::GpuCompute),
        (0x03, Opcode::CpuReserve),
        (0x04, Opcode::CpuRelease),
        (0x05, Opcode::CpuExec),
        (0x06, Opcode::MemAlloc),
        (0x07, Opcode::MemFree),
        (0x08, Opcode::StreamVideo),
        (0x09, Opcode::StreamAudio),
        (0x0A, Opcode::QueryCores),
        (0x0B, Opcode::QueryGpu),
        (0x0C, Opcode::Heartbeat),
        (0x0F, Opcode::ThermalStatus),
        (0x30, Opcode::ContextCreate),
        (0x31, Opcode::ContextShare),
    ] {
        assert_eq!(Opcode::from_byte(*byte).unwrap(), *expected);
    }
}

#[test]
fn test_unknown_opcode() {
    assert!(Opcode::from_byte(0xFF).is_err());
}

#[test]
fn test_flags_response() {
    let header = SbpHeader::new(Opcode::Heartbeat, 2, 0, 0)
        .with_flags(Flags::RESPONSE);
    let encoded = header.encode();
    let decoded = SbpHeader::decode(&encoded).unwrap();
    assert!(decoded.flags.contains(Flags::RESPONSE));
    assert!(!decoded.flags.contains(Flags::ERROR));
}

#[test]
fn test_flags_error() {
    let header = SbpHeader::new(Opcode::Heartbeat, 2, 0, 0)
        .with_flags(Flags::RESPONSE | Flags::ERROR);
    let encoded = header.encode();
    let decoded = SbpHeader::decode(&encoded).unwrap();
    assert!(decoded.flags.contains(Flags::ERROR));
}

#[test]
fn test_opcode_names() {
    assert_eq!(Opcode::GpuRender.name(), "GPU_RENDER");
    assert_eq!(Opcode::Heartbeat.name(), "HEARTBEAT");
    assert_eq!(Opcode::ThermalStatus.name(), "THERMAL_STATUS");
}

#[test]
fn test_opcode_classifications() {
    assert!(Opcode::QueryCores.is_query());
    assert!(Opcode::GpuRender.is_gpu());
    assert!(Opcode::CpuExec.is_compute());
    assert!(Opcode::MemAlloc.is_memory());
    assert!(Opcode::StreamVideo.is_stream());
    assert!(!Opcode::Heartbeat.is_gpu());
}
