use crate::core::error::VgmError;
use crate::core::result::VgmResult;
use crate::sbp_gpu::handler::SbpGpuHandler;
use crate::sbp_gpu::message::{MessageFlags, SbpGpuMessage};
use crate::sbp_gpu::opcode::SbpGpuOpcode;
use crate::sbp_gpu::response::SbpGpuResponse;
use dashmap::DashMap;
use std::sync::Arc;

pub struct SbpGpuRouter {
    handlers: DashMap<u8, Arc<dyn SbpGpuHandler>>,
}

impl SbpGpuRouter {
    pub fn new() -> Self {
        SbpGpuRouter {
            handlers: DashMap::new(),
        }
    }

    pub fn register(&self, handler: Arc<dyn SbpGpuHandler>) {
        let opcode_byte = handler.opcode() as u8;
        self.handlers.insert(opcode_byte, handler);
    }

    pub fn route(&self, message: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        if !message.flags.contains(MessageFlags::REQUEST) {
            return Err(VgmError::InvalidSbpMessage(
                "can only route request messages".into(),
            ));
        }

        let opcode_byte = message.opcode as u8;
        match self.handlers.get(&opcode_byte) {
            Some(handler) => handler.handle(message),
            None => {
                let resp = SbpGpuResponse::error(
                    message.request_id,
                    &VgmError::UnsupportedOpcode(format!(
                        "no handler registered for opcode {:#04x} ({})",
                        opcode_byte,
                        message.opcode.name()
                    )),
                );
                Ok(resp)
            }
        }
    }

    pub fn has_handler_for(&self, opcode: SbpGpuOpcode) -> bool {
        self.handlers.contains_key(&(opcode as u8))
    }

    pub fn registered_count(&self) -> usize {
        self.handlers.len()
    }

    pub fn clear(&self) {
        self.handlers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::result::VgmResult;
    use crate::sbp_gpu::handler::{
        ComputeHandler, MetricsHandler, RenderHandler, StatusHandler,
        ThermalHandler, VramHandler,
    };

    fn make_request(opcode: SbpGpuOpcode) -> SbpGpuMessage {
        let mut msg = SbpGpuMessage::new(opcode, vec![]);
        msg.flags = MessageFlags::REQUEST;
        msg.request_id = 1;
        msg.recompute_checksum();
        msg
    }

    #[test]
    fn test_new_router_empty() {
        let router = SbpGpuRouter::new();
        assert_eq!(router.registered_count(), 0);
    }

    #[test]
    fn test_register_handler() {
        let router = SbpGpuRouter::new();
        router.register(Arc::new(StatusHandler));
        assert!(router.has_handler_for(SbpGpuOpcode::GpuStatus));
        assert_eq!(router.registered_count(), 1);
    }

    #[test]
    fn test_route_to_status_handler() {
        let router = SbpGpuRouter::new();
        router.register(Arc::new(StatusHandler));
        let msg = make_request(SbpGpuOpcode::GpuStatus);
        let resp = router.route(&msg).unwrap();
        assert_eq!(resp.message.request_id, 1);
    }

    #[test]
    fn test_route_unregistered_opcode() {
        let router = SbpGpuRouter::new();
        let msg = make_request(SbpGpuOpcode::GpuEvictResource);
        let resp = router.route(&msg).unwrap();
        assert!(resp.message.flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_route_response_message_fails() {
        let router = SbpGpuRouter::new();
        let mut msg = SbpGpuMessage::new(SbpGpuOpcode::GpuStatus, vec![]);
        msg.flags = MessageFlags::RESPONSE;
        let result = router.route(&msg);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_handlers() {
        let router = SbpGpuRouter::new();
        router.register(Arc::new(StatusHandler));
        router.register(Arc::new(ComputeHandler));
        router.register(Arc::new(RenderHandler));
        router.register(Arc::new(ThermalHandler));
        router.register(Arc::new(VramHandler));
        router.register(Arc::new(MetricsHandler));
        assert_eq!(router.registered_count(), 6);

        for op in &[
            SbpGpuOpcode::GpuStatus,
            SbpGpuOpcode::GpuExecCompute,
            SbpGpuOpcode::GpuRenderFrame,
            SbpGpuOpcode::GpuThermalStatus,
            SbpGpuOpcode::GpuVramStatus,
            SbpGpuOpcode::GpuMetricsSnapshot,
        ] {
            let msg = make_request(*op);
            let resp = router.route(&msg).unwrap();
            assert!(!resp.message.flags.contains(MessageFlags::ERROR));
        }
    }

    #[test]
    fn test_clear_router() {
        let router = SbpGpuRouter::new();
        router.register(Arc::new(StatusHandler));
        assert_eq!(router.registered_count(), 1);
        router.clear();
        assert_eq!(router.registered_count(), 0);
    }

    #[test]
    fn test_route_alloc_resource() {
        let router = SbpGpuRouter::new();
        router.register(Arc::new(VramHandler));
        let msg = make_request(SbpGpuOpcode::GpuVramStatus);
        let resp = router.route(&msg).unwrap();
        assert!(!resp.message.flags.contains(MessageFlags::ERROR));
    }

    #[test]
    fn test_route_vram_alloc_direct() {
        struct AllocHandler;
        impl SbpGpuHandler for AllocHandler {
            fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
                Ok(SbpGpuResponse::success(SbpGpuOpcode::GpuAllocResource, msg.request_id, b"allocated".to_vec()))
            }
            fn opcode(&self) -> SbpGpuOpcode { SbpGpuOpcode::GpuAllocResource }
        }
        let router = SbpGpuRouter::new();
        router.register(Arc::new(AllocHandler));
        let msg = make_request(SbpGpuOpcode::GpuAllocResource);
        let resp = router.route(&msg).unwrap();
        assert!(!resp.message.flags.contains(MessageFlags::ERROR));
    }
}
