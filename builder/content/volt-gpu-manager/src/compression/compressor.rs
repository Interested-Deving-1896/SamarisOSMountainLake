use crate::compression::algorithm::GpuCompressionAlgorithm;
use crate::compression::cpu_fallback::CpuFallback;
use crate::core::result::VgmResult;

pub struct GpuCompressor;

impl GpuCompressor {
    pub fn compress(data: &[u8], algo: GpuCompressionAlgorithm) -> VgmResult<(Vec<u8>, u64)> {
        match algo {
            GpuCompressionAlgorithm::None => Ok((data.to_vec(), data.len() as u64)),
            GpuCompressionAlgorithm::VgmZstdL1
            | GpuCompressionAlgorithm::VgmLz4
            | GpuCompressionAlgorithm::CpuFallbackZstd
            | GpuCompressionAlgorithm::Zstd
            | GpuCompressionAlgorithm::Lz4 => {
                CpuFallback::compress(data, algo)
            }
            GpuCompressionAlgorithm::Crc => {
                let crc = crate::compression::checksum::crc32(data);
                let mut result = crc.to_le_bytes().to_vec();
                result.extend_from_slice(data);
                let len = result.len() as u64;
                Ok((result, len))
            }
            GpuCompressionAlgorithm::NativeTextureCompression => {
                #[cfg(feature = "native_texture_compression")]
                {
                    crate::compression::native_texture::compress_native(data)
                }
                #[cfg(not(feature = "native_texture_compression"))]
                {
                    Err(crate::core::error::VgmError::UnsupportedFeature(
                        "native_texture_compression feature not enabled".into(),
                    ))
                }
            }
        }
    }

    pub fn decompress(
        data: &[u8],
        original_size: u64,
        algo: GpuCompressionAlgorithm,
    ) -> VgmResult<Vec<u8>> {
        match algo {
            GpuCompressionAlgorithm::None => Ok(data.to_vec()),
            GpuCompressionAlgorithm::VgmZstdL1
            | GpuCompressionAlgorithm::VgmLz4
            | GpuCompressionAlgorithm::CpuFallbackZstd
            | GpuCompressionAlgorithm::Zstd
            | GpuCompressionAlgorithm::Lz4 => {
                CpuFallback::decompress(data, original_size, algo)
            }
            GpuCompressionAlgorithm::NativeTextureCompression => {
                #[cfg(feature = "native_texture_compression")]
                {
                    crate::compression::native_texture::decompress_native(
                        data,
                        original_size as usize,
                    )
                }
                #[cfg(not(feature = "native_texture_compression"))]
                {
                    Err(crate::core::error::VgmError::UnsupportedFeature(
                        "native_texture_compression feature not enabled".into(),
                    ))
                }
            }
            GpuCompressionAlgorithm::Crc => {
                if data.len() < 4 {
                    return Err(crate::core::error::VgmError::DecompressionFailed(
                        "CRC data too short".into(),
                    ));
                }
                let stored_crc = u32::from_le_bytes(data[0..4].try_into().unwrap());
                let actual = &data[4..];
                let computed = crate::compression::checksum::crc32(actual);
                if stored_crc != computed {
                    return Err(crate::core::error::VgmError::ChecksumMismatch(format!(
                        "CRC mismatch: stored={:#x}, computed={:#x}",
                        stored_crc, computed
                    )));
                }
                Ok(actual.to_vec())
            }
        }
    }

    pub fn select_algorithm(
        can_use_gpu: bool,
        is_texture: bool,
        size_bytes: u64,
    ) -> GpuCompressionAlgorithm {
        if is_texture {
            return GpuCompressionAlgorithm::NativeTextureCompression;
        }
        if can_use_gpu {
            if size_bytes < 4096 {
                GpuCompressionAlgorithm::VgmLz4
            } else {
                GpuCompressionAlgorithm::VgmZstdL1
            }
        } else {
            GpuCompressionAlgorithm::CpuFallbackZstd
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_roundtrip() {
        let data = b"hello world";
        let (compressed, size) = GpuCompressor::compress(data, GpuCompressionAlgorithm::None).unwrap();
        assert_eq!(size as usize, data.len());
        let decompressed = GpuCompressor::decompress(&compressed, size, GpuCompressionAlgorithm::None).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_zstd_fallback_roundtrip() {
        let data = b"test data for zstd fallback";
        let (compressed, _) = GpuCompressor::compress(data, GpuCompressionAlgorithm::CpuFallbackZstd).unwrap();
        let decompressed = GpuCompressor::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::CpuFallbackZstd).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_lz4_fallback_roundtrip() {
        let data = b"lz4 test payload here";
        let (compressed, _) = GpuCompressor::compress(data, GpuCompressionAlgorithm::VgmLz4).unwrap();
        let decompressed = GpuCompressor::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::VgmLz4).unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_select_algorithm_no_gpu_small() {
        let algo = GpuCompressor::select_algorithm(false, false, 100);
        assert_eq!(algo, GpuCompressionAlgorithm::CpuFallbackZstd);
    }

    #[test]
    fn test_select_algorithm_gpu_small() {
        let algo = GpuCompressor::select_algorithm(true, false, 100);
        assert_eq!(algo, GpuCompressionAlgorithm::VgmLz4);
    }

    #[test]
    fn test_select_algorithm_gpu_large() {
        let algo = GpuCompressor::select_algorithm(true, false, 100_000);
        assert_eq!(algo, GpuCompressionAlgorithm::VgmZstdL1);
    }

    #[test]
    fn test_select_algorithm_texture() {
        let algo = GpuCompressor::select_algorithm(true, true, 100);
        assert_eq!(algo, GpuCompressionAlgorithm::NativeTextureCompression);
    }

    #[test]
    fn test_compress_empty() {
        let data = b"";
        let (compressed, size) = GpuCompressor::compress(data, GpuCompressionAlgorithm::None).unwrap();
        assert_eq!(size, 0);
        assert!(compressed.is_empty());
    }
}
