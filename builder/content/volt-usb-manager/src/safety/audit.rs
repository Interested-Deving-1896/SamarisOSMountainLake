use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub action: String,
    pub path: Option<String>,
    pub details: String,
    pub success: bool,
}

impl AuditEvent {
    pub fn new(action: &str, path: Option<&str>, details: &str, success: bool) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        AuditEvent {
            timestamp,
            action: action.to_string(),
            path: path.map(String::from),
            details: details.to_string(),
            success,
        }
    }
}

pub struct AuditLog {
    events: VecDeque<AuditEvent>,
    max_entries: usize,
}

impl AuditLog {
    pub fn new(max: usize) -> Self {
        AuditLog {
            events: VecDeque::with_capacity(max.min(1024)),
            max_entries: max.max(1),
        }
    }

    pub fn log(&mut self, event: AuditEvent) {
        if self.events.len() >= self.max_entries {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn recent(&self, n: usize) -> Vec<&AuditEvent> {
        let count = n.min(self.events.len());
        self.events.iter().rev().take(count).collect()
    }

    pub fn export_json(&self) -> String {
        let events: Vec<&AuditEvent> = self.events.iter().collect();
        serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_audit_log_empty() {
        let log = AuditLog::new(100);
        assert!(log.is_empty());
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn test_log_event() {
        let mut log = AuditLog::new(100);
        let event = AuditEvent::new("mount", Some("/dev/sda1"), "Mounted successfully", true);
        log.log(event);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn test_recent_events() {
        let mut log = AuditLog::new(100);
        for i in 0..10 {
            log.log(AuditEvent::new(
                &format!("action_{}", i),
                None,
                "test",
                true,
            ));
        }
        let recent = log.recent(3);
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].action, "action_9");
        assert_eq!(recent[2].action, "action_7");
    }

    #[test]
    fn test_max_entries_enforced() {
        let mut log = AuditLog::new(5);
        for i in 0..10 {
            log.log(AuditEvent::new(
                &format!("action_{}", i),
                None,
                "overflow test",
                true,
            ));
        }
        assert_eq!(log.len(), 5);
        assert_eq!(log.events[0].action, "action_5");
    }

    #[test]
    fn test_export_json() {
        let mut log = AuditLog::new(10);
        log.log(AuditEvent::new("eject", Some("/dev/sda1"), "Ejected safely", true));
        let json = log.export_json();
        assert!(json.contains("eject"));
        assert!(json.contains("/dev/sda1"));
        assert!(json.contains("success"));
    }

    #[test]
    fn test_clear() {
        let mut log = AuditLog::new(10);
        log.log(AuditEvent::new("test", None, "clear test", true));
        assert_eq!(log.len(), 1);
        log.clear();
        assert_eq!(log.len(), 0);
    }

    #[test]
    fn test_event_timestamp() {
        let event = AuditEvent::new("test", None, "timestamp check", true);
        assert!(event.timestamp > 1_700_000_000);
    }
}
