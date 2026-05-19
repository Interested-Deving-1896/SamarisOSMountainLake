use crate::runtime::shutdown::ShutdownController;

pub fn setup_signal_handler(controller: &ShutdownController) {
    let trigger = controller.clone_trigger();
    ctrlc::set_handler(move || {
        tracing::info!("Received shutdown signal");
        trigger.store(true, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_handler_setup() {
        let ctrl = ShutdownController::new();
        setup_signal_handler(&ctrl);
        assert!(!ctrl.is_triggered());
    }
}
