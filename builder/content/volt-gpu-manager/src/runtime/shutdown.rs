use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ShutdownController {
    flag: Arc<AtomicBool>,
}

impl ShutdownController {
    pub fn new() -> Self {
        Self {
            flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn shutdown(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    pub fn flag(&self) -> Arc<AtomicBool> {
        self.flag.clone()
    }

    pub fn reset(&self) {
        self.flag.store(false, Ordering::SeqCst);
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
        assert!(!ctrl.is_shutdown_requested());
        ctrl.shutdown();
        assert!(ctrl.is_shutdown_requested());
    }

    #[test]
    fn test_shutdown_reset() {
        let ctrl = ShutdownController::new();
        ctrl.shutdown();
        assert!(ctrl.is_shutdown_requested());
        ctrl.reset();
        assert!(!ctrl.is_shutdown_requested());
    }

    #[test]
    fn test_shutdown_clone() {
        let ctrl = ShutdownController::new();
        let ctrl2 = ctrl.clone();
        ctrl.shutdown();
        assert!(ctrl2.is_shutdown_requested());
    }
}
