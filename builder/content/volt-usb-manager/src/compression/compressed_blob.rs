use crate::compression::algorithm::CompressionAlgorithm;
use crc32fast::Hasher;

pub struct CompressedBlob {
    pub algorithm: CompressionAlgorithm,
    pub uncompressed_size: u64,
    pub compressed_size: u64,
    pub data: Vec<u8>,
    pub checksum: u32,
}

impl CompressedBlob {
    pub fn new(algo: CompressionAlgorithm, original: &[u8], compressed: Vec<u8>) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(&compressed);
        let checksum = hasher.finalize();
        CompressedBlob {
            algorithm: algo,
            uncompressed_size: original.len() as u64,
            compressed_size: compressed.len() as u64,
            data: compressed,
            checksum,
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.uncompressed_size == 0 {
            return 1.0;
        }
        self.compressed_size as f64 / self.uncompressed_size as f64
    }

    pub fn savings(&self) -> u64 {
        self.uncompressed_size.saturating_sub(self.compressed_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blob() {
        let original = b"Hello, world!";
        let compressed = vec![0x1f, 0x8b, 0x08];
        let blob = CompressedBlob::new(CompressionAlgorithm::None, original, compressed.clone());
        assert_eq!(blob.algorithm, CompressionAlgorithm::None);
        assert_eq!(blob.uncompressed_size, 13);
        assert_eq!(blob.compressed_size, 3);
        assert_eq!(blob.data, compressed);
    }

    #[test]
    fn test_checksum() {
        let original = b"test data";
        let compressed = b"compressed".to_vec();
        let blob = CompressedBlob::new(CompressionAlgorithm::Lz4, original, compressed);
        let mut hasher = Hasher::new();
        hasher.update(b"compressed");
        assert_eq!(blob.checksum, hasher.finalize());
    }

    #[test]
    fn test_ratio() {
        let blob = CompressedBlob::new(
            CompressionAlgorithm::Zstd { level: 3 },
            &[0u8; 100],
            vec![0u8; 30],
        );
        assert!((blob.ratio() - 0.3).abs() < 0.001);
    }

    #[test]
    fn test_savings() {
        let blob = CompressedBlob::new(
            CompressionAlgorithm::Zstd { level: 3 },
            &[0u8; 100],
            vec![0u8; 30],
        );
        assert_eq!(blob.savings(), 70);
    }

    #[test]
    fn test_savings_zero_when_larger() {
        let blob = CompressedBlob::new(
            CompressionAlgorithm::None,
            &[0u8; 10],
            vec![0u8; 100],
        );
        assert_eq!(blob.savings(), 0);
    }
}
