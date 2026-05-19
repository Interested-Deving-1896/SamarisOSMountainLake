use volt_gpu_manager::vram::compressed_pool::CompressedVramBlock;
use volt_gpu_manager::compression::GpuCompressionAlgorithm;
use volt_gpu_manager::resources::GpuResourceId;
use volt_gpu_manager::vram::t2_compressed::CompressedVramPool;

#[test]
fn insert_compressed_block() {
    let mut pool = CompressedVramPool::new(1024 * 1024);
    let id = GpuResourceId::new();
    let block = CompressedVramBlock::new(id, GpuCompressionAlgorithm::Zstd, 1000, 300, 0x1234);
    assert!(pool.insert(id, block).is_ok());
}

#[test]
fn used_bytes_increments() {
    let mut pool = CompressedVramPool::new(1024 * 1024);
    let id = GpuResourceId::new();
    let block = CompressedVramBlock::new(id, GpuCompressionAlgorithm::Zstd, 1000, 300, 0);
    pool.insert(id, block).unwrap();
    assert_eq!(pool.used_bytes, 300);
}

#[test]
fn saved_bytes_calculated() {
    let mut pool = CompressedVramPool::new(1024 * 1024);
    let id = GpuResourceId::new();
    let block = CompressedVramBlock::new(id, GpuCompressionAlgorithm::Zstd, 1000, 300, 0);
    pool.insert(id, block).unwrap();
    assert_eq!(pool.compression_saved_bytes(), 700);
}

#[test]
fn checksum_mismatch_rejected() {
    let data = b"test data";
    let mut stored = data.to_vec();
    stored[0] ^= 0xFF;
    let actual = volt_gpu_manager::compression::checksum::crc32(data);
    let computed = volt_gpu_manager::compression::checksum::crc32(&stored);
    assert_ne!(actual, computed);
}

#[test]
fn pool_full_triggers_error() {
    let mut pool = CompressedVramPool::new(100);
    let id1 = GpuResourceId::new();
    let b1 = CompressedVramBlock::new(id1, GpuCompressionAlgorithm::None, 50, 50, 0);
    assert!(pool.insert(id1, b1).is_ok());
    let id2 = GpuResourceId::new();
    let b2 = CompressedVramBlock::new(id2, GpuCompressionAlgorithm::None, 100, 100, 0);
    assert!(pool.insert(id2, b2).is_err());
}

#[test]
fn duplicate_insert_rejected() {
    let mut pool = CompressedVramPool::new(1024);
    let id = GpuResourceId::new();
    let b = CompressedVramBlock::new(id, GpuCompressionAlgorithm::None, 10, 10, 0);
    pool.insert(id, b.clone()).unwrap();
    assert!(pool.insert(id, b).is_err());
}
