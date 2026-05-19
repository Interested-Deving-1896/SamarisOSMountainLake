use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone)]
pub struct GpuAuditEvent {
    pub timestamp: Instant,
    pub action: String,
    pub resource: String,
    pub result: String,
    pub details: String,
}

pub struct AuditLog {
    events: std::sync::Mutex<Vec<GpuAuditEvent>>,
    max_entries: usize,
    dropped_count: AtomicU64,
}

impl AuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            events: std::sync::Mutex::new(Vec::with_capacity(max_entries)),
            max_entries,
            dropped_count: AtomicU64::new(0),
        }
    }

    pub fn record(&self, action: &str, resource: &str, result: &str, details: &str) -> VgmResult<()> {
        let event = GpuAuditEvent {
            timestamp: Instant::now(),
            action: action.to_string(),
            resource: resource.to_string(),
            result: result.to_string(),
            details: details.to_string(),
        };
        let mut events = self.events.lock().map_err(|e| {
            VgmError::InternalInvariantViolation(format!("AuditLog lock poisoned: {}", e))
        })?;
        if events.len() >= self.max_entries {
            events.remove(0);
            self.dropped_count.fetch_add(1, Ordering::Relaxed);
        }
        events.push(event);
        Ok(())
    }

    pub fn events(&self) -> Vec<GpuAuditEvent> {
        self.events.lock()
            .map(|e| e.clone())
            .unwrap_or_default()
    }

    pub fn count(&self) -> usize {
        self.events.lock()
            .map(|e| e.len())
            .unwrap_or(0)
    }

    pub fn clear(&self) {
        if let Ok(mut events) = self.events.lock() {
            events.clear();
        }
    }

    pub fn dropped_count(&self) -> u64 {
        self.dropped_count.load(Ordering::Relaxed)
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new(1024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_audit_log() {
        let log = AuditLog::new(100);
        assert_eq!(log.count(), 0);
        assert_eq!(log.dropped_count(), 0);
    }

    #[test]
    fn test_record_event() {
        let log = AuditLog::new(100);
        log.record("allocate", "buf:0x1", "ok", "Allocated 4KB buffer").unwrap();
        assert_eq!(log.count(), 1);
    }

    #[test]
    fn test_max_entries_eviction() {
        let log = AuditLog::new(3);
        for i in 0..5 {
            log.record("op", &format!("res:{}", i), "ok", "").unwrap();
        }
        assert_eq!(log.count(), 3);
        assert_eq!(log.dropped_count(), 2);
    }

    #[test]
    fn test_clear() {
        let log = AuditLog::new(100);
        log.record("test", "res", "ok", "").unwrap();
        log.clear();
        assert_eq!(log.count(), 0);
    }

    #[test]
    fn test_events_list() {
        let log = AuditLog::new(10);
        log.record("compress", "tex:42", "ok", "Compressed 1MB").unwrap();
        let events = log.events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].action, "compress");
    }
}
