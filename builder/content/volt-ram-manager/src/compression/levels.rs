use crate::compression::algorithm::CompressionAlgorithm;

pub fn default_level() -> i32 {
    3
}

pub fn ui_level() -> i32 {
    1
}

pub fn archive_level() -> i32 {
    5
}

pub fn for_kind(kind: &str) -> CompressionAlgorithm {
    match kind.trim().to_lowercase().as_str() {
        "zstd" | "zstandard" => CompressionAlgorithm::Zstd { level: default_level() },
        "lz4" => CompressionAlgorithm::Lz4,
        _ => CompressionAlgorithm::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_defaults() {
        assert_eq!(default_level(), 3);
        assert_eq!(ui_level(), 1);
        assert_eq!(archive_level(), 5);
    }

    #[test]
    fn test_for_kind() {
        assert_eq!(for_kind("zstd"), CompressionAlgorithm::Zstd { level: 3 });
        assert_eq!(for_kind("lz4"), CompressionAlgorithm::Lz4);
        assert_eq!(for_kind("unknown"), CompressionAlgorithm::None);
    }
}
