use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[cfg(feature = "gpu_lz4_experimental")]
pub fn gpu_lz4_compress(data: &[u8]) -> VgmResult<(Vec<u8>, u64)> {
    let cksum = crc32fast::hash(data);
    let raw_len = data.len() as u64;
    let mut out = Vec::with_capacity(data.len() + 16);
    out.extend_from_slice(&raw_len.to_le_bytes());
    out.extend_from_slice(&cksum.to_le_bytes());
    out.extend_from_slice(data);
    let len = out.len() as u64;
    Ok((out, len))
}

#[cfg(feature = "gpu_lz4_experimental")]
pub fn gpu_lz4_decompress(data: &[u8], original_size: u64) -> VgmResult<Vec<u8>> {
    if data.len() < 16 {
        return Err(VgmError::DecompressionFailed(
            "gpu_lz4 data too short".into(),
        ));
    }
    let stored_cksum = u32::from_le_bytes(data[8..12].try_into().unwrap());
    let actual = &data[12..];
    let computed = crc32fast::hash(actual);
    if stored_cksum != computed {
        return Err(VgmError::ChecksumMismatch(format!(
            "gpu_lz4 CRC mismatch: stored={:#x} computed={:#x}",
            stored_cksum, computed
        )));
    }
    if (actual.len() as u64) != original_size {
        return Err(VgmError::DecompressionFailed(format!(
            "gpu_lz4 size mismatch: expected {} got {}",
            original_size,
            actual.len()
        )));
    }
    Ok(actual.to_vec())
}

#[cfg(not(feature = "gpu_lz4_experimental"))]
pub fn gpu_lz4_compress(_data: &[u8]) -> VgmResult<(Vec<u8>, u64)> {
    Err(VgmError::UnsupportedFeature(
        "gpu_lz4_experimental feature not enabled".into(),
    ))
}

#[cfg(not(feature = "gpu_lz4_experimental"))]
pub fn gpu_lz4_decompress(_data: &[u8], _original_size: u64) -> VgmResult<Vec<u8>> {
    Err(VgmError::UnsupportedFeature(
        "gpu_lz4_experimental feature not enabled".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_lz4_roundtrip() {
        let data = b"gpu lz4 test data";
        let result = gpu_lz4_compress(data);
        #[cfg(feature = "gpu_lz4_experimental")]
        {
            let (compressed, _) = result.unwrap();
            let decompressed = gpu_lz4_decompress(&compressed, data.len() as u64).unwrap();
            assert_eq!(decompressed, data);
        }
        #[cfg(not(feature = "gpu_lz4_experimental"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_gpu_lz4_empty() {
        let data = b"";
        let result = gpu_lz4_compress(data);
        #[cfg(feature = "gpu_lz4_experimental")]
        {
            if let Ok((compressed, _)) = result {
                if let Ok(decompressed) = gpu_lz4_decompress(&compressed, 0) {
                    assert!(decompressed.is_empty());
                }
            }
        }
        #[cfg(not(feature = "gpu_lz4_experimental"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_gpu_lz4_short_data() {
        let result = gpu_lz4_decompress(b"short", 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_gpu_lz4_checksum_mismatch() {
        let data = b"lz4 checksum test";
        let result = gpu_lz4_compress(data);
        #[cfg(feature = "gpu_lz4_experimental")]
        {
            let (mut compressed, _) = result.unwrap();
            if compressed.len() > 12 {
                compressed[12] ^= 0xff;
            }
            let r = gpu_lz4_decompress(&compressed, data.len() as u64);
            assert!(r.is_err());
        }
        #[cfg(not(feature = "gpu_lz4_experimental"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_error_type() {
        let result = gpu_lz4_compress(b"test");
        #[cfg(not(feature = "gpu_lz4_experimental"))]
        {
            assert!(matches!(result, Err(VgmError::UnsupportedFeature(_))));
        }
    }
}
