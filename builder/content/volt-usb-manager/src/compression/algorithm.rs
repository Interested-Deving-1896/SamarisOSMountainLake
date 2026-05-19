#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Zstd { level: i32 },
    Lz4,
}

impl CompressionAlgorithm {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "zstd" => CompressionAlgorithm::Zstd { level: 3 },
            "lz4" => CompressionAlgorithm::Lz4,
            _ => CompressionAlgorithm::None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            CompressionAlgorithm::None => "none",
            CompressionAlgorithm::Zstd { .. } => "zstd",
            CompressionAlgorithm::Lz4 => "lz4",
        }
    }

    pub fn is_enabled(&self) -> bool {
        #[cfg(feature = "compression")]
        {
            !matches!(self, CompressionAlgorithm::None)
        }
        #[cfg(not(feature = "compression"))]
        {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name() {
        assert_eq!(CompressionAlgorithm::from_name("zstd"), CompressionAlgorithm::Zstd { level: 3 });
        assert_eq!(CompressionAlgorithm::from_name("ZSTD"), CompressionAlgorithm::Zstd { level: 3 });
        assert_eq!(CompressionAlgorithm::from_name("lz4"), CompressionAlgorithm::Lz4);
        assert_eq!(CompressionAlgorithm::from_name("LZ4"), CompressionAlgorithm::Lz4);
        assert_eq!(CompressionAlgorithm::from_name("none"), CompressionAlgorithm::None);
        assert_eq!(CompressionAlgorithm::from_name("unknown"), CompressionAlgorithm::None);
    }

    #[test]
    fn test_name() {
        assert_eq!(CompressionAlgorithm::None.name(), "none");
        assert_eq!(CompressionAlgorithm::Zstd { level: 3 }.name(), "zstd");
        assert_eq!(CompressionAlgorithm::Lz4.name(), "lz4");
    }

    #[test]
    fn test_equality() {
        assert_eq!(CompressionAlgorithm::Zstd { level: 3 }, CompressionAlgorithm::Zstd { level: 3 });
        assert_ne!(CompressionAlgorithm::Zstd { level: 1 }, CompressionAlgorithm::Zstd { level: 3 });
        assert_ne!(CompressionAlgorithm::Lz4, CompressionAlgorithm::None);
    }
}
