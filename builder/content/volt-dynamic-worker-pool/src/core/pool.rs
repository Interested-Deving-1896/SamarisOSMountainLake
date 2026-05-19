use crossbeam::channel::{Receiver, Sender};
use parking_lot::RwLock;

use crate::config::schema::WorkerPoolConfig;
use crate::core::error::WorkerPoolError;
use crate::core::lifecycle::Lifecycle;
use crate::core::result::WorkerPoolResult;
use crate::core::scheduler::Scheduler;
use crate::core::state::WorkerPoolState;
use crate::desktop_guard::frame_pressure::FramePressure;
use crate::job::job::Job;
use crate::job::job_context::JobContext;
use crate::job::job_handle::JobHandle;
use crate::job::job_id::JobId;
use crate::metrics::counters::MetricsCounters;
use crate::metrics::snapshot::MetricsSnapshot;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;
use crate::modules::registry::ModuleRegistry;
use crate::orbit::burst_controller::{OrbitBurstController, OrbitBurstDecision, OrbitBurstRequest};
use crate::preemption::cooperative::CooperativeScheduler;
use crate::priority::multi_queue::MultiQueue;
use crate::scaling::hardware_probe::HardwareProbe;

#[allow(dead_code)]
pub struct DynamicWorkerPool {
    config: WorkerPoolConfig,
    lifecycle: RwLock<Lifecycle>,
    scheduler: RwLock<Scheduler>,
    job_queue: MultiQueue,
    metrics: MetricsCounters,
    hardware: HardwareProbe,
    preemption: CooperativeScheduler,
    burst_controller: OrbitBurstController,
    module_registry: RwLock<ModuleRegistry>,
    job_tx: Sender<Job>,
    job_rx: Receiver<Job>,
}

impl DynamicWorkerPool {
    pub fn new(config: WorkerPoolConfig) -> Self {
        let hw_cfg = crate::config::schema::HardwareConfig {
            default_cpu_cores: config.worker_pool.hardware.default_cpu_cores as u32,
            min_workers_override: config.worker_pool.scaling.min_workers_override.unwrap_or(0) as u32,
            max_workers_override: config.worker_pool.scaling.max_workers_override.unwrap_or(0) as u32,
            ram_bytes: 8_589_934_592,
        };
        let hardware = HardwareProbe::new(hw_cfg);
        let (job_tx, job_rx) = crossbeam::channel::unbounded();
        Self {
            preemption: CooperativeScheduler::new(
                true,
                config.worker_pool.yield_budget_us,
                5000,
            ),
            burst_controller: OrbitBurstController::new(
                10,
                config.worker_pool.reservations.orbit_burst_cooldown_ms,
                5,
            ),
            scheduler: RwLock::new(Scheduler::new(
                hardware.min_workers,
                hardware.max_workers,
            )),
            lifecycle: RwLock::new(Lifecycle::new()),
            config,
            job_queue: MultiQueue::new(),
            metrics: MetricsCounters::new(),
            hardware,
            module_registry: RwLock::new(ModuleRegistry::new()),
            job_tx,
            job_rx,
        }
    }

    pub fn start(&self) -> WorkerPoolResult<()> {
        self.lifecycle.write().start()
    }

    pub fn shutdown(&self) -> WorkerPoolResult<()> {
        let mut lc = self.lifecycle.write();
        if !lc.state().is_running() {
            return Err(WorkerPoolError::PoolNotStarted);
        }
        lc.stop()
    }

    pub fn submit_job(&self, job: Job) -> WorkerPoolResult<JobHandle> {
        if !self.lifecycle.read().state().is_running() {
            return Err(WorkerPoolError::PoolNotStarted);
        }
        let handle = JobHandle::new(job.id().clone(), job.name().into());
        self.metrics.record_submission();
        self.job_queue.enqueue(job);
        Ok(handle)
    }

    pub fn submit_chunked_job(&self, job: Job) -> WorkerPoolResult<JobHandle> {
        self.submit_job(job)
    }

    pub fn cancel_job(&self, job_id: &JobId) -> WorkerPoolResult<bool> {
        if !self.lifecycle.read().state().is_running() {
            return Err(WorkerPoolError::PoolNotStarted);
        }
        let removed = self.job_queue.cancel(job_id);
        if removed {
            self.metrics.record_cancellation();
            Ok(true)
        } else {
            Err(WorkerPoolError::JobNotFound(job_id.to_string()))
        }
    }

    pub fn yield_point(&self, ctx: &mut JobContext) -> WorkerPoolResult<bool> {
        self.metrics.record_yield();
        if ctx.is_cancelled() {
            return Ok(false);
        }
        if self.preemption.should_preempt(ctx.elapsed_ms() * 1000) {
            self.metrics.record_preemption();
            ctx.reset_yield_budget();
            return Ok(true);
        }
        Ok(true)
    }

    pub fn metrics(&self) -> MetricsSnapshot {
        let lc = self.lifecycle.read();
        let sched = self.scheduler.read();
        self.metrics.snapshot(
            sched.active_workers,
            sched.idle_workers,
            self.job_queue.queue_depth(),
            self.job_queue.queue_depth(),
            lc.uptime_ms(),
            lc.state().name().into(),
        )
    }

    pub fn set_desktop_pressure(&self, pressure: f64) {
        let _fp = FramePressure::from_f64(pressure);
        self.metrics.set_desktop_pressure(pressure);
        let mut sched = self.scheduler.write();
        sched.desktop_frame_pressure = pressure;
    }

    pub fn request_orbit_burst(&self, req: OrbitBurstRequest) -> OrbitBurstDecision {
        self.metrics.record_orbit_burst();
        if *self.metrics.desktop_pressure.read() > 0.6 {
            return OrbitBurstDecision::Rejected("desktop_pressure".into());
        }
        self.burst_controller.request_burst(&req)
    }

    pub fn register_module(&self, profile: ModuleProfile) -> WorkerPoolResult<()> {
        let mut reg = self.module_registry.write();
        reg.register(profile)
    }

    pub fn get_module_metrics(&self, module_id: &ModuleId) -> WorkerPoolResult<MetricsSnapshot> {
        let _reg = self.module_registry.read();
        if !_reg.is_registered(module_id) {
            return Err(WorkerPoolError::ModuleNotFound(module_id.to_string()));
        }
        Ok(self.metrics())
    }

    pub fn state(&self) -> WorkerPoolState {
        self.lifecycle.read().state()
    }

    pub fn config(&self) -> &WorkerPoolConfig {
        &self.config
    }

    pub fn hardware(&self) -> &HardwareProbe {
        &self.hardware
    }
}
