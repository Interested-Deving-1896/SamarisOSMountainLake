use crate::compression::GpuCompressionAlgorithm;
use crate::resources::resource_id::GpuResourceId;

#[derive(Debug, Clone)]
pub struct CompressedVramBlock {
    pub resource_id: GpuResourceId,
    pub algorithm: GpuCompressionAlgorithm,
    pub original_size: u64,
    pub compressed_size: u64,
    pub checksum: u32,
    pub created_at_ms: u64,
    pub last_restore_ms: Option<u64>,
}

impl CompressedVramBlock {
    pub fn new(
        resource_id: GpuResourceId,
        algo: GpuCompressionAlgorithm,
        original: u64,
        compressed: u64,
        checksum: u32,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self {
            resource_id,
            algorithm: algo,
            original_size: original,
            compressed_size: compressed,
            checksum,
            created_at_ms: now,
            last_restore_ms: None,
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.compressed_size == 0 {
            return 0.0;
        }
        self.original_size as f64 / self.compressed_size as f64
    }

    pub fn savings(&self) -> u64 {
        self.original_size.saturating_sub(self.compressed_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_block() -> CompressedVramBlock {
        CompressedVramBlock::new(
            GpuResourceId::new(),
            GpuCompressionAlgorithm::Zstd,
            1000,
            300,
            0x12345678,
        )
    }

    #[test]
    fn test_new_block() {
        let block = make_block();
        assert_eq!(block.original_size, 1000);
        assert_eq!(block.compressed_size, 300);
        assert_eq!(block.checksum, 0x12345678);
        assert_eq!(block.algorithm, GpuCompressionAlgorithm::Zstd);
        assert!(block.last_restore_ms.is_none());
    }

    #[test]
    fn test_ratio() {
        let block = make_block();
        assert!((block.ratio() - 3.333).abs() < 0.01);
    }

    #[test]
    fn test_ratio_zero_compressed() {
        let block = CompressedVramBlock::new(
            GpuResourceId::new(),
            GpuCompressionAlgorithm::None,
            100,
            0,
            0,
        );
        assert_eq!(block.ratio(), 0.0);
    }

    #[test]
    fn test_savings() {
        let block = make_block();
        assert_eq!(block.savings(), 700);
    }

    #[test]
    fn test_savings_no_compression() {
        let block = CompressedVramBlock::new(
            GpuResourceId::new(),
            GpuCompressionAlgorithm::None,
            100,
            100,
            0,
        );
        assert_eq!(block.savings(), 0);
    }

    #[test]
    fn test_created_at_set() {
        let block = make_block();
        assert!(block.created_at_ms > 0);
    }

    #[test]
    fn test_different_algorithms() {
        let zstd = CompressedVramBlock::new(
            GpuResourceId::new(),
            GpuCompressionAlgorithm::Zstd,
            1000, 300, 0,
        );
        let lz4 = CompressedVramBlock::new(
            GpuResourceId::new(),
            GpuCompressionAlgorithm::Lz4,
            1000, 400, 0,
        );
        assert!(zstd.ratio() > lz4.ratio());
    }

    #[test]
    fn test_clone() {
        let block = make_block();
        let cloned = block.clone();
        assert_eq!(block.original_size, cloned.original_size);
        assert_eq!(block.checksum, cloned.checksum);
    }
}
