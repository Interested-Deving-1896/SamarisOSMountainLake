pub mod shm;
pub mod unix_socket;
pub mod websocket;

use std::sync::Arc;

use crate::core::config::TesseractConfig;
use crate::core::error::Result;
use crate::safety::SafetySupervisor;
use crate::scheduler::Scheduler;
use crate::security::SecurityManager;
use crate::system::SystemMonitor;
use crate::telemetry::Telemetry;

pub enum IpcServer {
    UnixSocket(unix_socket::UnixSocketServer),
    WebSocket(websocket::WebSocketServer),
    Both(unix_socket::UnixSocketServer, websocket::WebSocketServer),
}

impl IpcServer {
    pub fn start(
        config: &TesseractConfig,
        scheduler: Arc<Scheduler>,
        security: Arc<SecurityManager>,
        telemetry: Arc<Telemetry>,
        system_monitor: Arc<SystemMonitor>,
        safety: Arc<SafetySupervisor>,
    ) -> Result<Self> {
        let unix = unix_socket::UnixSocketServer::start(
            config, scheduler.clone(), security.clone(),
            telemetry.clone(), system_monitor.clone(), safety.clone(),
        )?;

        if config.debug_mode {
            let ws = websocket::WebSocketServer::start(
                config, scheduler, security,
                telemetry, system_monitor, safety,
            )?;
            tracing::info!("WebSocket server on ws://127.0.0.1:{} (debug)", config.websocket_port);
            Ok(Self::Both(unix, ws))
        } else {
            Ok(Self::UnixSocket(unix))
        }
    }

    pub fn shutdown(&self) {
        match self {
            Self::UnixSocket(srv) => srv.shutdown(),
            Self::WebSocket(srv) => srv.shutdown(),
            Self::Both(unix, ws) => { unix.shutdown(); ws.shutdown(); }
        }
    }
}
