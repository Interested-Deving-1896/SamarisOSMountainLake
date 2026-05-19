use crate::compression::algorithm::CompressionAlgorithm;
use crate::compression::compressed_blob::CompressedBlob;
use crate::compression::zstd_backend::ZstdBackend;
use crate::compression::lz4_backend::Lz4Backend;
use crate::core::error::VumError;
use crate::core::result::VumResult;

pub struct Compressor;

impl Compressor {
    pub fn compress(data: &[u8], algo: CompressionAlgorithm) -> VumResult<CompressedBlob> {
        match algo {
            CompressionAlgorithm::None => Ok(CompressedBlob::new(algo, data, data.to_vec())),
            #[cfg(feature = "compression")]
            CompressionAlgorithm::Zstd { level } => {
                let compressed = ZstdBackend::compress(data, level)?;
                Ok(CompressedBlob::new(algo, data, compressed))
            }
            #[cfg(feature = "compression")]
            CompressionAlgorithm::Lz4 => {
                let compressed = Lz4Backend::compress(data)?;
                Ok(CompressedBlob::new(algo, data, compressed))
            }
            #[cfg(not(feature = "compression"))]
            CompressionAlgorithm::Zstd { .. } | CompressionAlgorithm::Lz4 => {
                Ok(CompressedBlob::new(CompressionAlgorithm::None, data, data.to_vec()))
            }
        }
    }

    pub fn decompress(blob: &CompressedBlob) -> VumResult<Vec<u8>> {
        match blob.algorithm {
            CompressionAlgorithm::None => Ok(blob.data.clone()),
            #[cfg(feature = "compression")]
            CompressionAlgorithm::Zstd { .. } => ZstdBackend::decompress(&blob.data),
            #[cfg(feature = "compression")]
            CompressionAlgorithm::Lz4 => Lz4Backend::decompress(&blob.data),
            #[cfg(not(feature = "compression"))]
            CompressionAlgorithm::Zstd { .. } | CompressionAlgorithm::Lz4 => {
                Err(VumError::DecompressionFailed(
                    "Compression feature not enabled".to_string(),
                ))
            }
        }
    }

    pub fn select_for_cache(path: &str, size: u64) -> CompressionAlgorithm {
        #[cfg(not(feature = "compression"))]
        {
            CompressionAlgorithm::None
        }

        #[cfg(feature = "compression")]
        {
            if size < 4096 {
                return CompressionAlgorithm::None;
            }

            let lower = path.to_lowercase();
            if lower.ends_with(".zip")
                || lower.ends_with(".gz")
                || lower.ends_with(".bz2")
                || lower.ends_with(".xz")
                || lower.ends_with(".zst")
                || lower.ends_with(".png")
                || lower.ends_with(".jpg")
                || lower.ends_with(".jpeg")
                || lower.ends_with(".gif")
                || lower.ends_with(".webp")
                || lower.ends_with(".mp4")
                || lower.ends_with(".mp3")
                || lower.ends_with(".ogg")
                || lower.ends_with(".woff")
                || lower.ends_with(".woff2")
            {
                return CompressionAlgorithm::None;
            }

            if size > 1024 * 1024 {
                CompressionAlgorithm::Zstd { level: 3 }
            } else {
                CompressionAlgorithm::Lz4
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_none_roundtrip() {
        let data = b"hello world";
        let blob = Compressor::compress(data, CompressionAlgorithm::None).unwrap();
        assert_eq!(blob.data, data);
        let decompressed = Compressor::decompress(&blob).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_compress_empty() {
        let data = b"";
        let blob = Compressor::compress(data, CompressionAlgorithm::None).unwrap();
        let decompressed = Compressor::decompress(&blob).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_select_for_cache_small() {
        let algo = Compressor::select_for_cache("/test/file.txt", 100);
        #[cfg(feature = "compression")]
        assert_eq!(algo, CompressionAlgorithm::None);
        #[cfg(not(feature = "compression"))]
        assert_eq!(algo, CompressionAlgorithm::None);
    }

    #[test]
    fn test_select_for_cache_already_compressed() {
        let algo = Compressor::select_for_cache("/test/image.png", 1024 * 1024);
        assert_eq!(algo, CompressionAlgorithm::None);
    }

    #[test]
    fn test_select_for_cache_large_file() {
        let algo = Compressor::select_for_cache("/test/data.bin", 5 * 1024 * 1024);
        #[cfg(feature = "compression")]
        assert_eq!(algo, CompressionAlgorithm::Zstd { level: 3 });
        #[cfg(not(feature = "compression"))]
        assert_eq!(algo, CompressionAlgorithm::None);
    }
}
