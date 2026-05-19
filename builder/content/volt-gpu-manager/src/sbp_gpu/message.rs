use crate::core::error::VgmError;
use crate::core::result::VgmResult;
use crate::sbp_gpu::opcode::SbpGpuOpcode;
use bitflags::bitflags;
use std::time::{SystemTime, UNIX_EPOCH};

pub const SBP_GPU_MAGIC: u32 = 0x47505542;
pub const SBP_GPU_HEADER_SIZE: usize = 36;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MessageFlags: u16 {
        const REQUEST = 1;
        const RESPONSE = 2;
        const ERROR = 4;
    }
}

#[derive(Debug, Clone)]
pub struct SbpGpuMessage {
    pub opcode: SbpGpuOpcode,
    pub flags: MessageFlags,
    pub request_id: u64,
    pub timestamp_us: u64,
    pub payload: Vec<u8>,
    pub checksum: u32,
}

fn compute_msg_checksum(
    opcode: u8,
    flags: u16,
    request_id: u64,
    timestamp_us: u64,
    payload: &[u8],
) -> u32 {
    let mut h = crc32fast::Hasher::new();
    h.update(&SBP_GPU_MAGIC.to_le_bytes());
    h.update(&[opcode]);
    h.update(&flags.to_le_bytes());
    h.update(&request_id.to_le_bytes());
    h.update(&timestamp_us.to_le_bytes());
    h.update(&(payload.len() as u32).to_le_bytes());
    h.update(&[0u8; 4]);
    h.update(payload);
    h.finalize()
}

impl SbpGpuMessage {
    pub fn new(opcode: SbpGpuOpcode, payload: Vec<u8>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let checksum = compute_msg_checksum(opcode as u8, 1, 0, now, &payload);
        SbpGpuMessage {
            opcode,
            flags: MessageFlags::REQUEST,
            request_id: 0,
            timestamp_us: now,
            payload,
            checksum,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(SBP_GPU_HEADER_SIZE + self.payload.len());
        buf.extend_from_slice(&SBP_GPU_MAGIC.to_le_bytes());
        buf.push(self.opcode as u8);
        buf.extend_from_slice(&self.flags.bits().to_le_bytes());
        buf.extend_from_slice(&self.request_id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp_us.to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.checksum.to_le_bytes());
        buf.resize(SBP_GPU_HEADER_SIZE, 0);
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> VgmResult<Self> {
        if data.len() < SBP_GPU_HEADER_SIZE {
            return Err(VgmError::InvalidSbpMessage(format!(
                "message too short: {} < {}",
                data.len(),
                SBP_GPU_HEADER_SIZE
            )));
        }
        let magic = u32::from_le_bytes(data[0..4].try_into().unwrap());
        if magic != SBP_GPU_MAGIC {
            return Err(VgmError::InvalidSbpMessage(format!(
                "bad magic: {:#010x}",
                magic
            )));
        }
        let opcode = SbpGpuOpcode::from_byte(data[4])?;
        let flags = MessageFlags::from_bits_truncate(u16::from_le_bytes(
            data[5..7].try_into().unwrap(),
        ));
        let request_id = u64::from_le_bytes(data[7..15].try_into().unwrap());
        let timestamp_us = u64::from_le_bytes(data[15..23].try_into().unwrap());
        let payload_len = u32::from_le_bytes(data[23..27].try_into().unwrap()) as usize;
        let stored_checksum = u32::from_le_bytes(data[27..31].try_into().unwrap());
        if data.len() < SBP_GPU_HEADER_SIZE + payload_len {
            return Err(VgmError::InvalidSbpMessage(
                "truncated payload".into(),
            ));
        }
        let payload = data[SBP_GPU_HEADER_SIZE..SBP_GPU_HEADER_SIZE + payload_len].to_vec();

        let verify = compute_msg_checksum(
            data[4],
            u16::from_le_bytes(data[5..7].try_into().unwrap()),
            request_id,
            timestamp_us,
            &payload,
        );
        if verify != stored_checksum {
            return Err(VgmError::ChecksumMismatch(format!(
                "SBP message checksum mismatch: stored={:#x} computed={:#x}",
                stored_checksum, verify
            )));
        }

        Ok(SbpGpuMessage {
            opcode,
            flags,
            request_id,
            timestamp_us,
            payload,
            checksum: stored_checksum,
        })
    }

    pub(crate) fn recompute_checksum(&mut self) {
        self.checksum = compute_msg_checksum(
            self.opcode as u8,
            self.flags.bits(),
            self.request_id,
            self.timestamp_us,
            &self.payload,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_new_sets_magic() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![1, 2, 3]);
        assert_eq!(msg.opcode, SbpGpuOpcode::GpuStatus);
        assert!(msg.flags.contains(MessageFlags::REQUEST));
        assert!(msg.timestamp_us > 0);
    }

    #[test]
    fn test_roundtrip() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuExecCompute, vec![0x10, 0x20, 0x30]);
        let bytes = msg.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, msg.opcode);
        assert_eq!(decoded.payload, msg.payload);
        assert_eq!(decoded.request_id, msg.request_id);
        assert_eq!(decoded.checksum, msg.checksum);
    }

    #[test]
    fn test_roundtrip_all_opcodes() {
        for op in &[
            SbpGpuOpcode::GpuStatus,
            SbpGpuOpcode::GpuExecCompute,
            SbpGpuOpcode::GpuRenderFrame,
            SbpGpuOpcode::GpuBatchSubmit,
            SbpGpuOpcode::GpuMetricsSnapshot,
        ] {
            let msg = SbpGpuMessage::new(*op, vec![*op as u8]);
            let bytes = msg.to_bytes();
            let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
            assert_eq!(decoded.opcode, *op);
            assert_eq!(decoded.payload, vec![*op as u8]);
        }
    }

    #[test]
    fn test_too_short_data() {
        let result = SbpGpuMessage::from_bytes(&[0u8; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_bad_magic() {
        let mut data = vec![0u8; SBP_GPU_HEADER_SIZE];
        data[0..4].copy_from_slice(&0xDEADBEEFu32.to_le_bytes());
        let result = SbpGpuMessage::from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_payload() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![]);
        let bytes = msg.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert!(decoded.payload.is_empty());
    }

    #[test]
    fn test_large_payload() {
        let payload: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuBatchSubmit, payload.clone());
        let bytes = msg.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn test_checksum_detects_tamper() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuRenderFrame, vec![0xAA]);
        let mut bytes = msg.to_bytes();
        bytes[SBP_GPU_HEADER_SIZE] ^= 0xFF;
        let result = SbpGpuMessage::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_message_flags() {
        let flags = MessageFlags::REQUEST | MessageFlags::RESPONSE;
        assert!(flags.contains(MessageFlags::REQUEST));
        assert!(flags.contains(MessageFlags::RESPONSE));
        assert!(!flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_header_tamper_detected() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![]);
        let mut bytes = msg.to_bytes();
        bytes[4] = 0xFF;
        let result = SbpGpuMessage::from_bytes(&bytes);
        assert!(result.is_err());
    }
}
