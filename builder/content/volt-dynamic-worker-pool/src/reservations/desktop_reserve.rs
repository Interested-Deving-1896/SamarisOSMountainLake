#[derive(Clone, Debug)]
pub struct DesktopReserveSnapshot {
    pub min_workers: u32,
    pub current_reserved: u32,
    pub active: bool,
}

pub struct DesktopReserve {
    pub min_workers: u32,
    pub current_reserved: u32,
}

impl DesktopReserve {
    pub fn new(min_workers: u32) -> Self {
        Self {
            min_workers,
            current_reserved: 0,
        }
    }

    pub fn reserve(&mut self, count: u32) {
        self.current_reserved = self.current_reserved.saturating_add(count);
    }

    pub fn release(&mut self, count: u32) {
        self.current_reserved = self.current_reserved.saturating_sub(count);
    }

    pub fn is_active(&self) -> bool {
        self.current_reserved > 0
    }

    pub fn snapshot(&self) -> DesktopReserveSnapshot {
        DesktopReserveSnapshot {
            min_workers: self.min_workers,
            current_reserved: self.current_reserved,
            active: self.is_active(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = DesktopReserve::new(5);
        assert_eq!(r.min_workers, 5);
        assert_eq!(r.current_reserved, 0);
    }

    #[test]
    fn test_reserve_and_release() {
        let mut r = DesktopReserve::new(5);
        r.reserve(3);
        assert_eq!(r.current_reserved, 3);
        assert!(r.is_active());
        r.release(2);
        assert_eq!(r.current_reserved, 1);
        assert!(r.is_active());
        r.release(1);
        assert_eq!(r.current_reserved, 0);
        assert!(!r.is_active());
    }

    #[test]
    fn test_release_saturating() {
        let mut r = DesktopReserve::new(5);
        r.release(10);
        assert_eq!(r.current_reserved, 0);
    }

    #[test]
    fn test_snapshot() {
        let mut r = DesktopReserve::new(5);
        r.reserve(3);
        let s = r.snapshot();
        assert_eq!(s.min_workers, 5);
        assert_eq!(s.current_reserved, 3);
        assert!(s.active);
    }
}
