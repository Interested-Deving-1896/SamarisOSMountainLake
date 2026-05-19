use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use parking_lot::Mutex;
use crossbeam::channel::{unbounded, Sender, Receiver};
use crate::pressure::level::PressureLevel;
use crate::pressure::hysteresis::HysteresisController;
use crate::pressure::thresholds::PressureThresholds;
use crate::pressure::actions::PressureActions;
use crate::pressure::cooldown::Cooldown;
use crate::metrics::registry::MetricsRegistry;

pub struct PressureMonitor {
    controller: Mutex<HysteresisController>,
    actions: Arc<PressureActions>,
    cooldown: Mutex<Cooldown>,
    running: Arc<AtomicBool>,
    metrics: Arc<MetricsRegistry>,
    interval_ms: u64,
    event_tx: Sender<PressureLevel>,
    event_rx: Receiver<PressureLevel>,
}

impl PressureMonitor {
    pub fn new(
        thresholds: PressureThresholds,
        actions: Arc<PressureActions>,
        metrics: Arc<MetricsRegistry>,
        interval_ms: u64,
    ) -> Self {
        let (tx, rx) = unbounded();
        PressureMonitor {
            controller: Mutex::new(HysteresisController::new(thresholds)),
            actions,
            cooldown: Mutex::new(Cooldown::new(interval_ms)),
            running: Arc::new(AtomicBool::new(false)),
            metrics,
            interval_ms,
            event_tx: tx,
            event_rx: rx,
        }
    }

    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        tracing::info!("pressure monitor started (interval: {}ms)", self.interval_ms);
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        tracing::info!("pressure monitor stopped");
    }

    pub fn current_level(&self) -> PressureLevel {
        self.controller.lock().current()
    }

    pub fn events(&self) -> Receiver<PressureLevel> {
        self.event_rx.clone()
    }

    pub fn evaluate(&self, usage_pct: f64) -> Option<PressureLevel> {
        let mut cooldown = self.cooldown.lock();
        if !cooldown.is_ready() {
            return None;
        }

        let mut controller = self.controller.lock();
        let transition = controller.evaluate(usage_pct);

        if let Some(level) = transition {
            let _ = self.actions.apply(level);
            let _ = self.event_tx.send(level);
            cooldown.reset();
            return Some(level);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_monitor() -> PressureMonitor {
        let metrics = Arc::new(MetricsRegistry::new());
        let actions = Arc::new(PressureActions::new());
        PressureMonitor::new(PressureThresholds::default(), actions, metrics, 100)
    }

    #[test]
    fn test_start_stop() {
        let m = make_monitor();
        m.start();
        m.stop();
    }

    #[test]
    fn test_initial_level() {
        let m = make_monitor();
        assert_eq!(m.current_level(), PressureLevel::Green);
    }

    #[test]
    fn test_evaluate_transition() {
        let m = make_monitor();
        let result = m.evaluate(75.0);
        assert_eq!(result, Some(PressureLevel::Yellow));
    }

    #[test]
    fn test_evaluate_no_transition() {
        let m = make_monitor();
        let result = m.evaluate(50.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_event_channel() {
        let m = make_monitor();
        m.evaluate(75.0);
        let event = m.events().try_recv().ok();
        assert_eq!(event, Some(PressureLevel::Yellow));
    }

    #[test]
    fn test_cooldown_blocks() {
        let m = make_monitor();
        m.evaluate(75.0);
        let second = m.evaluate(88.0);
        assert!(second.is_none() || second == Some(PressureLevel::Orange));
    }
}
