use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use parking_lot::{Mutex, RwLock};
use crate::scheduler::priority::GpuPriority;
use crate::thermal::backoff::ThermalBackoff;
use crate::thermal::policy::ThermalPolicy;
use crate::thermal::sensors::ThermalSensor;
use crate::thermal::state::{ThermalLevel, ThermalState};

pub struct ThermalMonitor {
    state: Arc<RwLock<ThermalState>>,
    policy: RwLock<ThermalPolicy>,
    backoff: Arc<Mutex<ThermalBackoff>>,
    running: Arc<AtomicBool>,
    interval_ms: u64,
}

impl ThermalMonitor {
    pub fn new(interval_ms: u64) -> Self {
        ThermalMonitor {
            state: Arc::new(RwLock::new(ThermalState::default())),
            policy: RwLock::new(ThermalPolicy::default()),
            backoff: Arc::new(Mutex::new(ThermalBackoff::new())),
            running: Arc::new(AtomicBool::new(false)),
            interval_ms,
        }
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::Relaxed);
        let running = self.running.clone();
        let state = self.state.clone();
        let backoff = self.backoff.clone();
        let interval = self.interval_ms;

        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                let temp = ThermalSensor::read_gpu_temp();
                let level = ThermalSensor::estimate_state(temp);
                {
                    let mut current = state.write();
                    current.level = level;
                    if let Some(t) = temp {
                        current.temperature_c = t;
                    }
                }
                if level >= ThermalLevel::Throttle {
                    backoff.lock().record_throttle();
                }
                thread::sleep(Duration::from_millis(interval));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn current_state(&self) -> ThermalState {
        self.state.read().clone()
    }

    pub fn should_block_priority(&self, priority: GpuPriority) -> bool {
        let state = self.state.read();
        let policy = self.policy.read();
        policy.should_block_priority(&state, priority)
    }

    pub fn should_cpu_fallback(&self) -> bool {
        let state = self.state.read();
        let policy = self.policy.read();
        policy.should_cpu_fallback(&state)
    }

    pub fn update_policy(&self, policy: ThermalPolicy) {
        *self.policy.write() = policy;
    }

    pub fn backoff_stats(&self) -> (u64, u64) {
        let b = self.backoff.lock();
        (b.throttle_count(), b.time_since_last_throttle_ms())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_monitor() {
        let m = ThermalMonitor::new(1000);
        assert_eq!(m.current_state().level, ThermalLevel::Normal);
    }

    #[test]
    fn test_start_stop() {
        let m = ThermalMonitor::new(50);
        m.start();
        thread::sleep(Duration::from_millis(150));
        m.stop();
        let state = m.current_state();
        assert!(state.level == ThermalLevel::Unknown || state.level == ThermalLevel::Normal);
    }

    #[test]
    fn test_should_block_priority() {
        let m = ThermalMonitor::new(1000);
        assert!(!m.should_block_priority(GpuPriority::Critical));
    }

    #[test]
    fn test_should_cpu_fallback() {
        let m = ThermalMonitor::new(1000);
        assert!(!m.should_cpu_fallback());
    }

    #[test]
    fn test_update_policy() {
        let m = ThermalMonitor::new(1000);
        let mut p = ThermalPolicy::default();
        p.cpu_fallback_at_100 = false;
        m.update_policy(p);
        assert!(!m.should_cpu_fallback());
    }

    #[test]
    fn test_backoff_stats() {
        let m = ThermalMonitor::new(1000);
        let (count, _) = m.backoff_stats();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_force_thermal_state() {
        let m = ThermalMonitor::new(1000);
        {
            let mut s = m.state.write();
            s.level = ThermalLevel::Critical;
            s.temperature_c = 88.0;
        }
        assert_eq!(m.current_state().level, ThermalLevel::Critical);
        assert!(m.should_block_priority(GpuPriority::Normal));
    }
}
