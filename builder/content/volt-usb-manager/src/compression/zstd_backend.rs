use crate::core::result::VumResult;
use crate::core::error::VumError;

pub struct ZstdBackend;

impl ZstdBackend {
    pub fn compress(data: &[u8], level: i32) -> VumResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            let mut cursor = std::io::Cursor::new(Vec::new());
            zstd::stream::copy_encode(data, &mut cursor, level)
                .map_err(|e| VumError::CompressionFailed(e.to_string()))?;
            Ok(cursor.into_inner())
        }
        #[cfg(not(feature = "compression"))]
        {
            let _ = level;
            Ok(data.to_vec())
        }
    }

    pub fn decompress(data: &[u8]) -> VumResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            let mut cursor = std::io::Cursor::new(Vec::new());
            zstd::stream::copy_decode(data, &mut cursor)
                .map_err(|e| VumError::DecompressionFailed(e.to_string()))?;
            Ok(cursor.into_inner())
        }
        #[cfg(not(feature = "compression"))]
        {
            Ok(data.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_roundtrip() {
        let data = vec![0xABu8; 5000];
        let compressed = ZstdBackend::compress(&data, 3).unwrap();
        let decompressed = ZstdBackend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_empty() {
        let data = b"";
        let compressed = ZstdBackend::compress(data, 1).unwrap();
        let decompressed = ZstdBackend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_large_data() {
        let data = vec![0xABu8; 10000];
        let compressed = ZstdBackend::compress(&data, 3).unwrap();
        let decompressed = ZstdBackend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
