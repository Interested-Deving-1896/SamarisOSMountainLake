use crc32fast::Hasher;

pub fn crc32(data: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(data);
    hasher.finalize()
}

pub fn crc32_of_records(records: &[u8]) -> u32 {
    crc32(records)
}

pub fn verify_checksum(data: &[u8], expected: u32) -> bool {
    crc32(data) == expected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32_empty() {
        assert_eq!(crc32(b""), 0u32);
    }

    #[test]
    fn test_crc32_known() {
        let result = crc32(b"hello");
        assert_ne!(result, 0);
    }

    #[test]
    fn test_crc32_different_inputs_different_hashes() {
        let a = crc32(b"foo");
        let b = crc32(b"bar");
        assert_ne!(a, b);
    }

    #[test]
    fn test_crc32_of_records() {
        let data = b"record1record2record3";
        let direct = crc32(data);
        let via_records = crc32_of_records(data);
        assert_eq!(direct, via_records);
    }

    #[test]
    fn test_verify_checksum_valid() {
        let data = b"verify me";
        let hash = crc32(data);
        assert!(verify_checksum(data, hash));
    }

    #[test]
    fn test_verify_checksum_invalid() {
        let data = b"verify me";
        let hash = crc32(data);
        assert!(!verify_checksum(b"wrong data", hash));
    }

    #[test]
    fn test_crc32_large_input() {
        let large: Vec<u8> = (0..u8::MAX).cycle().take(10000).collect();
        let hash = crc32(&large);
        assert_ne!(hash, 0);
        assert!(verify_checksum(&large, hash));
    }
}
