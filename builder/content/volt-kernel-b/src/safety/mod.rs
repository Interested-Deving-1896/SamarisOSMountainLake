pub mod limits;
pub mod watchdog;

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use parking_lot::RwLock;

use crate::safety::limits::ResourceLimiter;
use crate::safety::watchdog::ThermalWatchdog;

pub struct SafetySupervisor {
    watchdog: RwLock<ThermalWatchdog>,
    limiter: RwLock<ResourceLimiter>,
    throttle_factor: AtomicU32,
    emergency_stop: AtomicBool,
}

impl SafetySupervisor {
    pub fn new() -> Self {
        Self {
            watchdog: RwLock::new(ThermalWatchdog::new()),
            limiter: RwLock::new(ResourceLimiter::new()),
            throttle_factor: AtomicU32::new(100),
            emergency_stop: AtomicBool::new(false),
        }
    }

    pub fn thermal_throttle(&self) {
        self.throttle_factor.store(50, Ordering::SeqCst);
        tracing::warn!("Thermal throttle: 50% factor");
    }

    pub fn emergency_throttle(&self) {
        self.throttle_factor.store(25, Ordering::SeqCst);
        tracing::error!("Emergency throttle: 25% — reserved cores released");
    }

    pub fn emergency_shutdown(&self) {
        self.emergency_stop.store(true, Ordering::SeqCst);
        tracing::error!("EMERGENCY SHUTDOWN — critical temperature exceeded");
    }

    pub fn current_throttle(&self) -> u32 {
        self.throttle_factor.load(Ordering::SeqCst)
    }

    pub fn is_emergency_stop(&self) -> bool {
        self.emergency_stop.load(Ordering::SeqCst)
    }

    pub fn clear_emergency(&self) {
        self.emergency_stop.store(false, Ordering::SeqCst);
        self.throttle_factor.store(100, Ordering::SeqCst);
        tracing::info!("Emergency state cleared");
    }

    pub fn watchdog(&self) -> &RwLock<ThermalWatchdog> {
        &self.watchdog
    }

    pub fn limiter(&self) -> &RwLock<ResourceLimiter> {
        &self.limiter
    }
}
