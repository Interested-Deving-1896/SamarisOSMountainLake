use crate::pages::page_id::PageId;
use crate::core::result::VrmResult;
use crate::core::error::VrmError;
use std::collections::HashMap;

fn page_id_key(id: &PageId) -> u64 {
    let bytes = id.0.as_bytes();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}

pub struct Verifier {
    pages: parking_lot::Mutex<HashMap<u64, Vec<u8>>>,
}

impl Verifier {
    pub fn new() -> Self {
        Verifier {
            pages: parking_lot::Mutex::new(HashMap::new()),
        }
    }

    pub fn store(&self, page_id: PageId, data: Vec<u8>) {
        let key = page_id_key(&page_id);
        self.pages.lock().insert(key, data);
    }

    pub fn verify(&self, page_id: PageId, data: &[u8]) -> VrmResult<bool> {
        let key = page_id_key(&page_id);
        let map = self.pages.lock();
        match map.get(&key) {
            Some(stored) => {
                if stored.len() != data.len() {
                    return Err(VrmError::DedupCollision);
                }
                Ok(stored.as_slice() == data)
            }
            None => Ok(false),
        }
    }

    pub fn remove(&self, page_id: PageId) {
        let key = page_id_key(&page_id);
        self.pages.lock().remove(&key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_verify_match() {
        let v = Verifier::new();
        let id = PageId::new();
        let data = vec![1, 2, 3, 4];
        v.store(id, data.clone());
        assert!(v.verify(id, &data).unwrap());
    }

    #[test]
    fn test_verify_no_match() {
        let v = Verifier::new();
        let id = PageId::new();
        let data = vec![1, 2, 3];
        v.store(id, data);
        assert!(!v.verify(id, &[4, 5, 6]).unwrap());
    }

    #[test]
    fn test_verify_nonexistent() {
        let v = Verifier::new();
        let id = PageId::new();
        assert!(!v.verify(id, &[1]).unwrap());
    }

    #[test]
    fn test_remove() {
        let v = Verifier::new();
        let id = PageId::new();
        v.store(id, vec![1, 2]);
        v.remove(id);
        assert!(!v.verify(id, &[1, 2]).unwrap());
    }

    #[test]
    fn test_verify_different_length() {
        let v = Verifier::new();
        let id = PageId::new();
        v.store(id, vec![1, 2, 3]);
        let result = v.verify(id, &[1, 2]);
        assert!(result.is_err());
    }
}
