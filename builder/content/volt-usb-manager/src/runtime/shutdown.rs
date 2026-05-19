use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct ShutdownController {
    triggered: Arc<AtomicBool>,
}

impl ShutdownController {
    pub fn new() -> Self {
        ShutdownController {
            triggered: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn trigger(&self) {
        self.triggered.store(true, Ordering::SeqCst);
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::SeqCst)
    }

    pub fn wait_for_shutdown(&self) {
        while !self.is_triggered() {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    pub fn clone_trigger(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.triggered)
    }
}

impl Default for ShutdownController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_controller() {
        let ctrl = ShutdownController::new();
        assert!(!ctrl.is_triggered());
        ctrl.trigger();
        assert!(ctrl.is_triggered());
    }

    #[test]
    fn test_shutdown_does_not_block_if_triggered() {
        let ctrl = ShutdownController::new();
        ctrl.trigger();
        ctrl.wait_for_shutdown();
        assert!(ctrl.is_triggered());
    }
}
