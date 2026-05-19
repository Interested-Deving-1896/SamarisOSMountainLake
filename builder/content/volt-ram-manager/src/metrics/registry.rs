use crate::core::state::VrmState;

#[derive(Clone)]
pub struct MetricsRegistry {
    pub counters: MetricsCounters,
}

pub struct MetricsCounters {
    pub apps_registered: std::sync::atomic::AtomicI64,
}

impl Clone for MetricsCounters {
    fn clone(&self) -> Self {
        Self {
            apps_registered: std::sync::atomic::AtomicI64::new(
                self.apps_registered.load(std::sync::atomic::Ordering::SeqCst),
            ),
        }
    }
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self {
            counters: MetricsCounters {
                apps_registered: std::sync::atomic::AtomicI64::new(0),
            },
        }
    }

    pub fn snapshot(&self, _state: &VrmState) -> crate::metrics::snapshot::MetricsSnapshot {
        crate::metrics::snapshot::MetricsSnapshot::default()
    }
}
