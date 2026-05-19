#[derive(Debug, Clone)]
pub struct DirtyPage {
    pub path: String,
    pub offset: u64,
    pub len: u64,
    pub is_metadata: bool,
}

impl DirtyPage {
    pub fn new(path: &str, offset: u64, len: u64) -> Self {
        Self {
            path: path.to_string(),
            offset,
            len,
            is_metadata: false,
        }
    }

    pub fn metadata(path: &str) -> Self {
        Self {
            path: path.to_string(),
            offset: 0,
            len: 0,
            is_metadata: true,
        }
    }

    pub fn end_offset(&self) -> u64 {
        self.offset + self.len
    }

    pub fn overlaps(&self, other: &DirtyPage) -> bool {
        self.path == other.path
            && self.offset < other.end_offset()
            && other.offset < self.end_offset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_page_new() {
        let page = DirtyPage::new("/test/file", 4096, 1024);
        assert_eq!(page.path, "/test/file");
        assert_eq!(page.offset, 4096);
        assert_eq!(page.len, 1024);
        assert!(!page.is_metadata);
    }

    #[test]
    fn test_dirty_page_metadata() {
        let page = DirtyPage::metadata("/meta");
        assert_eq!(page.path, "/meta");
        assert_eq!(page.offset, 0);
        assert_eq!(page.len, 0);
        assert!(page.is_metadata);
    }

    #[test]
    fn test_end_offset() {
        let page = DirtyPage::new("/f", 100, 50);
        assert_eq!(page.end_offset(), 150);
    }

    #[test]
    fn test_overlaps_same_region() {
        let a = DirtyPage::new("/f", 0, 100);
        let b = DirtyPage::new("/f", 50, 100);
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_no_overlap() {
        let a = DirtyPage::new("/f", 0, 100);
        let b = DirtyPage::new("/f", 100, 100);
        assert!(!a.overlaps(&b));
        assert!(!b.overlaps(&a));
    }

    #[test]
    fn test_overlaps_different_paths() {
        let a = DirtyPage::new("/a", 0, 100);
        let b = DirtyPage::new("/b", 0, 100);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_overlaps_adjacent() {
        let a = DirtyPage::new("/f", 0, 100);
        let b = DirtyPage::new("/f", 100, 1);
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_clone() {
        let a = DirtyPage::new("/clone", 10, 20);
        let b = a.clone();
        assert_eq!(a.path, b.path);
        assert_eq!(a.offset, b.offset);
        assert_eq!(a.len, b.len);
    }
}
