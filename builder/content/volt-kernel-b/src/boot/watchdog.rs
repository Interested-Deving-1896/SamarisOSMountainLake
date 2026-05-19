use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::error::{Result, TesseractError};

pub struct WatchdogFiles;

impl WatchdogFiles {
    pub fn ready_dir() -> PathBuf {
        PathBuf::from("/run")
    }

    pub fn write(name: &str) -> Result<()> {
        let path = Self::ready_dir().join(format!("volt-{name}.ready"));

        let pid = std::process::id();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let content = format!("PID={pid}\nREADY_AT={timestamp}\n");

        match std::fs::File::create(&path) {
            Ok(mut file) => {
                file.write_all(content.as_bytes())
                    .map_err(|e| TesseractError::System(format!("write watchdog {path:?}: {e}")))?;
                tracing::info!("Watchdog file created: {path:?}");
            }
            Err(e) => {
                tracing::warn!("Cannot create watchdog {path:?}: {e} — not running on Linux?");
            }
        }

        Ok(())
    }

    pub fn exists(name: &str) -> bool {
        Self::ready_dir().join(format!("volt-{name}.ready")).exists()
    }

    pub fn remove(name: &str) {
        let path = Self::ready_dir().join(format!("volt-{name}.ready"));
        if path.exists() {
            std::fs::remove_file(&path).ok();
        }
    }

    pub fn write_kernel_b_ready() {
        Self::write("kernel-b").ok();
    }

    pub fn write_all() {
        Self::write_kernel_b_ready();
    }

    pub fn wait_for(name: &str, timeout_ms: u64) -> bool {
        let start = std::time::Instant::now();
        let path = Self::ready_dir().join(format!("volt-{name}.ready"));
        while start.elapsed().as_millis() < timeout_ms as u128 {
            if path.exists() {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        false
    }
}
