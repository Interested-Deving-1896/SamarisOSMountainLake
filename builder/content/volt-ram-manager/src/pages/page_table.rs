use dashmap::DashMap;
use uuid::Uuid;

use crate::apps::app_id::AppId;
use crate::core::error::VrmError;
use crate::core::result::VrmResult;
use crate::pages::page::Page;
use crate::pages::page_id::PageId;
use crate::tiers::tier::MemoryTier;

pub struct PageTable {
    pages: DashMap<Uuid, Page>,
}

impl PageTable {
    pub fn new() -> Self {
        PageTable {
            pages: DashMap::new(),
        }
    }

    pub fn insert(&self, page: Page) -> VrmResult<()> {
        let id = page.id().0;
        self.pages.insert(id, page);
        Ok(())
    }

    pub fn get(&self, id: PageId) -> Option<Page> {
        self.pages.get(&id.0).map(|r| r.value().clone())
    }

    pub fn remove(&self, id: PageId) -> VrmResult<()> {
        let key = id.0;
        self.pages
            .remove(&key)
            .ok_or(VrmError::PageNotFound(key.as_u128() as u64))?;
        Ok(())
    }

    pub fn get_by_app(&self, app_id: AppId) -> Vec<Page> {
        self.pages
            .iter()
            .filter(|r| r.meta.app_id == app_id)
            .map(|r| r.value().clone())
            .collect()
    }

    pub fn get_by_tier(&self, tier: MemoryTier) -> Vec<Page> {
        self.pages
            .iter()
            .filter(|r| r.meta.tier == tier)
            .map(|r| r.value().clone())
            .collect()
    }

    pub fn total_pages(&self) -> usize {
        self.pages.len()
    }

    pub fn total_bytes(&self) -> u64 {
        self.pages.iter().map(|r| r.value().meta.size).sum()
    }

    pub fn total_bytes_by_tier(&self, tier: MemoryTier) -> u64 {
        self.pages
            .iter()
            .filter(|r| r.value().meta.tier == tier)
            .map(|r| r.value().meta.size)
            .sum()
    }
}

impl Clone for PageTable {
    fn clone(&self) -> Self {
        PageTable {
            pages: self.pages.clone(),
        }
    }
}

impl Default for PageTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_page(app_id: u64, tier: MemoryTier, data: Vec<u8>) -> Page {
        Page::new(app_id, tier, data)
    }

    #[test]
    fn test_insert_and_get() {
        let table = PageTable::new();
        let page = make_page(1, MemoryTier::T1Shm, vec![0; 128]);
        let id = page.id();
        table.insert(page).unwrap();
        let fetched = table.get(id).unwrap();
        assert_eq!(fetched.size(), 128);
        assert_eq!(fetched.meta.app_id.as_u64(), 1);
    }

    #[test]
    fn test_get_missing() {
        let table = PageTable::new();
        assert!(table.get(PageId::nil()).is_none());
    }

    #[test]
    fn test_remove() {
        let table = PageTable::new();
        let page = make_page(1, MemoryTier::T1Shm, vec![]);
        let id = page.id();
        table.insert(page).unwrap();
        table.remove(id).unwrap();
        assert!(table.get(id).is_none());
    }

    #[test]
    fn test_remove_missing() {
        let table = PageTable::new();
        assert!(table.remove(PageId::nil()).is_err());
    }

    #[test]
    fn test_total_pages() {
        let table = PageTable::new();
        assert_eq!(table.total_pages(), 0);
        table
            .insert(make_page(1, MemoryTier::T1Shm, vec![0; 64]))
            .unwrap();
        table
            .insert(make_page(1, MemoryTier::T2Direct, vec![0; 128]))
            .unwrap();
        assert_eq!(table.total_pages(), 2);
    }

    #[test]
    fn test_total_bytes() {
        let table = PageTable::new();
        table
            .insert(make_page(1, MemoryTier::T1Shm, vec![0; 100]))
            .unwrap();
        table
            .insert(make_page(1, MemoryTier::T2Direct, vec![0; 200]))
            .unwrap();
        assert_eq!(table.total_bytes(), 300);
    }

    #[test]
    fn test_get_by_app() {
        let table = PageTable::new();
        table
            .insert(make_page(1, MemoryTier::T1Shm, vec![]))
            .unwrap();
        table
            .insert(make_page(2, MemoryTier::T1Shm, vec![]))
            .unwrap();
        table
            .insert(make_page(1, MemoryTier::T2Direct, vec![]))
            .unwrap();
        assert_eq!(table.get_by_app(AppId::new(1)).len(), 2);
        assert_eq!(table.get_by_app(AppId::new(2)).len(), 1);
    }

    #[test]
    fn test_get_by_tier() {
        let table = PageTable::new();
        table
            .insert(make_page(1, MemoryTier::T1Shm, vec![]))
            .unwrap();
        table
            .insert(make_page(2, MemoryTier::T2Direct, vec![]))
            .unwrap();
        table
            .insert(make_page(3, MemoryTier::T1Shm, vec![]))
            .unwrap();
        assert_eq!(table.get_by_tier(MemoryTier::T1Shm).len(), 2);
        assert_eq!(table.get_by_tier(MemoryTier::T2Direct).len(), 1);
        assert_eq!(table.get_by_tier(MemoryTier::T3Compressed).len(), 0);
    }

    #[test]
    fn test_total_bytes_by_tier() {
        let table = PageTable::new();
        table
            .insert(make_page(1, MemoryTier::T1Shm, vec![0; 50]))
            .unwrap();
        table
            .insert(make_page(2, MemoryTier::T2Direct, vec![0; 150]))
            .unwrap();
        table
            .insert(make_page(3, MemoryTier::T1Shm, vec![0; 30]))
            .unwrap();
        assert_eq!(table.total_bytes_by_tier(MemoryTier::T1Shm), 80);
        assert_eq!(table.total_bytes_by_tier(MemoryTier::T2Direct), 150);
        assert_eq!(table.total_bytes_by_tier(MemoryTier::T3Compressed), 0);
    }
}
