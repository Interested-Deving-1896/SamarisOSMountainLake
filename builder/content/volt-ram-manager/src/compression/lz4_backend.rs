use crate::core::result::VrmResult;
use crate::core::error::VrmError;

pub struct Lz4Backend;

impl Lz4Backend {
    pub fn compress(data: &[u8]) -> VrmResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            return Ok(lz4_flex::block::compress_prepend_size(data));
        }
        #[cfg(not(feature = "compression"))]
        {
            Err(VrmError::CompressionFailed("compression feature not enabled".into()))
        }
    }

    pub fn decompress(data: &[u8]) -> VrmResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            lz4_flex::block::decompress_size_prepended(data)
                .map_err(|e| VrmError::DecompressionFailed(e.to_string()))
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
    fn test_lz4_roundtrip() {
        let data = b"hello world lz4 compression test";
        let compressed = Lz4Backend::compress(data).unwrap();
        let decompressed = Lz4Backend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lz4_compression_reduces_size() {
        let data = vec![0xABu8; 512];
        let compressed = Lz4Backend::compress(&data).unwrap();
        assert!(compressed.len() <= data.len() + 8);
    }
}
