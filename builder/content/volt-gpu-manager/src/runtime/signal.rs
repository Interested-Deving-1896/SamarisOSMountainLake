use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn setup_signal_handler(shutdown_flag: Arc<AtomicBool>) -> Result<(), Box<dyn std::error::Error>> {
    ctrlc::set_handler(move || {
        tracing::info!("Received SIGINT/SIGTERM, initiating graceful shutdown");
        shutdown_flag.store(true, Ordering::SeqCst);
    })?;
    tracing::debug!("Signal handler registered");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    #[test]
    fn test_signal_handler_creation() {
        let flag = Arc::new(AtomicBool::new(false));
        let result = setup_signal_handler(flag.clone());
        assert!(result.is_ok());
    }
}
