use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: u64,
    pub app_id: u32,
    pub opcode: u8,
    pub action: String,
    pub allowed: bool,
    pub reason: String,
}

pub struct AuditLog {
    entries: VecDeque<AuditEntry>,
    max_entries: usize,
}

impl AuditLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries.min(100)),
            max_entries,
        }
    }

    pub fn log(&mut self, entry: AuditEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn query(&self, filter: Option<u32>) -> Vec<&AuditEntry> {
        match filter {
            Some(app_id) => self
                .entries
                .iter()
                .filter(|e| e.app_id == app_id)
                .collect(),
            None => self.entries.iter().collect(),
        }
    }

    pub fn recent(&self, n: usize) -> Vec<&AuditEntry> {
        self.entries.iter().rev().take(n).collect()
    }

    pub fn export_json(&self) -> String {
        let entries: Vec<serde_json::Value> = self
            .entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "timestamp": e.timestamp,
                    "app_id": format!("0x{:08X}", e.app_id),
                    "opcode": format!("0x{:02X}", e.opcode),
                    "action": e.action,
                    "allowed": e.allowed,
                    "reason": e.reason,
                })
            })
            .collect();
        serde_json::to_string_pretty(&entries).unwrap_or_default()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
