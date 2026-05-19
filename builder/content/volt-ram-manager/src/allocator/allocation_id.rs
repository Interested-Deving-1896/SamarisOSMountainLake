use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllocationId(pub Uuid);

impl AllocationId {
    pub fn new() -> Self {
        AllocationId(Uuid::new_v4())
    }
}

impl Default for AllocationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AllocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AllocationId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_id_new() {
        let id = AllocationId::new();
        let id2 = AllocationId::new();
        assert_ne!(id, id2);
    }

    #[test]
    fn test_allocation_id_display() {
        let id = AllocationId::new();
        let s = format!("{}", id);
        assert!(s.starts_with("AllocationId("));
    }

    #[test]
    fn test_allocation_id_eq() {
        let id = AllocationId::new();
        assert_eq!(id, id);
    }
}
