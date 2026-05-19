use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

use crate::allocator::allocation::Allocation;
use crate::allocator::allocation_kind::AllocationKind;
use crate::apps::app_id::AppId;
use crate::core::result::VrmResult;

pub struct LargeAlloc {
    entries: Mutex<HashMap<u64, Allocation>>,
    next_id: Mutex<u64>,
    allocated: AtomicU64,
}

impl LargeAlloc {
    pub fn new() -> Self {
        LargeAlloc {
            entries: Mutex::new(HashMap::new()),
            next_id: Mutex::new(0),
            allocated: AtomicU64::new(0),
        }
    }

    pub fn allocate(
        &self,
        app_id: AppId,
        size: u64,
        kind: AllocationKind,
    ) -> VrmResult<Allocation> {
        let mut alloc = Allocation::new(app_id, kind, size);
        let mut id_guard = self.next_id.lock();
        let entry_id = *id_guard;
        *id_guard += 1;
        alloc.ptr = Some(entry_id);
        let mut entries = self.entries.lock();
        entries.insert(entry_id, alloc.clone());
        self.allocated.fetch_add(size, Ordering::Relaxed);
        Ok(alloc)
    }

    pub fn free(&self, alloc: &Allocation) -> VrmResult<()> {
        if let Some(ptr) = alloc.ptr {
            let mut entries = self.entries.lock();
            entries.remove(&ptr);
        }
        Ok(())
    }

    pub fn total_allocated(&self) -> u64 {
        self.allocated.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_large_alloc_allocate() {
        let allocator = LargeAlloc::new();
        let app = AppId::new(1);
        let alloc = allocator
            .allocate(app, 8192, AllocationKind::Video)
            .unwrap();
        assert_eq!(alloc.size, 8192);
        assert!(alloc.ptr.is_some());
    }

    #[test]
    fn test_large_alloc_total() {
        let allocator = LargeAlloc::new();
        let app = AppId::new(1);
        allocator
            .allocate(app, 10000, AllocationKind::FileCache)
            .unwrap();
        allocator
            .allocate(app, 20000, AllocationKind::Image)
            .unwrap();
        assert_eq!(allocator.total_allocated(), 30000);
    }

    #[test]
    fn test_large_alloc_free() {
        let allocator = LargeAlloc::new();
        let app = AppId::new(1);
        let alloc = allocator
            .allocate(app, 16384, AllocationKind::Audio)
            .unwrap();
        allocator.free(&alloc).unwrap();
    }

    #[test]
    fn test_unique_ptr_per_allocation() {
        let allocator = LargeAlloc::new();
        let app = AppId::new(1);
        let a = allocator.allocate(app, 100, AllocationKind::Generic).unwrap();
        let b = allocator.allocate(app, 100, AllocationKind::Generic).unwrap();
        assert_ne!(a.ptr, b.ptr);
    }
}
