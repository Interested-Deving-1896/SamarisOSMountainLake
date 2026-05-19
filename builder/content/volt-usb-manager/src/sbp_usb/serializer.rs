use crate::metrics::snapshot::MetricsSnapshot;
use crate::sbp_usb::message::SbpUsbMessage;

pub fn serialize(msg: &SbpUsbMessage) -> Vec<u8> {
    msg.to_bytes()
}

pub fn serialize_status(snapshot: &MetricsSnapshot) -> Vec<u8> {
    serde_json::to_vec(snapshot).unwrap_or_else(|_| vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_usb::message::SbpUsbMessage;
    use crate::sbp_usb::opcode::SbpUsbOpcode;
    use crate::metrics::snapshot::MetricsSnapshot;

    #[test]
    fn test_serialize_message() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbRead, 1, vec![0x01]);
        let bytes = serialize(&msg);
        assert!(!bytes.is_empty());
        let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, msg.opcode);
    }

    #[test]
    fn test_serialize_status_returns_json() {
        let snapshot = MetricsSnapshot::default();
        let json = serialize_status(&snapshot);
        assert!(!json.is_empty());
        let parsed: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert!(parsed.get("uptime_ms").is_some());
    }

    #[test]
    fn test_serialize_status_contains_state() {
        let snapshot = MetricsSnapshot::default();
        let json = serialize_status(&snapshot);
        let parsed: serde_json::Value = serde_json::from_slice(&json).unwrap();
        assert_eq!(parsed["uptime_ms"], 0);
    }

    #[test]
    fn test_serialize_empty_payload() {
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 0, vec![]);
        let bytes = serialize(&msg);
        assert!(bytes.len() >= 40);
    }

    #[test]
    fn test_serialize_large_payload() {
        let payload = vec![0xFF; 4096];
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 2, payload.clone());
        let bytes = serialize(&msg);
        let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.payload.len(), 4096);
    }
}
