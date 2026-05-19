use crate::apps::app_id::AppId;
use crate::tiers::tier::MemoryTier;

#[derive(Debug, Clone)]
pub struct AppProfile {
    pub app_id: AppId,
    pub name: String,
    pub priority: AppPriority,
    pub max_quota_mb: u64,
    pub compression_allowed: bool,
    pub inactive_after_ms: u64,
    pub preferred_tier: MemoryTier,
}

impl AppProfile {
    pub fn new(
        app_id: AppId,
        name: impl Into<String>,
        priority: AppPriority,
        max_quota_mb: u64,
        compression_allowed: bool,
        inactive_after_ms: u64,
        preferred_tier: MemoryTier,
    ) -> Self {
        Self {
            app_id,
            name: name.into(),
            priority,
            max_quota_mb,
            compression_allowed,
            inactive_after_ms,
            preferred_tier,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
    Idle = 4,
}

impl AppPriority {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "critical" => AppPriority::Critical,
            "high" => AppPriority::High,
            "normal" => AppPriority::Normal,
            "low" => AppPriority::Low,
            "idle" => AppPriority::Idle,
            _ => AppPriority::Normal,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AppPriority::Critical => "critical",
            AppPriority::High => "high",
            AppPriority::Normal => "normal",
            AppPriority::Low => "low",
            AppPriority::Idle => "idle",
        }
    }

    pub fn from_u32(val: u32) -> Option<Self> {
        match val {
            0 => Some(AppPriority::Critical),
            1 => Some(AppPriority::High),
            2 => Some(AppPriority::Normal),
            3 => Some(AppPriority::Low),
            4 => Some(AppPriority::Idle),
            _ => None,
        }
    }

    pub fn as_u32(&self) -> u32 {
        *self as u32
    }
}

impl Default for AppPriority {
    fn default() -> Self {
        AppPriority::Normal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_ordering() {
        assert!(AppPriority::Critical < AppPriority::High);
        assert!(AppPriority::High < AppPriority::Normal);
        assert!(AppPriority::Normal < AppPriority::Low);
        assert!(AppPriority::Low < AppPriority::Idle);
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!(AppPriority::from_str("critical"), AppPriority::Critical);
        assert_eq!(AppPriority::from_str("HIGH"), AppPriority::High);
        assert_eq!(AppPriority::from_str("Normal"), AppPriority::Normal);
        assert_eq!(AppPriority::from_str("  low  "), AppPriority::Low);
        assert_eq!(AppPriority::from_str("idle"), AppPriority::Idle);
        assert_eq!(AppPriority::from_str("unknown"), AppPriority::Normal);
    }

    #[test]
    fn test_priority_as_str() {
        assert_eq!(AppPriority::Critical.as_str(), "critical");
        assert_eq!(AppPriority::Idle.as_str(), "idle");
    }

    #[test]
    fn test_priority_u32_conversion() {
        assert_eq!(AppPriority::from_u32(0), Some(AppPriority::Critical));
        assert_eq!(AppPriority::from_u32(4), Some(AppPriority::Idle));
        assert_eq!(AppPriority::from_u32(5), None);
        assert_eq!(AppPriority::Critical.as_u32(), 0);
        assert_eq!(AppPriority::Idle.as_u32(), 4);
    }

    #[test]
    fn test_app_profile_creation() {
        let id = AppId::new(1);
        let profile = AppProfile::new(id, "test-app", AppPriority::High, 1024, true, 30000, MemoryTier::T1Shm);
        assert_eq!(profile.app_id, id);
        assert_eq!(profile.name, "test-app");
        assert_eq!(profile.priority, AppPriority::High);
        assert_eq!(profile.max_quota_mb, 1024);
        assert!(profile.compression_allowed);
        assert_eq!(profile.preferred_tier, MemoryTier::T1Shm);
    }
}
