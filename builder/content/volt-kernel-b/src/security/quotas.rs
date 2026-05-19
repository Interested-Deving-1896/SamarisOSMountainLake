use std::collections::HashMap;
use std::time::Instant;

use crate::core::config::TesseractConfig;
use crate::core::error::{Result, TesseractError};

#[derive(Debug, Clone)]
pub struct AppQuota {
    pub max_memory_bytes: u64,
    pub max_concurrent_tasks: u32,
    pub max_commands_per_sec: u32,
    pub memory_used: u64,
    pub tasks_running: u32,
    pub commands_in_window: u32,
    pub window_start: Instant,
}

impl AppQuota {
    fn new(config: &TesseractConfig) -> Self {
        Self {
            max_memory_bytes: config.quota_default_max_memory_mb * 1024 * 1024,
            max_concurrent_tasks: config.quota_default_max_tasks,
            max_commands_per_sec: config.quota_default_max_commands_per_sec,
            memory_used: 0,
            tasks_running: 0,
            commands_in_window: 0,
            window_start: Instant::now(),
        }
    }
}

pub struct ResourceQuotas {
    quotas: HashMap<u32, AppQuota>,
    default_quota: AppQuota,
}

impl ResourceQuotas {
    pub fn new(config: &TesseractConfig) -> Self {
        Self {
            quotas: HashMap::new(),
            default_quota: AppQuota::new(config),
        }
    }

    pub fn check_command_quota(&mut self, app_id: u32) -> Result<()> {
        let quota = self
            .quotas
            .entry(app_id)
            .or_insert_with(|| self.default_quota.clone());

        let now = Instant::now();
        if now.duration_since(quota.window_start).as_secs() >= 1 {
            quota.commands_in_window = 0;
            quota.window_start = now;
        }

        if quota.commands_in_window >= quota.max_commands_per_sec {
            return Err(TesseractError::QuotaExceeded(format!(
                "command rate limit: {} per sec",
                quota.max_commands_per_sec
            )));
        }

        quota.commands_in_window += 1;
        Ok(())
    }

    pub fn check_memory_quota(&mut self, app_id: u32, additional: u64) -> Result<()> {
        let quota = self
            .quotas
            .entry(app_id)
            .or_insert_with(|| self.default_quota.clone());

        if quota.memory_used + additional > quota.max_memory_bytes {
            return Err(TesseractError::QuotaExceeded(format!(
                "memory limit: {} used + {} requested > {} max",
                quota.memory_used, additional, quota.max_memory_bytes
            )));
        }

        quota.memory_used += additional;
        Ok(())
    }

    pub fn release_memory(&mut self, app_id: u32, amount: u64) {
        if let Some(quota) = self.quotas.get_mut(&app_id) {
            quota.memory_used = quota.memory_used.saturating_sub(amount);
        }
    }

    pub fn set_quota(&mut self, app_id: u32, quota: AppQuota) {
        self.quotas.insert(app_id, quota);
    }

    pub fn get_quota(&self, app_id: u32) -> Option<&AppQuota> {
        self.quotas.get(&app_id)
    }

    pub fn reset(&mut self, app_id: u32) {
        self.quotas.remove(&app_id);
    }

    pub fn reset_all(&mut self) {
        self.quotas.clear();
    }
}
