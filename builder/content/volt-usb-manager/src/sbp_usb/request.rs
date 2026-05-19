use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::sbp_usb::message::{MessageFlags, SbpUsbMessage};

pub fn parse_request(data: &[u8]) -> VumResult<SbpUsbMessage> {
    let msg = SbpUsbMessage::from_bytes(data)?;
    if !msg.flags.contains(MessageFlags::REQUEST) {
        return Err(VumError::InvalidSbpMessage(
            "message is not a request".into(),
        ));
    }
    Ok(msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_usb::message::SbpUsbMessage;
    use crate::sbp_usb::opcode::SbpUsbOpcode;

    #[test]
    fn test_parse_request_valid() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbRead, 1, vec![0x00]);
        let bytes = msg.to_bytes();
        let parsed = parse_request(&bytes).unwrap();
        assert_eq!(parsed.opcode, SbpUsbOpcode::UsbRead);
    }

    #[test]
    fn test_parse_request_no_request_flag() {
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 1, vec![]);
        msg.flags = crate::sbp_usb::message::MessageFlags::EVENT;
        let bytes = msg.to_bytes();
        let result = parse_request(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_request_invalid_data() {
        let result = parse_request(b"too short");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_request_roundtrip() {
        let original = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 99, vec![0x01, 0x02]);
        let bytes = original.to_bytes();
        let parsed = parse_request(&bytes).unwrap();
        assert_eq!(parsed.app_id, 99);
        assert_eq!(parsed.payload, vec![0x01, 0x02]);
    }

    #[test]
    fn test_parse_request_checksum_validated() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbFlush, 0, vec![]);
        let mut bytes = msg.to_bytes();
        bytes[0] ^= 0xFF;
        let result = parse_request(&bytes);
        assert!(result.is_err());
    }
}
