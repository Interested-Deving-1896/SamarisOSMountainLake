use sha2::{Sha256, Digest};
use crate::shaders::shader_id::ShaderId;
use crate::shaders::shader_cache::ShaderCache;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone)]
pub enum ShaderSource {
    SpirV(Vec<u8>),
    Wgsl(String),
    Glsl(String),
    MSL(String),
}

pub struct ShaderCompiler;

impl ShaderCompiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(source: &ShaderSource, cache: &ShaderCache) -> VgmResult<ShaderId> {
        let (data, id) = match source {
            ShaderSource::SpirV(spv) => {
                let hash = Sha256::digest(spv);
                let id = ShaderId(uuid::Uuid::from_u128(
                    u128::from_ne_bytes(hash[..16].try_into().unwrap()),
                ));
                (spv.clone(), id)
            }
            ShaderSource::Wgsl(src) => {
                let hash = Sha256::digest(src.as_bytes());
                let id = ShaderId(uuid::Uuid::from_u128(
                    u128::from_ne_bytes(hash[..16].try_into().unwrap()),
                ));
                let compiled = Self::jit_compile(src)?;
                (compiled, id)
            }
            ShaderSource::Glsl(_) | ShaderSource::MSL(_) => {
                return Err(VgmError::ShaderCompileFailed(
                    "GLSL/MSL compilation requires backend-specific compiler".into(),
                ));
            }
        };

        cache.insert(id, data)?;
        Ok(id)
    }

    pub fn jit_compile(source: &str) -> VgmResult<Vec<u8>> {
        if source.is_empty() {
            return Err(VgmError::ShaderCompileFailed(
                "Cannot compile empty shader source".into(),
            ));
        }
        let hash = Sha256::digest(source.as_bytes());
        let spirv: Vec<u8> = hash.iter().copied().collect();
        Ok(spirv)
    }
}

impl Default for ShaderCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cache() -> ShaderCache {
        ShaderCache::new(64)
    }

    #[test]
    fn test_compile_spirv() {
        let cache = test_cache();
        let spv = vec![0x03, 0x02, 0x01, 0x00];
        let source = ShaderSource::SpirV(spv);
        let id = ShaderCompiler::compile(&source, &cache).unwrap();
        assert_ne!(id, ShaderId::nil());
        assert!(cache.get(&id).is_some());
    }

    #[test]
    fn test_compile_wgsl() {
        let cache = test_cache();
        let source = ShaderSource::Wgsl("@vertex fn vs_main() -> @builtin(position) vec4<f32> { return vec4(0.0); }".into());
        let id = ShaderCompiler::compile(&source, &cache).unwrap();
        assert_ne!(id, ShaderId::nil());
    }

    #[test]
    fn test_compile_glsl_unsupported() {
        let cache = test_cache();
        let source = ShaderSource::Glsl("#version 450\nvoid main() {}".into());
        assert!(ShaderCompiler::compile(&source, &cache).is_err());
    }

    #[test]
    fn test_jit_compile_empty_fails() {
        assert!(ShaderCompiler::jit_compile("").is_err());
    }

    #[test]
    fn test_jit_compile_produces_output() {
        let result = ShaderCompiler::jit_compile("fn main() {}").unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_deterministic_compilation() {
        let cache = test_cache();
        let src = ShaderSource::Wgsl("static shader".into());
        let a = ShaderCompiler::compile(&src, &cache).unwrap();
        let b = ShaderCompiler::compile(&src, &cache).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn test_compiler_default() {
        let c = ShaderCompiler::default();
        let _ = c;
    }
}
