use std::time::{Duration, Instant};
use tracing::debug;

/// Event debouncer — coalesces rapid events into a single stable trigger.
///
/// Used by the hotplug watcher to avoid running detect → plan → apply
/// on every single udev event during cable insertion/removal.
pub struct Debouncer {
    delay: Duration,
    last_event: Option<Instant>,
    pending: bool,
}

impl Debouncer {
    /// Create a debouncer with the given cooldown duration.
    /// Recommended: 300–800ms for display hotplug.
    pub fn new(delay_ms: u64) -> Self {
        Self {
            delay: Duration::from_millis(delay_ms),
            last_event: None,
            pending: false,
        }
    }

    /// Register an event. Returns true when the cooldown has elapsed
    /// and it is safe to trigger the real handler.
    pub fn should_fire(&mut self) -> bool {
        let now = Instant::now();
        match self.last_event {
            None => {
                self.last_event = Some(now);
                self.pending = true;
                debug!("Debouncer: first event, scheduling fire after {:?}", self.delay);
                false
            }
            Some(last) => {
                if now.duration_since(last) >= self.delay {
                    self.last_event = Some(now);
                    self.pending = false;
                    debug!("Debouncer: cooldown elapsed, firing");
                    true
                } else {
                    self.last_event = Some(now);
                    debug!("Debouncer: event coalesced");
                    false
                }
            }
        }
    }

    /// Force fire — resets state.
    pub fn force_fire(&mut self) {
        self.last_event = None;
        self.pending = false;
    }

    pub fn is_pending(&self) -> bool {
        self.pending
    }
}
