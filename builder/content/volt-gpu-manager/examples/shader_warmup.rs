use volt_gpu_manager::shaders::shader_cache::ShaderCache;
use volt_gpu_manager::shaders::shader_id::ShaderId;
use volt_gpu_manager::shaders::builtin::BuiltinShaderRegistry;
use volt_gpu_manager::shaders::compiler::{ShaderCompiler, ShaderSource};

fn main() {
    let cache = ShaderCache::new(128);

    println!("=== Shader Warmup ===");

    let builtins = BuiltinShaderRegistry::register_all();
    println!("Registered {} builtin shaders.", builtins.len());

    let critical = BuiltinShaderRegistry::critical_shaders();
    println!("Critical shaders to warmup:");

    for name in &critical {
        if let Some(data) = BuiltinShaderRegistry::get(name) {
            let sid = ShaderId::new();
            match cache.insert(sid, data.to_vec()) {
                Ok(()) => println!("  Warmed up: {}", name),
                Err(e) => eprintln!("  Failed to warmup {}: {}", name, e),
            }
        }
    }

    println!("Cache entries: {}", cache.entry_count());
    println!("Cache hits:    {}", cache.hit_count());
    println!("Cache misses:  {}", cache.miss_count());
    println!("Shader warmup complete.");
}
