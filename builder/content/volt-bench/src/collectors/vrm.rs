use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

const SOCKET_PATH: &str = "/run/samaris/volt-ram-manager.sock";

pub struct VrmCollector;

impl Collector for VrmCollector {
    fn name(&self) -> &'static str { "vrm" }

    fn collect(&self, _hardware: &HardwareInfo, _env: &EnvironmentInfo) -> CollectorResult {
        match send_sbp_request("status") {
            Ok(response) => Ok(response),
            Err(e) => {
                tracing::warn!("VRM SBP IPC failed: {} (socket: {})", e, SOCKET_PATH);
                Ok(json!({
                    "compression_ratio": 0.0,
                    "dedup_ratio": 0.0,
                    "quota_usage_per_app": {},
                    "pressure_zone": 0,
                    "tier_migration_count": 0,
                    "swap_avoidance_score": 0.0,
                    "memory_reclaim_latency_ms": 0.0,
                    "error": e.to_string(),
                }))
            }
        }
    }
}

fn send_sbp_request(_command: &str) -> Result<Value, crate::errors::BenchError> {
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    let mut socket = UnixStream::connect(SOCKET_PATH)
        .map_err(|e| crate::errors::BenchError::SbpIpcError(format!("connect: {}", e)))?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| crate::errors::BenchError::SbpIpcError(format!("set_timeout: {}", e)))?;
    socket.write_all(b"status\n")
        .map_err(|e| crate::errors::BenchError::SbpIpcError(format!("write: {}", e)))?;

    let mut buf = vec![0u8; 65536];
    let n = socket.read(&mut buf)
        .map_err(|e| crate::errors::BenchError::SbpIpcError(format!("read: {}", e)))?;
    buf.truncate(n);

    let response: Value = serde_json::from_slice(&buf)?;
    Ok(response)
}
