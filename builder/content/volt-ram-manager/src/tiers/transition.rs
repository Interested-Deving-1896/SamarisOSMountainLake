use std::sync::Arc;

use parking_lot::RwLock;

use crate::core::error::VrmError;
use crate::core::result::VrmResult;
use crate::pages::page_id::PageId;
use crate::pages::page_table::PageTable;
use crate::tiers::tier::MemoryTier;

pub struct TierController {
    page_table: Arc<RwLock<PageTable>>,
}

impl TierController {
    pub fn new(page_table: Arc<RwLock<PageTable>>) -> Self {
        TierController { page_table }
    }

    pub fn promote(&self, page_id: PageId) -> VrmResult<()> {
        let table = self.page_table.write();
        let page = table
            .get(page_id)
            .ok_or(VrmError::PageNotFound(page_id.0.as_u128() as u64))?;
        let mut page = page;
        if page.is_pinned() {
            return Err(VrmError::PagePinned);
        }
        match page.meta.tier {
            MemoryTier::T3Compressed => {
                page.set_tier(MemoryTier::T2Direct);
            }
            _ => {
                return Err(VrmError::InvalidState(format!(
                    "Cannot promote page from tier {:?}",
                    page.meta.tier
                )));
            }
        }
        table.remove(page_id)?;
        table.insert(page)?;
        Ok(())
    }

    pub fn demote(&self, page_id: PageId) -> VrmResult<()> {
        let table = self.page_table.write();
        let page = table
            .get(page_id)
            .ok_or(VrmError::PageNotFound(page_id.0.as_u128() as u64))?;
        let mut page = page;
        if page.is_pinned() {
            return Err(VrmError::PagePinned);
        }
        match page.meta.tier {
            MemoryTier::T2Direct => {
                page.set_tier(MemoryTier::T3Compressed);
            }
            _ => {
                return Err(VrmError::InvalidState(format!(
                    "Cannot demote page from tier {:?}",
                    page.meta.tier
                )));
            }
        }
        table.remove(page_id)?;
        table.insert(page)?;
        Ok(())
    }

    pub fn is_transition_allowed(&self, page_id: PageId) -> bool {
        let table = self.page_table.read();
        match table.get(page_id) {
            Some(page) => !page.is_pinned(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::page::Page;

    fn make_table_with_page(
        tier: MemoryTier,
        pinned: bool,
    ) -> (Arc<RwLock<PageTable>>, PageId) {
        let table = Arc::new(RwLock::new(PageTable::new()));
        let data = vec![0u8; 256];
        let mut page = Page::new(1u64, tier, data);
        let id = page.id();
        if pinned {
            page.meta.flags |= 0b0001;
        }
        table.write().insert(page).unwrap();
        (table, id)
    }

    #[test]
    fn test_promote_t3_to_t2() {
        let (table, id) = make_table_with_page(MemoryTier::T3Compressed, false);
        let controller = TierController::new(table.clone());
        controller.promote(id).unwrap();
        let page = table.read().get(id).unwrap();
        assert_eq!(page.meta.tier, MemoryTier::T2Direct);
    }

    #[test]
    fn test_demote_t2_to_t3() {
        let (table, id) = make_table_with_page(MemoryTier::T2Direct, false);
        let controller = TierController::new(table.clone());
        controller.demote(id).unwrap();
        let page = table.read().get(id).unwrap();
        assert_eq!(page.meta.tier, MemoryTier::T3Compressed);
    }

    #[test]
    fn test_promote_pinned_fails() {
        let (table, id) = make_table_with_page(MemoryTier::T3Compressed, true);
        let controller = TierController::new(table);
        assert!(controller.promote(id).is_err());
    }

    #[test]
    fn test_demote_pinned_fails() {
        let (table, id) = make_table_with_page(MemoryTier::T2Direct, true);
        let controller = TierController::new(table);
        assert!(controller.demote(id).is_err());
    }

    #[test]
    fn test_promote_wrong_tier_fails() {
        let (table, id) = make_table_with_page(MemoryTier::T1Shm, false);
        let controller = TierController::new(table);
        assert!(controller.promote(id).is_err());
    }

    #[test]
    fn test_demote_wrong_tier_fails() {
        let (table, id) = make_table_with_page(MemoryTier::T3Compressed, false);
        let controller = TierController::new(table);
        assert!(controller.demote(id).is_err());
    }

    #[test]
    fn test_is_transition_allowed() {
        let (table, id) = make_table_with_page(MemoryTier::T2Direct, false);
        let controller = TierController::new(table);
        assert!(controller.is_transition_allowed(id));
    }

    #[test]
    fn test_is_transition_allowed_pinned() {
        let (table, id) = make_table_with_page(MemoryTier::T2Direct, true);
        let controller = TierController::new(table);
        assert!(!controller.is_transition_allowed(id));
    }

    #[test]
    fn test_is_transition_allowed_missing() {
        let table = Arc::new(RwLock::new(PageTable::new()));
        let controller = TierController::new(table);
        assert!(!controller.is_transition_allowed(PageId::new()));
    }
}
