use crate::core::error::VumError;
use crate::sbp_usb::message::{MessageFlags, SbpUsbMessage};
use crate::sbp_usb::opcode::SbpUsbOpcode;

#[derive(Debug, Clone)]
pub struct SbpUsbResponse {
    pub message: SbpUsbMessage,
}

impl SbpUsbResponse {
    pub fn success(opcode: SbpUsbOpcode, request_id: u64, payload: Vec<u8>) -> Self {
        let mut msg = SbpUsbMessage::new(opcode, 0, payload);
        msg.flags = MessageFlags::RESPONSE;
        msg.request_id = request_id;
        SbpUsbResponse { message: msg }
    }

    pub fn error(request_id: u64, error: &VumError) -> Self {
        let payload = error.to_string().into_bytes();
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 0, payload);
        msg.flags = MessageFlags::RESPONSE | MessageFlags::ERROR;
        msg.request_id = request_id;
        SbpUsbResponse { message: msg }
    }

    pub fn ack_buffered(request_id: u64) -> Self {
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWriteAckEvent, 0, vec![]);
        msg.flags = MessageFlags::RESPONSE | MessageFlags::ACK_BUFFERED;
        msg.request_id = request_id;
        SbpUsbResponse { message: msg }
    }

    pub fn ack_durable(request_id: u64) -> Self {
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWriteAckEvent, 0, vec![]);
        msg.flags = MessageFlags::RESPONSE | MessageFlags::ACK_DURABLE;
        msg.request_id = request_id;
        SbpUsbResponse { message: msg }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.message.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_sets_response_flag() {
        let resp = SbpUsbResponse::success(SbpUsbOpcode::UsbStatus, 42, vec![0x01]);
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(!resp.message.flags.contains(MessageFlags::ERROR));
        assert_eq!(resp.message.request_id, 42);
    }

    #[test]
    fn test_error_sets_error_flag() {
        let err = VumError::DeviceNotFound;
        let resp = SbpUsbResponse::error(7, &err);
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(resp.message.flags.contains(MessageFlags::ERROR));
        assert_eq!(resp.message.request_id, 7);
    }

    #[test]
    fn test_ack_buffered_has_ack_flag() {
        let resp = SbpUsbResponse::ack_buffered(99);
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(resp.message.flags.contains(MessageFlags::ACK_BUFFERED));
        assert!(!resp.message.flags.contains(MessageFlags::ACK_DURABLE));
        assert_eq!(resp.message.request_id, 99);
    }

    #[test]
    fn test_ack_durable_has_durable_flag() {
        let resp = SbpUsbResponse::ack_durable(88);
        assert!(resp.message.flags.contains(MessageFlags::ACK_DURABLE));
        assert_eq!(resp.message.request_id, 88);
    }

    #[test]
    fn test_to_bytes_roundtrip() {
        let resp = SbpUsbResponse::success(SbpUsbOpcode::UsbRead, 1, vec![0xAA, 0xBB]);
        let bytes = resp.to_bytes();
        let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, SbpUsbOpcode::UsbRead);
        assert_eq!(decoded.request_id, 1);
        assert_eq!(decoded.payload, vec![0xAA, 0xBB]);
    }

    #[test]
    fn test_error_payload_contains_message() {
        let err = VumError::PermissionDenied("access denied".into());
        let resp = SbpUsbResponse::error(0, &err);
        let payload_str = String::from_utf8_lossy(&resp.message.payload);
        assert!(payload_str.contains("access denied"));
    }
}
