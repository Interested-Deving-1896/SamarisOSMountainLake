use dashmap::DashMap;
use parking_lot::Mutex;
use crate::dedup::fingerprint::Fingerprint;
use crate::dedup::ref_counter::RefCounter;
use crate::dedup::verifier::Verifier;
use crate::pages::page_id::PageId;
use crate::core::result::VrmResult;

fn page_id_key(id: &PageId) -> u64 {
    let bytes = id.0.as_bytes();
    u64::from_le_bytes(bytes[0..8].try_into().unwrap())
}

#[derive(Debug, Clone)]
pub struct DedupEntry {
    pub fingerprint: Fingerprint,
    pub page_id: PageId,
    pub size: u64,
}

pub struct DedupTable {
    by_fingerprint: DashMap<u64, DedupEntry>,
    by_page: DashMap<u64, PageId>,
    ref_counter: RefCounter,
    verifier: Verifier,
    hits: Mutex<u64>,
    misses: Mutex<u64>,
}

impl DedupTable {
    pub fn new() -> Self {
        DedupTable {
            by_fingerprint: DashMap::new(),
            by_page: DashMap::new(),
            ref_counter: RefCounter::new(),
            verifier: Verifier::new(),
            hits: Mutex::new(0),
            misses: Mutex::new(0),
        }
    }

    pub fn find(&self, data: &[u8], page_id: PageId) -> VrmResult<Option<PageId>> {
        let fp = Fingerprint::from_data(data);
        let fp_val = fp.as_u64();

        if let Some(entry) = self.by_fingerprint.get(&fp_val) {
            if self.verifier.verify(entry.page_id, data)? {
                *self.hits.lock() += 1;
                self.ref_counter.increment(entry.page_id);
                return Ok(Some(entry.page_id));
            }
        }

        *self.misses.lock() += 1;
        Ok(None)
    }

    pub fn insert(&self, data: &[u8], page_id: PageId) -> VrmResult<()> {
        let fp = Fingerprint::from_data(data);
        let fp_val = fp.as_u64();
        let size = data.len() as u64;

        self.verifier.store(page_id, data.to_vec());

        let entry = DedupEntry {
            fingerprint: fp,
            page_id,
            size,
        };

        self.by_fingerprint.insert(fp_val, entry);
        self.by_page.insert(page_id_key(&page_id), page_id);
        self.ref_counter.increment(page_id);

        Ok(())
    }

    pub fn remove(&self, page_id: PageId) -> VrmResult<()> {
        let key = page_id_key(&page_id);
        self.by_page.remove(&key);
        self.ref_counter.remove(page_id);
        self.verifier.remove(page_id);

        self.by_fingerprint.retain(|_, entry| entry.page_id != page_id);

        Ok(())
    }

    pub fn hit_count(&self) -> u64 {
        *self.hits.lock()
    }

    pub fn miss_count(&self) -> u64 {
        *self.misses.lock()
    }

    pub fn total_entries(&self) -> usize {
        self.by_fingerprint.len()
    }

    pub fn total_dedup_savings(&self) -> u64 {
        let mut savings = 0u64;
        for entry in self.by_fingerprint.iter() {
            let count = self.ref_counter.count(entry.page_id);
            if count > 1 {
                savings += entry.size * (count - 1);
            }
        }
        savings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_find() {
        let table = DedupTable::new();
        let data = b"deduplicatable data block content";
        let id1 = PageId::new();
        table.insert(data, id1).unwrap();

        let id2 = PageId::new();
        let found = table.find(data, id2).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), id1);
    }

    #[test]
    fn test_find_no_match() {
        let table = DedupTable::new();
        let data = b"some data";
        let id = PageId::new();
        let found = table.find(data, id).unwrap();
        assert!(found.is_none());
    }

    #[test]
    fn test_hit_miss_counts() {
        let table = DedupTable::new();
        let data1 = b"data block a";
        let data2 = b"data block b";
        let id1 = PageId::new();
        let id2 = PageId::new();
        let id3 = PageId::new();

        table.insert(data1, id1).unwrap();
        table.find(data1, id2).unwrap();
        table.find(data2, id3).unwrap();

        assert_eq!(table.hit_count(), 1);
        assert_eq!(table.miss_count(), 1);
    }

    #[test]
    fn test_remove() {
        let table = DedupTable::new();
        let data = b"removable data";
        let id = PageId::new();
        table.insert(data, id).unwrap();
        table.remove(id).unwrap();
        assert_eq!(table.total_entries(), 0);
    }
}
