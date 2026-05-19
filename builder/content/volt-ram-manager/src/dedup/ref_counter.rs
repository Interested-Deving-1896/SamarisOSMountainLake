use dashmap::DashMap;
use crate::pages::page_id::PageId;

fn page_id_key(id: &PageId) -> u64 {
    let bytes = id.0.as_bytes();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}

pub struct RefCounter {
    refs: DashMap<u64, u64>,
}

impl RefCounter {
    pub fn new() -> Self {
        RefCounter {
            refs: DashMap::new(),
        }
    }

    pub fn increment(&self, page_id: PageId) -> u64 {
        let key = page_id_key(&page_id);
        let mut entry = self.refs.entry(key).or_insert(0);
        *entry += 1;
        *entry
    }

    pub fn decrement(&self, page_id: PageId) -> u64 {
        let key = page_id_key(&page_id);
        let mut entry = self.refs.entry(key).or_insert(0);
        if *entry > 0 {
            *entry -= 1;
        }
        *entry
    }

    pub fn count(&self, page_id: PageId) -> u64 {
        let key = page_id_key(&page_id);
        self.refs.get(&key).map(|e| *e).unwrap_or(0)
    }

    pub fn is_single_ref(&self, page_id: PageId) -> bool {
        self.count(page_id) <= 1
    }

    pub fn remove(&self, page_id: PageId) {
        let key = page_id_key(&page_id);
        self.refs.remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let rc = RefCounter::new();
        let id = PageId::new();
        assert_eq!(rc.increment(id), 1);
        assert_eq!(rc.increment(id), 2);
    }

    #[test]
    fn test_decrement() {
        let rc = RefCounter::new();
        let id = PageId::new();
        rc.increment(id);
        rc.increment(id);
        assert_eq!(rc.decrement(id), 1);
        assert_eq!(rc.decrement(id), 0);
    }

    #[test]
    fn test_count() {
        let rc = RefCounter::new();
        let id = PageId::new();
        assert_eq!(rc.count(id), 0);
        rc.increment(id);
        assert_eq!(rc.count(id), 1);
    }

    #[test]
    fn test_is_single_ref() {
        let rc = RefCounter::new();
        let id = PageId::new();
        assert!(rc.is_single_ref(id));
        rc.increment(id);
        assert!(rc.is_single_ref(id));
        rc.increment(id);
        assert!(!rc.is_single_ref(id));
    }

    #[test]
    fn test_remove() {
        let rc = RefCounter::new();
        let id = PageId::new();
        rc.increment(id);
        rc.remove(id);
        assert_eq!(rc.count(id), 0);
    }

    #[test]
    fn test_decrement_below_zero() {
        let rc = RefCounter::new();
        let id = PageId::new();
        assert_eq!(rc.decrement(id), 0);
    }
}
