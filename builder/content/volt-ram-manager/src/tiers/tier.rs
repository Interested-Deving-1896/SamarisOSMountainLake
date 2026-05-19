use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryTier {
    #[serde(rename = "t1_shm")]
    T1Shm,
    #[serde(rename = "t2_direct")]
    T2Direct,
    #[serde(rename = "t3_compressed")]
    T3Compressed,
}

impl MemoryTier {
    pub fn is_compressible(&self) -> bool {
        matches!(self, Self::T3Compressed)
    }

    pub fn is_volatile(&self) -> bool {
        matches!(self, Self::T2Direct | Self::T3Compressed)
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::T1Shm => "t1_shm",
            Self::T2Direct => "t2_direct",
            Self::T3Compressed => "t3_compressed",
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Self::T1Shm => 0,
            Self::T2Direct => 1,
            Self::T3Compressed => 2,
        }
    }
}

impl Default for MemoryTier {
    fn default() -> Self {
        Self::T1Shm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_name() {
        assert_eq!(MemoryTier::T1Shm.name(), "t1_shm");
        assert_eq!(MemoryTier::T2Direct.name(), "t2_direct");
        assert_eq!(MemoryTier::T3Compressed.name(), "t3_compressed");
    }

    #[test]
    fn test_tier_priority() {
        assert_eq!(MemoryTier::T1Shm.priority(), 0);
        assert_eq!(MemoryTier::T2Direct.priority(), 1);
        assert_eq!(MemoryTier::T3Compressed.priority(), 2);
    }

    #[test]
    fn test_is_compressible() {
        assert!(!MemoryTier::T1Shm.is_compressible());
        assert!(!MemoryTier::T2Direct.is_compressible());
        assert!(MemoryTier::T3Compressed.is_compressible());
    }

    #[test]
    fn test_is_volatile() {
        assert!(!MemoryTier::T1Shm.is_volatile());
        assert!(MemoryTier::T2Direct.is_volatile());
        assert!(MemoryTier::T3Compressed.is_volatile());
    }

    #[test]
    fn test_default() {
        assert_eq!(MemoryTier::default(), MemoryTier::T1Shm);
    }

    #[test]
    fn test_equality() {
        assert_eq!(MemoryTier::T1Shm, MemoryTier::T1Shm);
        assert_ne!(MemoryTier::T1Shm, MemoryTier::T2Direct);
    }
}
