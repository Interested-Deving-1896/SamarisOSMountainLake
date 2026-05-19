use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PageFlags: u8 {
        const PINNED    = 0b0001;
        const COMPRESSED = 0b0010;
        const COW       = 0b0100;
        const DIRTY     = 0b1000;
    }
}

impl PageFlags {
    pub fn is_pinned(&self) -> bool {
        self.contains(Self::PINNED)
    }

    pub fn is_compressed(&self) -> bool {
        self.contains(Self::COMPRESSED)
    }

    pub fn is_cow(&self) -> bool {
        self.contains(Self::COW)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned() {
        let f = PageFlags::PINNED;
        assert!(f.is_pinned());
        assert!(!f.is_compressed());
        assert!(!f.is_cow());
    }

    #[test]
    fn test_combined() {
        let f = PageFlags::PINNED | PageFlags::DIRTY;
        assert!(f.is_pinned());
        assert!(f.contains(PageFlags::DIRTY));
        assert!(!f.is_compressed());
    }

    #[test]
    fn test_empty() {
        let f = PageFlags::empty();
        assert!(!f.is_pinned());
        assert!(!f.is_compressed());
        assert!(!f.is_cow());
    }

    #[test]
    fn test_all() {
        let f = PageFlags::all();
        assert!(f.is_pinned());
        assert!(f.is_compressed());
        assert!(f.is_cow());
        assert!(f.contains(PageFlags::DIRTY));
    }
}
