use crc32fast::Hasher;

pub fn crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn verify(data: &[u8], expected: u32) -> bool {
    crc32(data) == expected
}

pub fn combine(checksums: &[u32]) -> u32 {
    if checksums.is_empty() {
        return 0;
    }
    let mut combined = checksums[0];
    for &c in &checksums[1..] {
        let mut h = Hasher::new();
        h.update(&combined.to_le_bytes());
        h.update(&c.to_le_bytes());
        combined = h.finalize();
    }
    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_deterministic() {
        let data = b"hello world";
        assert_eq!(crc32(data), crc32(data));
    }

    #[test]
    fn test_crc32_empty() {
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn test_verify_ok() {
        let data = b"verify this";
        let hash = crc32(data);
        assert!(verify(data, hash));
    }

    #[test]
    fn test_verify_fail() {
        let data = b"verify this";
        assert!(!verify(data, 0xdeadbeef));
    }

    #[test]
    fn test_combine_empty() {
        assert_eq!(combine(&[]), 0);
    }

    #[test]
    fn test_combine_single() {
        let h = crc32(b"single");
        assert_eq!(combine(&[h]), h);
    }

    #[test]
    fn test_combine_multiple() {
        let a = crc32(b"part1");
        let b = crc32(b"part2");
        let combined = combine(&[a, b]);
        assert_ne!(combined, a);
        assert_ne!(combined, b);
    }

    #[test]
    fn test_crc32_differs_for_different_data() {
        let a = crc32(b"data one");
        let b = crc32(b"data two");
        assert_ne!(a, b);
    }

    #[test]
    fn test_verify_roundtrip() {
        let data = b"roundtrip verification test";
        let hash = crc32(data);
        assert!(verify(data, hash));
        assert!(!verify(data, hash.wrapping_add(1)));
    }
}
