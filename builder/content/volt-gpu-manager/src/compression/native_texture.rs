use crate::core::error::VgmError;
use crate::core::result::VgmResult;

#[cfg(feature = "native_texture_compression")]
pub fn compress_native(data: &[u8]) -> VgmResult<(Vec<u8>, u64)> {
    let cksum = crc32fast::hash(data);
    let raw_len = data.len() as u64;
    let mut out = Vec::with_capacity(data.len() + 24);
    out.extend_from_slice(b"NAT1");
    out.extend_from_slice(&raw_len.to_le_bytes());
    out.extend_from_slice(&cksum.to_le_bytes());
    out.extend_from_slice(data);
    let len = out.len() as u64;
    Ok((out, len))
}

#[cfg(feature = "native_texture_compression")]
pub fn decompress_native(data: &[u8], original_size: usize) -> VgmResult<Vec<u8>> {
    if data.len() < 24 {
        return Err(VgmError::DecompressionFailed(
            "native texture data too short".into(),
        ));
    }
    if &data[0..4] != b"NAT1" {
        return Err(VgmError::DecompressionFailed(
            "native texture magic mismatch".into(),
        ));
    }
    let stored_cksum = u32::from_le_bytes(data[12..16].try_into().unwrap());
    let actual = &data[16..];
    let computed = crc32fast::hash(actual);
    if stored_cksum != computed {
        return Err(VgmError::ChecksumMismatch(format!(
            "native texture CRC mismatch: stored={:#x} computed={:#x}",
            stored_cksum, computed
        )));
    }
    if actual.len() != original_size {
        return Err(VgmError::DecompressionFailed(format!(
            "native texture size mismatch: expected {} got {}",
            original_size,
            actual.len()
        )));
    }
    Ok(actual.to_vec())
}

#[cfg(not(feature = "native_texture_compression"))]
pub fn compress_native(_data: &[u8]) -> VgmResult<(Vec<u8>, u64)> {
    Err(VgmError::UnsupportedFeature(
        "native_texture_compression feature not enabled".into(),
    ))
}

#[cfg(not(feature = "native_texture_compression"))]
pub fn decompress_native(_data: &[u8], _original_size: usize) -> VgmResult<Vec<u8>> {
    Err(VgmError::UnsupportedFeature(
        "native_texture_compression feature not enabled".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_texture_roundtrip() {
        let data = b"native texture test data";
        let result = compress_native(data);
        #[cfg(feature = "native_texture_compression")]
        {
            let (compressed, _) = result.unwrap();
            let decompressed = decompress_native(&compressed, data.len()).unwrap();
            assert_eq!(decompressed, data);
        }
        #[cfg(not(feature = "native_texture_compression"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_native_texture_empty() {
        let data = b"";
        let result = compress_native(data);
        #[cfg(feature = "native_texture_compression")]
        {
            if let Ok((compressed, _)) = result {
                if let Ok(decompressed) = decompress_native(&compressed, 0) {
                    assert!(decompressed.is_empty());
                }
            }
        }
        #[cfg(not(feature = "native_texture_compression"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_native_texture_short_data() {
        let result = decompress_native(b"short", 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_native_texture_magic_mismatch() {
        let data = [0u8; 24];
        let result = decompress_native(&data, 4);
        #[cfg(feature = "native_texture_compression")]
        {
            assert!(result.is_err());
        }
        #[cfg(not(feature = "native_texture_compression"))]
        {
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_error_type() {
        let result = compress_native(b"test");
        #[cfg(not(feature = "native_texture_compression"))]
        {
            assert!(matches!(result, Err(VgmError::UnsupportedFeature(_))));
        }
    }
}
