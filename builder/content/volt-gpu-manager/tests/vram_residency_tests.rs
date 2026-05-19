use volt_gpu_manager::vram::{VramResidencyManager, VramResidencyTier};
use volt_gpu_manager::resources::GpuResourceId;

#[test]
fn t1_resource_created() {
    let mgr = VramResidencyManager::new(64 * 1024);
    let id = GpuResourceId::new();
    mgr.allocate_t1(id, 1024, 1).unwrap();
    assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
}

#[test]
fn t1_to_t2_changes_tier() {
    let mgr = VramResidencyManager::new(64 * 1024);
    let id = GpuResourceId::new();
    mgr.allocate_t1(id, 1024, 1).unwrap();
    mgr.compress_to_t2(id).unwrap();
    assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T2Compressed));
}

#[test]
fn t2_stays_counted_as_vram() {
    let mgr = VramResidencyManager::new(64 * 1024);
    let id = GpuResourceId::new();
    mgr.allocate_t1(id, 1024, 1).unwrap();
    mgr.compress_to_t2(id).unwrap();
    let (_, t2, _) = mgr.used_bytes();
    assert!(t2 > 0);
}

#[test]
fn t2_not_bindable() {
    assert!(!VramResidencyTier::T2Compressed.is_bindable());
}

#[test]
fn t2_to_t1_restore_works() {
    let mgr = VramResidencyManager::new(64 * 1024);
    let id = GpuResourceId::new();
    mgr.allocate_t1(id, 4096, 1).unwrap();
    mgr.compress_to_t2(id).unwrap();
    mgr.restore_to_t1(id).unwrap();
    assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T1Active));
}

#[test]
fn t3_used_only_as_fallback() {
    let mgr = VramResidencyManager::new(64 * 1024);
    let id = GpuResourceId::new();
    mgr.allocate_t1(id, 4096, 1).unwrap();
    mgr.evict_to_t3(id).unwrap();
    assert_eq!(mgr.tier_of(id), Some(VramResidencyTier::T3Fallback));
}

#[test]
fn desktop_never_compressible() {
    use volt_gpu_manager::resources::GpuResourceUsage;
    assert!(!GpuResourceUsage::DesktopFrame.can_compress());
}

#[test]
fn t2_requires_restore() {
    assert!(VramResidencyTier::T2Compressed.requires_restore());
    assert!(VramResidencyTier::T3Fallback.requires_restore());
    assert!(!VramResidencyTier::T1Active.requires_restore());
}
