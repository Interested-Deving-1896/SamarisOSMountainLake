use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use parking_lot::{Mutex, RwLock};

use crate::core::error::{Result, TesseractError};
use crate::protocol::TesseractCommand;
use crate::compute_bridge::task::{ComputeKind, ComputeTask};
use crate::compute_bridge::ComputeBridge;
use crate::gpu_canvas::{GpuCanvas, commands::GpuCommand, commands::RgbaColor};
use crate::media::MediaEngine;

type JobFn = Box<dyn Fn(&TesseractCommand) -> Result<Vec<u8>> + Send + Sync>;

pub struct WorkerPool {
    workers: Vec<Worker>,
    job_registry: Arc<RwLock<JobRegistry>>,
    active_count: Arc<AtomicU32>,
    gpu: Arc<Mutex<GpuCanvas>>,
    compute: Arc<Mutex<ComputeBridge>>,
    media: Arc<Mutex<MediaEngine>>,
}

struct Worker { id: usize }

struct JobRegistry {
    handlers: Vec<(u8, JobFn)>,
}

impl JobRegistry {
    fn new() -> Self { Self { handlers: Vec::new() } }
    fn register(&mut self, opcode: u8, handler: JobFn) {
        self.handlers.push((opcode, handler));
    }
    fn dispatch(&self, cmd: &TesseractCommand) -> Result<Vec<u8>> {
        for (opcode, handler) in &self.handlers {
            if *opcode == cmd.header.opcode { return handler(cmd); }
        }
        Err(TesseractError::Protocol(format!("no handler for opcode 0x{:02X}", cmd.header.opcode)))
    }
}

impl WorkerPool {
    pub fn new(size: usize) -> Self {
        let pool = Self {
            workers: (0..size).map(|id| Worker { id }).collect(),
            job_registry: Arc::new(RwLock::new(JobRegistry::new())),
            active_count: Arc::new(AtomicU32::new(0)),
            gpu: Arc::new(Mutex::new(GpuCanvas::new())),
            compute: Arc::new(Mutex::new(ComputeBridge::new())),
            media: Arc::new(Mutex::new(MediaEngine::new())),
        };
        pool.register_defaults();
        pool
    }

    fn register_defaults(&self) {
        let gpu = self.gpu.clone();
        let compute = self.compute.clone();
        let media = self.media.clone();
        let mut reg = self.job_registry.write();

        // 0x01 GpuRender — render a rectangle via GpuCanvas
        reg.register(0x01, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let (x, y, w, h) = (
                parsed.int32_values.get(0).copied().unwrap_or(0),
                parsed.int32_values.get(1).copied().unwrap_or(0),
                parsed.int32_values.get(2).copied().unwrap_or(0) as u32,
                parsed.int32_values.get(3).copied().unwrap_or(0) as u32,
            );
            let (fr, fg, fb, fa) = (
                parsed.byte_data.get(0).copied().unwrap_or(0),
                parsed.byte_data.get(1).copied().unwrap_or(0),
                parsed.byte_data.get(2).copied().unwrap_or(0),
                parsed.byte_data.get(3).copied().unwrap_or(255),
            );
            let cmd = GpuCommand::RenderRect {
                x, y, w, h, border_radius: 0.0, shadow_blur: 0.0,
                shadow_offset_x: 0.0, shadow_offset_y: 0.0,
                fill_color: RgbaColor::new(fr, fg, fb, fa),
                border_color: RgbaColor::BLACK, border_width: 0.0,
            };
            let output = gpu.lock().execute_command(&cmd)?;
            let resp = serde_json::json!({
                "rendered": true, "gpu": output.gpu,
                "width": output.width, "height": output.height,
                "pixels_bytes": output.pixels.len(),
            });
            Ok(resp.to_string().into_bytes())
        }));

        // 0x02 GpuCompute — compute task via ComputeBridge
        let compute_clone = compute.clone();
        reg.register(0x02, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let kind_byte = parsed.int32_values.first().copied().unwrap_or(0) as u8;
            let task = ComputeTask::new(cmd.header.app_id, ComputeKind::from_byte(kind_byte), parsed.byte_data.clone());
            let result = compute_clone.lock().execute_task(&task)?;
            let resp = serde_json::json!({
                "computed": true, "task_id": result.task_id.to_string(),
                "output_bytes": result.output.len(), "elapsed_us": result.elapsed_us,
            });
            Ok(resp.to_string().into_bytes())
        }));

        // 0x03 CpuReserve
        reg.register(0x03, Box::new(|_cmd| {
            Ok(br#"{"reserved":true}"#.to_vec())
        }));

        // 0x04 CpuRelease
        reg.register(0x04, Box::new(|_cmd| {
            Ok(br#"{"released":true}"#.to_vec())
        }));

        // 0x05 CpuExec — compute task via ComputeBridge
        let compute_clone2 = compute.clone();
        reg.register(0x05, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let kind_byte = parsed.int32_values.first().copied().unwrap_or(0) as u8;
            let task = ComputeTask::new(cmd.header.app_id, ComputeKind::from_byte(kind_byte), parsed.byte_data.clone());
            let result = compute_clone2.lock().execute_task(&task)?;
            let resp = serde_json::json!({
                "executed": true, "task_id": result.task_id.to_string(),
                "output_bytes": result.output.len(), "elapsed_us": result.elapsed_us,
            });
            Ok(resp.to_string().into_bytes())
        }));

        // 0x06 MemAlloc — allocate buffer via BufferManager
        let compute_c = compute.clone();
        reg.register(0x06, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let size = parsed.int32_values.first().copied().unwrap_or(0) as u64;
            let handle = compute_c.lock().buffer_manager_mut().allocate(cmd.header.app_id, size)?;
            let resp = serde_json::json!({"allocated":true,"buffer_id":handle.id.to_string(),"size":handle.size});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x07 MemFree — free buffer
        let compute_c2 = compute.clone();
        reg.register(0x07, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let _addr = parsed.int32_values.first().copied().unwrap_or(0) as u64;
            let freed = compute_c2.lock().buffer_manager_mut().cleanup_app(cmd.header.app_id);
            let resp = serde_json::json!({"freed":true,"count":freed});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x08 StreamVideo — process video frame via MediaEngine
        let media_c = media.clone();
        reg.register(0x08, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let pts = parsed.uint32_values.get(1).copied().unwrap_or(0) as u64;
            let output = media_c.lock().process_video_frame(&parsed.byte_data, pts)?;
            let resp = serde_json::json!({"streamed":true,"type":"video","pts_us":pts,"output_bytes":output.len()});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x09 StreamAudio — process audio frame via MediaEngine
        let media_c2 = media.clone();
        reg.register(0x09, Box::new(move |cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let sr = parsed.uint32_values.first().copied().unwrap_or(44100);
            let pts = parsed.uint32_values.get(1).copied().unwrap_or(0) as u64;
            let output = media_c2.lock().process_audio_frame(&parsed.byte_data, pts, sr, 2)?;
            let resp = serde_json::json!({"streamed":true,"type":"audio","sample_rate":sr,"pts_us":pts,"output_bytes":output.len()});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x0A QueryCores
        reg.register(0x0A, Box::new(|_cmd| {
            let count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
            Ok(format!("{{\"cores\":{count}}}").into_bytes())
        }));

        // 0x0B QueryGpu
        reg.register(0x0B, Box::new(|_cmd| {
            let has_dri = std::path::Path::new("/dev/dri").exists();
            let has_nvidia = std::path::Path::new("/dev/nvidia0").exists();
            let resp = serde_json::json!({"available":has_dri||has_nvidia,"dri":has_dri,"nvidia":has_nvidia});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x0C Heartbeat
        reg.register(0x0C, Box::new(|_cmd| Ok(b"{\"status\":\"ok\"}".to_vec())));

        // 0x0F ThermalStatus
        reg.register(0x0F, Box::new(|_cmd| {
            let zones: Vec<f64> = (0..16).filter_map(|i| {
                let path = format!("/sys/class/thermal/thermal_zone{i}/temp");
                std::fs::read_to_string(&path).ok()
                    .and_then(|s| s.trim().parse::<f64>().ok().map(|t| t / 1000.0))
            }).collect();
            let resp = serde_json::json!({"zones":zones.len(),"temperatures_c":zones});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x30 ContextCreate
        reg.register(0x30, Box::new(|cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let resp = serde_json::json!({"created":true,"name":parsed.string_value,"context_id":1});
            Ok(resp.to_string().into_bytes())
        }));

        // 0x31 ContextShare
        reg.register(0x31, Box::new(|cmd| {
            let parsed = crate::protocol::flatbuffer::parse_packet(&cmd.payload)?;
            let resp = serde_json::json!({"shared":true,"target_app_id":parsed.int32_values.first().copied().unwrap_or(0)});
            Ok(resp.to_string().into_bytes())
        }));
    }

    pub fn register_handler<F>(&self, opcode: u8, handler: F)
    where F: Fn(&TesseractCommand) -> Result<Vec<u8>> + Send + Sync + 'static {
        self.job_registry.write().register(opcode, Box::new(handler));
    }

    pub fn execute(&self, cmd: &TesseractCommand) -> Result<Vec<u8>> {
        self.active_count.fetch_add(1, Ordering::SeqCst);
        let result = self.job_registry.read().dispatch(cmd);
        self.active_count.fetch_sub(1, Ordering::SeqCst);
        result
    }

    pub fn count(&self) -> usize { self.workers.len() }
    pub fn active_count(&self) -> u32 { self.active_count.load(Ordering::SeqCst) }
}
