use dashmap::DashMap;
use crate::pages::page_id::PageId;

fn page_id_key(id: &PageId) -> u64 {
    let bytes = id.0.as_bytes();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}

pub struct CowState {
    shared: DashMap<u64, Vec<PageId>>,
}

impl CowState {
    pub fn new() -> Self {
        CowState {
            shared: DashMap::new(),
        }
    }

    pub fn register(&self, original: PageId, clone: PageId) {
        let key = page_id_key(&original);
        self.shared.entry(key).or_insert_with(Vec::new).push(clone);
    }

    pub fn unregister(&self, page_id: PageId) {
        let key = page_id_key(&page_id);
        self.shared.remove(&key);
        for mut entry in self.shared.iter_mut() {
            entry.retain(|id| page_id_key(id) != key);
        }
    }

    pub fn clones_of(&self, original: &PageId) -> Vec<PageId> {
        let key = page_id_key(original);
        self.shared
            .get(&key)
            .map(|e| e.value().clone())
            .unwrap_or_default()
    }

    pub fn is_shared(&self, page_id: &PageId) -> bool {
        let key = page_id_key(page_id);
        if self.shared.contains_key(&key) {
            return true;
        }
        for entry in self.shared.iter() {
            if entry.value().iter().any(|id| page_id_key(id) == key) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_clones() {
        let cow = CowState::new();
        let orig = PageId::new();
        let clone = PageId::new();
        cow.register(orig, clone);
        let clones = cow.clones_of(&orig);
        assert_eq!(clones.len(), 1);
        assert_eq!(clones[0], clone);
    }

    #[test]
    fn test_no_clones() {
        let cow = CowState::new();
        let orig = PageId::new();
        assert!(cow.clones_of(&orig).is_empty());
    }

    #[test]
    fn test_is_shared_original() {
        let cow = CowState::new();
        let orig = PageId::new();
        let clone = PageId::new();
        cow.register(orig, clone);
        assert!(cow.is_shared(&orig));
    }

    #[test]
    fn test_is_shared_clone() {
        let cow = CowState::new();
        let orig = PageId::new();
        let clone = PageId::new();
        cow.register(orig, clone);
        assert!(cow.is_shared(&clone));
    }

    #[test]
    fn test_not_shared() {
        let cow = CowState::new();
        let id = PageId::new();
        assert!(!cow.is_shared(&id));
    }

    #[test]
    fn test_unregister() {
        let cow = CowState::new();
        let orig = PageId::new();
        let clone = PageId::new();
        cow.register(orig, clone);
        cow.unregister(orig);
        assert!(!cow.is_shared(&orig));
    }
}
