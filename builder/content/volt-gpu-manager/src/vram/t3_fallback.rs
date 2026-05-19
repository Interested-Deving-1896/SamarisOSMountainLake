use dashmap::DashMap;
use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

fn id_to_u64(id: GpuResourceId) -> u64 {
    let v = id.0.as_u128();
    (v ^ (v >> 64)) as u64
}

pub struct FallbackVramStore {
    entries: DashMap<u64, Vec<u8>>,
    pub used_bytes: u64,
    pub max_bytes: u64,
}

impl FallbackVramStore {
    pub fn new(max_bytes: u64) -> Self {
        Self {
            entries: DashMap::new(),
            used_bytes: 0,
            max_bytes,
        }
    }

    pub fn store(&mut self, id: GpuResourceId, data: Vec<u8>) -> VgmResult<()> {
        let key = id_to_u64(id);
        if self.entries.contains_key(&key) {
            return Err(crate::core::error::VgmError::ResourceAlreadyExists(format!(
                "Resource {} already in fallback store",
                id
            )));
        }
        let total = self.used_bytes + data.len() as u64;
        if total > self.max_bytes {
            return Err(crate::core::error::VgmError::VramAllocationFailed(format!(
                "Fallback store full: {}+{} > {}",
                self.used_bytes,
                data.len(),
                self.max_bytes
            )));
        }
        self.entries.insert(key, data);
        self.used_bytes = total;
        Ok(())
    }

    pub fn retrieve(&self, id: GpuResourceId) -> Option<Vec<u8>> {
        self.entries.get(&id_to_u64(id)).map(|r| r.clone())
    }

    pub fn remove(&mut self, id: GpuResourceId) {
        if let Some((_, data)) = self.entries.remove(&id_to_u64(id)) {
            self.used_bytes = self.used_bytes.saturating_sub(data.len() as u64);
        }
    }

    pub fn usage_pct(&self) -> f64 {
        if self.max_bytes == 0 {
            return 0.0;
        }
        (self.used_bytes as f64 / self.max_bytes as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store() {
        let store = FallbackVramStore::new(1024);
        assert_eq!(store.used_bytes, 0);
        assert_eq!(store.max_bytes, 1024);
    }

    #[test]
    fn test_store_and_retrieve() {
        let mut store = FallbackVramStore::new(1024);
        let id = GpuResourceId::new();
        let data = vec![0xAB, 0xCD, 0xEF];
        assert!(store.store(id, data.clone()).is_ok());
        assert_eq!(store.used_bytes, 3);
        let retrieved = store.retrieve(id).unwrap();
        assert_eq!(retrieved, data);
    }

    #[test]
    fn test_store_duplicate_fails() {
        let mut store = FallbackVramStore::new(1024);
        let id = GpuResourceId::new();
        store.store(id, vec![1, 2, 3]).unwrap();
        let result = store.store(id, vec![4, 5, 6]);
        assert!(result.is_err());
    }

    #[test]
    fn test_store_exhausts_capacity() {
        let mut store = FallbackVramStore::new(10);
        assert!(store.store(GpuResourceId::new(), vec![0; 10]).is_ok());
        let result = store.store(GpuResourceId::new(), vec![0; 1]);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_frees_bytes() {
        let mut store = FallbackVramStore::new(1024);
        let id = GpuResourceId::new();
        store.store(id, vec![0u8; 100]).unwrap();
        assert_eq!(store.used_bytes, 100);
        store.remove(id);
        assert_eq!(store.used_bytes, 0);
    }

    #[test]
    fn test_retrieve_nonexistent() {
        let store = FallbackVramStore::new(1024);
        assert!(store.retrieve(GpuResourceId::new()).is_none());
    }

    #[test]
    fn test_usage_pct() {
        let mut store = FallbackVramStore::new(1000);
        store.store(GpuResourceId::new(), vec![0u8; 250]).unwrap();
        assert!((store.usage_pct() - 25.0).abs() < 0.001);
    }

    #[test]
    fn test_remove_unknown_id() {
        let mut store = FallbackVramStore::new(1024);
        store.remove(GpuResourceId::new());
        assert_eq!(store.used_bytes, 0);
    }

    #[test]
    fn test_multiple_stores() {
        let mut store = FallbackVramStore::new(1024);
        let id1 = GpuResourceId::new();
        let id2 = GpuResourceId::new();
        store.store(id1, vec![0u8; 100]).unwrap();
        store.store(id2, vec![0u8; 200]).unwrap();
        assert_eq!(store.used_bytes, 300);
        store.remove(id1);
        assert_eq!(store.used_bytes, 200);
    }
}
