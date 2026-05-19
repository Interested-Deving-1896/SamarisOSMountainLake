use dashmap::DashMap;
use sha2::{Sha256, Digest};
use crate::shaders::shader_id::ShaderId;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PipelineId(pub u64);

pub struct PipelineCache {
    entries: DashMap<u64, Vec<u8>>,
}

impl PipelineCache {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn insert(&self, id: PipelineId, data: Vec<u8>) {
        self.entries.insert(id.0, data);
    }

    pub fn get(&self, id: &PipelineId) -> Option<Vec<u8>> {
        self.entries.get(&id.0).map(|e| e.clone())
    }

    pub fn compile(&self, vertex: ShaderId, fragment: ShaderId) -> VgmResult<PipelineId> {
        let mut hasher = Sha256::new();
        hasher.update(vertex.0.as_bytes());
        hasher.update(fragment.0.as_bytes());
        let hash = hasher.finalize();
        let id = PipelineId(u64::from_ne_bytes(hash[..8].try_into().map_err(|_| {
            VgmError::PipelineCreationFailed("Failed to derive pipeline ID from shaders".into())
        })?));

        if self.entries.contains_key(&id.0) {
            return Ok(id);
        }

        let pipeline_data: Vec<u8> = hash.iter().copied().collect();
        self.entries.insert(id.0, pipeline_data);
        Ok(id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_shader_id(val: u128) -> ShaderId {
        ShaderId(uuid::Uuid::from_u128(val))
    }

    #[test]
    fn test_new_cache_is_empty() {
        let cache = PipelineCache::new();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_insert_and_get() {
        let cache = PipelineCache::new();
        let pid = PipelineId(42);
        cache.insert(pid, vec![1, 2, 3]);
        assert_eq!(cache.get(&pid), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_get_missing_returns_none() {
        let cache = PipelineCache::new();
        assert_eq!(cache.get(&PipelineId(99)), None);
    }

    #[test]
    fn test_compile_creates_pipeline() {
        let cache = PipelineCache::new();
        let vert = dummy_shader_id(100);
        let frag = dummy_shader_id(200);
        let pid = cache.compile(vert, frag).unwrap();
        assert!(cache.get(&pid).is_some());
    }

    #[test]
    fn test_compile_deduplicates() {
        let cache = PipelineCache::new();
        let vert = dummy_shader_id(100);
        let frag = dummy_shader_id(200);
        let a = cache.compile(vert, frag).unwrap();
        let b = cache.compile(vert, frag).unwrap();
        assert_eq!(a, b);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_different_shaders_different_pipelines() {
        let cache = PipelineCache::new();
        let a = cache.compile(dummy_shader_id(1), dummy_shader_id(2)).unwrap();
        let b = cache.compile(dummy_shader_id(3), dummy_shader_id(4)).unwrap();
        assert_ne!(a, b);
    }
}
