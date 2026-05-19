use dashmap::DashMap;
use crate::resources::fingerprint::GpuFingerprint;
use crate::resources::resource_id::GpuResourceId;
use crate::core::result::VgmResult;

pub struct GpuDedupTable {
    entries: DashMap<GpuFingerprint, GpuResourceId>,
}

impl GpuDedupTable {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn insert(
        &self,
        fingerprint: GpuFingerprint,
        id: GpuResourceId,
    ) -> VgmResult<()> {
        if self.entries.contains_key(&fingerprint) {
            return Err(crate::core::error::VgmError::DedupCollision(format!(
                "Fingerprint collision for resource {}",
                id
            )));
        }
        self.entries.insert(fingerprint, id);
        Ok(())
    }

    pub fn lookup(&self, fingerprint: &GpuFingerprint) -> Option<GpuResourceId> {
        self.entries.get(fingerprint).map(|r| *r)
    }

    pub fn remove(&self, fingerprint: &GpuFingerprint) {
        self.entries.remove(fingerprint);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains(&self, fingerprint: &GpuFingerprint) -> bool {
        self.entries.contains_key(fingerprint)
    }
}

impl Default for GpuDedupTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table() {
        let table = GpuDedupTable::new();
        assert!(table.is_empty());
    }

    #[test]
    fn test_insert_and_lookup() {
        let table = GpuDedupTable::new();
        let fp = GpuFingerprint::from_data(b"unique data");
        let id = GpuResourceId::new();
        table.insert(fp, id).unwrap();
        let found = table.lookup(&fp);
        assert_eq!(found, Some(id));
    }

    #[test]
    fn test_lookup_missing() {
        let table = GpuDedupTable::new();
        let fp = GpuFingerprint::from_data(b"missing");
        assert_eq!(table.lookup(&fp), None);
    }

    #[test]
    fn test_insert_duplicate_fails() {
        let table = GpuDedupTable::new();
        let fp = GpuFingerprint::from_data(b"dup");
        let id1 = GpuResourceId::new();
        let id2 = GpuResourceId::new();
        table.insert(fp, id1).unwrap();
        let result = table.insert(fp, id2);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove() {
        let table = GpuDedupTable::new();
        let fp = GpuFingerprint::from_data(b"removable");
        let id = GpuResourceId::new();
        table.insert(fp, id).unwrap();
        assert!(table.contains(&fp));
        table.remove(&fp);
        assert!(!table.contains(&fp));
    }

    #[test]
    fn test_contains() {
        let table = GpuDedupTable::new();
        let fp = GpuFingerprint::from_data(b"present");
        assert!(!table.contains(&fp));
        table.insert(fp, GpuResourceId::new()).unwrap();
        assert!(table.contains(&fp));
    }

    #[test]
    fn test_len() {
        let table = GpuDedupTable::new();
        let fp1 = GpuFingerprint::from_data(b"a");
        let fp2 = GpuFingerprint::from_data(b"b");
        table.insert(fp1, GpuResourceId::new()).unwrap();
        table.insert(fp2, GpuResourceId::new()).unwrap();
        assert_eq!(table.len(), 2);
    }

    #[test]
    fn test_default() {
        let table: GpuDedupTable = Default::default();
        assert!(table.is_empty());
    }
}
