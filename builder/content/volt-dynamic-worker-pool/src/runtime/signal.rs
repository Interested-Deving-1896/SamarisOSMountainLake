use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct SignalHandler {
    pub shutdown_requested: Arc<AtomicBool>,
}

impl SignalHandler {
    pub fn new() -> Self {
        let shutdown_requested = Arc::new(AtomicBool::new(false));
        let flag = shutdown_requested.clone();

        ctrlc::set_handler(move || {
            flag.store(true, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        Self { shutdown_requested }
    }

    pub fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::SeqCst)
    }

    pub fn wait_for_shutdown(&self) {
        while !self.is_shutdown_requested() {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self::new()
    }
}
