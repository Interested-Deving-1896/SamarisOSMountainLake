#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Zstd { level: i32 },
    Lz4,
}

impl CompressionAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            CompressionAlgorithm::None => "none",
            CompressionAlgorithm::Zstd { .. } => "zstd",
            CompressionAlgorithm::Lz4 => "lz4",
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            CompressionAlgorithm::None => false,
            CompressionAlgorithm::Zstd { .. } => cfg!(feature = "compression"),
            CompressionAlgorithm::Lz4 => cfg!(feature = "compression"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_name() {
        assert_eq!(CompressionAlgorithm::None.name(), "none");
        assert_eq!(CompressionAlgorithm::Zstd { level: 3 }.name(), "zstd");
        assert_eq!(CompressionAlgorithm::Lz4.name(), "lz4");
    }

    #[test]
    fn test_none_disabled() {
        assert!(!CompressionAlgorithm::None.is_enabled());
    }
}
