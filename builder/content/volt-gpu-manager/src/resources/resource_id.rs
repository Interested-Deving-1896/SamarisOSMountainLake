use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GpuResourceId(pub Uuid);

impl GpuResourceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }

    pub fn from_u128(v: u128) -> Self {
        Self(Uuid::from_u128(v))
    }

    pub fn as_u128(&self) -> u128 {
        self.0.as_u128()
    }
}

impl Default for GpuResourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GpuResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generates_unique_ids() {
        let a = GpuResourceId::new();
        let b = GpuResourceId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn test_nil_is_zero_uuid() {
        let nil = GpuResourceId::nil();
        assert_eq!(nil.0, Uuid::nil());
    }

    #[test]
    fn test_from_u128_roundtrip() {
        let val: u128 = 0xDEADBEEFCAFE;
        let id = GpuResourceId::from_u128(val);
        assert_eq!(id.as_u128(), val);
    }

    #[test]
    fn test_display() {
        let id = GpuResourceId::nil();
        let s = format!("{}", id);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_default_eq_new() {
        let a = GpuResourceId::default();
        let b = GpuResourceId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn test_clone() {
        let a = GpuResourceId::new();
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(GpuResourceId::new());
        set.insert(GpuResourceId::new());
        assert_eq!(set.len(), 2);
    }
}
