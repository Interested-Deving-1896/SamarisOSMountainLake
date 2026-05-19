use crate::core::error::VgmError;
use crate::core::result::VgmResult;
use crate::sbp_gpu::message::SbpGpuMessage;
use crate::sbp_gpu::opcode::SbpGpuOpcode;
use crate::sbp_gpu::response::SbpGpuResponse;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait SbpGpuHandler: Send + Sync {
    fn handle(&self, message: &SbpGpuMessage) -> VgmResult<SbpGpuResponse>;
    fn opcode(&self) -> SbpGpuOpcode;
}

pub struct StatusHandler;

impl SbpGpuHandler for StatusHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let payload = format!(
            "status:ok:opcode={} request_id={}",
            msg.opcode.name(),
            msg.request_id
        );
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuStatus,
            msg.request_id,
            payload.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuStatus
    }
}

pub struct ComputeHandler;

impl SbpGpuHandler for ComputeHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let payload = format!("compute:dispatched:request_id={}:ts={}", msg.request_id, now);
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuExecCompute,
            msg.request_id,
            payload.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuExecCompute
    }
}

impl ComputeHandler {
    pub fn handle_batch(msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let payload = format!(
            "batch:submitted:{}_commands:request_id={}",
            msg.payload.len(),
            msg.request_id
        );
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuBatchSubmit,
            msg.request_id,
            payload.into_bytes(),
        ))
    }
}

pub struct RenderHandler;

impl SbpGpuHandler for RenderHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let payload = format!(
            "render:queued:request_id={}:ts={}:priority=critical",
            msg.request_id, now
        );
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuRenderFrame,
            msg.request_id,
            payload.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuRenderFrame
    }
}

pub struct ThermalHandler;

impl SbpGpuHandler for ThermalHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let payload = format!(
            "thermal:nominal:request_id={}",
            msg.request_id
        );
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuThermalStatus,
            msg.request_id,
            payload.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuThermalStatus
    }
}

pub struct VramHandler;

impl SbpGpuHandler for VramHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let response = match msg.opcode {
            SbpGpuOpcode::GpuVramStatus => {
                format!("vram:status:total_mb=8192:used_mb=2048:request_id={}", msg.request_id)
            }
            SbpGpuOpcode::GpuAllocResource => {
                let id = if msg.payload.len() >= 16 {
                    hex::encode(&msg.payload[..8])
                } else {
                    "unknown".to_string()
                };
                format!("vram:allocated:resource_id={}:request_id={}", id, msg.request_id)
            }
            SbpGpuOpcode::GpuFreeResource => {
                format!("vram:freed:request_id={}", msg.request_id)
            }
            _ => {
                return Err(VgmError::UnsupportedOpcode(format!(
                    "VramHandler cannot handle opcode {}",
                    msg.opcode.name()
                )));
            }
        };
        Ok(SbpGpuResponse::success(
            msg.opcode,
            msg.request_id,
            response.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuVramStatus
    }
}

pub struct MetricsHandler;

impl SbpGpuHandler for MetricsHandler {
    fn handle(&self, msg: &SbpGpuMessage) -> VgmResult<SbpGpuResponse> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let payload = format!(
            "metrics:snapshot:request_id={}:ts={}:fps=60:gpu_util=45:temp_c=65",
            msg.request_id, now
        );
        Ok(SbpGpuResponse::success(
            SbpGpuOpcode::GpuMetricsSnapshot,
            msg.request_id,
            payload.into_bytes(),
        ))
    }

    fn opcode(&self) -> SbpGpuOpcode {
        SbpGpuOpcode::GpuMetricsSnapshot
    }
}

mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_msg(opcode: SbpGpuOpcode, payload: Vec<u8>) -> SbpGpuMessage {
        let mut msg = SbpGpuMessage::new(opcode, payload);
        msg.request_id = 42;
        msg.recompute_checksum();
        msg
    }

    #[test]
    fn test_status_handler() {
        let handler = StatusHandler;
        assert_eq!(handler.opcode(), SbpGpuOpcode::GpuStatus);
        let msg = make_msg(SbpGpuOpcode::GpuStatus, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert!(resp.message.flags.contains(crate::sbp_gpu::message::MessageFlags::RESPONSE));
        assert_eq!(resp.message.request_id, 42);
    }

    #[test]
    fn test_compute_handler() {
        let handler = ComputeHandler;
        let msg = make_msg(SbpGpuOpcode::GpuExecCompute, vec![0x01]);
        let resp = handler.handle(&msg).unwrap();
        assert_eq!(resp.message.request_id, 42);
    }

    #[test]
    fn test_render_handler() {
        let handler = RenderHandler;
        let msg = make_msg(SbpGpuOpcode::GpuRenderFrame, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert!(String::from_utf8_lossy(&resp.message.payload).contains("render"));
    }

    #[test]
    fn test_thermal_handler() {
        let handler = ThermalHandler;
        let msg = make_msg(SbpGpuOpcode::GpuThermalStatus, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert_eq!(resp.message.opcode, SbpGpuOpcode::GpuThermalStatus);
    }

    #[test]
    fn test_vram_status_handler() {
        let handler = VramHandler;
        let msg = make_msg(SbpGpuOpcode::GpuVramStatus, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert!(String::from_utf8_lossy(&resp.message.payload).contains("vram:status"));
    }

    #[test]
    fn test_vram_alloc_handler() {
        let handler = VramHandler;
        let payload = (0..16).collect::<Vec<_>>();
        let msg = make_msg(SbpGpuOpcode::GpuAllocResource, payload);
        let resp = handler.handle(&msg).unwrap();
        assert!(String::from_utf8_lossy(&resp.message.payload).contains("vram:allocated"));
    }

    #[test]
    fn test_vram_free_handler() {
        let handler = VramHandler;
        let msg = make_msg(SbpGpuOpcode::GpuFreeResource, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert!(String::from_utf8_lossy(&resp.message.payload).contains("vram:freed"));
    }

    #[test]
    fn test_metrics_handler() {
        let handler = MetricsHandler;
        let msg = make_msg(SbpGpuOpcode::GpuMetricsSnapshot, vec![]);
        let resp = handler.handle(&msg).unwrap();
        assert!(String::from_utf8_lossy(&resp.message.payload).contains("metrics:snapshot"));
    }

    #[test]
    fn test_vram_handler_unsupported_opcode() {
        let handler = VramHandler;
        let msg = make_msg(SbpGpuOpcode::GpuRenderFrame, vec![]);
        let result = handler.handle(&msg);
        assert!(result.is_err());
    }
}
