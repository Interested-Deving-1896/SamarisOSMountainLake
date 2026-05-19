use crate::core::result::VumResult;
use crate::sbp_usb::message::SbpUsbMessage;

pub fn deserialize(data: &[u8]) -> VumResult<SbpUsbMessage> {
    SbpUsbMessage::from_bytes(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_usb::opcode::SbpUsbOpcode;

    #[test]
    fn test_deserialize_valid_message() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 1, vec![0x42]);
        let bytes = msg.to_bytes();
        let result = deserialize(&bytes).unwrap();
        assert_eq!(result.opcode, SbpUsbOpcode::UsbStatus);
        assert_eq!(result.payload, vec![0x42]);
    }

    #[test]
    fn test_deserialize_invalid_data() {
        let result = deserialize(b"not a valid message");
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_roundtrip() {
        let original = SbpUsbMessage::new(SbpUsbOpcode::UsbFlush, 7, vec![1, 2, 3]);
        let bytes = original.to_bytes();
        let parsed = deserialize(&bytes).unwrap();
        assert_eq!(parsed.opcode, original.opcode);
        assert_eq!(parsed.app_id, original.app_id);
        assert_eq!(parsed.payload, original.payload);
        assert_eq!(parsed.checksum, original.checksum);
    }

    #[test]
    fn test_deserialize_preserves_flags() {
        use crate::sbp_usb::message::MessageFlags;
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 0, vec![]);
        msg.flags = MessageFlags::EVENT;
        let bytes = msg.to_bytes();
        let parsed = deserialize(&bytes).unwrap();
        assert!(parsed.flags.contains(MessageFlags::EVENT));
    }

    #[test]
    fn test_deserialize_truncated() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 0, vec![0x00; 128]);
        let bytes = msg.to_bytes();
        let truncated = &bytes[..bytes.len() - 10];
        let result = deserialize(truncated);
        assert!(result.is_err());
    }
}
