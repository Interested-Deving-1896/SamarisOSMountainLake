use volt_gpu_manager::sbp_gpu::{SbpGpuMessage, SbpGpuOpcode, SbpGpuResponse, SbpGpuPermission};
use volt_gpu_manager::core::VgmError;

#[test]
fn serialize_deserialize() {
    let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![0x01, 0x02]);
    let bytes = msg.to_bytes();
    let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.opcode, SbpGpuOpcode::GpuStatus);
    assert_eq!(decoded.payload, vec![0x01, 0x02]);
}

#[test]
fn invalid_magic_rejected() {
    let mut data = vec![0u8; 36];
    data[0..4].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
    let result = SbpGpuMessage::from_bytes(&data);
    assert!(result.is_err());
}

#[test]
fn invalid_checksum_rejected() {
    let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuRenderFrame, vec![0xAA]);
    let mut bytes = msg.to_bytes();
    bytes[36] ^= 0xFF;
    let result = SbpGpuMessage::from_bytes(&bytes);
    assert!(result.is_err());
}

#[test]
fn unsupported_opcode_rejected() {
    let result = SbpGpuOpcode::from_byte(0xFF);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), VgmError::UnsupportedOpcode(_)));
}

#[test]
fn permissions_enforced() {
    assert_eq!(SbpGpuOpcode::GpuStatus.permission(), SbpGpuPermission::CAP_READ_STATUS);
    assert_eq!(SbpGpuOpcode::GpuAllocResource.permission(), SbpGpuPermission::CAP_GPU_ALLOC);
    assert_eq!(SbpGpuOpcode::GpuExecCompute.permission(), SbpGpuPermission::CAP_GPU_COMPUTE);
    assert_eq!(SbpGpuOpcode::GpuRenderFrame.permission(), SbpGpuPermission::CAP_GPU_RENDER);
    assert_eq!(SbpGpuOpcode::GpuSwitchDevice.permission(), SbpGpuPermission::CAP_ADMIN_GPU);
}

#[test]
fn error_response() {
    let err = VgmError::PermissionDenied("denied".into());
    let resp = SbpGpuResponse::error(1, &err);
    assert!(!resp.is_success());
    assert!(resp.message.flags.bits() & 4 != 0);
}
