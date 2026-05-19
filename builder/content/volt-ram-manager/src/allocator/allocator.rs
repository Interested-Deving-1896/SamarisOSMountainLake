use std::sync::Arc;

use crate::allocator::allocation::Allocation;
use crate::allocator::allocation_kind::AllocationKind;
use crate::allocator::deallocator::Deallocator;
use crate::allocator::large_alloc::LargeAlloc;
use crate::allocator::small_alloc::SmallAlloc;
use crate::apps::app_id::AppId;
use crate::core::result::VrmResult;
use crate::pools::global_pool::GlobalPool;

pub struct VrmAllocator {
    small: SmallAlloc,
    large: LargeAlloc,
    dealloc: Deallocator,
    #[allow(dead_code)]
    global_pool: Option<Arc<GlobalPool>>,
}

impl VrmAllocator {
    pub fn new(global_pool: Option<Arc<GlobalPool>>) -> Self {
        VrmAllocator {
            small: SmallAlloc::new(),
            large: LargeAlloc::new(),
            dealloc: Deallocator::new(),
            global_pool,
        }
    }

    pub fn allocate(
        &self,
        app_id: AppId,
        size: u64,
        kind: AllocationKind,
    ) -> VrmResult<Allocation> {
        if size <= 4096 {
            self.small.allocate(app_id, size, kind)
        } else {
            self.large.allocate(app_id, size, kind)
        }
    }

    pub fn free(&self, alloc: Allocation) -> VrmResult<()> {
        self.dealloc.free(alloc)
    }

    pub fn total_allocated(&self) -> u64 {
        self.small.total_allocated() + self.large.total_allocated()
    }

    pub fn total_freed(&self) -> u64 {
        self.dealloc.total_freed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_small() {
        let allocator = VrmAllocator::new(None);
        let app = AppId::new(1);
        let alloc = allocator.allocate(app, 256, AllocationKind::Generic).unwrap();
        assert_eq!(alloc.size, 256);
        assert_eq!(alloc.app_id, app);
    }

    #[test]
    fn test_allocate_large() {
        let allocator = VrmAllocator::new(None);
        let app = AppId::new(1);
        let alloc = allocator.allocate(app, 8192, AllocationKind::Image).unwrap();
        assert_eq!(alloc.size, 8192);
    }

    #[test]
    fn test_free() {
        let allocator = VrmAllocator::new(None);
        let app = AppId::new(1);
        let alloc = allocator.allocate(app, 128, AllocationKind::Text).unwrap();
        allocator.free(alloc).unwrap();
    }

    #[test]
    fn test_totals() {
        let allocator = VrmAllocator::new(None);
        assert_eq!(allocator.total_allocated(), 0);
        assert_eq!(allocator.total_freed(), 0);
        let app = AppId::new(1);
        let a = allocator.allocate(app, 256, AllocationKind::Generic).unwrap();
        allocator.allocate(app, 8192, AllocationKind::Binary).unwrap();
        assert_eq!(allocator.total_allocated(), 256 + 8192);
        allocator.free(a).unwrap();
        assert_eq!(allocator.total_freed(), 256);
    }
}
