use std::sync::Mutex;
use std::time::Instant;

pub struct AuditLogger {
    pub enabled: bool,
    pub events: Mutex<Vec<AuditEvent>>,
    pub max_events: usize,
}

#[derive(Clone)]
pub struct AuditEvent {
    pub timestamp: Instant,
    pub event_type: String,
    pub message: String,
}

impl AuditLogger {
    pub fn new(enabled: bool, max_events: usize) -> Self {
        Self {
            enabled,
            events: Mutex::new(Vec::with_capacity(max_events)),
            max_events,
        }
    }

    pub fn log(&self, event_type: &str, message: &str) {
        if !self.enabled {
            return;
        }
        let mut events = self.events.lock().unwrap();
        if events.len() >= self.max_events {
            events.remove(0);
        }
        events.push(AuditEvent {
            timestamp: Instant::now(),
            event_type: event_type.to_string(),
            message: message.to_string(),
        });
    }

    pub fn recent_events(&self, n: usize) -> Vec<AuditEvent> {
        let events = self.events.lock().unwrap();
        let len = events.len();
        if n >= len {
            events.clone()
        } else {
            events[len - n..].to_vec()
        }
    }

    pub fn clear(&self) {
        let mut events = self.events.lock().unwrap();
        events.clear();
    }

    pub fn count(&self) -> usize {
        let events = self.events.lock().unwrap();
        events.len()
    }
}

impl AuditEvent {
    pub fn new(event_type: String, message: String) -> Self {
        Self {
            timestamp: Instant::now(),
            event_type,
            message,
        }
    }
}
