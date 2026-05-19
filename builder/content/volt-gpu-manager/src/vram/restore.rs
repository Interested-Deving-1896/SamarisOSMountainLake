use crate::vram::residency::VramResidencyManager;
use crate::vram::tier::VramResidencyTier;
use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

pub fn compress_resource(
    manager: &mut VramResidencyManager,
    id: GpuResourceId,
) -> VgmResult<()> {
    let tier = manager.tier_of(id);
    match tier {
        Some(VramResidencyTier::T1Active) => manager.compress_to_t2(id),
        Some(_) => Err(crate::core::error::VgmError::ResourceNotCompressible(format!(
            "Resource {} is not in T1Active; cannot compress",
            id
        ))),
        None => Err(crate::core::error::VgmError::ResourceNotFound(id.to_string())),
    }
}

pub fn restore_resource(
    manager: &mut VramResidencyManager,
    id: GpuResourceId,
) -> VgmResult<()> {
    let tier = manager.tier_of(id);
    match tier {
        Some(VramResidencyTier::T2Compressed) => manager.restore_to_t1(id),
        Some(_) => Err(crate::core::error::VgmError::ResourceNotRestorable(format!(
            "Resource {} is not in T2Compressed; cannot restore",
            id
        ))),
        None => Err(crate::core::error::VgmError::ResourceNotFound(id.to_string())),
    }
}

pub fn evict_resource(
    manager: &mut VramResidencyManager,
    id: GpuResourceId,
) -> VgmResult<()> {
    let tier = manager.tier_of(id);
    match tier {
        Some(VramResidencyTier::T1Active) | Some(VramResidencyTier::T2Compressed) => {
            manager.evict_to_t3(id)
        }
        Some(VramResidencyTier::T3Fallback) => {
            Err(crate::core::error::VgmError::ResourceNotRestorable(format!(
                "Resource {} is already in T3Fallback",
                id
            )))
        }
        None => Err(crate::core::error::VgmError::ResourceNotFound(id.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vram::residency::VramResidencyManager;

    fn setup_id(manager: &mut VramResidencyManager) -> GpuResourceId {
        let id = GpuResourceId::new();
        manager.allocate_t1(id, 1024, 1).unwrap();
        id
    }

    #[test]
    fn test_compress_resource_success() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = setup_id(&mut mgr);
        assert!(compress_resource(&mut mgr, id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T2Compressed));
    }

    #[test]
    fn test_compress_resource_not_in_t1() {
        let mut mgr = VramResidencyManager::new(1024);
        let result = compress_resource(&mut mgr, GpuResourceId::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_restore_resource_success() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = setup_id(&mut mgr);
        compress_resource(&mut mgr, id).unwrap();
        assert!(restore_resource(&mut mgr, id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
    }

    #[test]
    fn test_restore_resource_not_compressible() {
        let mut mgr = VramResidencyManager::new(1024);
        let result = restore_resource(&mut mgr, GpuResourceId::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_evict_resource_from_t1() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = setup_id(&mut mgr);
        assert!(evict_resource(&mut mgr, id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
    }

    #[test]
    fn test_evict_resource_from_t2() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = setup_id(&mut mgr);
        compress_resource(&mut mgr, id).unwrap();
        assert!(evict_resource(&mut mgr, id).is_ok());
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
    }

    #[test]
    fn test_evict_resource_already_t3() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = setup_id(&mut mgr);
        evict_resource(&mut mgr, id).unwrap();
        let result = evict_resource(&mut mgr, id);
        assert!(result.is_err());
    }

    #[test]
    fn test_full_cycle_via_functions() {
        let mut mgr = VramResidencyManager::new(64 * 1024);
        let id = GpuResourceId::new();
        mgr.allocate_t1(id, 4096, 1).unwrap();
        compress_resource(&mut mgr, id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T2Compressed));
        restore_resource(&mut mgr, id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
        evict_resource(&mut mgr, id).unwrap();
        assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
    }
}
