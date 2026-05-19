use volt_usb_manager::writeback::ack::WriteAckKind;
use volt_usb_manager::writeback::write_buffer::WriteBuffer;

#[test]
fn test_write_returns_ack_buffered() {
    let mut buf = WriteBuffer::new(10);
    buf.enqueue("/test/file", 0, vec![1, 2, 3], 0, 0).unwrap();
    let ack = buf.acks().recv().unwrap();
    assert_eq!(ack.kind, WriteAckKind::Buffered);
    assert_eq!(ack.write_id, 1);
}

#[test]
fn test_flush_produces_flushed_batch() {
    let mut buf = WriteBuffer::new(10);
    buf.enqueue("/a", 0, vec![0u8; 100], 0, 0).unwrap();
    buf.enqueue("/b", 0, vec![0u8; 200], 0, 0).unwrap();
    let batch = buf.flush_batch(1024);
    assert_eq!(batch.len(), 2);
    for pw in &batch {
        assert!(pw.is_flushable() || pw.status == volt_usb_manager::writeback::write_status::WriteStatus::Flushing);
    }
}

#[test]
fn test_buffer_limit_enforced() {
    let mut buf = WriteBuffer::new(1);
    for i in 0..5 {
        buf.enqueue(&format!("/file{}", i), 0, vec![0u8; 200 * 1024], 0, 0)
            .unwrap();
    }
    let batch = buf.flush_batch(64);
    assert!(batch.len() <= 3);
    assert!(buf.dirty_bytes() < buf.max_bytes());
}

#[test]
fn test_metadata_priority() {
    let mut buf = WriteBuffer::new(10);
    let meta = buf
        .enqueue("/meta", 0, vec![0u8; 10], 255, 0)
        .unwrap();
    let data = buf
        .enqueue("/data", 0, vec![0u8; 10], 0, 0)
        .unwrap();
    assert_eq!(meta.priority, 255);
    assert_eq!(data.priority, 0);
    assert!(meta.priority > data.priority);
}

#[test]
fn test_flush_interval_trigger() {
    let mut buf = WriteBuffer::new(1);
    assert!(!buf.needs_flush(50));
    buf.enqueue("/big", 0, vec![0u8; 600 * 1024], 0, 0)
        .unwrap();
    assert!(buf.needs_flush(50));
}

#[test]
fn test_acknowledge_durable() {
    let mut buf = WriteBuffer::new(10);
    let pw = buf.enqueue("/f", 0, vec![1], 0, 0).unwrap();
    buf.acknowledge(pw.write_id, WriteAckKind::Durable);
    assert_eq!(buf.pending_count(), 0);
}

#[test]
fn test_multiple_enqueues_have_monotonic_ids() {
    let mut buf = WriteBuffer::new(10);
    for i in 0..5 {
        buf.enqueue(&format!("/f{}", i), 0, vec![i as u8], 0, 0)
            .unwrap();
    }
    assert_eq!(buf.next_id(), 6);
}

#[test]
fn test_flush_batch_empty_buffer() {
    let mut buf = WriteBuffer::new(10);
    let batch = buf.flush_batch(1024);
    assert!(batch.is_empty());
}

#[test]
fn test_usage_percent_calculation() {
    let mut buf = WriteBuffer::new(10);
    assert!((buf.usage_pct() - 0.0).abs() < 0.01);
    buf.enqueue("/x", 0, vec![0u8; 5 * 1024 * 1024], 0, 0)
        .unwrap();
    assert!((buf.usage_pct() - 50.0).abs() < 1.0);
}

#[test]
fn test_acknowledge_error() {
    let mut buf = WriteBuffer::new(10);
    let pw = buf.enqueue("/f", 0, vec![1], 0, 0).unwrap();
    buf.acknowledge(pw.write_id, WriteAckKind::Error);
}

#[test]
fn test_dirty_bytes_tracked() {
    let mut buf = WriteBuffer::new(10);
    assert_eq!(buf.dirty_bytes(), 0);
    buf.enqueue("/a", 0, vec![0u8; 1000], 0, 0).unwrap();
    assert_eq!(buf.dirty_bytes(), 1000);
    buf.enqueue("/b", 0, vec![0u8; 500], 0, 0).unwrap();
    assert_eq!(buf.dirty_bytes(), 1500);
}
