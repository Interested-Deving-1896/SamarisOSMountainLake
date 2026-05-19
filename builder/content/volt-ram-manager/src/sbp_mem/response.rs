use crate::sbp_mem::message::SbpMessage;
use crate::sbp_mem::opcode::SbpOpcode;
use crate::core::error::VrmError;

#[derive(Debug, Clone)]
pub struct SbpResponse {
    pub message: SbpMessage,
}

impl SbpResponse {
    pub fn success(opcode: SbpOpcode, request_id: u64, payload: Vec<u8>) -> Self {
        Self {
            message: SbpMessage {
                opcode,
                request_id,
                app_id: 0,
                payload,
            },
        }
    }

    pub fn error(request_id: u64, error: &VrmError) -> Self {
        let payload = format!("error: {error}").into_bytes();
        Self {
            message: SbpMessage {
                opcode: SbpOpcode::RamStatus,
                request_id,
                app_id: 0,
                payload,
            },
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.message.to_bytes()
    }
}
