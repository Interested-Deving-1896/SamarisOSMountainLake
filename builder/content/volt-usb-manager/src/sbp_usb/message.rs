use bitflags::bitflags;

use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::sbp_usb::opcode::SbpUsbOpcode;

pub const SBP_USB_MAGIC: u32 = 0x5553424D;
pub const SBP_USB_VERSION: u8 = 1;
pub const SBP_USB_HEADER_SIZE: usize = 36;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MessageFlags: u16 {
        const REQUEST      = 0x0001;
        const RESPONSE     = 0x0002;
        const ERROR        = 0x0004;
        const EVENT        = 0x0008;
        const ACK_BUFFERED = 0x0010;
        const ACK_DURABLE  = 0x0020;
    }
}

#[derive(Debug, Clone)]
pub struct SbpUsbMessage {
    pub opcode: SbpUsbOpcode,
    pub flags: MessageFlags,
    pub request_id: u64,
    pub timestamp_us: u64,
    pub app_id: u64,
    pub payload: Vec<u8>,
    pub checksum: u32,
}

impl SbpUsbMessage {
    pub fn new(opcode: SbpUsbOpcode, app_id: u64, payload: Vec<u8>) -> Self {
        let msg = SbpUsbMessage {
            opcode,
            flags: MessageFlags::REQUEST,
            request_id: 0,
            timestamp_us: 0,
            app_id,
            payload,
            checksum: 0,
        };
        let header = msg.encode_header();
        let checksum = Self::compute_checksum(&header, &msg.payload);
        SbpUsbMessage { checksum, ..msg }
    }

    fn encode_header(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(SBP_USB_HEADER_SIZE);
        buf.extend_from_slice(&SBP_USB_MAGIC.to_le_bytes());
        buf.push(SBP_USB_VERSION);
        buf.push(self.opcode as u8);
        buf.extend_from_slice(&self.flags.bits().to_le_bytes());
        buf.extend_from_slice(&self.request_id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp_us.to_le_bytes());
        buf.extend_from_slice(&self.app_id.to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let header = self.encode_header();
        let checksum = Self::compute_checksum(&header, &self.payload);
        let mut buf = header;
        buf.extend_from_slice(&self.payload);
        buf.extend_from_slice(&checksum.to_le_bytes());
        buf
    }

    pub fn from_bytes(data: &[u8]) -> VumResult<Self> {
        if data.len() < SBP_USB_HEADER_SIZE + 4 {
            return Err(VumError::InvalidSbpMessage(
                "message too short".into(),
            ));
        }

        let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        if magic != SBP_USB_MAGIC {
            return Err(VumError::InvalidSbpMessage(format!(
                "invalid magic: 0x{:08X}",
                magic
            )));
        }

        let version = data[4];
        if version != SBP_USB_VERSION {
            return Err(VumError::InvalidSbpMessage(format!(
                "unsupported version {}",
                version
            )));
        }

        let opcode = SbpUsbOpcode::from_byte(data[5])?;
        let flags = MessageFlags::from_bits(u16::from_le_bytes([data[6], data[7]]))
            .ok_or_else(|| VumError::InvalidSbpMessage("invalid flags".into()))?;
        let request_id = u64::from_le_bytes([
            data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15],
        ]);
        let timestamp_us = u64::from_le_bytes([
            data[16], data[17], data[18], data[19], data[20], data[21], data[22], data[23],
        ]);
        let app_id = u64::from_le_bytes([
            data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
        ]);
        let payload_len =
            u32::from_le_bytes([data[32], data[33], data[34], data[35]]) as usize;

        let total_before_payload = SBP_USB_HEADER_SIZE;
        let total_without_checksum = total_before_payload + payload_len;
        if data.len() < total_without_checksum + 4 {
            return Err(VumError::InvalidSbpMessage("truncated payload".into()));
        }

        let payload = data[total_before_payload..total_before_payload + payload_len].to_vec();
        let stored_checksum = u32::from_le_bytes([
            data[total_without_checksum],
            data[total_without_checksum + 1],
            data[total_without_checksum + 2],
            data[total_without_checksum + 3],
        ]);

        let computed = Self::compute_checksum(&data[..total_without_checksum], &[]);
        if computed != stored_checksum {
            return Err(VumError::ChecksumMismatch);
        }

        Ok(SbpUsbMessage {
            opcode,
            flags,
            request_id,
            timestamp_us,
            app_id,
            payload,
            checksum: stored_checksum,
        })
    }

    pub fn compute_checksum(header: &[u8], payload: &[u8]) -> u32 {
        let mut combined = Vec::with_capacity(header.len() + payload.len());
        combined.extend_from_slice(header);
        combined.extend_from_slice(payload);
        crc32fast::hash(&combined)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_message() -> SbpUsbMessage {
        let mut msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 42, vec![0xAB, 0xCD]);
        msg.request_id = 1001;
        msg.timestamp_us = 5000;
        msg
    }

    #[test]
    fn test_new_sets_magic_version_and_request_flag() {
        let msg = sample_message();
        assert_eq!(msg.opcode, SbpUsbOpcode::UsbWrite);
        assert!(msg.flags.contains(MessageFlags::REQUEST));
        assert_eq!(msg.app_id, 42);
        assert_eq!(msg.payload, vec![0xAB, 0xCD]);
    }

    #[test]
    fn test_to_bytes_roundtrip() {
        let original = sample_message();
        let bytes = original.to_bytes();
        let decoded = SbpUsbMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, original.opcode);
        assert_eq!(decoded.flags, original.flags);
        assert_eq!(decoded.request_id, original.request_id);
        assert_eq!(decoded.timestamp_us, original.timestamp_us);
        assert_eq!(decoded.app_id, original.app_id);
        assert_eq!(decoded.payload, original.payload);
        assert!(decoded.checksum != 0);
    }

    #[test]
    fn test_from_bytes_invalid_magic() {
        let data = vec![0x00; 40];
        let result = SbpUsbMessage::from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_too_short() {
        let data = vec![0x00; 10];
        let result = SbpUsbMessage::from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_checksum_mismatch() {
        let mut msg = sample_message();
        msg.checksum = 0;
        let mut bytes = msg.to_bytes();
        // Corrupt payload
        bytes[SBP_USB_HEADER_SIZE] ^= 0xFF;
        let result = SbpUsbMessage::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_compute_checksum_deterministic() {
        let h = b"header_data";
        let p = b"payload_data";
        let c1 = SbpUsbMessage::compute_checksum(h, p);
        let c2 = SbpUsbMessage::compute_checksum(h, p);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_compute_checksum_different_headers() {
        let c1 = SbpUsbMessage::compute_checksum(b"header1", b"payload");
        let c2 = SbpUsbMessage::compute_checksum(b"header2", b"payload");
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_header_size_constant() {
        assert_eq!(SBP_USB_HEADER_SIZE, 36);
    }

    #[test]
    fn test_magic_constant_value() {
        assert_eq!(SBP_USB_MAGIC, 0x5553424D);
    }

    #[test]
    fn test_to_bytes_header_version() {
        let msg = sample_message();
        let bytes = msg.to_bytes();
        assert_eq!(bytes[4], 1);
    }

    #[test]
    fn test_new_preserves_payload() {
        let payload = vec![1, 2, 3, 4, 5];
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 7, payload.clone());
        assert_eq!(msg.payload, payload);
    }
}
