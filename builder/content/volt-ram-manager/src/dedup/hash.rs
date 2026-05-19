pub fn sha256(data: &[u8]) -> [u8; 32] {
    #[cfg(feature = "dedup")]
    {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        result.into()
    }
    #[cfg(not(feature = "dedup"))]
    {
        let _ = data;
        [0u8; 32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "dedup")]
    fn test_sha256_known() {
        let result = sha256(b"hello");
        assert_eq!(result[0], 0x2c);
        assert_eq!(result[1], 0xf2);
    }

    #[test]
    fn test_sha256_length() {
        let result = sha256(b"test data");
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_deterministic() {
        let a = sha256(b"same data");
        let b = sha256(b"same data");
        assert_eq!(a, b);
    }

    #[test]
    fn test_different_inputs() {
        let a = sha256(b"input a");
        let b = sha256(b"input b");
        if cfg!(feature = "dedup") {
            assert_ne!(a, b);
        }
    }
}
