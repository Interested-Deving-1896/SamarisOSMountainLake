use tesseract_engine::compute_bridge::task::{ComputeKind, ComputeTask, execute_compute};
use tesseract_engine::compute_bridge::buffer::BufferManager;
use tesseract_engine::compute_bridge::context::ComputeContext;
use tesseract_engine::compute_bridge::ComputeBridge;

#[test]
fn test_compute_hash_sha256_deterministic() {
    let task = ComputeTask::new(0x01, ComputeKind::HashSha256, b"hello".to_vec());
    let result = execute_compute(&task).unwrap();
    assert_eq!(result.len(), 32, "SHA-256 output must be 32 bytes");
    let result2 = execute_compute(&task).unwrap();
    assert_eq!(result, result2, "SHA-256 must be deterministic");
}

#[test]
fn test_compute_hash_sha256_differs_for_diff_inputs() {
    let t1 = execute_compute(&ComputeTask::new(0x01, ComputeKind::HashSha256, b"hello".to_vec())).unwrap();
    let t2 = execute_compute(&ComputeTask::new(0x01, ComputeKind::HashSha256, b"world".to_vec())).unwrap();
    assert_ne!(t1, t2, "different inputs should produce different hashes");
}

#[test]
fn test_compute_hash_empty_input() {
    let result = execute_compute(&ComputeTask::new(0x01, ComputeKind::HashSha256, vec![]));
    assert!(result.is_err(), "empty input should fail");
}

#[test]
fn test_compute_rle_compress_decompress_roundtrip() {
    let original = b"AAAABBBBCCCCDDDDAAAAEEEEFFFF".to_vec();
    let compressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Compress, original.clone())).unwrap();
    assert!(compressed.len() <= original.len(), "RLE should compress repeated data");
    let decompressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Decompress, compressed)).unwrap();
    assert_eq!(decompressed, original, "RLE roundtrip failed");
}

#[test]
fn test_compute_rle_compress_non_repeated() {
    let original = b"abcdefghijklmnop".to_vec();
    let compressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Compress, original.clone())).unwrap();
    let decompressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Decompress, compressed)).unwrap();
    assert_eq!(decompressed, original);
}

#[test]
fn test_compute_rle_empty() {
    let result = execute_compute(&ComputeTask::new(0x01, ComputeKind::Compress, vec![])).unwrap();
    assert!(result.is_empty(), "empty compress should produce empty");
}

#[test]
fn test_compute_rle_single_byte() {
    let original = b"A".to_vec();
    let compressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Compress, original.clone())).unwrap();
    let decompressed = execute_compute(&ComputeTask::new(0x01, ComputeKind::Decompress, compressed)).unwrap();
    assert_eq!(decompressed, original);
}

#[test]
fn test_compute_image_blur_passthrough() {
    let data = vec![0u8; 1024];
    let result = execute_compute(&ComputeTask::new(0x01, ComputeKind::ImageBlur, data.clone())).unwrap();
    assert_eq!(result, data, "alpha blur should passthrough");
}

#[test]
fn test_buffer_allocate_free() {
    let mut mgr = BufferManager::new(1024 * 1024);
    let handle = mgr.allocate(0x01, 4096).unwrap();
    assert_eq!(mgr.total_allocated(), 4096);
    assert_eq!(mgr.buffer_count(), 1);
    mgr.free(&handle.id).unwrap();
    assert_eq!(mgr.total_allocated(), 0);
}

#[test]
fn test_buffer_write_read() {
    let mut mgr = BufferManager::new(1024 * 1024);
    let handle = mgr.allocate(0x01, 1024).unwrap();
    let data = vec![0xAB; 512];
    mgr.write(&handle.id, &data, 0).unwrap();
    let read = mgr.read(&handle.id, 0, 512).unwrap();
    assert_eq!(read, data);
}

#[test]
fn test_buffer_allocate_zero_fails() {
    let mut mgr = BufferManager::new(1024);
    assert!(mgr.allocate(0x01, 0).is_err());
}

#[test]
fn test_buffer_oob_write_fails() {
    let mut mgr = BufferManager::new(1024);
    let handle = mgr.allocate(0x01, 100).unwrap();
    assert!(mgr.write(&handle.id, &[0u8; 200], 0).is_err());
}

#[test]
fn test_buffer_oob_read_fails() {
    let mut mgr = BufferManager::new(1024);
    let handle = mgr.allocate(0x01, 100).unwrap();
    assert!(mgr.read(&handle.id, 50, 100).is_err());
}

#[test]
fn test_buffer_cleanup_app() {
    let mut mgr = BufferManager::new(1024 * 1024);
    let _h1 = mgr.allocate(0x01, 100).unwrap();
    let _h2 = mgr.allocate(0x02, 200).unwrap();
    mgr.cleanup_app(0x01);
    assert_eq!(mgr.total_allocated(), 200);
}

#[test]
fn test_buffer_reset() {
    let mut mgr = BufferManager::new(1024);
    mgr.allocate(0x01, 100).unwrap();
    mgr.reset();
    assert_eq!(mgr.total_allocated(), 0);
    assert_eq!(mgr.buffer_count(), 0);
}

#[test]
fn test_compute_context_tracking() {
    let mut ctx = ComputeContext::new(0x01);
    ctx.record_failure();
    ctx.allocate_memory(4096);
    ctx.deallocate_memory(1024);
    assert_eq!(ctx.tasks_failed, 1);
    assert_eq!(ctx.memory_allocated, 3072);
}

#[test]
fn test_compute_bridge_execute() {
    let mut bridge = ComputeBridge::new();
    let task = ComputeTask::new(0x01, ComputeKind::HashSha256, b"bridge test".to_vec());
    let result = bridge.execute_task(&task).unwrap();
    assert_eq!(result.output.len(), 32);
    assert!(result.elapsed_us >= 0);
}
