use std::sync::atomic::{AtomicU64, Ordering};

use crate::allocator::allocation::Allocation;
use crate::allocator::allocation_kind::AllocationKind;
use crate::apps::app_id::AppId;
use crate::core::result::VrmResult;

pub struct SmallAlloc {
    allocated: AtomicU64,
    freed: AtomicU64,
}

impl SmallAlloc {
    pub fn new() -> Self {
        SmallAlloc {
            allocated: AtomicU64::new(0),
            freed: AtomicU64::new(0),
        }
    }

    pub fn allocate(
        &self,
        app_id: AppId,
        size: u64,
        kind: AllocationKind,
    ) -> VrmResult<Allocation> {
        let alloc = Allocation::new(app_id, kind, size);
        self.allocated.fetch_add(size, Ordering::Relaxed);
        Ok(alloc)
    }

    pub fn free(&self, alloc: &Allocation) -> VrmResult<()> {
        self.freed.fetch_add(alloc.size, Ordering::Relaxed);
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
    fn test_small_alloc_allocate() {
        let allocator = SmallAlloc::new();
        let app = AppId::new(1);
        let alloc = allocator
            .allocate(app, 256, AllocationKind::UI)
            .unwrap();
        assert_eq!(alloc.size, 256);
        assert_eq!(alloc.app_id, app);
        assert_eq!(alloc.kind, AllocationKind::UI);
    }

    #[test]
    fn test_small_alloc_total() {
        let allocator = SmallAlloc::new();
        let app = AppId::new(1);
        allocator.allocate(app, 128, AllocationKind::Generic).unwrap();
        allocator.allocate(app, 64, AllocationKind::Generic).unwrap();
        assert_eq!(allocator.total_allocated(), 192);
    }

    #[test]
    fn test_small_alloc_free() {
        let allocator = SmallAlloc::new();
        let app = AppId::new(1);
        let alloc = allocator.allocate(app, 512, AllocationKind::Binary).unwrap();
        allocator.free(&alloc).unwrap();
    }
}
