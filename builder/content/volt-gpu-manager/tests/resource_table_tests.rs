use volt_gpu_manager::resources::{ResourceTable, GpuResourceMeta, GpuResourceId, GpuResourceType, GpuResourceUsage};
use volt_gpu_manager::vram::VramResidencyTier;

fn make_meta(id: GpuResourceId, usage: GpuResourceUsage) -> GpuResourceMeta {
    GpuResourceMeta::new(id, 1, "test", GpuResourceType::Buffer, usage, 1024)
}

#[test]
fn create_resource_meta() {
    let id = GpuResourceId::new();
    let meta = GpuResourceMeta::new(id, 42, "my_resource", GpuResourceType::Texture2D, GpuResourceUsage::Background, 4096);
    assert_eq!(meta.resource_id, id);
    assert_eq!(meta.app_id, 42);
    assert_eq!(meta.name, "my_resource");
    assert_eq!(meta.original_size, 4096);
}

#[test]
fn lookup() {
    let table = ResourceTable::new();
    let id = GpuResourceId::new();
    table.register(make_meta(id, GpuResourceUsage::Cache)).unwrap();
    assert!(table.exists(id));
    let meta = table.get(id).unwrap();
    assert_eq!(meta.resource_id, id);
}

#[test]
fn update_tier() {
    let table = ResourceTable::new();
    let id = GpuResourceId::new();
    table.register(make_meta(id, GpuResourceUsage::Cache)).unwrap();
    table.update_tier(id, VramResidencyTier::T2Compressed).unwrap();
    assert_eq!(table.get(id).unwrap().tier, VramResidencyTier::T2Compressed);
}

#[test]
fn pinned_cant_compress() {
    let id = GpuResourceId::new();
    let mut meta = make_meta(id, GpuResourceUsage::Background);
    meta.pinned = true;
    meta.compression_allowed = false;
    assert!(!meta.compression_allowed);
}

#[test]
fn current_frame_resource_cant_compress() {
    let id = GpuResourceId::new();
    let meta = GpuResourceMeta::new(id, 1, "frame", GpuResourceType::Texture2D, GpuResourceUsage::DesktopFrame, 1024);
    assert!(!meta.compression_allowed);
}
