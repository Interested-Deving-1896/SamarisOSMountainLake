use crate::apps::app_id::AppId;
use crate::pages::page_id::PageId;
use crate::tiers::tier::MemoryTier;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct PageMeta {
    pub page_id: PageId,
    pub app_id: AppId,
    pub tier: MemoryTier,
    pub size: u64,
    pub compressed_size: Option<u64>,
    pub created_at: u64,
    pub last_access: u64,
    pub flags: u8,
}

impl PageMeta {
    pub fn new(page_id: PageId, app_id: AppId, tier: MemoryTier, size: u64) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        PageMeta {
            page_id,
            app_id,
            tier,
            size,
            compressed_size: None,
            created_at: now,
            last_access: now,
            flags: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::app_id::AppId;
    use crate::tiers::tier::MemoryTier;

    #[test]
    fn test_page_meta_new() {
        let pid = PageId::new();
        let app = AppId::new(1);
        let meta = PageMeta::new(pid, app, MemoryTier::T1Shm, 4096);
        assert_eq!(meta.page_id, pid);
        assert_eq!(meta.app_id, app);
        assert_eq!(meta.tier, MemoryTier::T1Shm);
        assert_eq!(meta.size, 4096);
        assert!(meta.compressed_size.is_none());
        assert!(meta.created_at > 0);
        assert_eq!(meta.last_access, meta.created_at);
        assert_eq!(meta.flags, 0);
    }

    #[test]
    fn test_page_meta_different_tiers() {
        let pid = PageId::new();
        let app = AppId::new(2);
        let meta = PageMeta::new(pid, app, MemoryTier::T3Compressed, 8192);
        assert_eq!(meta.tier, MemoryTier::T3Compressed);
        assert_eq!(meta.size, 8192);
    }
}
