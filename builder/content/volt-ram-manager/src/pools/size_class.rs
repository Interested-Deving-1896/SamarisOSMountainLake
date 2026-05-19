use serde::de::{self, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SizeClass(pub u64);

impl<'de> serde::Deserialize<'de> for SizeClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SizeClassVisitor;
        impl Visitor<'_> for SizeClassVisitor {
            type Value = SizeClass;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a size class string like \"16B\", \"64B\", \"256B\", \"1KB\", \"4KB\", \"16KB\", or \"64KB\"")
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<SizeClass, E> {
                match s {
                    "16B" => Ok(SizeClass::B16),
                    "64B" => Ok(SizeClass::B64),
                    "256B" => Ok(SizeClass::B256),
                    "1KB" => Ok(SizeClass::KB1),
                    "4KB" => Ok(SizeClass::KB4),
                    "16KB" => Ok(SizeClass::KB16),
                    "64KB" => Ok(SizeClass::KB64),
                    _ => Err(E::custom(format!("unknown size class: {s}"))),
                }
            }
        }
        deserializer.deserialize_str(SizeClassVisitor)
    }
}

impl SizeClass {
    pub const B16: Self = Self(16);
    pub const B64: Self = Self(64);
    pub const B256: Self = Self(256);
    pub const KB1: Self = Self(1024);
    pub const KB4: Self = Self(4096);
    pub const KB16: Self = Self(16384);
    pub const KB64: Self = Self(65536);

    pub fn for_size(size: u64) -> Self {
        if size <= 16 {
            Self::B16
        } else if size <= 64 {
            Self::B64
        } else if size <= 256 {
            Self::B256
        } else if size <= 1024 {
            Self::KB1
        } else if size <= 4096 {
            Self::KB4
        } else if size <= 16384 {
            Self::KB16
        } else {
            Self::KB64
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::B16,
            Self::B64,
            Self::B256,
            Self::KB1,
            Self::KB4,
            Self::KB16,
            Self::KB64,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_size_exact() {
        assert_eq!(SizeClass::for_size(16), SizeClass::B16);
        assert_eq!(SizeClass::for_size(64), SizeClass::B64);
        assert_eq!(SizeClass::for_size(256), SizeClass::B256);
        assert_eq!(SizeClass::for_size(1024), SizeClass::KB1);
        assert_eq!(SizeClass::for_size(4096), SizeClass::KB4);
    }

    #[test]
    fn test_for_size_round_up() {
        assert_eq!(SizeClass::for_size(1), SizeClass::B16);
        assert_eq!(SizeClass::for_size(30), SizeClass::B64);
        assert_eq!(SizeClass::for_size(100), SizeClass::B256);
        assert_eq!(SizeClass::for_size(500), SizeClass::KB1);
    }

    #[test]
    fn test_for_size_large() {
        assert_eq!(SizeClass::for_size(20000), SizeClass::KB64);
        assert_eq!(SizeClass::for_size(65536), SizeClass::KB64);
    }

    #[test]
    fn test_all_contains() {
        let all = SizeClass::all();
        assert!(all.contains(&SizeClass::B16));
        assert!(all.contains(&SizeClass::KB64));
        assert_eq!(all.len(), 7);
    }

    #[test]
    fn test_ordering() {
        assert!(SizeClass::B16 < SizeClass::B64);
        assert!(SizeClass::KB1 > SizeClass::B256);
    }
}
