use crate::core::result::VumResult;
use crate::core::error::VumError;

pub struct Lz4Backend;

impl Lz4Backend {
    pub fn compress(data: &[u8]) -> VumResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            Ok(lz4_flex::block::compress_prepend_size(data))
        }
        #[cfg(not(feature = "compression"))]
        {
            Ok(data.to_vec())
        }
    }

    pub fn decompress(data: &[u8]) -> VumResult<Vec<u8>> {
        #[cfg(feature = "compression")]
        {
            lz4_flex::block::decompress_size_prepended(data)
                .map_err(|e| VumError::DecompressionFailed(e.to_string()))
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
        let data = b"Hello, LZ4 compression backend!";
        let compressed = Lz4Backend::compress(data).unwrap();
        let decompressed = Lz4Backend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_empty() {
        let data = b"";
        let compressed = Lz4Backend::compress(data).unwrap();
        let decompressed = Lz4Backend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_repeated_bytes() {
        let data = vec![0x42u8; 5000];
        let compressed = Lz4Backend::compress(&data).unwrap();

        #[cfg(feature = "compression")]
        assert!(compressed.len() < data.len());

        let decompressed = Lz4Backend::decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);
    }
}
