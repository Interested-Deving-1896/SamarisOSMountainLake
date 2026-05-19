use crate::pools::size_class::SizeClass;

pub struct Slab {
    #[allow(dead_code)]
    size_class: SizeClass,
    entries: parking_lot::Mutex<Vec<Vec<u8>>>,
    capacity: usize,
}

impl Slab {
    pub fn new(size_class: SizeClass, count: usize) -> Self {
        let entry_size = size_class.0 as usize;
        let mut entries = Vec::with_capacity(count);
        for _ in 0..count {
            entries.push(vec![0u8; entry_size]);
        }
        Slab {
            size_class,
            entries: parking_lot::Mutex::new(entries),
            capacity: count,
        }
    }

    pub fn allocate(&self) -> Option<Vec<u8>> {
        let mut entries = self.entries.lock();
        entries.pop()
    }

    pub fn deallocate(&self, data: Vec<u8>) {
        let mut entries = self.entries.lock();
        entries.push(data);
    }

    pub fn available(&self) -> usize {
        self.entries.lock().len()
    }

    pub fn total(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_new() {
        let slab = Slab::new(SizeClass::B256, 10);
        assert_eq!(slab.total(), 10);
        assert_eq!(slab.available(), 10);
    }

    #[test]
    fn test_slab_allocate() {
        let slab = Slab::new(SizeClass::B64, 5);
        let data = slab.allocate();
        assert!(data.is_some());
        assert_eq!(data.unwrap().len(), 64);
        assert_eq!(slab.available(), 4);
    }

    #[test]
    fn test_slab_exhaust() {
        let slab = Slab::new(SizeClass::B16, 2);
        assert!(slab.allocate().is_some());
        assert!(slab.allocate().is_some());
        assert!(slab.allocate().is_none());
    }

    #[test]
    fn test_slab_deallocate() {
        let slab = Slab::new(SizeClass::KB1, 3);
        let data = slab.allocate().unwrap();
        assert_eq!(slab.available(), 2);
        slab.deallocate(data);
        assert_eq!(slab.available(), 3);
    }
}
