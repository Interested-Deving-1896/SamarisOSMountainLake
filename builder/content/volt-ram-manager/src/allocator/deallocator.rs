use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;

use crate::allocator::allocation::Allocation;
use crate::core::result::VrmResult;

pub struct Deallocator {
    freed: AtomicU64,
    pending: Mutex<Vec<Allocation>>,
}

impl Deallocator {
    pub fn new() -> Self {
        Deallocator {
            freed: AtomicU64::new(0),
            pending: Mutex::new(Vec::new()),
        }
    }

    pub fn free(&self, alloc: Allocation) -> VrmResult<()> {
        self.freed.fetch_add(alloc.size, Ordering::Relaxed);
        let mut pending = self.pending.lock();
        pending.push(alloc);
        Ok(())
    }

    pub fn total_freed(&self) -> u64 {
        self.freed.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::allocation_kind::AllocationKind;
    use crate::apps::app_id::AppId;

    #[test]
    fn test_deallocator_free() {
        let dealloc = Deallocator::new();
        let app = AppId::new(1);
        let alloc = Allocation::new(app, AllocationKind::Generic, 512);
        dealloc.free(alloc).unwrap();
        assert_eq!(dealloc.total_freed(), 512);
    }

    #[test]
    fn test_deallocator_multiple() {
        let dealloc = Deallocator::new();
        let app = AppId::new(1);
        dealloc
            .free(Allocation::new(app, AllocationKind::Generic, 100))
            .unwrap();
        dealloc
            .free(Allocation::new(app, AllocationKind::Generic, 200))
            .unwrap();
        assert_eq!(dealloc.total_freed(), 300);
    }
}
