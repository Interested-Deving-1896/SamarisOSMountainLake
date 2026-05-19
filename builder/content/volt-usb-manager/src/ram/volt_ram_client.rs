use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use crate::core::error::VumError;
use crate::core::result::VumResult;

const DEFAULT_SOCKET_PATH: &str = "/tmp/volt-ram-mgr.sock";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

pub struct VoltRamClient {
    connected: bool,
    quota_mb: u64,
    socket_path: String,
}

impl VoltRamClient {
    pub fn new() -> Self {
        VoltRamClient {
            connected: false,
            quota_mb: 512,
            socket_path: DEFAULT_SOCKET_PATH.to_string(),
        }
    }

    pub fn connect(&mut self) -> VumResult<()> {
        let socket_path = Path::new(&self.socket_path);
        if socket_path.exists() {
            match UnixStream::connect(socket_path) {
                Ok(mut stream) => {
                    stream
                        .set_read_timeout(Some(CONNECT_TIMEOUT))
                        .ok();
                    stream
                        .set_write_timeout(Some(CONNECT_TIMEOUT))
                        .ok();
                    let msg = b"SBP-MEM:CONNECT\n";
                    stream
                        .write_all(msg)
                        .map_err(|e| VumError::InternalInvariantViolation(format!(
                            "Failed to send connect to volt-ram-manager: {}",
                            e
                        )))?;
                    let mut response = [0u8; 64];
                    let n = stream
                        .read(&mut response)
                        .map_err(|e| VumError::InternalInvariantViolation(format!(
                            "Failed to read response from volt-ram-manager: {}",
                            e
                        )))?;
                    let resp = String::from_utf8_lossy(&response[..n]);
                    if resp.starts_with("OK") || resp.contains("CONNECTED") {
                        self.connected = true;
                        return Ok(());
                    }
                }
                Err(e) => {
                    return Err(VumError::InternalInvariantViolation(format!(
                        "Cannot connect to volt-ram-manager at {}: {}",
                        self.socket_path, e
                    )));
                }
            }
        }
        self.connected = false;
        Err(VumError::InternalInvariantViolation(
            "volt-ram-manager socket not available".into(),
        ))
    }

    pub fn request_quota(&mut self, mb: u64) -> VumResult<()> {
        if !self.connected {
            return Err(VumError::InternalInvariantViolation(
                "Not connected to volt-ram-manager".into(),
            ));
        }
        let socket_path = Path::new(&self.socket_path);
        let mut stream = UnixStream::connect(socket_path)
            .map_err(|e| VumError::InternalInvariantViolation(format!(
                "Reconnect failed: {}",
                e
            )))?;
        stream
            .set_read_timeout(Some(CONNECT_TIMEOUT))
            .ok();
        stream
            .set_write_timeout(Some(CONNECT_TIMEOUT))
            .ok();

        let msg = format!("SBP-MEM:REQUEST_QUOTA:{}\n", mb);
        stream
            .write_all(msg.as_bytes())
            .map_err(|e| VumError::InternalInvariantViolation(format!(
                "Failed to request quota: {}",
                e
            )))?;

        let mut response = [0u8; 64];
        let n = stream
            .read(&mut response)
            .map_err(|e| VumError::InternalInvariantViolation(format!(
                "Failed to read quota response: {}",
                e
            )))?;
        let resp = String::from_utf8_lossy(&response[..n]);
        if resp.starts_with("OK") || resp.contains("GRANTED") {
            self.quota_mb = mb;
            Ok(())
        } else {
            Err(VumError::InternalInvariantViolation(format!(
                "Quota request denied: {}",
                resp
            )))
        }
    }

    pub fn release_quota(&mut self) -> VumResult<()> {
        if !self.connected {
            return Err(VumError::InternalInvariantViolation(
                "Not connected to volt-ram-manager".into(),
            ));
        }
        let socket_path = Path::new(&self.socket_path);
        let mut stream = UnixStream::connect(socket_path)
            .map_err(|e| VumError::InternalInvariantViolation(format!(
                "Reconnect failed: {}",
                e
            )))?;
        stream
            .set_read_timeout(Some(CONNECT_TIMEOUT))
            .ok();
        stream
            .set_write_timeout(Some(CONNECT_TIMEOUT))
            .ok();

        let msg = format!("SBP-MEM:RELEASE_QUOTA:{}\n", self.quota_mb);
        stream
            .write_all(msg.as_bytes())
            .map_err(|e| VumError::InternalInvariantViolation(format!(
                "Failed to release quota: {}",
                e
            )))?;

        let mut response = [0u8; 64];
        let _ = stream.read(&mut response);
        self.connected = false;
        self.quota_mb = 0;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn quota_mb(&self) -> u64 {
        self.quota_mb
    }

    pub fn set_socket_path(&mut self, path: &str) {
        self.socket_path = path.to_string();
    }
}

impl Default for VoltRamClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client_not_connected() {
        let client = VoltRamClient::new();
        assert!(!client.is_connected());
        assert_eq!(client.quota_mb(), 512);
    }

    #[test]
    fn test_connect_no_server() {
        let mut client = VoltRamClient::new();
        client.set_socket_path("/tmp/__vum_test_nonexistent_socket_volt_ram");
        let result = client.connect();
        assert!(result.is_err());
        assert!(!client.is_connected());
    }

    #[test]
    fn test_request_quota_not_connected() {
        let mut client = VoltRamClient::new();
        let result = client.request_quota(256);
        assert!(result.is_err());
    }

    #[test]
    fn test_release_quota_not_connected() {
        let mut client = VoltRamClient::new();
        let result = client.release_quota();
        assert!(result.is_err());
    }

    #[test]
    fn test_set_socket_path() {
        let mut client = VoltRamClient::new();
        client.set_socket_path("/tmp/custom.sock");
        assert!(!client.is_connected());
    }
}
