use crate::display::debounce::Debouncer;
use crate::display::types::*;
use crate::display::{detector, planner, applier, persist, notifier, rollback};
use tracing::{info, warn, error};
use std::time::Duration;

/// Watch for hotplug events and reconfigure displays automatically.
///
/// Primary strategy: poll xrandr every 2 seconds, detect screen count changes.
/// Once DRM/KMS inotify is available, use that instead.
pub fn watch_hotplug() -> Result<(), DisplayError> {
    info!("Hotplug watcher started (poll mode, 2s interval)");

    let user_config = persist::load_user_config().unwrap_or_default();
    let mut debouncer = Debouncer::new(500);
    let mut last_count: usize = 0;
    let mut last_primary: Option<String> = None;

    if let Ok(screens) = detector::detect() {
        last_count = screens.iter().filter(|s| s.connected).count();
        last_primary = screens.iter().find(|s| s.primary).map(|s| s.name.clone());
        info!("Initial state: {last_count} connected screen(s), primary: {:?}", last_primary);
    }

    loop {
        std::thread::sleep(Duration::from_secs(2));

        let screens = match detector::detect() {
            Ok(s) => s,
            Err(e) => {
                warn!("Hotplug detect failed: {e}");
                continue;
            }
        };

        let current_count = screens.iter().filter(|s| s.connected).count();
        let current_primary = screens.iter().find(|s| s.primary).map(|s| s.name.clone());

        if current_count != last_count || current_primary != last_primary {
            info!(
                "Display change detected: {}→{} screens, primary: {:?}→{:?}",
                last_count, current_count, last_primary, current_primary
            );

            if !debouncer.should_fire() {
                last_count = current_count;
                last_primary = current_primary;
                continue;
            }

            // Preserve primary preference
            if let Some(ref prev) = last_primary {
                if !screens.iter().any(|s| s.connected && s.name == *prev) {
                    warn!("Primary screen '{prev}' disconnected");
                }
            }

            last_count = current_count;
            last_primary = current_primary;

            // Run full cycle
            match planner::plan(&screens, &user_config) {
                Ok(planned) => {
                    let _ = rollback::store_last_known_good();
                    match applier::apply(&planned) {
                        Ok(()) => {
                            let _ = persist::persist_runtime(&planned);
                            let _ = notifier::notify_kernel_a("display.changed", &planned);
                            info!("Hotplug reconfiguration complete");
                        }
                        Err(e) => {
                            error!("Hotplug apply failed: {e}. Attempting rollback.");
                            let _ = rollback::rollback();
                        }
                    }
                }
                Err(e) => {
                    error!("Hotplug plan failed: {e}");
                }
            }
        }
    }
}
