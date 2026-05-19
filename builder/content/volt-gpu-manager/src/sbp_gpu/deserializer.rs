use crate::core::result::VgmResult;
use crate::sbp_gpu::message::SbpGpuMessage;

pub fn deserialize(data: &[u8]) -> VgmResult<SbpGpuMessage> {
    SbpGpuMessage::from_bytes(data)
}

pub fn deserialize_payload(message: &SbpGpuMessage) -> &[u8] {
    &message.payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_gpu::opcode::SbpGpuOpcode;

    #[test]
    fn test_deserialize_valid() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![0x01, 0x02]);
        let bytes = msg.to_bytes();
        let decoded = deserialize(&bytes).unwrap();
        assert_eq!(decoded.opcode, SbpGpuOpcode::GpuStatus);
        assert_eq!(decoded.payload, vec![0x01, 0x02]);
    }

    #[test]
    fn test_deserialize_invalid_truncated() {
        let result = deserialize(&[0u8; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_magic() {
        let mut bytes = vec![0u8; 36];
        bytes[0..4].copy_from_slice(&0xBADu32.to_le_bytes());
        let result = deserialize(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_invalid_opcode() {
        let mut bytes = vec![0u8; 36];
        bytes[0..4].copy_from_slice(&0x47505542u32.to_le_bytes());
        bytes[4] = 0xFF;
        let result = deserialize(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_payload() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuExecCompute, vec![0xAA, 0xBB, 0xCC]);
        let payload = deserialize_payload(&msg);
        assert_eq!(payload, &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_roundtrip_with_all_fields() {
        let original = SbpGpuMessage::new(SbpGpuOpcode::GpuBatchSubmit, (0..64).collect());
        let bytes = original.to_bytes();
        let decoded = deserialize(&bytes).unwrap();
        assert_eq!(decoded.opcode, original.opcode);
        assert_eq!(decoded.payload, original.payload);
        assert_eq!(decoded.flags, original.flags);
        assert_eq!(decoded.checksum, original.checksum);
    }
}
