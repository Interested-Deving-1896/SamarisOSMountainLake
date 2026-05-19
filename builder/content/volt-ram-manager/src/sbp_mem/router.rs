use dashmap::DashMap;
use crate::core::result::VrmResult;
use crate::sbp_mem::handler::SbpHandler;
use crate::sbp_mem::message::SbpMessage;
use crate::sbp_mem::response::SbpResponse;
use crate::core::manager::VoltRamManager;

pub struct SbpRouter {
    handlers: DashMap<u8, Box<dyn SbpHandler>>,
}

impl SbpRouter {
    pub fn new() -> Self {
        Self {
            handlers: DashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn SbpHandler>) {
        let opcode = handler.opcode() as u8;
        self.handlers.insert(opcode, handler);
    }

    pub fn route(&self, msg: &SbpMessage, manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        match self.handlers.get(&(msg.opcode as u8)) {
            Some(handler) => handler.handle(msg, manager),
            None => Ok(SbpResponse::error(msg.request_id, &crate::core::error::VrmError::UnsupportedOpcode(msg.opcode as u8))),
        }
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for SbpRouter {
    fn default() -> Self {
        Self::new()
    }
}
