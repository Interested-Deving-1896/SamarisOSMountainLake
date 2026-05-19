use crate::pages::page_id::PageId;
use crate::tiers::tier::MemoryTier;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct PageResidency {
    pub page_id: PageId,
    pub tier: MemoryTier,
    pub promoted_count: u64,
    pub demoted_count: u64,
    pub last_transition: u64,
}

impl PageResidency {
    pub fn new(page_id: PageId, tier: MemoryTier) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        PageResidency {
            page_id,
            tier,
            promoted_count: 0,
            demoted_count: 0,
            last_transition: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_residency_new() {
        let pid = PageId::new();
        let res = PageResidency::new(pid, MemoryTier::T2Direct);
        assert_eq!(res.page_id, pid);
        assert_eq!(res.tier, MemoryTier::T2Direct);
        assert_eq!(res.promoted_count, 0);
        assert_eq!(res.demoted_count, 0);
        assert!(res.last_transition > 0);
    }

    #[test]
    fn test_residency_different_tier() {
        let pid = PageId::new();
        let res = PageResidency::new(pid, MemoryTier::T3Compressed);
        assert_eq!(res.tier, MemoryTier::T3Compressed);
    }
}
