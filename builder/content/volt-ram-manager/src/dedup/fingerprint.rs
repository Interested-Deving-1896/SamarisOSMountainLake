use crate::dedup::hash::sha256;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fingerprint(pub u64);

impl Fingerprint {
    pub fn from_data(data: &[u8]) -> Self {
        let hash = sha256(data);
        Fingerprint::from(hash)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<[u8; 32]> for Fingerprint {
    fn from(bytes: [u8; 32]) -> Self {
        let low = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        Fingerprint(low)
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fp({:016x})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_data() {
        let fp = Fingerprint::from_data(b"test");
        assert_ne!(fp.as_u64(), 0);
    }

    #[test]
    fn test_from_hash() {
        let hash = [0x01u8; 32];
        let fp = Fingerprint::from(hash);
        assert_eq!(fp.as_u64(), 0x0101010101010101);
    }

    #[test]
    fn test_display() {
        let fp = Fingerprint(0xABCD);
        let s = format!("{}", fp);
        assert!(s.contains("abcd"));
    }

    #[test]
    fn test_equality() {
        let a = Fingerprint(42);
        let b = Fingerprint(42);
        let c = Fingerprint(43);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
