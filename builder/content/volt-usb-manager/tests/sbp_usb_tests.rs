use volt_usb_manager::sbp_usb::message::{MessageFlags, SbpUsbMessage, SBP_USB_MAGIC};
use volt_usb_manager::sbp_usb::opcode::SbpUsbOpcode;
use volt_usb_manager::sbp_usb::response::SbpUsbResponse;

#[test]
fn test_serialize_deserialize_roundtrip() {
    let original = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 42, vec![0xAB, 0xCD]);
    let bytes = original.to_bytes();
    let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
    assert_eq!(decoded.opcode, original.opcode);
    assert_eq!(decoded.flags, original.flags);
    assert_eq!(decoded.request_id, original.request_id);
    assert_eq!(decoded.app_id, original.app_id);
    assert_eq!(decoded.payload, original.payload);
    assert!(decoded.checksum != 0);
}

#[test]
fn test_invalid_magic_rejected() {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 0, vec![]);
    let mut bytes = msg.to_bytes();
    bytes[0..4].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
    let result = SbpUsbMessage::from_bytes(&bytes);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("magic"));
}

#[test]
fn test_invalid_checksum_rejected() {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbRead, 1, vec![0x01]);
    let mut bytes = msg.to_bytes();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF;
    let result = SbpUsbMessage::from_bytes(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_unsupported_opcode_rejected() {
    let result = SbpUsbOpcode::from_byte(0x00);
    assert!(result.is_err());
    let result = SbpUsbOpcode::from_byte(0x40);
    assert!(result.is_err());
    let result = SbpUsbOpcode::from_byte(0xFF);
    assert!(result.is_err());
}

#[test]
fn test_permissions_enforced() {
    use volt_usb_manager::sbp_usb::permissions::SbpUsbPermission;
    assert_eq!(
        SbpUsbOpcode::UsbStatus.permission(),
        SbpUsbPermission::CAP_READ_STATUS
    );
    assert_eq!(
        SbpUsbOpcode::UsbRead.permission(),
        SbpUsbPermission::CAP_READ_FILE
    );
    assert_eq!(
        SbpUsbOpcode::UsbWrite.permission(),
        SbpUsbPermission::CAP_WRITE_FILE
    );
    assert_eq!(
        SbpUsbOpcode::UsbEject.permission(),
        SbpUsbPermission::CAP_ADMIN_STORAGE
    );
    assert_eq!(
        SbpUsbOpcode::UsbDeviceEvent.permission(),
        SbpUsbPermission::INTERNAL
    );
}

#[test]
fn test_ack_buffered_flag_works() {
    let resp = SbpUsbResponse::ack_buffered(99);
    assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
    assert!(resp.message.flags.contains(MessageFlags::ACK_BUFFERED));
    assert!(!resp.message.flags.contains(MessageFlags::ACK_DURABLE));
    assert_eq!(resp.message.request_id, 99);
}

#[test]
fn test_ack_durable_event_works() {
    let resp = SbpUsbResponse::ack_durable(42);
    assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
    assert!(resp.message.flags.contains(MessageFlags::ACK_DURABLE));
    assert!(!resp.message.flags.contains(MessageFlags::ACK_BUFFERED));
    assert_eq!(resp.message.request_id, 42);
}

#[test]
fn test_magic_constant_is_correct() {
    assert_eq!(SBP_USB_MAGIC, 0x5553424D);
}

#[test]
fn test_all_opcodes_are_valid() {
    for code in 0x30..=0x3F {
        assert!(SbpUsbOpcode::from_byte(code).is_ok());
    }
}

#[test]
fn test_opcode_names_are_not_empty() {
    for code in 0x30..=0x3F {
        let op = SbpUsbOpcode::from_byte(code).unwrap();
        assert!(!op.name().is_empty());
    }
}

#[test]
fn test_response_success_roundtrip() {
    let resp = SbpUsbResponse::success(SbpUsbOpcode::UsbStatus, 7, vec![0x01, 0x02]);
    let bytes = resp.to_bytes();
    let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
    assert!(decoded.flags.contains(MessageFlags::RESPONSE));
    assert_eq!(decoded.request_id, 7);
    assert_eq!(decoded.payload, vec![0x01, 0x02]);
}

#[test]
fn test_response_error_has_error_flag() {
    use volt_usb_manager::core::error::VumError;
    let err = VumError::PermissionDenied("test denied".into());
    let resp = SbpUsbResponse::error(5, &err);
    assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
    assert!(resp.message.flags.contains(MessageFlags::ERROR));
    assert_eq!(resp.message.request_id, 5);
}

#[test]
fn test_message_too_short_rejected() {
    let result = SbpUsbMessage::from_bytes(&[0u8; 10]);
    assert!(result.is_err());
}

#[test]
fn test_truncated_payload_rejected() {
    let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 0, vec![0x00; 64]);
    let bytes = msg.to_bytes();
    let truncated = &bytes[..bytes.len() - 8];
    let result = SbpUsbMessage::from_bytes(truncated);
    assert!(result.is_err());
}
