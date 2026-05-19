use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Inode(pub u64);

impl Inode {
    pub const ROOT: Self = Self(1);

    pub fn new() -> Self {
        Self(Uuid::new_v4().as_u128() as u64)
    }

    pub fn from_u64(v: u64) -> Self {
        Self(v)
    }
}

impl Default for Inode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_value() {
        assert_eq!(Inode::ROOT.0, 1);
    }

    #[test]
    fn test_new_creates_unique() {
        let a = Inode::new();
        let b = Inode::new();
        assert_ne!(a, b);
    }

    #[test]
    fn test_from_u64_roundtrip() {
        let inode = Inode::from_u64(42);
        assert_eq!(inode.0, 42);
    }

    #[test]
    fn test_eq_and_hash() {
        use std::collections::HashSet;
        let a = Inode::from_u64(10);
        let b = Inode::from_u64(10);
        let mut set = HashSet::new();
        set.insert(a);
        assert!(set.contains(&b));
    }
}
