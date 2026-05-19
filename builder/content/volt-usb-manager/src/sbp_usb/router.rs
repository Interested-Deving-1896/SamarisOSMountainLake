use dashmap::DashMap;

use crate::core::error::VumError;
use crate::core::manager::VoltUsbManager;
use crate::core::result::VumResult;
use crate::sbp_usb::handler::SbpUsbHandler;
use crate::sbp_usb::message::SbpUsbMessage;
use crate::sbp_usb::response::SbpUsbResponse;

pub struct SbpUsbRouter {
    handlers: DashMap<u8, Box<dyn SbpUsbHandler>>,
}

impl SbpUsbRouter {
    pub fn new() -> Self {
        SbpUsbRouter {
            handlers: DashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn SbpUsbHandler>) {
        self.handlers.insert(handler.opcode() as u8, handler);
    }

    pub fn route(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        if let Some(handler) = self.handlers.get(&(msg.opcode as u8)) {
            handler.handle(msg, manager)
        } else {
            Ok(SbpUsbResponse::error(
                msg.request_id,
                &VumError::UnsupportedOpcode(msg.opcode as u8),
            ))
        }
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for SbpUsbRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VumConfig;
    use crate::core::manager::VoltUsbManager;
    use crate::sbp_usb::handler::{HeartbeatHandler, StatusHandler};
    use crate::sbp_usb::opcode::SbpUsbOpcode;

    #[test]
    fn test_new_router_empty() {
        let router = SbpUsbRouter::new();
        assert_eq!(router.handler_count(), 0);
    }

    #[test]
    fn test_register_increases_count() {
        let mut router = SbpUsbRouter::new();
        router.register(Box::new(StatusHandler));
        assert_eq!(router.handler_count(), 1);
    }

    #[test]
    fn test_register_multiple() {
        let mut router = SbpUsbRouter::new();
        router.register(Box::new(StatusHandler));
        router.register(Box::new(HeartbeatHandler));
        assert_eq!(router.handler_count(), 2);
    }

    #[test]
    fn test_route_registered_handler() {
        let mut router = SbpUsbRouter::new();
        router.register(Box::new(StatusHandler));
        let config = VumConfig::default();
        let manager = VoltUsbManager::new(config);
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 1, vec![]);
        let result = router.route(&msg, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_route_unregistered_returns_error() {
        let router = SbpUsbRouter::new();
        let config = VumConfig::default();
        let manager = VoltUsbManager::new(config);
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 1, vec![]);
        let result = router.route(&msg, &manager);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.message.flags.contains(
            crate::sbp_usb::message::MessageFlags::ERROR
        ));
    }

    #[test]
    fn test_handler_count_default() {
        let router = SbpUsbRouter::default();
        assert_eq!(router.handler_count(), 0);
    }
}
