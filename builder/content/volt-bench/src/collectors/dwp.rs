use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

const SOCKET_PATH: &str = "/run/samaris/volt-worker-pool.sock";

pub struct DwpCollector;

impl Collector for DwpCollector {
    fn name(&self) -> &'static str { "dwp" }

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
                    "workers_min": 0,
                    "workers_active": 0,
                    "workers_max": 0,
                    "queue_depth": 0,
                    "burst_usage": 0.0,
                    "thermal_scaling_events": 0,
                    "task_completion_latency_ms": 0.0,
                }))
            }
            Err(e) => {
                tracing::warn!("DWP SBP IPC failed: {}", e);
                Ok(json!({
                    "workers_min": 0,
                    "workers_active": 0,
                    "workers_max": 0,
                    "queue_depth": 0,
                    "burst_usage": 0.0,
                    "thermal_scaling_events": 0,
                    "task_completion_latency_ms": 0.0,
                    "error": e.to_string(),
                }))
            }
        }
    }
}
