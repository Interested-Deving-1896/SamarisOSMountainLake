use volt_gpu_manager::vram::VramResidencyManager;
use volt_gpu_manager::resources::GpuResourceId;
use volt_gpu_manager::vram::VramResidencyTier;

fn main() {
    let vram = VramResidencyManager::new(64 * 1024);

    let id = GpuResourceId::new();
    println!("Created resource: {}", id);

    vram.allocate_t1(id, 4096, 1).expect("T1 allocation");
    println!("Tier: {:?}", vram.tier_of(id));

    vram.compress_to_t2(id).expect("Compress to T2");
    println!("Compressed tier: {:?}", vram.tier_of(id));
    assert_eq!(vram.tier_of(id), Some(VramResidencyTier::T2Compressed));

    vram.restore_to_t1(id).expect("Restore to T1");
    println!("Restored tier: {:?}", vram.tier_of(id));
    assert_eq!(vram.tier_of(id), Some(VramResidencyTier::T1Active));

    vram.evict_to_t3(id).expect("Evict to T3");
    println!("Evicted tier: {:?}", vram.tier_of(id));
    assert_eq!(vram.tier_of(id), Some(VramResidencyTier::T3Fallback));

    let (t1, t2, t3) = vram.used_bytes();
    println!("VRAM usage — T1: {}  T2: {}  T3: {}", t1, t2, t3);
    println!("Compress/restore cycle completed successfully.");
}
