use dashmap::DashMap;

use crate::core::error::VrmError;
use crate::core::result::VrmResult;
use crate::quotas::app_quota::AppQuota;

#[derive(Clone)]
pub struct QuotaTable {
    quotas: DashMap<u64, AppQuota>,
}

impl QuotaTable {
    pub fn new() -> Self {
        Self {
            quotas: DashMap::new(),
        }
    }

    pub fn set(&self, app_id: u64, quota: AppQuota) {
        self.quotas.insert(app_id, quota);
    }

    pub fn get(&self, app_id: u64) -> Option<AppQuota> {
        self.quotas.get(&app_id).map(|r| *r.value())
    }

    pub fn remove(&self, app_id: u64) {
        self.quotas.remove(&app_id);
    }

    pub fn contains(&self, app_id: u64) -> bool {
        self.quotas.contains_key(&app_id)
    }

    pub fn len(&self) -> usize {
        self.quotas.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quotas.is_empty()
    }

    pub fn check(&self, app_id: u64, additional: u64) -> VrmResult<()> {
        let quota = self
            .quotas
            .get(&app_id)
            .ok_or(VrmError::AppNotRegistered(app_id))?;
        let used = quota.current_usage;
        let limit = quota.max_memory_bytes;
        if used.saturating_add(additional) > limit {
            return Err(VrmError::QuotaExceeded {
                app_id,
                used: used + additional,
                limit,
            });
        }
        Ok(())
    }

    pub fn record_usage(&self, app_id: u64, bytes: u64) {
        if let Some(mut quota) = self.quotas.get_mut(&app_id) {
            quota.current_usage = quota.current_usage.saturating_add(bytes);
        }
    }

    pub fn release_usage(&self, app_id: u64, bytes: u64) {
        if let Some(mut quota) = self.quotas.get_mut(&app_id) {
            quota.current_usage = quota.current_usage.saturating_sub(bytes);
        }
    }

    pub fn reset(&self, app_id: u64) {
        if let Some(mut quota) = self.quotas.get_mut(&app_id) {
            quota.current_usage = 0;
        }
    }

    pub fn all_quotas(&self) -> Vec<(u64, AppQuota)> {
        self.quotas
            .iter()
            .map(|entry| (*entry.key(), *entry.value()))
            .collect()
    }

    pub fn total_usage(&self) -> u64 {
        self.quotas
            .iter()
            .map(|entry| entry.value().current_usage)
            .sum()
    }
}

impl Default for QuotaTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let table = QuotaTable::new();
        let q = AppQuota::new(128);
        table.set(1, q);
        let retrieved = table.get(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().max_memory_bytes, 128 * 1024 * 1024);
    }

    #[test]
    fn test_remove() {
        let table = QuotaTable::new();
        table.set(1, AppQuota::new(64));
        assert!(table.contains(1));
        table.remove(1);
        assert!(!table.contains(1));
    }

    #[test]
    fn test_check_ok() {
        let table = QuotaTable::new();
        table.set(1, AppQuota::new(64));
        assert!(table.check(1, 4096).is_ok());
    }

    #[test]
    fn test_check_exceeded() {
        let table = QuotaTable::new();
        let mut q = AppQuota::new(1);
        q.current_usage = 1024 * 1024; // 1MB
        table.set(1, q);
        let result = table.check(1, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VrmError::QuotaExceeded { .. }));
    }

    #[test]
    fn test_check_not_registered() {
        let table = QuotaTable::new();
        let result = table.check(999, 4096);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VrmError::AppNotRegistered(999)));
    }

    #[test]
    fn test_record_and_release() {
        let table = QuotaTable::new();
        table.set(1, AppQuota::new(64));
        table.record_usage(1, 4096);
        assert_eq!(table.get(1).unwrap().current_usage, 4096);
        table.release_usage(1, 2048);
        assert_eq!(table.get(1).unwrap().current_usage, 2048);
    }

    #[test]
    fn test_reset() {
        let table = QuotaTable::new();
        table.set(1, AppQuota::new(64));
        table.record_usage(1, 10000);
        table.reset(1);
        assert_eq!(table.get(1).unwrap().current_usage, 0);
    }
}
