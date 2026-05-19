use std::sync::atomic::{AtomicU64, Ordering};
use dashmap::DashMap;
use crate::shaders::shader_id::ShaderId;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct ShaderCache {
    entries: DashMap<u64, Vec<u8>>,
    hits: AtomicU64,
    misses: AtomicU64,
    max_bytes: u64,
    used_bytes: AtomicU64,
}

impl ShaderCache {
    pub fn new(max_mb: u64) -> Self {
        Self {
            entries: DashMap::new(),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            max_bytes: max_mb * 1024 * 1024,
            used_bytes: AtomicU64::new(0),
        }
    }

    pub fn insert(&self, id: ShaderId, data: Vec<u8>) -> VgmResult<()> {
        let key = id.0.as_u128() as u64;
        let size = data.len() as u64;

        if size > self.max_bytes {
            return Err(VgmError::VramQuotaExceeded(
                "Shader data exceeds cache size limit".into(),
            ));
        }

        let prev = self.entries.get(&key).map(|e| e.len() as u64).unwrap_or(0);
        let new_used = self.used_bytes.load(Ordering::Relaxed) + size - prev;

        if new_used > self.max_bytes {
            self.entries.clear();
            self.used_bytes.store(size, Ordering::Relaxed);
        } else {
            self.used_bytes.store(new_used, Ordering::Relaxed);
        }

        self.entries.insert(key, data);
        Ok(())
    }

    pub fn get(&self, id: &ShaderId) -> Option<Vec<u8>> {
        let key = id.0.as_u128() as u64;
        if let Some(data) = self.entries.get(&key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(data.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    pub fn hit_count(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }

    pub fn miss_count(&self) -> u64 {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let cache = ShaderCache::new(64);
        let id = ShaderId::new();
        assert!(cache.insert(id, vec![1, 2, 3]).is_ok());
        assert_eq!(cache.get(&id), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_miss_returns_none() {
        let cache = ShaderCache::new(64);
        let id = ShaderId::new();
        assert_eq!(cache.get(&id), None);
    }

    #[test]
    fn test_hit_and_miss_counts() {
        let cache = ShaderCache::new(64);
        let id = ShaderId::new();
        assert_eq!(cache.get(&id), None);
        assert_eq!(cache.miss_count(), 1);
        assert_eq!(cache.hit_count(), 0);
        cache.insert(id, vec![42]).unwrap();
        assert_eq!(cache.get(&id), Some(vec![42]));
        assert_eq!(cache.hit_count(), 1);
        assert_eq!(cache.miss_count(), 1);
    }

    #[test]
    fn test_entry_count() {
        let cache = ShaderCache::new(64);
        assert_eq!(cache.entry_count(), 0);
        cache.insert(ShaderId::new(), vec![1]).unwrap();
        assert_eq!(cache.entry_count(), 1);
        cache.insert(ShaderId::new(), vec![2]).unwrap();
        assert_eq!(cache.entry_count(), 2);
    }

    #[test]
    fn test_eviction_when_full() {
        let cache = ShaderCache::new(1);
        let id1 = ShaderId::new();
        let big_data = vec![0u8; 2 * 1024 * 1024];
        assert!(cache.insert(id1, big_data).is_err());
    }

    #[test]
    fn test_overwrite_same_id() {
        let cache = ShaderCache::new(64);
        let id = ShaderId::new();
        cache.insert(id, vec![1]).unwrap();
        cache.insert(id, vec![2]).unwrap();
        assert_eq!(cache.get(&id), Some(vec![2]));
        assert_eq!(cache.entry_count(), 1);
    }
}
