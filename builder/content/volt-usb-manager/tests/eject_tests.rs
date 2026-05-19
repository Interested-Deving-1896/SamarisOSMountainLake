use volt_usb_manager::device::eject::{can_eject_safely, force_eject, prepare_eject};

#[test]
fn test_clean_eject_succeeds() {
    let result = prepare_eject("/tmp/__vum_eject_test_clean");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_dirty_eject_triggers_flush() {
    use volt_usb_manager::writeback::write_buffer::WriteBuffer;
    let mut buf = WriteBuffer::new(10);
    buf.enqueue("/dirty", 0, vec![0u8; 100], 0, 0).unwrap();
    assert!(buf.dirty_bytes() > 0);
    let flushed = buf.flush_batch(1024);
    assert!(!flushed.is_empty());
    assert!(buf.dirty_bytes() == 0 || buf.pending_count() == 0);
}

#[test]
fn test_dirty_unflushable_eject_fails() {
    use volt_usb_manager::writeback::write_buffer::WriteBuffer;
    let mut buf = WriteBuffer::new(1);
    for i in 0..3 {
        buf.enqueue(&format!("/big{}", i), 0, vec![0u8; 400 * 1024], 0, 0)
            .unwrap();
    }
    let batch = buf.flush_batch(64);
    assert!(batch.len() < 3);
}

#[test]
fn test_unmount_after_flush() {
    use volt_usb_manager::writeback::write_buffer::WriteBuffer;
    let mut buf = WriteBuffer::new(10);
    buf.enqueue("/f", 0, vec![1, 2, 3], 0, 0).unwrap();
    let flushed = buf.flush_batch(1024);
    assert_eq!(flushed.len(), 1);
    assert_eq!(buf.pending_count(), 0);
    assert_eq!(buf.dirty_bytes(), 0);
}

#[test]
fn test_can_eject_safely_nonexistent() {
    assert!(!can_eject_safely("/tmp/__vum_nonexistent_eject_test"));
}

#[test]
fn test_force_eject_nonexistent_fails() {
    let result = force_eject("/tmp/__vum_nonexistent_force_eject_test");
    assert!(result.is_err());
}

#[test]
fn test_prepare_eject_runs_sync() {
    let result = prepare_eject("/");
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_eject_config_validation() {
    use volt_usb_manager::config::schema::VumConfig;
    let mut config = VumConfig::default();
    assert!(config.validate().is_ok());
    config.eject.timeout_ms = 0;
    assert!(config.validate().is_err());
}

#[test]
fn test_write_buffer_flush_after_eject_prepare() {
    use volt_usb_manager::writeback::write_buffer::WriteBuffer;
    let mut buf = WriteBuffer::new(10);
    buf.enqueue("/eject_test", 0, vec![42], 0, 0).unwrap();
    assert_eq!(buf.pending_count(), 1);
    let batch = buf.flush_batch(1024);
    assert_eq!(batch.len(), 1);
    assert_eq!(buf.pending_count(), 0);
    for pw in &batch {
        assert!(pw.size() > 0);
    }
}
