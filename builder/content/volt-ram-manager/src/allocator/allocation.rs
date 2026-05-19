use crate::allocator::allocation_id::AllocationId;
use crate::allocator::allocation_kind::AllocationKind;
use crate::apps::app_id::AppId;
use crate::pages::page_id::PageId;

#[derive(Debug, Clone)]
pub struct Allocation {
    pub id: AllocationId,
    pub app_id: AppId,
    pub kind: AllocationKind,
    pub size: u64,
    pub ptr: Option<u64>,
    pub page_id: Option<PageId>,
}

impl Allocation {
    pub fn new(app_id: AppId, kind: AllocationKind, size: u64) -> Self {
        Allocation {
            id: AllocationId::new(),
            app_id,
            kind,
            size,
            ptr: None,
            page_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_new() {
        let app = AppId::new(42);
        let alloc = Allocation::new(app, AllocationKind::Generic, 1024);
        assert_eq!(alloc.app_id, app);
        assert_eq!(alloc.kind, AllocationKind::Generic);
        assert_eq!(alloc.size, 1024);
        assert!(alloc.ptr.is_none());
        assert!(alloc.page_id.is_none());
    }

    #[test]
    fn test_allocation_unique_ids() {
        let app = AppId::new(1);
        let a = Allocation::new(app, AllocationKind::Text, 64);
        let b = Allocation::new(app, AllocationKind::Text, 64);
        assert_ne!(a.id, b.id);
    }
}
