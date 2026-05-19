use crate::core::result::VrmResult;
use crate::core::error::VrmError;

pub struct ZstdBackend;

impl ZstdBackend {
    pub fn compress(data: &[u8], level: i32) -> VrmResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            use std::io::Read;
            let mut encoder = zstd::stream::Encoder::new(Vec::new(), level)
                .map_err(|e| VrmError::CompressionFailed(e.to_string()))?;
            std::io::Write::write_all(&mut encoder, data)
                .map_err(|e| VrmError::CompressionFailed(e.to_string()))?;
            let compressed = encoder
                .finish()
                .map_err(|e| VrmError::CompressionFailed(e.to_string()))?;
            Ok(compressed)
        }
        #[cfg(not(feature = "compression"))]
        {
            let _ = level;
            Err(VrmError::CompressionFailed(
                "compression feature not enabled".into(),
            ))
        }
    }

    pub fn decompress(data: &[u8]) -> VrmResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            use std::io::Read;
            let mut decoder = zstd::stream::Decoder::new(data)
                .map_err(|e| VrmError::DecompressionFailed(e.to_string()))?;
            let mut buf = Vec::new();
            decoder
                .read_to_end(&mut buf)
                .map_err(|e| VrmError::DecompressionFailed(e.to_string()))?;
            Ok(buf)
        }
        #[cfg(not(feature = "compression"))]
        {
            Err(VrmError::DecompressionFailed(
                "compression feature not enabled".into(),
            ))
        }
    }
}

#[cfg(test)]
#[cfg(feature = "compression")]
mod tests {
    use super::*;

    #[test]
    fn test_zstd_roundtrip() {
        let data = b"hello world this is test data for zstd compression";
        let compressed = ZstdBackend::compress(data, 3).unwrap();
        let decompressed = ZstdBackend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zstd_compression_reduces_size() {
        let data = vec![0u8; 1024];
        let compressed = ZstdBackend::compress(&data, 3).unwrap();
        assert!(compressed.len() < data.len());
    }
}
