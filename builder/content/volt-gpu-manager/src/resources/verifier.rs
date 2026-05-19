use crate::resources::fingerprint::GpuFingerprint;
use crate::core::result::VgmResult;

pub struct GpuVerifier;

impl GpuVerifier {
    pub fn verify(data: &[u8], expected: &GpuFingerprint) -> VgmResult<bool> {
        let actual = GpuFingerprint::from_data(data);
        if &actual == expected {
            Ok(true)
        } else {
            Err(crate::core::error::VgmError::VerificationFailed(format!(
                "Fingerprint mismatch: expected {:016x}, got {:016x}",
                expected.as_u64(),
                actual.as_u64()
            )))
        }
    }

    pub fn verify_integrity(original: &[u8], restored: &[u8]) -> VgmResult<bool> {
        if original.len() != restored.len() {
            return Err(crate::core::error::VgmError::VerificationFailed(format!(
                "Size mismatch: original {} bytes, restored {} bytes",
                original.len(),
                restored.len()
            )));
        }
        if original == restored {
            Ok(true)
        } else {
            Err(crate::core::error::VgmError::VerificationFailed(
                "Data content mismatch after restore".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_success() {
        let data = b"verify me";
        let fp = GpuFingerprint::from_data(data);
        assert!(GpuVerifier::verify(data, &fp).is_ok());
    }

    #[test]
    fn test_verify_failure() {
        let data = b"original";
        let wrong_fp = GpuFingerprint::from_data(b"different");
        let result = GpuVerifier::verify(data, &wrong_fp);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_integrity_match() {
        let original = b"hello world";
        let restored = b"hello world";
        assert!(GpuVerifier::verify_integrity(original, restored).is_ok());
    }

    #[test]
    fn test_verify_integrity_size_mismatch() {
        let original = b"hello";
        let restored = b"hello!";
        let result = GpuVerifier::verify_integrity(original, restored);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_integrity_content_mismatch() {
        let original = b"hello world";
        let restored = b"hello WORLd";
        let result = GpuVerifier::verify_integrity(original, restored);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_empty_data() {
        let data = b"";
        let fp = GpuFingerprint::from_data(data);
        assert!(GpuVerifier::verify(data, &fp).is_ok());
    }

    #[test]
    fn test_verify_integrity_empty() {
        let data: &[u8] = &[];
        assert!(GpuVerifier::verify_integrity(data, data).is_ok());
    }
}
