use parking_lot::Mutex;

pub struct FreeList {
    items: Mutex<Vec<Vec<u8>>>,
    max_items: usize,
}

impl FreeList {
    pub fn new(max: usize) -> Self {
        FreeList {
            items: Mutex::new(Vec::with_capacity(max)),
            max_items: max,
        }
    }

    pub fn push(&self, item: Vec<u8>) -> bool {
        let mut items = self.items.lock();
        if items.len() >= self.max_items {
            false
        } else {
            items.push(item);
            true
        }
    }

    pub fn pop(&self) -> Option<Vec<u8>> {
        let mut items = self.items.lock();
        items.pop()
    }

    pub fn len(&self) -> usize {
        self.items.lock().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_list_push_pop() {
        let fl = FreeList::new(5);
        assert!(fl.push(vec![1, 2, 3]));
        assert_eq!(fl.len(), 1);
        let item = fl.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap(), vec![1, 2, 3]);
        assert_eq!(fl.len(), 0);
    }

    #[test]
    fn test_free_list_max() {
        let fl = FreeList::new(2);
        assert!(fl.push(vec![1]));
        assert!(fl.push(vec![2]));
        assert!(!fl.push(vec![3]));
        assert_eq!(fl.len(), 2);
    }

    #[test]
    fn test_free_list_pop_empty() {
        let fl = FreeList::new(3);
        assert!(fl.pop().is_none());
    }

    #[test]
    fn test_free_list_lifo() {
        let fl = FreeList::new(10);
        fl.push(vec![1]);
        fl.push(vec![2]);
        assert_eq!(fl.pop(), Some(vec![2]));
        assert_eq!(fl.pop(), Some(vec![1]));
    }
}
