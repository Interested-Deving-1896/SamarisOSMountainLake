use std::collections::HashMap;
use once_cell::sync::Lazy;

// Placeholder SPIR-V binary stubs — arbitrary non-empty byte sequences for test/warmup purposes.
fn stub(name: &str) -> Vec<u8> {
    let mut data = Vec::with_capacity(64);
    data.extend_from_slice(name.as_bytes());
    data.resize(64, 0);
    data
}

static BUILTIN_SHADERS: Lazy<HashMap<&'static str, Vec<u8>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("blur_gaussian", stub("blur_gaussian"));
    m.insert("shadow_box", stub("shadow_box"));
    m.insert("gradient_linear", stub("gradient_linear"));
    m.insert("transform_2d", stub("transform_2d"));
    m.insert("composite_alpha", stub("composite_alpha"));
    m.insert("border_radius_vert", stub("border_radius_vert"));
    m.insert("border_radius_frag", stub("border_radius_frag"));
    m.insert("text_glyph_vert", stub("text_glyph_vert"));
    m.insert("text_glyph_frag", stub("text_glyph_frag"));
    m.insert("ai_matmul", stub("ai_matmul"));
    m.insert("vram_compress", stub("vram_compress"));
    m.insert("vram_decompress", stub("vram_decompress"));
    m
});

pub struct BuiltinShaderRegistry;

impl BuiltinShaderRegistry {
    pub fn register_all() -> Vec<(String, Vec<u8>)> {
        BUILTIN_SHADERS
            .iter()
            .map(|(name, data)| (name.to_string(), data.clone()))
            .collect()
    }

    pub fn get(name: &str) -> Option<&'static [u8]> {
        BUILTIN_SHADERS.get(name).map(|v| v.as_slice())
    }

    pub fn critical_shaders() -> Vec<&'static str> {
        vec![
            "transform_2d",
            "composite_alpha",
            "text_glyph_vert",
            "text_glyph_frag",
            "border_radius_vert",
            "border_radius_frag",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_contains_expected() {
        let shaders = BuiltinShaderRegistry::register_all();
        let names: Vec<_> = shaders.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"blur_gaussian"));
        assert!(names.contains(&"shadow_box"));
        assert!(names.contains(&"ai_matmul"));
        assert!(names.contains(&"vram_compress"));
        assert!(names.contains(&"vram_decompress"));
    }

    #[test]
    fn test_get_known_shader() {
        let data = BuiltinShaderRegistry::get("blur_gaussian");
        assert!(data.is_some());
        assert!(!data.unwrap().is_empty());
    }

    #[test]
    fn test_get_unknown_shader() {
        assert!(BuiltinShaderRegistry::get("nonexistent").is_none());
    }

    #[test]
    fn test_critical_shaders_not_empty() {
        let critical = BuiltinShaderRegistry::critical_shaders();
        assert!(!critical.is_empty());
        assert!(critical.contains(&"transform_2d"));
        assert!(critical.contains(&"composite_alpha"));
    }

    #[test]
    fn test_all_shaders_have_non_empty_data() {
        let shaders = BuiltinShaderRegistry::register_all();
        for (_name, data) in &shaders {
            assert!(!data.is_empty());
        }
    }

    #[test]
    fn test_register_all_count() {
        let shaders = BuiltinShaderRegistry::register_all();
        assert_eq!(shaders.len(), 12);
    }
}
