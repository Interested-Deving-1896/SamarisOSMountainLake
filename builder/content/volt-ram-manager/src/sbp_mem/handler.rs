use crate::sbp_mem::opcode::SbpOpcode;
use crate::sbp_mem::message::SbpMessage;
use crate::sbp_mem::response::SbpResponse;
use crate::core::manager::VoltRamManager;
use crate::core::result::VrmResult;

pub trait SbpHandler: Send + Sync {
    fn handle(&self, msg: &SbpMessage, manager: &VoltRamManager) -> VrmResult<SbpResponse>;
    fn opcode(&self) -> SbpOpcode;
}

pub struct StatusHandler;
impl SbpHandler for StatusHandler {
    fn handle(&self, _msg: &SbpMessage, manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        let snap = manager.snapshot();
        let payload = serde_json::to_vec(&snap).unwrap_or_default();
        Ok(SbpResponse::success(SbpOpcode::RamStatus, _msg.request_id, payload))
    }
    fn opcode(&self) -> SbpOpcode { SbpOpcode::RamStatus }
}

pub struct RegisterAppHandler;
impl SbpHandler for RegisterAppHandler {
    fn handle(&self, _msg: &SbpMessage, _manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        Ok(SbpResponse::success(SbpOpcode::RamRegisterApp, _msg.request_id, vec![]))
    }
    fn opcode(&self) -> SbpOpcode { SbpOpcode::RamRegisterApp }
}

pub struct SetQuotaHandler;
impl SbpHandler for SetQuotaHandler {
    fn handle(&self, _msg: &SbpMessage, _manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        Ok(SbpResponse::success(SbpOpcode::RamSetQuota, _msg.request_id, vec![]))
    }
    fn opcode(&self) -> SbpOpcode { SbpOpcode::RamSetQuota }
}

pub struct HeartbeatHandler;
impl SbpHandler for HeartbeatHandler {
    fn handle(&self, _msg: &SbpMessage, _manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        Ok(SbpResponse::success(SbpOpcode::RamHeartbeat, _msg.request_id, b"pong".to_vec()))
    }
    fn opcode(&self) -> SbpOpcode { SbpOpcode::RamHeartbeat }
}

pub struct SnapshotHandler;
impl SbpHandler for SnapshotHandler {
    fn handle(&self, _msg: &SbpMessage, manager: &VoltRamManager) -> VrmResult<SbpResponse> {
        let snap = manager.snapshot();
        let payload = serde_json::to_vec(&snap).unwrap_or_default();
        Ok(SbpResponse::success(SbpOpcode::RamSnapshot, _msg.request_id, payload))
    }
    fn opcode(&self) -> SbpOpcode { SbpOpcode::RamSnapshot }
}
