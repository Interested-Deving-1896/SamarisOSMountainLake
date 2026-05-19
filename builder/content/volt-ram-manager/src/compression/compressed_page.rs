use crate::compression::algorithm::CompressionAlgorithm;

#[derive(Debug, Clone)]
pub struct CompressedPage {
    pub original_size: u64,
    pub compressed_size: u64,
    pub algorithm: CompressionAlgorithm,
    pub data: Vec<u8>,
}

impl CompressedPage {
    pub fn new(original_size: u64, algorithm: CompressionAlgorithm, data: Vec<u8>) -> Self {
        let compressed_size = data.len() as u64;
        CompressedPage {
            original_size,
            compressed_size,
            algorithm,
            data,
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 1.0;
        }
        self.compressed_size as f64 / self.original_size as f64
    }

    pub fn savings(&self) -> u64 {
        if self.original_size > self.compressed_size {
            self.original_size - self.compressed_size
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_page() {
        let page = CompressedPage::new(100, CompressionAlgorithm::None, vec![1, 2, 3]);
        assert_eq!(page.original_size, 100);
        assert_eq!(page.compressed_size, 3);
    }

    #[test]
    fn test_ratio() {
        let page = CompressedPage::new(100, CompressionAlgorithm::None, vec![0u8; 50]);
        assert!((page.ratio() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_zero_original_ratio() {
        let page = CompressedPage::new(0, CompressionAlgorithm::None, vec![0u8; 10]);
        assert!((page.ratio() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_savings() {
        let page = CompressedPage::new(100, CompressionAlgorithm::None, vec![0u8; 30]);
        assert_eq!(page.savings(), 70);
    }

    #[test]
    fn test_no_savings_when_larger() {
        let page = CompressedPage::new(10, CompressionAlgorithm::None, vec![0u8; 100]);
        assert_eq!(page.savings(), 0);
    }
}
