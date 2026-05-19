use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

use crate::core::config::TesseractConfig;
use crate::core::error::{Result, TesseractError};
use crate::protocol::flatbuffer;
use crate::protocol::header::{SbpHeader, SBP_HEADER_SIZE};
use crate::protocol::opcodes::Opcode;
use crate::protocol::{CommandPayload, TesseractCommand};
use crate::safety::SafetySupervisor;
use crate::scheduler::Scheduler;
use crate::security::SecurityManager;
use crate::system::SystemMonitor;
use crate::telemetry::Telemetry;

pub struct WebSocketServer {
    shutdown_tx: broadcast::Sender<()>,
    running: Arc<AtomicBool>,
}

impl WebSocketServer {
    pub fn start(
        config: &TesseractConfig,
        scheduler: Arc<Scheduler>,
        security: Arc<SecurityManager>,
        _telemetry: Arc<Telemetry>,
        _system_monitor: Arc<SystemMonitor>,
        _safety: Arc<SafetySupervisor>,
    ) -> Result<Self> {
        let addr = format!("127.0.0.1:{}", config.websocket_port);

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| TesseractError::Ipc(format!("tokio rt: {e}")))?;

        let listener = rt.block_on(TcpListener::bind(&addr))
            .map_err(|e| TesseractError::Ipc(format!("ws bind: {e}")))?;

        let (shutdown_tx, _) = broadcast::channel::<()>(1);
        let running = Arc::new(AtomicBool::new(true));
        let shutdown_tx_return = shutdown_tx.clone();

        let srv_running = running.clone();
        let mut shutdown_rx = shutdown_tx.subscribe();

        std::thread::spawn(move || {
            rt.block_on(async move {
                loop {
                    tokio::select! {
                        result = listener.accept() => {
                            match result {
                                Ok((stream, _)) => {
                                    let sched = scheduler.clone();
                                    let sec = security.clone();
                                    let mut shutdown_rx = shutdown_tx.subscribe();
                                    tokio::spawn(async move {
                                        if let Err(e) = handle_ws_client(stream, sched, sec, &mut shutdown_rx).await {
                                            tracing::debug!("ws client: {e}");
                                        }
                                    });
                                }
                                Err(e) => {
                                    tracing::error!("ws accept: {e}");
                                    break;
                                }
                            }
                        }
                        _ = shutdown_rx.recv() => {
                            break;
                        }
                    }
                }
                srv_running.store(false, Ordering::SeqCst);
            });
            tracing::info!("WebSocket server stopped");
        });

        tracing::info!("WebSocket debug server on ws://{addr}");
        Ok(Self { shutdown_tx: shutdown_tx_return, running })
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

async fn handle_ws_client(
    mut stream: tokio::net::TcpStream,
    scheduler: Arc<Scheduler>,
    security: Arc<SecurityManager>,
    shutdown_rx: &mut broadcast::Receiver<()>,
) -> Result<()> {
    let mut header_buf = [0u8; SBP_HEADER_SIZE];

    tokio::select! {
        result = stream.read_exact(&mut header_buf) => {
            result.map_err(|e| TesseractError::Ipc(format!("ws read header: {e}")))?;
        }
        _ = shutdown_rx.recv() => {
            return Ok(());
        }
    }

    let header = SbpHeader::decode(&header_buf)?;
    let payload_len = header.payload_len as usize;

    let mut payload = vec![0u8; payload_len];
    if payload_len > 0 {
        tokio::select! {
            result = stream.read_exact(&mut payload) => {
                result.map_err(|e| TesseractError::Ipc(format!("ws read payload: {e}")))?;
            }
            _ = shutdown_rx.recv() => {
                return Ok(());
            }
        }
    }

    let cmd = TesseractCommand::new(header, payload);
    let opcode_byte = cmd.header.opcode;
    let priority = cmd.header.priority;
    let app_id = cmd.header.app_id;

    if let Err(e) = security.authorize(&cmd) {
        let resp = make_error_response(opcode_byte, priority, app_id, &e.to_string());
        stream.write_all(&resp).await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
        return Ok(());
    }

    let response = scheduler.submit(cmd);

    match response {
        Ok(result_payload) => {
            let fb_data = flatbuffer::command_to_payload(
                &result_payload,
                Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
            );
            let resp_header = SbpHeader::new(
                Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
                priority,
                app_id,
                fb_data.len() as u32,
            )
            .with_flags(crate::protocol::header::Flags::RESPONSE);
            let mut resp = resp_header.encode().to_vec();
            resp.extend_from_slice(&fb_data);
            stream.write_all(&resp).await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
        }
        Err(error_msg) => {
            let resp = make_error_response(opcode_byte, priority, app_id, &error_msg);
            stream.write_all(&resp).await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
        }
    }

    Ok(())
}

fn make_error_response(opcode_byte: u8, priority: u8, app_id: u32, msg: &str) -> Vec<u8> {
    let fb_data = flatbuffer::command_to_payload(
        &CommandPayload::Empty,
        Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
    );
    let resp_header = SbpHeader::new(
        Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
        priority,
        app_id,
        fb_data.len() as u32,
    )
    .with_flags(
        crate::protocol::header::Flags::RESPONSE | crate::protocol::header::Flags::ERROR,
    );
    let mut resp = resp_header.encode().to_vec();
    resp.extend_from_slice(&fb_data);
    tracing::warn!("WS error response: {msg}");
    resp
}
