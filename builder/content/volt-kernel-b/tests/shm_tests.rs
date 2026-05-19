use tesseract_engine::protocol::header::SbpHeader;
use tesseract_engine::protocol::opcodes::Opcode;
use tesseract_engine::protocol::TesseractCommand;
use tesseract_engine::ipc::shm::SharedMemoryRing;

#[test]
fn test_shm_new() {
    let ring = SharedMemoryRing::new("test", 64).unwrap();
    assert!(ring.send(&make_cmd()).is_ok());
    let _ = ring.attach();
    let _ = ring.detach();
}

#[test]
fn test_shm_send_recv_sbp_cmd() {
    let ring = SharedMemoryRing::new("test-send-recv", 256).unwrap();
    let cmd = make_cmd();
    ring.send(&cmd).unwrap();
    let received = ring.recv().unwrap();
    assert_eq!(received.header.opcode, cmd.header.opcode);
    assert_eq!(received.header.app_id, cmd.header.app_id);
    assert_eq!(received.header.priority, cmd.header.priority);
}

#[test]
fn test_shm_send_multiple_in_order() {
    let ring = SharedMemoryRing::new("test-multi", 1024).unwrap();
    let n = 100;
    for i in 0..n {
        let h = SbpHeader::new(Opcode::Heartbeat, (i % 5) as u8, i as u32, 0);
        let cmd = TesseractCommand::new(h, vec![]);
        ring.send(&cmd).unwrap();
    }
    for i in 0..n {
        let received = ring.recv().unwrap();
        assert_eq!(received.header.app_id, i as u32);
    }
}

#[test]
fn test_shm_recv_empty_returns_none() {
    let ring = SharedMemoryRing::new("test-empty", 64).unwrap();
    assert!(ring.recv().is_none());
}

fn make_cmd() -> TesseractCommand {
    let header = SbpHeader::new(Opcode::Heartbeat, 2, 0x42, 0);
    TesseractCommand::new(header, vec![])
}

#[test]
fn test_shm_heap_backpressure() {
    let ring = SharedMemoryRing::new("test-full", 4).unwrap();
    for i in 0..4 {
        let h = SbpHeader::new(Opcode::Heartbeat, 2, i, 0);
        ring.send(&TesseractCommand::new(h, vec![])).unwrap();
    }
    // Ring is full, next send should fail
    let h = SbpHeader::new(Opcode::Heartbeat, 2, 99, 0);
    assert!(ring.send(&TesseractCommand::new(h, vec![])).is_err());
}
