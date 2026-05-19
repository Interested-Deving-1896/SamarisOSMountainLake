pub mod assets;
pub mod notifier;
pub mod watchdog;

use std::sync::Arc;
use std::time::Instant;

use crate::boot::assets::AssetCache;
use crate::boot::notifier::SystemdNotifier;
use crate::boot::watchdog::WatchdogFiles;
use crate::core::error::Result;
use crate::gpu_canvas::GpuCanvas;
use crate::ipc::shm::SharedMemoryRing;
use crate::scheduler::Scheduler;
use crate::telemetry::Telemetry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootMode {
    Normal,
    Fast,
}

impl BootMode {
    pub fn is_fast(self) -> bool {
        matches!(self, Self::Fast)
    }
}

pub struct BootSequence {
    mode: BootMode,
    asset_root: String,
    shm_size: u64,
    shm_name: String,
    worker_count: usize,
    gpu_preinit: bool,
    scheduler_tick_ms: u64,
}

impl Default for BootSequence {
    fn default() -> Self {
        Self {
            mode: BootMode::Normal,
            asset_root: "/opt/volt/desktop".into(),
            shm_size: 64 * 1024 * 1024,
            shm_name: "volt-sbp-ring".into(),
            worker_count: 4,
            gpu_preinit: false,
            scheduler_tick_ms: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BootPhase {
    pub name: &'static str,
    pub elapsed_us: u64,
}

#[derive(Debug, Clone)]
pub struct BootTiming {
    pub total_us: u64,
    pub scheduler_init_us: u64,
    pub asset_precache_us: u64,
    pub shm_alloc_us: u64,
    pub gpu_init_us: u64,
    pub watchdog_write_us: u64,
    pub sd_notify_us: u64,
    pub phases: Vec<BootPhase>,
}

impl BootTiming {
    pub fn new() -> Self {
        Self {
            total_us: 0,
            scheduler_init_us: 0,
            asset_precache_us: 0,
            shm_alloc_us: 0,
            gpu_init_us: 0,
            watchdog_write_us: 0,
            sd_notify_us: 0,
            phases: Vec::new(),
        }
    }
}

pub struct BootResult {
    pub scheduler: Arc<Scheduler>,
    pub asset_cache: Option<AssetCache>,
    pub shm_ring: Option<SharedMemoryRing>,
    pub gpu_canvas: Option<GpuCanvas>,
    pub elapsed: std::time::Duration,
    pub timing: BootTiming,
}

impl BootSequence {
    pub fn new(mode: BootMode) -> Self {
        let mut seq = Self::default();
        seq.mode = mode;
        if mode.is_fast() {
            seq.worker_count = 8;
            seq.gpu_preinit = true;
        }
        seq
    }

    pub fn with_workers(mut self, n: usize) -> Self {
        self.worker_count = n;
        self
    }

    pub fn with_asset_root(mut self, root: &str) -> Self {
        self.asset_root = root.into();
        self
    }

    pub fn execute(&self) -> Result<BootResult> {
        let started = Instant::now();
        let mut timing = BootTiming::new();

        if self.mode.is_fast() {
            #[cfg(target_os = "linux")]
            self.apply_process_priority();
        }

        let t0 = Instant::now();
        let telemetry = Arc::new(Telemetry::new());
        let scheduler = Arc::new(Scheduler::new(
            self.worker_count,
            self.scheduler_tick_ms,
            telemetry,
        ));
        timing.scheduler_init_us = t0.elapsed().as_micros() as u64;
        timing.phases.push(BootPhase { name: "scheduler_init", elapsed_us: timing.scheduler_init_us });

        let asset_cache = if self.mode.is_fast() {
            let t1 = Instant::now();
            let result = match AssetCache::precache(&self.asset_root) {
                Ok(cache) => {
                    tracing::info!("Pre-cached {} assets ({} bytes)", cache.len(), cache.total_bytes());
                    Some(cache)
                }
                Err(e) => {
                    tracing::warn!("Asset pre-cache failed: {e}");
                    None
                }
            };
            timing.asset_precache_us = t1.elapsed().as_micros() as u64;
            timing.phases.push(BootPhase { name: "asset_precache", elapsed_us: timing.asset_precache_us });
            result
        } else {
            None
        };

        let shm_ring = if self.mode.is_fast() {
            let t2 = Instant::now();
            let result = match SharedMemoryRing::create_linux_shm(&self.shm_name, self.shm_size as usize) {
                Ok(ring) => {
                    tracing::info!("SHM ring {} allocated ({} MB)", self.shm_name, self.shm_size / 1024 / 1024);
                    Some(ring)
                }
                Err(e) => {
                    tracing::warn!("SHM allocation skipped: {e}");
                    None
                }
            };
            timing.shm_alloc_us = t2.elapsed().as_micros() as u64;
            timing.phases.push(BootPhase { name: "shm_alloc", elapsed_us: timing.shm_alloc_us });
            result
        } else {
            None
        };

        let gpu_canvas = if self.gpu_preinit {
            let t3 = Instant::now();
            let canvas = GpuCanvas::new();
            tracing::info!("GPU pre-init: available={}", canvas.is_gpu_available());
            timing.gpu_init_us = t3.elapsed().as_micros() as u64;
            timing.phases.push(BootPhase { name: "gpu_init", elapsed_us: timing.gpu_init_us });
            Some(canvas)
        } else {
            None
        };

        if self.mode.is_fast() {
            let t4 = Instant::now();
            WatchdogFiles::write_kernel_b_ready();
            timing.watchdog_write_us = t4.elapsed().as_micros() as u64;
            timing.phases.push(BootPhase { name: "watchdog_write", elapsed_us: timing.watchdog_write_us });

            let t5 = Instant::now();
            SystemdNotifier::notify("READY=1").ok();
            timing.sd_notify_us = t5.elapsed().as_micros() as u64;
            timing.phases.push(BootPhase { name: "sd_notify", elapsed_us: timing.sd_notify_us });
        }

        let elapsed = started.elapsed();
        timing.total_us = elapsed.as_micros() as u64;
        timing.phases.push(BootPhase { name: "total", elapsed_us: timing.total_us });

        tracing::info!(
            "Boot {} completed in {:?} (workers={}, gpu={}, shm={}, assets={})",
            if self.mode.is_fast() { "FAST" } else { "NORMAL" },
            elapsed,
            self.worker_count,
            gpu_canvas.is_some(),
            shm_ring.is_some(),
            asset_cache.as_ref().map(|a| a.len()).unwrap_or(0),
        );
        tracing::debug!(
            "Boot timing: scheduler={}µs asset={}µs shm={}µs gpu={}µs watchdog={}µs notify={}µs",
            timing.scheduler_init_us, timing.asset_precache_us,
            timing.shm_alloc_us, timing.gpu_init_us,
            timing.watchdog_write_us, timing.sd_notify_us,
        );

        Ok(BootResult {
            scheduler,
            asset_cache,
            shm_ring,
            gpu_canvas,
            elapsed,
            timing,
        })
    }

    #[cfg(target_os = "linux")]
    fn apply_process_priority(&self) {
        let prio_ret = unsafe { libc::setpriority(libc::PRIO_PROCESS, 0, -20) };
        if prio_ret != 0 {
            tracing::warn!("setpriority(-20) failed: {} — not running as root?", std::io::Error::last_os_error());
        } else {
            tracing::info!("Process priority set to nice -20");
        }
    }
}
