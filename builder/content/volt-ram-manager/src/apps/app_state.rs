use std::time::Instant;

use crate::apps::app_profile::AppProfile;

#[derive(Debug, Clone)]
pub struct AppState {
    pub profile: AppProfile,
    pub registered_at: Instant,
    pub total_allocated: u64,
    pub total_freed: u64,
    pub current_usage: u64,
    pub last_access: Instant,
    pub suspended: bool,
}

impl AppState {
    pub fn new(profile: AppProfile) -> Self {
        let now = Instant::now();
        Self {
            profile,
            registered_at: now,
            total_allocated: 0,
            total_freed: 0,
            current_usage: 0,
            last_access: now,
            suspended: false,
        }
    }

    pub fn mark_access(&mut self) {
        self.last_access = Instant::now();
    }

    pub fn record_alloc(&mut self, bytes: u64) {
        self.total_allocated = self.total_allocated.saturating_add(bytes);
        self.current_usage = self.current_usage.saturating_add(bytes);
        self.last_access = Instant::now();
    }

    pub fn record_free(&mut self, bytes: u64) {
        self.total_freed = self.total_freed.saturating_add(bytes);
        self.current_usage = self.current_usage.saturating_sub(bytes);
        self.last_access = Instant::now();
    }

    pub fn uptime(&self) -> std::time::Duration {
        self.registered_at.elapsed()
    }

    pub fn idle_duration(&self) -> std::time::Duration {
        self.last_access.elapsed()
    }

    pub fn suspend(&mut self) {
        self.suspended = true;
    }

    pub fn resume(&mut self) {
        self.suspended = false;
        self.last_access = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::app_id::AppId;
    use crate::apps::app_profile::AppPriority;
    use crate::tiers::tier::MemoryTier;

    fn make_profile(id: u64) -> AppProfile {
        AppProfile::new(
            AppId::new(id),
            format!("app-{}", id),
            AppPriority::Normal,
            512,
            true,
            30000,
            MemoryTier::T1Shm,
        )
    }

    #[test]
    fn test_app_state_new() {
        let profile = make_profile(1);
        let state = AppState::new(profile.clone());
        assert_eq!(state.profile.app_id, profile.app_id);
        assert_eq!(state.total_allocated, 0);
        assert_eq!(state.current_usage, 0);
        assert!(!state.suspended);
    }

    #[test]
    fn test_record_alloc() {
        let profile = make_profile(1);
        let mut state = AppState::new(profile);
        state.record_alloc(4096);
        assert_eq!(state.total_allocated, 4096);
        assert_eq!(state.current_usage, 4096);
    }

    #[test]
    fn test_record_free() {
        let profile = make_profile(1);
        let mut state = AppState::new(profile);
        state.record_alloc(8192);
        state.record_free(4096);
        assert_eq!(state.total_allocated, 8192);
        assert_eq!(state.total_freed, 4096);
        assert_eq!(state.current_usage, 4096);
    }

    #[test]
    fn test_suspend_resume() {
        let profile = make_profile(1);
        let mut state = AppState::new(profile);
        assert!(!state.suspended);
        state.suspend();
        assert!(state.suspended);
        state.resume();
        assert!(!state.suspended);
    }

    #[test]
    fn test_mark_access() {
        let profile = make_profile(1);
        let mut state = AppState::new(profile);
        let before = state.last_access;
        std::thread::sleep(std::time::Duration::from_millis(1));
        state.mark_access();
        assert!(state.last_access > before);
    }
}
