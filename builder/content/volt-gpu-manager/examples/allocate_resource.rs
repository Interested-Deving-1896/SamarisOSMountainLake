use volt_gpu_manager::config::schema::VgmConfig;
use volt_gpu_manager::resources::{ResourceTable, GpuResourceMeta, GpuResourceId, GpuResourceType, GpuResourceUsage};
use volt_gpu_manager::vram::VramResidencyManager;

fn main() {
    let _config = VgmConfig::default();
    let table = ResourceTable::new();
    let vram = VramResidencyManager::new(1024);

    let id = GpuResourceId::new();
    let meta = GpuResourceMeta::new(
        id,
        42,
        "my_texture",
        GpuResourceType::Texture2D,
        GpuResourceUsage::Background,
        65536,
    );

    match table.register(meta) {
        Ok(()) => println!("Resource registered in table."),
        Err(e) => {
            eprintln!("Failed to register resource: {}", e);
            return;
        }
    }

    match vram.allocate_t1(id, 65536, 42) {
        Ok(()) => println!("Allocated {} bytes in T1 active VRAM.", 65536),
        Err(e) => {
            eprintln!("Failed to allocate VRAM: {}", e);
            return;
        }
    }

    println!("Resource ID:  {}", id);
    println!("Tier:         {:?}", vram.tier_of(id));
    println!("Table count:  {}", table.count());

    vram.compress_to_t2(id).unwrap();
    println!("Compressed to T2: {:?}", vram.tier_of(id));
}
