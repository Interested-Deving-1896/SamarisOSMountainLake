use std::sync::atomic::{AtomicUsize, Ordering};

use crate::core::error::{Result, TesseractError};

#[derive(Debug)]
pub struct RingBuffer<T> {
    buffer: Vec<parking_lot::Mutex<Option<T>>>,
    capacity: usize,
    write_index: AtomicUsize,
    read_index: AtomicUsize,
}

impl<T: Send> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(parking_lot::Mutex::new(None));
        }
        Self {
            buffer,
            capacity,
            write_index: AtomicUsize::new(0),
            read_index: AtomicUsize::new(0),
        }
    }

    pub fn try_push(&self, item: T) -> Result<()> {
        let write = self.write_index.load(Ordering::Acquire);
        let read = self.read_index.load(Ordering::Acquire);

        if write.wrapping_sub(read) >= self.capacity {
            return Err(TesseractError::Internal("ring buffer full".into()));
        }

        let idx = write % self.capacity;
        let mut slot = self.buffer[idx].lock();
        *slot = Some(item);
        self.write_index.store(write.wrapping_add(1), Ordering::Release);
        Ok(())
    }

    pub fn try_pop(&self) -> Option<T> {
        let read = self.read_index.load(Ordering::Acquire);
        let write = self.write_index.load(Ordering::Acquire);

        if read == write {
            return None;
        }

        let idx = read % self.capacity;
        let mut slot = self.buffer[idx].lock();
        let item = slot.take();
        if item.is_some() {
            self.read_index.store(read.wrapping_add(1), Ordering::Release);
        }
        item
    }

    pub fn push(&self, item: T) where T: Clone {
        loop {
            match self.try_push(item.clone()) {
                Ok(_) => return,
                Err(_) => std::hint::spin_loop(),
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let result = self.try_pop();
            if result.is_some() {
                return result;
            }
            std::hint::spin_loop();
        }
    }

    pub fn len(&self) -> usize {
        let write = self.write_index.load(Ordering::Acquire);
        let read = self.read_index.load(Ordering::Acquire);
        write.wrapping_sub(read)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let buf = RingBuffer::new(4);
        assert!(buf.is_empty());
        buf.try_push(1).unwrap();
        buf.try_push(2).unwrap();
        assert_eq!(buf.len(), 2);
        assert_eq!(buf.try_pop(), Some(1));
        assert_eq!(buf.try_pop(), Some(2));
        assert_eq!(buf.try_pop(), None);
    }

    #[test]
    fn test_full() {
        let buf = RingBuffer::new(2);
        buf.try_push(1).unwrap();
        buf.try_push(2).unwrap();
        assert!(buf.is_full());
        assert!(buf.try_push(3).is_err());
    }

    #[test]
    fn test_wrapping() {
        let buf = RingBuffer::new(2);
        buf.try_push(1).unwrap();
        buf.try_push(2).unwrap();
        assert_eq!(buf.try_pop(), Some(1));
        buf.try_push(3).unwrap();
        assert_eq!(buf.try_pop(), Some(2));
        assert_eq!(buf.try_pop(), Some(3));
        assert_eq!(buf.try_pop(), None);
    }

    #[test]
    fn test_empty() {
        let buf: RingBuffer<i32> = RingBuffer::new(4);
        assert_eq!(buf.try_pop(), None);
    }
}
