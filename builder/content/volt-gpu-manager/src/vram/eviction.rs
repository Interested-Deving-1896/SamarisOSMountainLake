use crate::vram::tier::VramResidencyTier;
use crate::vram::residency::VramResidencyManager;
use crate::resources::resource_id::GpuResourceId;

pub enum EvictionTarget {
    Idle,
    Normal,
    Any,
}

pub struct VramEviction;

impl VramEviction {
    pub fn can_evict(tier: VramResidencyTier, pinned: bool, in_frame: bool) -> bool {
        if pinned {
            return false;
        }
        match tier {
            VramResidencyTier::T1Active => !in_frame,
            VramResidencyTier::T2Compressed => true,
            VramResidencyTier::T3Fallback => true,
        }
    }

    pub fn select_victim(
        manager: &VramResidencyManager,
        target: EvictionTarget,
    ) -> Option<GpuResourceId> {
        let mut candidates: Vec<(u64, VramResidencyTier)> = Vec::new();

        for entry in manager.tiers.iter() {
            let id_key = *entry.key();
            let tier = *entry.value();
            candidates.push((id_key, tier));
        }

        if candidates.is_empty() {
            return None;
        }

        // Sort: T3 first, then T2, then T1, within each tier by insertion order
        candidates.sort_by_key(|(_, tier)| match tier {
            VramResidencyTier::T3Fallback => 0,
            VramResidencyTier::T2Compressed => 1,
            VramResidencyTier::T1Active => 2,
        });

        match target {
            EvictionTarget::Idle => {
                candidates
                    .iter()
                    .find(|(_, t)| matches!(t, VramResidencyTier::T3Fallback | VramResidencyTier::T2Compressed))
                    .map(|(id, _)| GpuResourceId(uuid::Uuid::from_u128(*id as u128)))
            }
            EvictionTarget::Normal => {
                candidates
                    .iter()
                    .find(|(_, t)| {
                        matches!(t, VramResidencyTier::T3Fallback | VramResidencyTier::T2Compressed)
                    })
                    .or_else(|| {
                        candidates
                            .iter()
                            .find(|(_, t)| matches!(t, VramResidencyTier::T1Active))
                    })
                    .map(|(id, _)| GpuResourceId(uuid::Uuid::from_u128(*id as u128)))
            }
            EvictionTarget::Any => {
                candidates
                    .first()
                    .map(|(id, _)| GpuResourceId(uuid::Uuid::from_u128(*id as u128)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manager() -> VramResidencyManager {
        VramResidencyManager::new(64 * 1024)
    }

    #[test]
    fn test_can_evict_unpinned_t1_not_in_frame() {
        assert!(VramEviction::can_evict(VramResidencyTier::T1Active, false, false));
    }

    #[test]
    fn test_can_evict_unpinned_t1_in_frame() {
        assert!(!VramEviction::can_evict(VramResidencyTier::T1Active, false, true));
    }

    #[test]
    fn test_can_evict_pinned_never() {
        assert!(!VramEviction::can_evict(VramResidencyTier::T1Active, true, false));
        assert!(!VramEviction::can_evict(VramResidencyTier::T2Compressed, true, false));
    }

    #[test]
    fn test_can_evict_t2_always() {
        assert!(VramEviction::can_evict(VramResidencyTier::T2Compressed, false, true));
    }

    #[test]
    fn test_can_evict_t3_always() {
        assert!(VramEviction::can_evict(VramResidencyTier::T3Fallback, false, true));
    }

    #[test]
    fn test_select_victim_empty() {
        let manager = make_manager();
        assert!(VramEviction::select_victim(&manager, EvictionTarget::Any).is_none());
    }

    #[test]
    fn test_select_victim_idle_with_t3() {
        let manager = make_manager();
        let id = GpuResourceId::new();
        let key = id.0.as_u128() as u64;
        manager.tiers.insert(key, VramResidencyTier::T3Fallback);
        let victim = VramEviction::select_victim(&manager, EvictionTarget::Idle);
        assert!(victim.is_some());
    }
}
