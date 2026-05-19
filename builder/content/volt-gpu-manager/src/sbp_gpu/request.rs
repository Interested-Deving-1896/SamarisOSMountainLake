use crate::core::result::VgmResult;
use crate::sbp_gpu::message::{MessageFlags, SbpGpuMessage};

pub fn parse_request(data: &[u8]) -> VgmResult<SbpGpuMessage> {
    let msg = SbpGpuMessage::from_bytes(data)?;
    if !msg.flags.contains(MessageFlags::REQUEST) {
        return Err(crate::core::error::VgmError::InvalidSbpMessage(
            "message is not a request".into(),
        ));
    }
    Ok(msg)
}

pub fn is_request(msg: &SbpGpuMessage) -> bool {
    msg.flags.contains(MessageFlags::REQUEST)
}

pub fn is_response(msg: &SbpGpuMessage) -> bool {
    msg.flags.contains(MessageFlags::RESPONSE)
}

pub fn is_error(msg: &SbpGpuMessage) -> bool {
    msg.flags.contains(MessageFlags::ERROR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sbp_gpu::opcode::SbpGpuOpcode;
    use crate::sbp_gpu::response::SbpGpuResponse;
    use crate::core::error::VgmError;

    #[test]
    fn test_parse_valid_request() {
        let msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![]);
        let bytes = msg.to_bytes();
        let parsed = parse_request(&bytes).unwrap();
        assert!(is_request(&parsed));
        assert!(!is_response(&parsed));
        assert!(!is_error(&parsed));
    }

    #[test]
    fn test_parse_response_fails() {
        let resp = SbpGpuResponse::success(SbpGpuOpcode::GpuStatus, 1, vec![]);
        let bytes = resp.to_bytes();
        let result = parse_request(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_error_fails() {
        let err = VgmError::PermissionDenied("denied".into());
        let resp = SbpGpuResponse::error(1, &err);
        let bytes = resp.to_bytes();
        let result = parse_request(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_request_response() {
        let req = SbpGpuMessage::new(SbpGpuOpcode::GpuExecCompute, vec![]);
        assert!(is_request(&req));
        assert!(!is_response(&req));

        let resp = SbpGpuResponse::success(SbpGpuOpcode::GpuExecCompute, 0, vec![]);
        assert!(!is_request(&resp.message));
        assert!(is_response(&resp.message));
    }

    #[test]
    fn test_is_error() {
        let err = SbpGpuResponse::error(0, &VgmError::GpuUnavailable("test".into()));
        assert!(is_error(&err.message));
        let ok = SbpGpuResponse::success(SbpGpuOpcode::GpuStatus, 0, vec![]);
        assert!(!is_error(&ok.message));
    }

    #[test]
    fn test_parse_bad_data() {
        let result = parse_request(&[0u8; 5]);
        assert!(result.is_err());
    }
}
