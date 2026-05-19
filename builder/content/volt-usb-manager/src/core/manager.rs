use std::sync::Arc;

use parking_lot::RwLock;

use crate::config::schema::VumConfig;
use crate::core::engine::VumEngine;
use crate::core::error::VumError;
use crate::core::result::VumResult;
use crate::core::state::VumState;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::sbp_usb::message::MessageFlags;
use crate::sbp_usb::message::SbpUsbMessage;
use crate::sbp_usb::opcode::SbpUsbOpcode;
use crate::sbp_usb::response::SbpUsbResponse;

pub struct VoltUsbManager {
    pub state: VumState,
    pub config: VumConfig,
    pub engine: Arc<RwLock<VumEngine>>,
}

impl VoltUsbManager {
    pub fn new(config: VumConfig) -> Self {
        VoltUsbManager {
            state: VumState::Uninitialized,
            config,
            engine: Arc::new(RwLock::new(VumEngine::new())),
        }
    }

    pub fn init(&mut self) -> VumResult<()> {
        if !self.state.can_transition_to(&VumState::ConfigLoaded) {
            return Err(VumError::InternalInvariantViolation(format!(
                "Cannot init from state {:?}",
                self.state
            )));
        }
        self.config.validate()?;
        self.engine.write().init(&self.config)?;
        self.state = VumState::ConfigLoaded;
        Ok(())
    }

    pub fn shutdown(&mut self) -> VumResult<()> {
        if !self.state.can_transition_to(&VumState::Shutdown) {
            return Err(VumError::InternalInvariantViolation(format!(
                "Cannot shutdown from state {:?}",
                self.state
            )));
        }
        let mut eng = self.engine.write();
        eng.read_cache = None;
        eng.write_buffer = None;
        eng.journal = None;
        eng.recovery = None;
        eng.scheduler = None;
        eng.ram_client = None;
        self.state = VumState::Shutdown;
        Ok(())
    }

    pub fn state(&self) -> VumState {
        self.state.clone()
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        self.engine.read().metrics.read().snapshot()
    }

    pub fn handle_sbp(&self, msg: SbpUsbMessage) -> VumResult<SbpUsbResponse> {
        match msg.opcode {
            SbpUsbOpcode::UsbStatus | SbpUsbOpcode::UsbMetricsSnapshot => {
                let snapshot = self.snapshot();
                let payload = serde_json::to_vec(&snapshot)
                    .map_err(|e| VumError::InvalidSbpMessage(e.to_string()))?;
                Ok(SbpUsbResponse::success(
                    msg.opcode,
                    msg.request_id,
                    payload,
                ))
            }
            SbpUsbOpcode::UsbFlush => {
                Ok(SbpUsbResponse::success(
                    msg.opcode,
                    msg.request_id,
                    vec![],
                ))
            }
            SbpUsbOpcode::UsbEject => {
                Ok(SbpUsbResponse::success(
                    msg.opcode,
                    msg.request_id,
                    vec![],
                ))
            }
            SbpUsbOpcode::UsbRead | SbpUsbOpcode::UsbWrite => {
                Ok(SbpUsbResponse::success(
                    msg.opcode,
                    msg.request_id,
                    msg.payload,
                ))
            }
            SbpUsbOpcode::UsbHeartbeat => {
                let mut resp =
                    SbpUsbResponse::success(msg.opcode, msg.request_id, vec![]);
                resp.message.flags = MessageFlags::EVENT;
                Ok(resp)
            }
            _ => Ok(SbpUsbResponse::success(
                msg.opcode,
                msg.request_id,
                msg.payload,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_new() {
        let config = VumConfig::default();
        let mgr = VoltUsbManager::new(config);
        assert_eq!(mgr.state(), VumState::Uninitialized);
    }

    #[test]
    fn test_manager_init_and_shutdown() {
        let config = VumConfig::default();
        let mut mgr = VoltUsbManager::new(config);
        assert!(mgr.init().is_ok());
        assert_eq!(mgr.state(), VumState::ConfigLoaded);
        assert!(mgr.shutdown().is_ok());
        assert_eq!(mgr.state(), VumState::Shutdown);
    }

    #[test]
    fn test_manager_handle_sbp_status() {
        let config = VumConfig::default();
        let mgr = VoltUsbManager::new(config);
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 0, vec![]);
        let resp = mgr.handle_sbp(msg).unwrap();
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
        assert!(!resp.message.payload.is_empty());
    }

    #[test]
    fn test_manager_handle_sbp_flush() {
        let config = VumConfig::default();
        let mgr = VoltUsbManager::new(config);
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbFlush, 0, vec![]);
        let resp = mgr.handle_sbp(msg).unwrap();
        assert!(resp.message.flags.contains(MessageFlags::RESPONSE));
    }

    #[test]
    fn test_manager_handle_sbp_heartbeat() {
        let config = VumConfig::default();
        let mgr = VoltUsbManager::new(config);
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 0, vec![]);
        let resp = mgr.handle_sbp(msg).unwrap();
        assert!(resp.message.flags.contains(MessageFlags::EVENT));
    }

    #[test]
    fn test_manager_snapshot_defaults() {
        let config = VumConfig::default();
        let mgr = VoltUsbManager::new(config);
        let snap = mgr.snapshot();
        assert_eq!(snap.cache_hit_count, 0);
        assert_eq!(snap.cache_miss_count, 0);
    }
}
