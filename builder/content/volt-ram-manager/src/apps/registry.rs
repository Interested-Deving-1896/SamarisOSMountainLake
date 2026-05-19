use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;

use crate::apps::app_id::AppId;
use crate::apps::app_memory::AppMemoryUsage;
use crate::apps::app_profile::AppProfile;
use crate::apps::app_state::AppState;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;

pub struct AppRegistry {
    apps: DashMap<u64, AppState>,
    next_id: AtomicU64,
}

impl Clone for AppRegistry {
    fn clone(&self) -> Self {
        Self {
            apps: self.apps.clone(),
            next_id: AtomicU64::new(self.next_id.load(Ordering::SeqCst)),
        }
    }
}

impl AppRegistry {
    pub fn new() -> Self {
        Self {
            apps: DashMap::new(),
            next_id: AtomicU64::new(1),
        }
    }

    pub fn register(&self, profile: AppProfile) -> VrmResult<AppId> {
        let app_id = profile.app_id;
        if self.apps.contains_key(&app_id.0) {
            return Err(VrmError::AppAlreadyRegistered(app_id.0));
        }
        let state = AppState::new(profile);
        self.apps.insert(app_id.0, state);
        tracing::info!("App registered: {}", app_id);
        Ok(app_id)
    }

    pub fn register_new(&self, name: &str, priority: crate::apps::app_profile::AppPriority, max_quota_mb: u64) -> VrmResult<AppId> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let app_id = AppId::new(id);
        let profile = AppProfile::new(
            app_id,
            name,
            priority,
            max_quota_mb,
            true,
            30000,
            crate::tiers::tier::MemoryTier::T1Shm,
        );
        self.register(profile)
    }

    pub fn unregister(&self, app_id: AppId) -> VrmResult<()> {
        self.apps
            .remove(&app_id.0)
            .ok_or(VrmError::AppNotRegistered(app_id.0))?;
        tracing::info!("App unregistered: {}", app_id);
        Ok(())
    }

    pub fn get(&self, app_id: AppId) -> Option<dashmap::mapref::one::Ref<'_, u64, AppState>> {
        self.apps.get(&app_id.0)
    }

    pub fn get_mut(&self, app_id: AppId) -> Option<dashmap::mapref::one::RefMut<'_, u64, AppState>> {
        self.apps.get_mut(&app_id.0)
    }

    pub fn contains(&self, app_id: AppId) -> bool {
        self.apps.contains_key(&app_id.0)
    }

    pub fn len(&self) -> usize {
        self.apps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.apps.is_empty()
    }

    pub fn memory_usage(&self, app_id: AppId) -> Option<AppMemoryUsage> {
        let state = self.apps.get(&app_id.0)?;
        let mut usage = AppMemoryUsage::new(app_id.0);
        usage.total_bytes = state.current_usage;
        usage.page_count = 0;
        Some(usage)
    }

    pub fn all_apps(&self) -> Vec<AppState> {
        self.apps.iter().map(|entry| entry.value().clone()).collect()
    }

    pub fn total_allocated(&self) -> u64 {
        self.apps
            .iter()
            .map(|entry| entry.value().total_allocated)
            .sum()
    }

    pub fn total_current_usage(&self) -> u64 {
        self.apps
            .iter()
            .map(|entry| entry.value().current_usage)
            .sum()
    }

    pub fn app_count_by_priority(&self, priority: crate::apps::app_profile::AppPriority) -> usize {
        self.apps
            .iter()
            .filter(|entry| entry.value().profile.priority == priority)
            .count()
    }

    pub fn iter_apps(&self) -> Vec<(u64, String, u64)> {
        self.apps
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().profile.name.clone(), entry.value().current_usage))
            .collect()
    }
}

impl Default for AppRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::app_profile::AppPriority;
    use crate::tiers::tier::MemoryTier;

    fn make_profile(id: u64, name: &str) -> AppProfile {
        AppProfile::new(AppId::new(id), name, AppPriority::Normal, 512, true, 30000, MemoryTier::T1Shm)
    }

    #[test]
    fn test_register_and_get() {
        let registry = AppRegistry::new();
        let profile = make_profile(1, "test-app");
        let id = registry.register(profile).unwrap();
        assert_eq!(id, AppId(1));
        assert!(registry.contains(id));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_register_duplicate_fails() {
        let registry = AppRegistry::new();
        let profile = make_profile(1, "app1");
        registry.register(profile).unwrap();
        let profile2 = make_profile(1, "app1-dupe");
        let result = registry.register(profile2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), VrmError::AppAlreadyRegistered(1)));
    }

    #[test]
    fn test_unregister() {
        let registry = AppRegistry::new();
        let profile = make_profile(1, "test-app");
        registry.register(profile).unwrap();
        registry.unregister(AppId(1)).unwrap();
        assert!(!registry.contains(AppId(1)));
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_unregister_nonexistent_fails() {
        let registry = AppRegistry::new();
        let result = registry.unregister(AppId(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_get_mut() {
        let registry = AppRegistry::new();
        let profile = make_profile(1, "test-app");
        registry.register(profile).unwrap();

        if let Some(mut state) = registry.get_mut(AppId(1)) {
            state.record_alloc(4096);
        }
        let state = registry.get(AppId(1)).unwrap();
        assert_eq!(state.current_usage, 4096);
    }

    #[test]
    fn test_total_allocated() {
        let registry = AppRegistry::new();
        let p1 = make_profile(1, "app1");
        let p2 = make_profile(2, "app2");
        registry.register(p1).unwrap();
        registry.register(p2).unwrap();

        if let Some(mut state) = registry.get_mut(AppId(1)) {
            state.record_alloc(1000);
        }
        if let Some(mut state) = registry.get_mut(AppId(2)) {
            state.record_alloc(2000);
        }
        assert_eq!(registry.total_allocated(), 3000);
    }

    #[test]
    fn test_register_new_auto_id() {
        let registry = AppRegistry::new();
        let id1 = registry.register_new("app1", AppPriority::Critical, 1024).unwrap();
        let id2 = registry.register_new("app2", AppPriority::Low, 256).unwrap();
        assert_ne!(id1, id2);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_memory_usage() {
        let registry = AppRegistry::new();
        let profile = make_profile(1, "test-app");
        registry.register(profile).unwrap();
        let usage = registry.memory_usage(AppId(1));
        assert!(usage.is_some());
        assert_eq!(usage.unwrap().total_bytes, 0);
    }

    #[test]
    fn test_all_apps() {
        let registry = AppRegistry::new();
        registry.register(make_profile(1, "a")).unwrap();
        registry.register(make_profile(2, "b")).unwrap();
        assert_eq!(registry.all_apps().len(), 2);
    }
}
