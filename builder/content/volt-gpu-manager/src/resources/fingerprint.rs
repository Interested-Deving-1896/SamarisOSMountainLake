use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GpuFingerprint(u64);

impl GpuFingerprint {
    pub fn from_data(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let fb = &result[..];
        let truncated = u64::from_le_bytes([
            fb[0], fb[1], fb[2], fb[3],
            fb[4], fb[5], fb[6], fb[7],
        ]);
        Self(truncated)
    }

    pub fn from_parts(parts: &[&[u8]]) -> Self {
        let mut hasher = Sha256::new();
        for part in parts {
            hasher.update(part);
        }
        let result = hasher.finalize();
        let fb = &result[..];
        let truncated = u64::from_le_bytes([
            fb[0], fb[1], fb[2], fb[3],
            fb[4], fb[5], fb[6], fb[7],
        ]);
        Self(truncated)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }

    pub fn nil() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_data() {
        let fp = GpuFingerprint::from_data(b"hello world");
        assert_ne!(fp, GpuFingerprint::nil());
    }

    #[test]
    fn test_deterministic() {
        let a = GpuFingerprint::from_data(b"test data");
        let b = GpuFingerprint::from_data(b"test data");
        assert_eq!(a, b);
    }

    #[test]
    fn test_different_data_different_fingerprint() {
        let a = GpuFingerprint::from_data(b"data one");
        let b = GpuFingerprint::from_data(b"data two");
        assert_ne!(a, b);
    }

    #[test]
    fn test_from_parts() {
        let fp = GpuFingerprint::from_parts(&[b"hello", b" ", b"world"]);
        let expected = GpuFingerprint::from_data(b"hello world");
        assert_eq!(fp, expected);
    }

    #[test]
    fn test_nil() {
        assert_eq!(GpuFingerprint::nil().as_u64(), 0);
    }

    #[test]
    fn test_as_u64() {
        let fp = GpuFingerprint::from_data(b"test");
        let val = fp.as_u64();
        assert_eq!(GpuFingerprint::from_data(b"test").as_u64(), val);
    }

    #[test]
    fn test_empty_data() {
        let fp = GpuFingerprint::from_data(b"");
        assert_ne!(fp, GpuFingerprint::nil());
    }

    #[test]
    fn test_hash_trait() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GpuFingerprint::from_data(b"a"));
        set.insert(GpuFingerprint::from_data(b"a"));
        set.insert(GpuFingerprint::from_data(b"b"));
        assert_eq!(set.len(), 2);
    }
}
