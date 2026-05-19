use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageId(pub Uuid);

impl PageId {
    pub fn new() -> Self {
        PageId(Uuid::new_v4())
    }

    pub fn nil() -> Self {
        PageId(Uuid::nil())
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

impl Default for PageId {
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Display for PageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PageId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_id_new() {
        let id = PageId::new();
        assert!(!id.is_nil());
    }

    #[test]
    fn test_page_id_nil() {
        let id = PageId::nil();
        assert!(id.is_nil());
    }

    #[test]
    fn test_page_id_unique() {
        let a = PageId::new();
        let b = PageId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn test_page_id_display() {
        let id = PageId::nil();
        let s = format!("{}", id);
        assert!(s.starts_with("PageId("));
    }

    #[test]
    fn test_page_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(PageId::new());
        set.insert(PageId::new());
        set.insert(PageId::nil());
        assert_eq!(set.len(), 3);
    }
}
