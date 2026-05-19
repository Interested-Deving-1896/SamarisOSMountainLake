use volt_gpu_manager::shaders::shader_cache::ShaderCache;
use volt_gpu_manager::shaders::shader_id::ShaderId;
use volt_gpu_manager::shaders::builtin::BuiltinShaderRegistry;
use volt_gpu_manager::shaders::compiler::{ShaderCompiler, ShaderSource};

#[test]
fn builtin_shaders_registered() {
    let shaders = BuiltinShaderRegistry::register_all();
    assert_eq!(shaders.len(), 12);
    let names: Vec<_> = shaders.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"blur_gaussian"));
    assert!(names.contains(&"vram_compress"));
    assert!(names.contains(&"ai_matmul"));
}

#[test]
fn warmup_inserts_critical() {
    let critical = BuiltinShaderRegistry::critical_shaders();
    assert!(critical.contains(&"transform_2d"));
    assert!(critical.contains(&"composite_alpha"));
    assert!(critical.contains(&"text_glyph_vert"));
    assert!(critical.contains(&"text_glyph_frag"));
    assert!(!critical.is_empty());
}

#[test]
fn cache_hit_counted() {
    let cache = ShaderCache::new(64);
    let id = ShaderId::new();
    cache.insert(id, vec![1, 2, 3]).unwrap();
    let _ = cache.get(&id);
    assert_eq!(cache.hit_count(), 1);
    assert_eq!(cache.miss_count(), 0);
}

#[test]
fn cache_miss_counted() {
    let cache = ShaderCache::new(64);
    let id = ShaderId::new();
    let _ = cache.get(&id);
    assert_eq!(cache.miss_count(), 1);
    assert_eq!(cache.hit_count(), 0);
}

#[test]
fn lru_eviction() {
    let cache = ShaderCache::new(1);
    let id = ShaderId::new();
    let big = vec![0u8; 2 * 1024 * 1024];
    let result = cache.insert(id, big);
    assert!(result.is_err());
}

#[test]
fn compile_failure_returns_error() {
    let cache = ShaderCache::new(64);
    let source = ShaderSource::Glsl("invalid glsl".into());
    let result = ShaderCompiler::compile(&source, &cache);
    assert!(result.is_err());
}
