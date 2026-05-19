use std::sync::Arc;
use std::thread;

use tesseract_engine::protocol::RingBuffer;

#[test]
fn test_basic_push_pop() {
    let buf = RingBuffer::new(4);
    assert!(buf.is_empty());
    assert_eq!(buf.capacity(), 4);

    buf.try_push(10).unwrap();
    buf.try_push(20).unwrap();
    assert_eq!(buf.len(), 2);
    assert!(!buf.is_full());

    assert_eq!(buf.try_pop(), Some(10));
    assert_eq!(buf.try_pop(), Some(20));
    assert!(buf.is_empty());
    assert_eq!(buf.try_pop(), None);
}

#[test]
fn test_full_buffer() {
    let buf = RingBuffer::new(3);
    buf.try_push(1).unwrap();
    buf.try_push(2).unwrap();
    buf.try_push(3).unwrap();
    assert!(buf.is_full());
    assert!(buf.try_push(4).is_err());
}

#[test]
fn test_wrapping_behavior() {
    let buf = RingBuffer::new(3);
    buf.try_push(1).unwrap();
    buf.try_push(2).unwrap();
    buf.try_push(3).unwrap();

    assert_eq!(buf.try_pop(), Some(1));
    buf.try_push(4).unwrap();

    assert_eq!(buf.try_pop(), Some(2));
    assert_eq!(buf.try_pop(), Some(3));
    assert_eq!(buf.try_pop(), Some(4));
    assert_eq!(buf.try_pop(), None);
}

#[test]
fn test_blocking_push_pop() {
    let buf = RingBuffer::new(2);
    buf.push(1);
    buf.push(2);
    assert_eq!(buf.pop(), Some(1));
    assert_eq!(buf.pop(), Some(2));
}

#[test]
fn test_multiple_wrap_rounds() {
    let buf = RingBuffer::new(4);
    for i in 0..100 {
        buf.try_push(i).unwrap();
        assert_eq!(buf.try_pop(), Some(i));
    }
    assert!(buf.is_empty());
}

#[test]
fn test_concurrent_spsc() {
    let buf = Arc::new(RingBuffer::new(1024));
    let buf_producer = buf.clone();
    let buf_consumer = buf.clone();

    let producer = thread::spawn(move || {
        for i in 0..5000 {
            loop {
                if buf_producer.try_push(i).is_ok() {
                    break;
                }
                thread::yield_now();
            }
        }
    });

    let consumer = thread::spawn(move || {
        let mut count = 0;
        let mut last = -1i32;
        while count < 5000 {
            if let Some(val) = buf_consumer.try_pop() {
                assert!(val > last, "out of order: {val} <= {last}");
                last = val;
                count += 1;
            } else {
                thread::yield_now();
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
