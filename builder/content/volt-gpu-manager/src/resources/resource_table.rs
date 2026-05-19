use dashmap::DashMap;
use crate::resources::resource_id::GpuResourceId;
use crate::resources::resource_meta::GpuResourceMeta;
use crate::vram::tier::VramResidencyTier;
use crate::core::result::VgmResult;

fn id_to_u64(id: GpuResourceId) -> u64 {
    let v = id.0.as_u128();
    (v ^ (v >> 64)) as u64
}

pub struct ResourceTable {
    entries: DashMap<u64, GpuResourceMeta>,
}

impl ResourceTable {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn register(&self, meta: GpuResourceMeta) -> VgmResult<()> {
        let key = id_to_u64(meta.resource_id);
        if self.entries.contains_key(&key) {
            return Err(crate::core::error::VgmError::ResourceAlreadyExists(format!(
                "Resource {} already registered",
                meta.resource_id
            )));
        }
        self.entries.insert(key, meta);
        Ok(())
    }

    pub fn unregister(&self, id: GpuResourceId) -> VgmResult<()> {
        let key = id_to_u64(id);
        self.entries
            .remove(&key)
            .ok_or_else(|| {
                crate::core::error::VgmError::ResourceNotFound(id.to_string())
            })?;
        Ok(())
    }

    pub fn get(&self, id: GpuResourceId) -> Option<GpuResourceMeta> {
        self.entries.get(&id_to_u64(id)).map(|r| r.clone())
    }

    pub fn exists(&self, id: GpuResourceId) -> bool {
        self.entries.contains_key(&id_to_u64(id))
    }

    pub fn update_tier(&self, id: GpuResourceId, tier: VramResidencyTier) -> VgmResult<()> {
        let key = id_to_u64(id);
        let mut entry = self.entries.get_mut(&key).ok_or_else(|| {
            crate::core::error::VgmError::ResourceNotFound(id.to_string())
        })?;
        entry.tier = tier;
        Ok(())
    }

    pub fn update_access(&self, id: GpuResourceId) {
        let key = id_to_u64(id);
        if let Some(mut entry) = self.entries.get_mut(&key) {
            use std::time::{SystemTime, UNIX_EPOCH};
            entry.last_access_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            entry.access_count = entry.access_count.saturating_add(1);
        }
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn total_size(&self) -> u64 {
        self.entries.iter().map(|r| r.current_size).sum()
    }

    pub fn resources_by_app(&self, app_id: u64) -> Vec<GpuResourceMeta> {
        self.entries
            .iter()
            .filter(|r| r.app_id == app_id)
            .map(|r| r.clone())
            .collect()
    }
}

impl Default for ResourceTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::resource_type::GpuResourceType;
    use crate::resources::resource_usage::GpuResourceUsage;

    fn make_meta(id: GpuResourceId, app_id: u64) -> GpuResourceMeta {
        GpuResourceMeta::new(
            id,
            app_id,
            "test",
            GpuResourceType::Buffer,
            GpuResourceUsage::Cache,
            1024,
        )
    }

    #[test]
    fn test_register_and_exists() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        table.register(make_meta(id, 1)).unwrap();
        assert!(table.exists(id));
    }

    #[test]
    fn test_register_duplicate_fails() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        table.register(make_meta(id, 1)).unwrap();
        let result = table.register(make_meta(id, 2));
        assert!(result.is_err());
    }

    #[test]
    fn test_unregister() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        table.register(make_meta(id, 1)).unwrap();
        assert!(table.unregister(id).is_ok());
        assert!(!table.exists(id));
    }

    #[test]
    fn test_unregister_nonexistent_fails() {
        let table = ResourceTable::new();
        let result = table.unregister(GpuResourceId::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_get() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        let meta = make_meta(id, 42);
        table.register(meta.clone()).unwrap();
        let retrieved = table.get(id).unwrap();
        assert_eq!(retrieved.app_id, 42);
        assert_eq!(retrieved.resource_id, id);
    }

    #[test]
    fn test_get_nonexistent() {
        let table = ResourceTable::new();
        assert!(table.get(GpuResourceId::new()).is_none());
    }

    #[test]
    fn test_update_tier() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        table.register(make_meta(id, 1)).unwrap();
        table.update_tier(id, VramResidencyTier::T2Compressed).unwrap();
        let meta = table.get(id).unwrap();
        assert_eq!(meta.tier, VramResidencyTier::T2Compressed);
    }

    #[test]
    fn test_update_tier_nonexistent() {
        let table = ResourceTable::new();
        let result = table.update_tier(GpuResourceId::new(), VramResidencyTier::T2Compressed);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_access() {
        let table = ResourceTable::new();
        let id = GpuResourceId::new();
        table.register(make_meta(id, 1)).unwrap();
        let before = table.get(id).unwrap().access_count;
        table.update_access(id);
        let after = table.get(id).unwrap().access_count;
        assert_eq!(after, before + 1);
    }

    #[test]
    fn test_count() {
        let table = ResourceTable::new();
        assert_eq!(table.count(), 0);
        table.register(make_meta(GpuResourceId::new(), 1)).unwrap();
        table.register(make_meta(GpuResourceId::new(), 1)).unwrap();
        assert_eq!(table.count(), 2);
    }

    #[test]
    fn test_total_size() {
        let table = ResourceTable::new();
        let id1 = GpuResourceId::new();
        let id2 = GpuResourceId::new();
        let mut m1 = make_meta(id1, 1);
        m1.current_size = 500;
        let mut m2 = make_meta(id2, 1);
        m2.current_size = 300;
        table.register(m1).unwrap();
        table.register(m2).unwrap();
        assert_eq!(table.total_size(), 800);
    }

    #[test]
    fn test_resources_by_app() {
        let table = ResourceTable::new();
        let id1 = GpuResourceId::new();
        let id2 = GpuResourceId::new();
        table.register(make_meta(id1, 1)).unwrap();
        table.register(make_meta(id2, 2)).unwrap();
        let app1 = table.resources_by_app(1);
        assert_eq!(app1.len(), 1);
        assert!(app1.iter().any(|r| r.app_id == 1));
        let app2 = table.resources_by_app(2);
        assert_eq!(app2.len(), 1);
    }

    #[test]
    fn test_default() {
        let table: ResourceTable = Default::default();
        assert_eq!(table.count(), 0);
    }
}
