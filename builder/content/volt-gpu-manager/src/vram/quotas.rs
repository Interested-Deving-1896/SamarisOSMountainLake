use dashmap::DashMap;
use crate::core::result::VgmResult;

#[derive(Debug, Clone)]
pub struct VramQuotaEntry {
    pub max_bytes: u64,
    pub used_bytes: u64,
    pub pinned: bool,
}

pub struct VramQuotaTable {
    entries: DashMap<u64, VramQuotaEntry>,
}

impl VramQuotaTable {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn set_quota(&self, app_id: u64, max_mb: u64, pinned: bool) {
        let max_bytes = max_mb * 1024 * 1024;
        self.entries.entry(app_id).or_insert_with(|| VramQuotaEntry {
            max_bytes,
            used_bytes: 0,
            pinned,
        });
    }

    pub fn check_quota(&self, app_id: u64, additional: u64) -> VgmResult<()> {
        if let Some(entry) = self.entries.get(&app_id) {
            let new_usage = entry.used_bytes + additional;
            if new_usage > entry.max_bytes {
                return Err(crate::core::error::VgmError::VramQuotaExceeded(format!(
                    "App {} quota exceeded: {}+{} > {}",
                    app_id, entry.used_bytes, additional, entry.max_bytes
                )));
            }
        }
        Ok(())
    }

    pub fn record_usage(&self, app_id: u64, bytes: u64) {
        self.entries
            .entry(app_id)
            .and_modify(|e| e.used_bytes = e.used_bytes.saturating_add(bytes));
    }

    pub fn release_usage(&self, app_id: u64, bytes: u64) {
        self.entries
            .entry(app_id)
            .and_modify(|e| e.used_bytes = e.used_bytes.saturating_sub(bytes));
    }

    pub fn usage_of(&self, app_id: u64) -> u64 {
        self.entries
            .get(&app_id)
            .map(|e| e.used_bytes)
            .unwrap_or(0)
    }
}

impl Default for VramQuotaTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table() {
        let table = VramQuotaTable::new();
        assert_eq!(table.usage_of(1), 0);
    }

    #[test]
    fn test_set_quota() {
        let table = VramQuotaTable::new();
        table.set_quota(42, 1024, false);
        assert!(table.check_quota(42, 0).is_ok());
    }

    #[test]
    fn test_check_quota_within_limit() {
        let table = VramQuotaTable::new();
        table.set_quota(1, 100, false);
        table.record_usage(1, 50 * 1024 * 1024);
        assert!(table.check_quota(1, 30 * 1024 * 1024).is_ok());
    }

    #[test]
    fn test_check_quota_exceeded() {
        let table = VramQuotaTable::new();
        table.set_quota(1, 100, false);
        table.record_usage(1, 90 * 1024 * 1024);
        let result = table.check_quota(1, 20 * 1024 * 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_quota_no_entry() {
        let table = VramQuotaTable::new();
        assert!(table.check_quota(999, 100).is_ok());
    }

    #[test]
    fn test_record_and_release() {
        let table = VramQuotaTable::new();
        table.set_quota(1, 100, false);
        table.record_usage(1, 60 * 1024 * 1024);
        assert_eq!(table.usage_of(1), 60 * 1024 * 1024);
        table.release_usage(1, 20 * 1024 * 1024);
        assert_eq!(table.usage_of(1), 40 * 1024 * 1024);
    }

    #[test]
    fn test_release_below_zero() {
        let table = VramQuotaTable::new();
        table.set_quota(1, 100, false);
        table.release_usage(1, 1000);
        assert_eq!(table.usage_of(1), 0);
    }

    #[test]
    fn test_usage_of_unknown() {
        let table = VramQuotaTable::new();
        assert_eq!(table.usage_of(999), 0);
    }

    #[test]
    fn test_default() {
        let table: VramQuotaTable = Default::default();
        assert_eq!(table.usage_of(0), 0);
    }
}
