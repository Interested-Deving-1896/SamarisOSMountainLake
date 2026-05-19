use crate::core::manager::VoltUsbManager;
use crate::core::result::VumResult;
use crate::sbp_usb::message::SbpUsbMessage;
use crate::sbp_usb::opcode::SbpUsbOpcode;
use crate::sbp_usb::response::SbpUsbResponse;

pub trait SbpUsbHandler: Send + Sync {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse>;
    fn opcode(&self) -> SbpUsbOpcode;
}

pub struct StatusHandler;
pub struct ReadHandler;
pub struct WriteHandler;
pub struct FlushHandler;
pub struct EjectHandler;
pub struct HeartbeatHandler;
pub struct MountHandler;
pub struct MetricsHandler;

impl SbpUsbHandler for StatusHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let snapshot = manager.snapshot();
        let status_str = crate::sbp_usb::status::format_short(&snapshot);
        Ok(SbpUsbResponse::success(
            SbpUsbOpcode::UsbStatus,
            msg.request_id,
            status_str.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbStatus
    }
}

impl SbpUsbHandler for ReadHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        _manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        Ok(SbpUsbResponse::success(
            SbpUsbOpcode::UsbRead,
            msg.request_id,
            msg.payload.clone(),
        ))
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbRead
    }
}

impl SbpUsbHandler for WriteHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let eng = manager.engine.read();
        if let Some(ref wb) = eng.write_buffer {
            let mut buf = wb.write();
            let path = String::from_utf8_lossy(&msg.payload);
            buf.enqueue(&path, msg.request_id, msg.payload.clone(), 0, 0)?;
            Ok(SbpUsbResponse::ack_buffered(msg.request_id))
        } else {
            Ok(SbpUsbResponse::success(
                SbpUsbOpcode::UsbWrite,
                msg.request_id,
                msg.payload.clone(),
            ))
        }
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbWrite
    }
}

impl SbpUsbHandler for FlushHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let eng = manager.engine.read();
        if let Some(ref wb) = eng.write_buffer {
            let buf = wb.read();
            let pending = buf.pending_count();
            let dirty = buf.dirty_bytes();
            let info = format!("{} pending, {} dirty bytes", pending, dirty);
            Ok(SbpUsbResponse::success(
                SbpUsbOpcode::UsbFlush,
                msg.request_id,
                info.into_bytes(),
            ))
        } else {
            Ok(SbpUsbResponse::success(
                SbpUsbOpcode::UsbFlush,
                msg.request_id,
                vec![],
            ))
        }
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbFlush
    }
}

impl SbpUsbHandler for EjectHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let eng = manager.engine.read();
        if let Some(ref wb) = eng.write_buffer {
            let buf = wb.read();
            if buf.pending_count() > 0 {
                return Err(crate::core::error::VumError::UnsafeToEject(
                    "pending writes exist".into(),
                ));
            }
        }
        Ok(SbpUsbResponse::success(
            SbpUsbOpcode::UsbEject,
            msg.request_id,
            vec![],
        ))
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbEject
    }
}

impl SbpUsbHandler for HeartbeatHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        _manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let mut resp = SbpUsbResponse::success(
            SbpUsbOpcode::UsbHeartbeat,
            msg.request_id,
            vec![],
        );
        resp.message.flags = crate::sbp_usb::message::MessageFlags::EVENT;
        Ok(resp)
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbHeartbeat
    }
}

impl SbpUsbHandler for MountHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let mount_point = if msg.payload.is_empty() {
            manager.config.manager.mount_point.clone()
        } else {
            String::from_utf8_lossy(&msg.payload).to_string()
        };
        let info = format!("mounted at {}", mount_point);
        Ok(SbpUsbResponse::success(
            SbpUsbOpcode::UsbMount,
            msg.request_id,
            info.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbMount
    }
}

impl SbpUsbHandler for MetricsHandler {
    fn handle(
        &self,
        msg: &SbpUsbMessage,
        manager: &VoltUsbManager,
    ) -> VumResult<SbpUsbResponse> {
        let snapshot = manager.snapshot();
        let payload = serde_json::to_vec(&snapshot).map_err(|e| {
            crate::core::error::VumError::InvalidSbpMessage(e.to_string())
        })?;
        Ok(SbpUsbResponse::success(
            SbpUsbOpcode::UsbMetricsSnapshot,
            msg.request_id,
            payload,
        ))
    }

    fn opcode(&self) -> SbpUsbOpcode {
        SbpUsbOpcode::UsbMetricsSnapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::VumConfig;
    use crate::core::manager::VoltUsbManager;

    fn make_manager() -> VoltUsbManager {
        VoltUsbManager::new(VumConfig::default())
    }

    #[test]
    fn test_status_handler_returns_status() {
        let handler = StatusHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbStatus, 0, vec![]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        assert_eq!(resp.message.opcode, SbpUsbOpcode::UsbStatus);
        assert!(!resp.message.payload.is_empty());
    }

    #[test]
    fn test_read_handler_echoes_payload() {
        let handler = ReadHandler;
        let mgr = make_manager();
        let payload = b"/test/file".to_vec();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbRead, 0, payload.clone());
        let resp = handler.handle(&msg, &mgr).unwrap();
        assert_eq!(resp.message.payload, payload);
    }

    #[test]
    fn test_write_handler_returns_success() {
        let handler = WriteHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbWrite, 0, vec![0x01]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        assert_eq!(resp.message.opcode, SbpUsbOpcode::UsbWrite);
    }

    #[test]
    fn test_flush_handler_returns_count() {
        let handler = FlushHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbFlush, 0, vec![]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        assert_eq!(resp.message.opcode, SbpUsbOpcode::UsbFlush);
    }

    #[test]
    fn test_eject_handler_safe_when_no_pending() {
        let handler = EjectHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbEject, 0, vec![]);
        let result = handler.handle(&msg, &mgr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_heartbeat_sets_event_flag() {
        let handler = HeartbeatHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbHeartbeat, 0, vec![]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        assert!(resp
            .message
            .flags
            .contains(crate::sbp_usb::message::MessageFlags::EVENT));
    }

    #[test]
    fn test_mount_handler_uses_config_default() {
        let handler = MountHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbMount, 0, vec![]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        let text = String::from_utf8_lossy(&resp.message.payload);
        assert!(text.contains("mounted at"));
    }

    #[test]
    fn test_metrics_handler_returns_json() {
        let handler = MetricsHandler;
        let mgr = make_manager();
        let msg = SbpUsbMessage::new(SbpUsbOpcode::UsbMetricsSnapshot, 0, vec![]);
        let resp = handler.handle(&msg, &mgr).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_slice(&resp.message.payload).unwrap();
        assert!(parsed.get("uptime_ms").is_some());
    }

    #[test]
    fn test_opcode_method_returns_correct() {
        assert_eq!(StatusHandler.opcode(), SbpUsbOpcode::UsbStatus);
        assert_eq!(ReadHandler.opcode(), SbpUsbOpcode::UsbRead);
        assert_eq!(WriteHandler.opcode(), SbpUsbOpcode::UsbWrite);
        assert_eq!(FlushHandler.opcode(), SbpUsbOpcode::UsbFlush);
        assert_eq!(EjectHandler.opcode(), SbpUsbOpcode::UsbEject);
        assert_eq!(HeartbeatHandler.opcode(), SbpUsbOpcode::UsbHeartbeat);
        assert_eq!(MountHandler.opcode(), SbpUsbOpcode::UsbMount);
        assert_eq!(MetricsHandler.opcode(), SbpUsbOpcode::UsbMetricsSnapshot);
    }
}
