use crate::core::error::{Result, TesseractError};
use crate::protocol::opcodes::Opcode;
use crate::protocol::TesseractCommand;

pub struct CommandSandbox;

impl CommandSandbox {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, cmd: &TesseractCommand) -> Result<()> {
        if cmd.app_id() == 0 {
            return Err(TesseractError::Security("app_id cannot be 0".into()));
        }

        if cmd.payload_len() > 10 * 1024 * 1024 {
            return Err(TesseractError::Security(format!(
                "payload too large: {} bytes (max 10MB)",
                cmd.payload_len()
            )));
        }

        if cmd.priority() > 4 {
            return Err(TesseractError::Security(format!(
                "invalid priority: {}",
                cmd.priority()
            )));
        }

        match Opcode::from_byte(cmd.header.opcode) {
            Ok(opcode) => {
                match opcode {
                    Opcode::MemAlloc | Opcode::MemFree => {
                        if cmd.app_id() == 0xFFFFFFFF {
                            return Err(TesseractError::Security(
                                "system app cannot allocate memory directly".into(),
                            ));
                        }
                    }
                    Opcode::CpuReserve | Opcode::CpuRelease => {
                        if cmd.priority() != 0 {
                            return Err(TesseractError::Security(
                                "CPU reserve/release requires CRITICAL priority".into(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                return Err(TesseractError::Security(format!("invalid opcode: {e}")));
            }
        }

        Ok(())
    }
}
