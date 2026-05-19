use crate::core::error::VgmError;
use crate::sbp_gpu::message::{MessageFlags, SbpGpuMessage};
use crate::sbp_gpu::opcode::SbpGpuOpcode;

pub struct SbpGpuResponse {
    pub message: SbpGpuMessage,
}

impl SbpGpuResponse {
    pub fn success(opcode: SbpGpuOpcode, request_id: u64, payload: Vec<u8>) -> Self {
        let mut msg = SbpGpuMessage::new(opcode, payload);
        msg.flags = MessageFlags::RESPONSE;
        msg.request_id = request_id;
        msg.recompute_checksum();
        SbpGpuResponse { message: msg }
    }

    pub fn error(request_id: u64, error: &VgmError) -> Self {
        let error_bytes = error.to_string().into_bytes();
        let mut msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, error_bytes);
        msg.flags = MessageFlags::RESPONSE | MessageFlags::ERROR;
        msg.request_id = request_id;
        msg.recompute_checksum();
        SbpGpuResponse { message: msg }
    }

    pub fn ok(data: Vec<u8>) -> Self {
        let mut msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, data);
        msg.flags = MessageFlags::RESPONSE;
        msg.recompute_checksum();
        SbpGpuResponse { message: msg }
    }

    pub fn err(data: Vec<u8>) -> Self {
        let mut msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, data);
        msg.flags = MessageFlags::RESPONSE | MessageFlags::ERROR;
        msg.recompute_checksum();
        SbpGpuResponse { message: msg }
    }

    pub fn is_success(&self) -> bool {
        !self.message.flags.contains(MessageFlags::ERROR)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.message.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let resp = SbpGpuResponse::success(SbpGpuOpcode::GpuStatus, 42, vec![1, 2, 3]);
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(!resp.message.flags.contains(MessageFlags::ERROR));
        assert_eq!(resp.message.request_id, 42);
        assert_eq!(resp.message.payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_error_response() {
        let err = VgmError::GpuUnavailable("no GPU".into());
        let resp = SbpGpuResponse::error(7, &err);
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(resp.message.flags.contains(MessageFlags::ERROR));
        assert_eq!(resp.message.request_id, 7);
        assert!(!resp.message.payload.is_empty());
    }

    #[test]
    fn test_ok_response() {
        let resp = SbpGpuResponse::ok(b"ok".to_vec());
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(!resp.message.flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_err_response() {
        let resp = SbpGpuResponse::err(b"fail".to_vec());
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(resp.message.flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_to_bytes_roundtrip() {
        let resp = SbpGpuResponse::success(SbpGpuOpcode::GpuExecCompute, 100, vec![0xDE, 0xAD]);
        let bytes = resp.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.opcode, SbpGpuOpcode::GpuExecCompute);
        assert_eq!(decoded.request_id, 100);
    }

    #[test]
    fn test_error_to_bytes() {
        let err = VgmError::PermissionDenied("access denied".into());
        let resp = SbpGpuResponse::error(1, &err);
        let bytes = resp.to_bytes();
        let decoded = SbpGpuMessage::from_bytes(&bytes).unwrap();
        assert!(decoded.flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_is_success() {
        let ok = SbpGpuResponse::success(SbpGpuOpcode::GpuStatus, 0, vec![]);
        assert!(ok.is_success());
        let err = SbpGpuResponse::error(0, &VgmError::GpuUnavailable("x".into()));
        assert!(!err.is_success());
    }
}
