use crate::apps::app_id::AppId;
use crate::pages::page_flags::PageFlags;
use crate::pages::page_id::PageId;
use crate::pages::page_meta::PageMeta;
use crate::tiers::tier::MemoryTier;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Page {
    pub meta: PageMeta,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(app_id: impl Into<AppId>, tier: MemoryTier, data: Vec<u8>) -> Self {
        let app_id = app_id.into();
        let size = data.len() as u64;
        let page_id = PageId::new();
        let meta = PageMeta::new(page_id, app_id, tier, size);
        Page { meta, data }
    }

    pub fn id(&self) -> PageId {
        self.meta.page_id
    }

    pub fn size(&self) -> u64 {
        self.meta.size
    }

    pub fn is_compressed(&self) -> bool {
        PageFlags::from_bits_truncate(self.meta.flags).is_compressed()
    }

    pub fn is_pinned(&self) -> bool {
        PageFlags::from_bits_truncate(self.meta.flags).is_pinned()
    }

    pub fn set_tier(&mut self, tier: MemoryTier) {
        self.meta.tier = tier;
    }

    pub fn touch(&mut self) {
        self.meta.last_access = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_new() {
        let data = vec![0u8; 256];
        let page = Page::new(42u64, MemoryTier::T2Direct, data.clone());
        assert_eq!(page.size(), 256);
        assert_eq!(page.meta.app_id.as_u64(), 42);
        assert_eq!(page.meta.tier, MemoryTier::T2Direct);
        assert_eq!(page.data, data);
    }

    #[test]
    fn test_page_id_unique() {
        let a = Page::new(1u64, MemoryTier::T1Shm, vec![1, 2, 3]);
        let b = Page::new(1u64, MemoryTier::T1Shm, vec![1, 2, 3]);
        assert_ne!(a.id(), b.id());
    }

    #[test]
    fn test_page_pinned_default() {
        let page = Page::new(1u64, MemoryTier::T1Shm, vec![]);
        assert!(!page.is_pinned());
        assert!(!page.is_compressed());
    }

    #[test]
    fn test_page_touch() {
        let mut page = Page::new(1u64, MemoryTier::T1Shm, vec![]);
        let before = page.meta.last_access;
        std::thread::sleep(std::time::Duration::from_millis(1));
        page.touch();
        assert!(page.meta.last_access > before);
    }

    #[test]
    fn test_page_set_tier() {
        let mut page = Page::new(1u64, MemoryTier::T1Shm, vec![]);
        assert_eq!(page.meta.tier, MemoryTier::T1Shm);
        page.set_tier(MemoryTier::T3Compressed);
        assert_eq!(page.meta.tier, MemoryTier::T3Compressed);
    }

    #[test]
    fn test_page_app_id_from_appid() {
        let app_id = AppId::new(99);
        let page = Page::new(app_id, MemoryTier::T2Direct, vec![0; 64]);
        assert_eq!(page.meta.app_id, app_id);
    }
}
