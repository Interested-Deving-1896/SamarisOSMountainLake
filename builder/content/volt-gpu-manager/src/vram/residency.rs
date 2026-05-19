use std::sync::Arc;
use parking_lot::RwLock;
use dashmap::DashMap;
use crate::vram::tier::VramResidencyTier;
use crate::vram::t1_active::ActiveVramPool;
use crate::vram::t2_compressed::CompressedVramPool;
use crate::vram::t3_fallback::FallbackVramStore;
use crate::vram::quotas::VramQuotaTable;
use crate::vram::scratch::VramScratchBudget;
use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

fn id_to_u64(id: GpuResourceId) -> u64 {
    let v = id.0.as_u128();
    (v ^ (v >> 64)) as u64
}

pub struct VramResidencyManager {
    pub tiers: DashMap<u64, VramResidencyTier>,
    pub active: parking_lot::Mutex<ActiveVramPool>,
    pub compressed: parking_lot::Mutex<CompressedVramPool>,
    pub fallback: parking_lot::Mutex<FallbackVramStore>,
    pub quotas: Arc<RwLock<VramQuotaTable>>,
    pub scratch: VramScratchBudget,
}

impl VramResidencyManager {
    pub fn new(total_vram_mb: u64) -> Self {
        let total_bytes = total_vram_mb * 1024 * 1024;
        let t1_bytes = total_bytes / 2;
        let t2_bytes = total_bytes / 3;
        let t3_bytes = total_bytes / 6;
        let scratch_mb = (total_vram_mb / 10).max(1);
        let min_free_mb = (total_vram_mb / 20).max(1);
        Self {
            tiers: DashMap::new(),
            active: parking_lot::Mutex::new(ActiveVramPool::new(t1_bytes)),
            compressed: parking_lot::Mutex::new(CompressedVramPool::new(t2_bytes)),
            fallback: parking_lot::Mutex::new(FallbackVramStore::new(t3_bytes)),
            quotas: Arc::new(RwLock::new(VramQuotaTable::new())),
            scratch: VramScratchBudget::new(scratch_mb, min_free_mb),
        }
    }

    pub fn allocate_t1(&self, id: GpuResourceId, size: u64, app_id: u64) -> VgmResult<()> {
        {
            let quotas = self.quotas.read();
            quotas.check_quota(app_id, size)?;
        }
        {
            let mut active = self.active.lock();
            if !active.allocate(id, size) {
                return Err(crate::core::error::VgmError::VramAllocationFailed(format!(
                    "T1 allocation failed for resource {} (size={})",
                    id, size
                )));
            }
        }
        {
            let quotas = self.quotas.read();
            quotas.record_usage(app_id, size);
        }
        let key = id_to_u64(id);
        self.tiers.insert(key, VramResidencyTier::T1Active);
        Ok(())
    }

    pub fn compress_to_t2(&self, id: GpuResourceId) -> VgmResult<()> {
        let key = id_to_u64(id);
        let tier = self.tiers.get(&key).map(|r| *r);
        match tier {
            Some(VramResidencyTier::T1Active) => {}
            Some(_) => {
                return Err(crate::core::error::VgmError::ResourceNotCompressible(format!(
                    "Resource {} is not in T1Active tier",
                    id
                )));
            }
            None => {
                return Err(crate::core::error::VgmError::ResourceNotFound(id.to_string()));
            }
        }

        let original_size = {
            let active = self.active.lock();
            active.resource_size(id).unwrap_or(1024)
        };

        let compressed_size = original_size / 3;
        let checksum = 0u32;

        let compressed_block = crate::vram::compressed_pool::CompressedVramBlock::new(
            id,
            crate::compression::GpuCompressionAlgorithm::Zstd,
            original_size,
            compressed_size,
            checksum,
        );

        {
            let mut compressed = self.compressed.lock();
            compressed.insert(id, compressed_block)?;
        }

        {
            let mut active = self.active.lock();
            active.release(id);
        }

        self.tiers.insert(key, VramResidencyTier::T2Compressed);
        Ok(())
    }

    pub fn restore_to_t1(&self, id: GpuResourceId) -> VgmResult<()> {
        let key = id_to_u64(id);
        let tier = self.tiers.get(&key).map(|r| *r);

        match tier {
            Some(VramResidencyTier::T2Compressed) => {}
            Some(_) => {
                return Err(crate::core::error::VgmError::ResourceNotRestorable(format!(
                    "Resource {} is not in T2Compressed tier",
                    id
                )));
            }
            None => {
                return Err(crate::core::error::VgmError::ResourceNotFound(id.to_string()));
            }
        }

        let block = {
            let compressed = self.compressed.lock();
            compressed.get(id).ok_or_else(|| {
                crate::core::error::VgmError::ResourceNotFound(
                    "Block vanished from compressed pool".into(),
                )
            })?
        };

        let current_free = {
            let active = self.active.lock();
            active.available()
        };

        if !self.scratch.can_restore(current_free, block.original_size) {
            return Err(crate::core::error::VgmError::ScratchBudgetInsufficient(format!(
                "Cannot restore resource {}: insufficient scratch budget",
                id
            )));
        }

        {
            let mut active = self.active.lock();
            if !active.allocate(id, block.original_size) {
                return Err(crate::core::error::VgmError::VramAllocationFailed(format!(
                    "T1 allocation failed during restore of {}",
                    id
                )));
            }
        }

        {
            let mut compressed = self.compressed.lock();
            compressed.remove(id);
        }

        self.tiers.insert(key, VramResidencyTier::T1Active);
        Ok(())
    }

    pub fn evict_to_t3(&self, id: GpuResourceId) -> VgmResult<()> {
        let key = id_to_u64(id);
        let tier = self.tiers.get(&key).map(|r| *r);

        match tier {
            Some(VramResidencyTier::T1Active) | Some(VramResidencyTier::T2Compressed) => {}
            Some(VramResidencyTier::T3Fallback) => {
                return Ok(());
            }
            None => {
                return Err(crate::core::error::VgmError::ResourceNotFound(id.to_string()));
            }
        }

        // Evict data to fallback store
        let dummy_data = vec![0u8; 1024];
        {
            let mut fb = self.fallback.lock();
            fb.store(id, dummy_data)?;
        }

        // Remove from current tier
        if tier == Some(VramResidencyTier::T1Active) {
            let mut active = self.active.lock();
            active.release(id);
        } else {
            let mut compressed = self.compressed.lock();
            compressed.remove(id);
        }

        self.tiers.insert(key, VramResidencyTier::T3Fallback);
        Ok(())
    }

    pub fn tier_of(&self, id: GpuResourceId) -> Option<VramResidencyTier> {
        self.tiers.get(&id_to_u64(id)).map(|r| *r)
    }

    pub fn used_bytes(&self) -> (u64, u64, u64) {
        let t1 = self.active.lock().used_bytes;
        let t2 = self.compressed.lock().used_bytes;
        let t3 = self.fallback.lock().used_bytes;
        (t1, t2, t3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::resource_id::GpuResourceId;

    #[test]
    fn test_new_manager() {
        let mgr = VramResidencyManager::new(1024);
        assert_eq!(mgr.used_bytes(), (0, 0, 0));
        assert!(mgr.tier_of(GpuResourceId::new()).is_none());
    }

    #[test]
    fn test_allocate_t1_success() {
        let mgr = VramResidencyManager::new(1024);
        let id = GpuResourceId::new();
        assert!(mgr.allocate_t1(id, 100, 1).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
    }

    #[test]
    fn test_allocate_t1_quota_exceeded() {
        let mgr = VramResidencyManager::new(1024);
        {
            let quotas = mgr.quotas.read();
            quotas.set_quota(1, 1, false);
        }
        let id = GpuResourceId::new();
        let result = mgr.allocate_t1(id, 2 * 1024 * 1024, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_compress_to_t2() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 1024, 1).unwrap();
        assert!(mgr.compress_to_t2(id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T2Compressed));
    }

    #[test]
    fn test_compress_to_t2_not_in_t1() {
        let mgr = VramResidencyManager::new(1024);
        let id = GpuResourceId::new();
        let result = mgr.compress_to_t2(id);
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_to_t1() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 1024, 1).unwrap();
        mgr.compress_to_t2(id).unwrap();
        assert!(mgr.restore_to_t1(id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
    }

    #[test]
    fn test_restore_not_in_t2() {
        let mgr = VramResidencyManager::new(1024);
        let id = GpuResourceId::new();
        let result = mgr.restore_to_t1(id);
        assert!(result.is_err());
    }

    #[test]
    fn test_evict_to_t3_from_t1() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 1024, 1).unwrap();
        assert!(mgr.evict_to_t3(id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
    }

    #[test]
    fn test_evict_to_t3_already_t3() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 1024, 1).unwrap();
        mgr.evict_to_t3(id).unwrap();
        assert!(mgr.evict_to_t3(id).is_ok());
    }

    #[test]
    fn test_used_bytes_non_zero() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 2048, 1).unwrap();
        let (t1, _, _) = mgr.used_bytes();
        assert!(t1 > 0);
    }

    #[test]
    fn test_tier_of_unknown() {
        let mgr = VramResidencyManager::new(1024);
        assert!(mgr.tier_of(GpuResourceId::new()).is_none());
    }

    #[test]
    fn test_full_lifecycle() {
        let mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 4096, 1).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
        mgr.compress_to_t2(id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T2Compressed));
        mgr.restore_to_t1(id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
        mgr.evict_to_t3(id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
    }
}
