use std::os::unix::net::UnixDatagram;

use crate::core::error::{Result, TesseractError};

pub struct SystemdNotifier;

impl SystemdNotifier {
    pub fn notify(state: &str) -> Result<()> {
        let sock_path = match std::env::var("NOTIFY_SOCKET") {
            Ok(p) => p,
            Err(_) => {
                tracing::debug!("sd_notify: NOTIFY_SOCKET not set — not running under systemd");
                return Ok(());
            }
        };

        let socket = UnixDatagram::unbound()
            .map_err(|e| TesseractError::System(format!("sd_notify socket: {e}")))?;

        socket
            .send_to(state.as_bytes(), &sock_path)
            .map_err(|e| TesseractError::System(format!("sd_notify send: {e}")))?;

        tracing::info!("sd_notify: {state}");
        Ok(())
    }

    pub fn notify_ready() {
        Self::notify("READY=1").ok();
    }

    pub fn notify_status(status: &str) {
        Self::notify(&format!("STATUS={status}")).ok();
    }

    pub fn notify_stopping() {
        Self::notify("STOPPING=1").ok();
    }
}
