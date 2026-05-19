use std::sync::Arc;

use crate::pools::free_list::FreeList;
use crate::pools::global_pool::GlobalPool;
use crate::pools::size_class::SizeClass;
use crate::pools::slab::Slab;
use crate::pools::stats::PoolStats;

pub struct WorkerPool {
    #[allow(dead_code)]
    id: u32,
    slabs: Vec<Slab>,
    free_lists: Vec<FreeList>,
    stats: Arc<PoolStats>,
    global: Option<Arc<GlobalPool>>,
}

impl WorkerPool {
    pub fn new(id: u32, global: Option<Arc<GlobalPool>>) -> Self {
        let classes = SizeClass::all();
        let slabs = classes.iter().map(|&sc| Slab::new(sc, 64)).collect();
        let free_lists = classes.iter().map(|_| FreeList::new(128)).collect();
        WorkerPool {
            id,
            slabs,
            free_lists,
            stats: Arc::new(PoolStats::new()),
            global,
        }
    }

    pub fn allocate(&self, size: u64) -> Option<Vec<u8>> {
        let sc = SizeClass::for_size(size);
        let idx = self.size_class_index(sc)?;

        if let Some(data) = self.free_lists[idx].pop() {
            self.stats.record_alloc(size);
            return Some(data);
        }

        if let Some(data) = self.slabs[idx].allocate() {
            self.stats.record_alloc(size);
            return Some(data);
        }

        if let Some(ref global) = self.global {
            let data = global.allocate(size);
            self.stats.record_alloc(size);
            return Some(data);
        }

        None
    }

    pub fn deallocate(&self, data: Vec<u8>, size: u64) {
        let sc = SizeClass::for_size(size);
        if let Some(idx) = self.size_class_index(sc) {
            if self.free_lists[idx].push(data.clone()) {
                self.stats.record_free(size);
                return;
            }
            // free list was full — fall through to global
        }

        if let Some(ref global) = self.global {
            global.deallocate(data);
        }
        self.stats.record_free(size);
    }

    pub fn stats(&self) -> &PoolStats {
        &self.stats
    }

    fn size_class_index(&self, sc: SizeClass) -> Option<usize> {
        let all = SizeClass::all();
        all.iter().position(|&s| s == sc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_pool_new() {
        let pool = WorkerPool::new(0, None);
        assert_eq!(pool.id, 0);
    }

    #[test]
    fn test_allocate_without_global() {
        let pool = WorkerPool::new(1, None);
        let data = pool.allocate(64);
        assert!(data.is_some());
        assert_eq!(data.unwrap().len(), 64);
    }

    #[test]
    fn test_allocate_and_deallocate() {
        let pool = WorkerPool::new(2, None);
        let data = pool.allocate(256).unwrap();
        assert_eq!(data.len(), 256);
        pool.deallocate(data, 256);
    }

    #[test]
    fn test_exhaust_slab_then_alloc_fails() {
        // A slab with just 1 entry and no global pool
        let mut pool = WorkerPool::new(3, None);
        // Override the slab for B16 to have only 1 entry
        pool.slabs[0] = Slab::new(SizeClass::B16, 1);
        let _first = pool.allocate(16).unwrap();
        // Second alloc from slab should fail, free list empty, no global -> None
        let second = pool.allocate(16);
        assert!(second.is_none());
    }

    #[test]
    fn test_stats_collected() {
        let pool = WorkerPool::new(4, None);
        let data = pool.allocate(128).unwrap();
        pool.deallocate(data, 128);
        let stats = pool.stats();
        assert_eq!(
            stats.allocations.load(std::sync::atomic::Ordering::Relaxed),
            1
        );
        assert_eq!(
            stats.deallocations.load(std::sync::atomic::Ordering::Relaxed),
            1
        );
    }

    #[test]
    fn test_with_global_pool_fallback() {
        let global = Arc::new(GlobalPool::new(10_000_000));
        let pool = WorkerPool::new(5, Some(global.clone()));
        let mut pool = pool;
        pool.slabs[3] = Slab::new(SizeClass::KB1, 0); // empty slab
        // free list is also empty for KB1
        // should fall through to global
        let data = pool.allocate(1024);
        assert!(data.is_some());
        assert_eq!(data.unwrap().len(), 1024);
    }
}
