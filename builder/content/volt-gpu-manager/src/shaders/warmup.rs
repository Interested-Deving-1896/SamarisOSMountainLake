use crate::shaders::shader_cache::ShaderCache;
use crate::shaders::builtin::BuiltinShaderRegistry;
use crate::shaders::shader_id::ShaderId;
use crate::shaders::compiler::{ShaderCompiler, ShaderSource};
use crate::core::result::VgmResult;

pub struct ShaderWarmup {
    count: usize,
}

impl ShaderWarmup {
    pub fn warmup_critical(cache: &ShaderCache) -> VgmResult<Vec<ShaderId>> {
        let mut ids = Vec::new();
        for name in BuiltinShaderRegistry::critical_shaders() {
            if let Some(data) = BuiltinShaderRegistry::get(name) {
                let source = ShaderSource::SpirV(data.to_vec());
                let id = ShaderCompiler::compile(&source, cache)?;
                ids.push(id);
            }
        }
        Ok(ids)
    }

    pub fn warmup_all(cache: &ShaderCache) -> VgmResult<Vec<ShaderId>> {
        let mut ids = Vec::new();
        for (_name, data) in BuiltinShaderRegistry::register_all() {
            let source = ShaderSource::SpirV(data);
            let id = ShaderCompiler::compile(&source, cache)?;
            ids.push(id);
        }
        Ok(ids)
    }

    pub fn warmup_count(&self) -> usize {
        self.count
    }
}

impl Default for ShaderWarmup {
    fn default() -> Self {
        Self { count: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cache() -> ShaderCache {
        ShaderCache::new(64)
    }

    #[test]
    fn test_warmup_critical_returns_ids() {
        let cache = test_cache();
        let ids = ShaderWarmup::warmup_critical(&cache).unwrap();
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_warmup_all_returns_ids() {
        let cache = test_cache();
        let ids = ShaderWarmup::warmup_all(&cache).unwrap();
        assert!(!ids.is_empty());
        assert!(ids.len() >= ShaderWarmup::warmup_critical(&ShaderCache::new(64)).unwrap().len());
    }

    #[test]
    fn test_warmup_count_default() {
        let w = ShaderWarmup::default();
        assert_eq!(w.warmup_count(), 0);
    }

    #[test]
    fn test_warmup_populates_cache() {
        let cache = test_cache();
        let ids = ShaderWarmup::warmup_critical(&cache).unwrap();
        for id in &ids {
            assert!(cache.get(id).is_some());
        }
    }
}
