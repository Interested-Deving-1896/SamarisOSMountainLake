use serde_json::{json, Value};
use crate::collectors::{Collector, CollectorResult};
use crate::hardware::HardwareInfo;
use crate::environment::EnvironmentInfo;

const SOCKET_PATH: &str = "/run/samaris/volt-usb-manager.sock";

pub struct VumCollector;

impl Collector for VumCollector {
    fn name(&self) -> &'static str { "vum" }

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
                    "cache_hit_rate": 0.0,
                    "writeback_queue_depth": 0,
                    "journal_replay_count": 0,
                    "fs_read_latency_ms": 0.0,
                    "fs_write_latency_ms": 0.0,
                }))
            }
            Err(e) => {
                tracing::warn!("VUM SBP IPC failed: {}", e);
                Ok(json!({
                    "cache_hit_rate": 0.0,
                    "writeback_queue_depth": 0,
                    "journal_replay_count": 0,
                    "fs_read_latency_ms": 0.0,
                    "fs_write_latency_ms": 0.0,
                    "error": e.to_string(),
                }))
            }
        }
    }
}
