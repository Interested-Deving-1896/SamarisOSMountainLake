use crate::compression::algorithm::GpuCompressionAlgorithm;
use crate::compression::checksum;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct CpuFallback;

impl CpuFallback {
    pub fn compress(
        data: &[u8],
        algorithm: GpuCompressionAlgorithm,
    ) -> VgmResult<(Vec<u8>, u64)> {
        match algorithm {
            GpuCompressionAlgorithm::VgmZstdL1
            | GpuCompressionAlgorithm::CpuFallbackZstd
            | GpuCompressionAlgorithm::Zstd
            | GpuCompressionAlgorithm::Lz4 => {
                let raw_len = data.len() as u64;
                let cksum = checksum::crc32(data);
                let out = Self::store_with_checksum(data, raw_len, cksum);
                let len = out.len() as u64;
                Ok((out, len))
            }
            GpuCompressionAlgorithm::VgmLz4 => {
                let raw_len = data.len() as u64;
                let cksum = checksum::crc32(data);
                let out = Self::store_with_checksum(data, raw_len, cksum);
                let len = out.len() as u64;
                Ok((out, len))
            }
            GpuCompressionAlgorithm::NativeTextureCompression => {
                Err(VgmError::UnsupportedFeature(
                    "Native texture compression requires GPU".into(),
                ))
            }
            GpuCompressionAlgorithm::None | GpuCompressionAlgorithm::Crc => {
                Ok((data.to_vec(), data.len() as u64))
            }
        }
    }

    pub fn decompress(
        data: &[u8],
        original_size: u64,
        algorithm: GpuCompressionAlgorithm,
    ) -> VgmResult<Vec<u8>> {
        match algorithm {
            GpuCompressionAlgorithm::VgmZstdL1
            | GpuCompressionAlgorithm::CpuFallbackZstd
            | GpuCompressionAlgorithm::VgmLz4
            | GpuCompressionAlgorithm::Zstd
            | GpuCompressionAlgorithm::Lz4 => Self::extract_payload(data, original_size),
            GpuCompressionAlgorithm::NativeTextureCompression => {
                Err(VgmError::UnsupportedFeature(
                    "Native texture decompression requires GPU".into(),
                ))
            }
            GpuCompressionAlgorithm::None | GpuCompressionAlgorithm::Crc => Ok(data.to_vec()),
        }
    }

    fn store_with_checksum(data: &[u8], raw_len: u64, cksum: u32) -> Vec<u8> {
        let mut out = Vec::with_capacity(data.len() + 16);
        out.extend_from_slice(&raw_len.to_le_bytes());
        out.extend_from_slice(&cksum.to_le_bytes());
        out.extend_from_slice(data);
        out
    }

    fn extract_payload(data: &[u8], original_size: u64) -> VgmResult<Vec<u8>> {
        if data.len() < 16 {
            return Err(VgmError::DecompressionFailed(
                "compressed data too short for cpu_fallback format".into(),
            ));
        }
        let stored_cksum = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let actual = &data[12..];
        if !checksum::verify(actual, stored_cksum) {
            return Err(VgmError::ChecksumMismatch(format!(
                "cpu_fallback checksum mismatch: stored={:#x}",
                stored_cksum
            )));
        }
        if (actual.len() as u64) != original_size {
            return Err(VgmError::DecompressionFailed(format!(
                "size mismatch: expected {} got {}",
                original_size,
                actual.len()
            )));
        }
        Ok(actual.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_zstd_roundtrip() {
        let data = b"hello cpu zstd fallback";
        let (compressed, _) =
            CpuFallback::compress(data, GpuCompressionAlgorithm::CpuFallbackZstd).unwrap();
        let decompressed = CpuFallback::decompress(
            &compressed,
            data.len() as u64,
            GpuCompressionAlgorithm::CpuFallbackZstd,
        )
        .unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_legacy_zstd_roundtrip() {
        let data = b"legacy zstd";
        let (compressed, _) = CpuFallback::compress(data, GpuCompressionAlgorithm::Zstd).unwrap();
        let decompressed =
            CpuFallback::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::Zstd)
                .unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_legacy_lz4_roundtrip() {
        let data = b"legacy lz4";
        let (compressed, _) = CpuFallback::compress(data, GpuCompressionAlgorithm::Lz4).unwrap();
        let decompressed =
            CpuFallback::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::Lz4)
                .unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_cpu_vgmzstd_roundtrip() {
        let data = b"vgm zstd l1 data";
        let (compressed, _) =
            CpuFallback::compress(data, GpuCompressionAlgorithm::VgmZstdL1).unwrap();
        let decompressed = CpuFallback::decompress(
            &compressed,
            data.len() as u64,
            GpuCompressionAlgorithm::VgmZstdL1,
        )
        .unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_cpu_lz4_roundtrip() {
        let data = b"lz4 data here";
        let (compressed, _) = CpuFallback::compress(data, GpuCompressionAlgorithm::VgmLz4).unwrap();
        let decompressed =
            CpuFallback::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::VgmLz4)
                .unwrap();
        assert_eq!(decompressed, data);
    }

    #[test]
    fn test_native_texture_returns_error() {
        let result = CpuFallback::compress(b"test", GpuCompressionAlgorithm::NativeTextureCompression);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_short_data() {
        let result = CpuFallback::decompress(b"too short", 100, GpuCompressionAlgorithm::VgmZstdL1);
        assert!(result.is_err());
    }

    #[test]
    fn test_checksum_mismatch() {
        let data = b"test data for checksum test";
        let (mut compressed, _) =
            CpuFallback::compress(data, GpuCompressionAlgorithm::CpuFallbackZstd).unwrap();
        if compressed.len() > 12 {
            compressed[12] ^= 0xff;
        }
        let result =
            CpuFallback::decompress(&compressed, data.len() as u64, GpuCompressionAlgorithm::CpuFallbackZstd);
        assert!(result.is_err());
    }

    #[test]
    fn test_size_mismatch() {
        let data = b"size check data";
        let (compressed, _) = CpuFallback::compress(data, GpuCompressionAlgorithm::VgmLz4).unwrap();
        let result = CpuFallback::decompress(&compressed, 9999, GpuCompressionAlgorithm::VgmLz4);
        assert!(result.is_err());
    }
}
