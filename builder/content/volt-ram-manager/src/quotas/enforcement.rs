use crate::apps::app_id::AppId;
use crate::core::result::VrmResult;
use crate::quotas::app_quota::AppQuota;
use crate::quotas::quota_table::QuotaTable;

#[derive(Clone)]
pub struct QuotaGovernor {
    table: QuotaTable,
}

impl QuotaGovernor {
    pub fn new() -> Self {
        Self {
            table: QuotaTable::new(),
        }
    }

    pub fn with_capacity(_capacity: usize) -> Self {
        Self {
            table: QuotaTable::new(),
        }
    }

    pub fn check_quota(&self, app_id: AppId, additional: u64) -> VrmResult<()> {
        let id = app_id.as_u64();
        self.table.check(id, additional)
    }

    pub fn set_quota(&mut self, app_id: AppId, quota: AppQuota) {
        self.table.set(app_id.as_u64(), quota);
    }

    pub fn remove_quota(&mut self, app_id: AppId) {
        self.table.remove(app_id.as_u64());
    }

    pub fn get_quota(&self, app_id: AppId) -> Option<AppQuota> {
        self.table.get(app_id.as_u64())
    }

    pub fn record_usage(&self, app_id: AppId, bytes: u64) {
        self.table.record_usage(app_id.as_u64(), bytes);
    }

    pub fn release_usage(&self, app_id: AppId, bytes: u64) {
        self.table.release_usage(app_id.as_u64(), bytes);
    }

    pub fn reset_quota(&self, app_id: AppId) {
        self.table.reset(app_id.as_u64());
    }

    pub fn total_usage(&self) -> u64 {
        self.table.total_usage()
    }

    pub fn contains(&self, app_id: AppId) -> bool {
        self.table.contains(app_id.as_u64())
    }

    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl Default for QuotaGovernor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::error::VrmError;

    #[test]
    fn test_set_and_get_quota() {
        let mut gov = QuotaGovernor::new();
        let id = AppId(1);
        let q = AppQuota::new(256);
        gov.set_quota(id, q);
        let retrieved = gov.get_quota(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().max_memory_bytes, 256 * 1024 * 1024);
    }

    #[test]
    fn test_check_quota_ok() {
        let mut gov = QuotaGovernor::new();
        let id = AppId(1);
        gov.set_quota(id, AppQuota::new(64));
        assert!(gov.check_quota(id, 4096).is_ok());
    }

    #[test]
    fn test_check_quota_exceeded() {
        let mut gov = QuotaGovernor::new();
        let id = AppId(1);
        let mut q = AppQuota::new(1);
        q.current_usage = 1024 * 1024;
        gov.set_quota(id, q);
        let result = gov.check_quota(id, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VrmError::QuotaExceeded { .. }));
    }

    #[test]
    fn test_remove_quota() {
        let mut gov = QuotaGovernor::new();
        let id = AppId(1);
        gov.set_quota(id, AppQuota::new(64));
        assert!(gov.contains(id));
        gov.remove_quota(id);
        assert!(!gov.contains(id));
    }

    #[test]
    fn test_record_and_release() {
        let mut gov = QuotaGovernor::new();
        let id = AppId(1);
        gov.set_quota(id, AppQuota::new(64));
        gov.record_usage(id, 8192);
        assert_eq!(gov.get_quota(id).unwrap().current_usage, 8192);
        gov.release_usage(id, 4096);
        assert_eq!(gov.get_quota(id).unwrap().current_usage, 4096);
    }

    #[test]
    fn test_total_usage() {
        let mut gov = QuotaGovernor::new();
        gov.set_quota(AppId(1), AppQuota::new(64));
        gov.set_quota(AppId(2), AppQuota::new(128));
        gov.record_usage(AppId(1), 1000);
        gov.record_usage(AppId(2), 2000);
        assert_eq!(gov.total_usage(), 3000);
    }
}
