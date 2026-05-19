use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio::sync::broadcast;

use crate::core::config::TesseractConfig;
use crate::core::error::{Result, TesseractError};
use crate::protocol::flatbuffer;
use crate::protocol::header::{Flags, SbpHeader, SBP_HEADER_SIZE};
use crate::protocol::opcodes::Opcode;
use crate::protocol::{CommandPayload, TesseractCommand};
use crate::safety::SafetySupervisor;
use crate::scheduler::Scheduler;
use crate::security::SecurityManager;
use crate::system::SystemMonitor;
use crate::telemetry::Telemetry;

const PROTO_SBP: u8 = b'S';
const PROTO_JSON: u8 = b'J';

pub struct UnixSocketServer {
    shutdown_tx: broadcast::Sender<()>,
    running: Arc<AtomicBool>,
}

struct ServerCtx {
    scheduler: Arc<Scheduler>,
    security: Arc<SecurityManager>,
    telemetry: Arc<Telemetry>,
    system_monitor: Arc<SystemMonitor>,
    safety: Arc<SafetySupervisor>,
    started_at: std::time::Instant,
}

impl UnixSocketServer {
    pub fn start(
        config: &TesseractConfig,
        scheduler: Arc<Scheduler>,
        security: Arc<SecurityManager>,
        telemetry: Arc<Telemetry>,
        system_monitor: Arc<SystemMonitor>,
        safety: Arc<SafetySupervisor>,
    ) -> Result<Self> {
        let socket_path = config.socket_path.clone();
        if let Some(parent) = std::path::Path::new(&socket_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| TesseractError::Ipc(format!("cannot create socket directory: {e}")))?;
        }

        if std::fs::metadata(&socket_path).is_ok() {
            std::fs::remove_file(&socket_path)
                .map_err(|e| TesseractError::Ipc(format!("cannot remove stale socket: {e}")))?;
        }

        let listener = UnixListener::bind(&socket_path)
            .map_err(|e| TesseractError::Ipc(format!("socket bind: {e}")))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&socket_path, std::fs::Permissions::from_mode(0o666)).ok();
        }

        let (shutdown_tx, _) = broadcast::channel::<()>(1);
        let running = Arc::new(AtomicBool::new(true));
        let shutdown_tx_return = shutdown_tx.clone();
        let srv_running = running.clone();

        let ctx = Arc::new(ServerCtx {
            scheduler, security, telemetry, system_monitor, safety,
            started_at: std::time::Instant::now(),
        });

        let mut shutdown_rx = shutdown_tx.subscribe();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = listener.accept() => {
                        match result {
                            Ok((stream, _)) => {
                                let ctx = ctx.clone();
                                let mut shutdown_rx = shutdown_tx.subscribe();
                                tokio::spawn(async move {
                                    if let Err(e) = handle_connection(stream, ctx, &mut shutdown_rx).await {
                                        tracing::debug!("connection handler: {e}");
                                    }
                                });
                            }
                            Err(e) => { tracing::error!("accept: {e}"); break; }
                        }
                    }
                    _ = shutdown_rx.recv() => { break; }
                }
            }
            srv_running.store(false, Ordering::SeqCst);
            tracing::info!("Unix socket server stopped");
        });

        tracing::info!("Unix socket server on {socket_path}");
        Ok(Self { shutdown_tx: shutdown_tx_return, running })
    }

    pub fn shutdown(&self) { let _ = self.shutdown_tx.send(()); }
    pub fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) }
}

async fn handle_connection(
    mut stream: tokio::net::UnixStream,
    ctx: Arc<ServerCtx>,
    shutdown_rx: &mut broadcast::Receiver<()>,
) -> Result<()> {
    let mut buf = [0u8; 1];
    tokio::select! {
        result = stream.read_exact(&mut buf) => {
            result.map_err(|e| TesseractError::Ipc(format!("read protocol byte: {e}")))?;
        }
        _ = shutdown_rx.recv() => { return Ok(()); }
    }

    match buf[0] {
        PROTO_SBP => handle_sbp(stream, ctx, shutdown_rx).await,
        PROTO_JSON => handle_json(stream, ctx).await,
        other => Err(TesseractError::Protocol(format!("unknown protocol byte: 0x{other:02X}"))),
    }
}

async fn handle_sbp(
    mut stream: tokio::net::UnixStream,
    ctx: Arc<ServerCtx>,
    shutdown_rx: &mut broadcast::Receiver<()>,
) -> Result<()> {
    let mut header_buf = [0u8; SBP_HEADER_SIZE - 1];
    tokio::select! {
        result = stream.read_exact(&mut header_buf) => {
            result.map_err(|e| TesseractError::Ipc(format!("read header: {e}")))?;
        }
        _ = shutdown_rx.recv() => { return Ok(()); }
    }

    let mut full_header = [0u8; SBP_HEADER_SIZE];
    full_header[0] = PROTO_SBP;
    full_header[1..].copy_from_slice(&header_buf);
    let header = SbpHeader::decode(&full_header)?;
    let payload_len = header.payload_len as usize;

    let mut payload = vec![0u8; payload_len];
    if payload_len > 0 {
        tokio::select! {
            result = stream.read_exact(&mut payload) => {
                result.map_err(|e| TesseractError::Ipc(format!("read payload: {e}")))?;
            }
            _ = shutdown_rx.recv() => { return Ok(()); }
        }
    }

    let cmd = TesseractCommand::new(header, payload.clone());
    let opcode_byte = cmd.header.opcode;
    let priority = cmd.header.priority;
    let app_id = cmd.header.app_id;

    if let Err(e) = ctx.security.authorize(&cmd) {
        send_sbp_error(&mut stream, opcode_byte, priority, app_id, &e.to_string()).await?;
        return Ok(());
    }

    let response = ctx.scheduler.submit(cmd);

    match response {
        Ok(result_payload) => {
            let fb_data = flatbuffer::command_to_payload(
                &result_payload,
                Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
            );
            let resp_header = SbpHeader::new(
                Opcode::from_byte(opcode_byte).unwrap_or(Opcode::Heartbeat),
                priority, app_id, fb_data.len() as u32,
            ).with_flags(Flags::RESPONSE);
            let mut resp = resp_header.encode().to_vec();
            resp.extend_from_slice(&fb_data);
            tokio::select! {
                result = stream.write_all(&resp) => {
                    result.map_err(|e| TesseractError::Ipc(format!("write response: {e}")))?;
                }
                _ = shutdown_rx.recv() => { return Ok(()); }
            }
        }
        Err(error_msg) => {
            send_sbp_error(&mut stream, opcode_byte, priority, app_id, &error_msg).await?;
        }
    }
    Ok(())
}

async fn send_sbp_error(
    stream: &mut tokio::net::UnixStream,
    _opcode_byte: u8, priority: u8, app_id: u32, msg: &str,
) -> Result<()> {
    let fbb = flatbuffer::command_to_payload(&CommandPayload::Empty, Opcode::Heartbeat);
    let resp_header = SbpHeader::new(Opcode::Heartbeat, priority, app_id, fbb.len() as u32)
        .with_flags(Flags::RESPONSE | Flags::ERROR);
    let mut resp = resp_header.encode().to_vec();
    resp.extend_from_slice(&fbb);
    stream.write_all(&resp).await
        .map_err(|e| TesseractError::Ipc(format!("write error: {e}")))?;
    tracing::warn!("SBP error response: {msg}");
    Ok(())
}

async fn handle_json(
    mut stream: tokio::net::UnixStream,
    ctx: Arc<ServerCtx>,
) -> Result<()> {
    use tokio::io::{BufReader, AsyncBufReadExt};

    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await.map_err(|e| TesseractError::Ipc(e.to_string()))? {
        if line.trim().is_empty() { continue; }

        let request: serde_json::Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(e) => {
                let err = serde_json::json!({"jsonrpc":"2.0","id":null,"error":{"code":"INVALID_JSON","message":e.to_string()}});
                writer.write_all(err.to_string().as_bytes()).await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
                writer.write_all(b"\n").await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
                continue;
            }
        };

        let id = request.get("id").cloned().unwrap_or(serde_json::Value::Null);
        let method = request.get("method").and_then(|v| v.as_str()).unwrap_or("").to_string();

        let response = match method.as_str() {
            "health" => serde_json::json!({
                "jsonrpc":"2.0","id":id,"result":{"ok":true,"service":"tesseract-engine","version":"1.0.0-alpha"}
            }),
            "ping" => serde_json::json!({"jsonrpc":"2.0","id":id,"result":{"pong":true}}),

            "query_cores" => submit_sbp(&ctx, Opcode::QueryCores, "").await.unwrap_or(serde_json::json!({"cores":0})),
            "query_gpu" => submit_sbp(&ctx, Opcode::QueryGpu, "").await.unwrap_or(serde_json::json!({"available":false})),
            "thermal_status" => submit_sbp(&ctx, Opcode::ThermalStatus, "").await.unwrap_or(serde_json::json!({"zones":0})),

            "metrics" => {
                let _uptime = ctx.started_at.elapsed().as_secs();
                let m = ctx.telemetry.snapshot();
                serde_json::json!({
                    "jsonrpc":"2.0","id":id,"result":{
                        "commands_processed": m.commands_processed,
                        "errors_count": m.errors_count,
                        "avg_execution_time_us": m.avg_execution_time_us,
                        "commands_per_second": m.commands_per_second,
                        "uptime_secs": m.uptime_secs,
                        "worker_active": ctx.scheduler.worker_count(),
                        "throttle_factor": ctx.safety.current_throttle(),
                    }
                })
            }

            "safety_status" => {
                let lim = ctx.safety.limiter().read();
                serde_json::json!({
                    "jsonrpc":"2.0","id":id,"result":{
                        "emergency_stop": ctx.safety.is_emergency_stop(),
                        "throttle_factor": ctx.safety.current_throttle(),
                        "max_memory_bytes": lim.max_total_memory,
                        "max_open_sockets": lim.max_open_sockets,
                        "max_concurrent_tasks": lim.max_concurrent_tasks,
                        "current_tasks": lim.current_tasks,
                        "current_sockets": lim.current_sockets,
                    }
                })
            }

            "system_status" => {
                let snap = ctx.system_monitor.collect_all().unwrap_or_default();
                serde_json::json!({
                    "jsonrpc":"2.0","id":id,"result":{
                        "cpu_cores": snap.cpu.count,
                        "cpu_load_percent": snap.cpu.total_load_percent,
                        "per_core_load": snap.cpu.per_core_load,
                        "memory_total_kb": snap.memory.total_kb,
                        "memory_available_kb": snap.memory.available_kb,
                        "memory_used_kb": snap.memory.used_kb,
                        "memory_swap_total_kb": snap.memory.swap_total_kb,
                        "memory_swap_used_kb": snap.memory.swap_used_kb,
                        "memory_usage_percent": snap.memory.usage_percent,
                        "thermal_max_celsius": snap.thermal_max,
                        "process_count": snap.processes.processes.len(),
                    }
                })
            }

            "audit_log" => {
                let audit = ctx.security.audit().read();
                let entries: Vec<serde_json::Value> = audit.recent(100).iter().map(|e| serde_json::json!({
                    "timestamp_us": e.timestamp, "app_id": e.app_id,
                    "opcode": e.opcode, "action": &e.action,
                    "allowed": e.allowed, "reason": &e.reason,
                })).collect();
                serde_json::json!({"jsonrpc":"2.0","id":id,"result":{"entries":entries,"count":entries.len()}})
            }

            "sys_info" => {
                let cores = std::thread::available_parallelism().map(|n|n.get()).unwrap_or(1);
                serde_json::json!({
                    "jsonrpc":"2.0","id":id,"result":{
                        "cores": cores, "worker_pool_size": ctx.scheduler.worker_count(),
                        "opcodes_supported": [
                            "gpu_render","gpu_compute","cpu_reserve","cpu_release",
                            "cpu_exec","mem_alloc","mem_free","stream_video","stream_audio",
                            "query_cores","query_gpu","heartbeat","thermal_status",
                            "context_create","context_share"
                        ],
                        "methods":["health","ping","query_cores","query_gpu","thermal_status",
                                   "metrics","safety_status","system_status","audit_log","sys_info"],
                        "transport":"unix_socket","protocols":["sbp_v5","json_rpc_2.0"],
                        "subsystems":["gpu_canvas","compute_bridge","media_engine","security","safety","telemetry","system_monitor"],
                    }
                })
            }

            _ => serde_json::json!({
                "jsonrpc":"2.0","id":id,
                "error":{"code":"METHOD_NOT_FOUND","message":format!("unknown method: {method}")}
            }),
        };

        writer.write_all(response.to_string().as_bytes()).await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
        writer.write_all(b"\n").await.map_err(|e| TesseractError::Ipc(e.to_string()))?;
    }
    Ok(())
}

async fn submit_sbp(ctx: &ServerCtx, opcode: Opcode, payload: &str) -> Option<serde_json::Value> {
    let cmd = TesseractCommand::new(
        SbpHeader::new(opcode, 2, 1, payload.len() as u32),
        payload.as_bytes().to_vec(),
    );
    match ctx.scheduler.submit(cmd) {
        Ok(p) => {
            let raw = match &p {
                CommandPayload::RawBytes(b) => String::from_utf8_lossy(b).to_string(),
                _ => String::new(),
            };
            serde_json::from_str(&raw).ok()
        }
        Err(_) => None,
    }
}
