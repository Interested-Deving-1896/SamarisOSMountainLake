use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

const SOCKET_PATH: &str = "/run/samaris/volt-gpu-manager.sock";

pub struct VgmCollector;

impl Collector for VgmCollector {
    fn name(&self) -> &'static str { "vgm" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        use std::io::{Read, Write};
        use std::os::unix::net::UnixStream;
        use std::time::Duration;

        match UnixStream::connect(SOCKET_PATH) {
            Ok(mut socket) => {
                socket.set_read_timeout(Some(Duration::from_secs(5))).ok();
                let _ = socket.write_all(b"status\n");
                let mut buf = vec![0u8; 65536];
                let n = socket.read(&mut buf).unwrap_or(0);
                buf.truncate(n);
                if !buf.is_empty() {
                    if let Ok(v) = serde_json::from_slice::<Value>(&buf) {
                        return Ok(v);
                    }
                }
                Ok(json!({
                    "vram_used_mb": 0.0,
                    "shader_cache_hit_rate": 0.0,
                    "frame_budget_ms": 0.0,
                    "thermal_backoff_events": 0,
                    "gpu_context_switch_latency_ms": 0.0,
                }))
            }
            Err(e) => {
                tracing::warn!("VGM SBP IPC failed: {}", e);
                Ok(json!({
                    "vram_used_mb": 0.0,
                    "shader_cache_hit_rate": 0.0,
                    "frame_budget_ms": 0.0,
                    "thermal_backoff_events": 0,
                    "gpu_context_switch_latency_ms": 0.0,
                    "error": e.to_string(),
                }))
            }
        }
    }
}
