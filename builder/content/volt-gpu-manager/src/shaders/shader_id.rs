use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShaderId(pub Uuid);

impl ShaderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for ShaderId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for ShaderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_new_generates_unique() {
        assert_ne!(ShaderId::new(), ShaderId::new());
    }

    #[test]
    fn test_nil_is_zero_uuid() {
        assert_eq!(ShaderId::nil().0, Uuid::nil());
    }

    #[test]
    fn test_default_is_not_nil() {
        assert_ne!(ShaderId::default().0, Uuid::nil());
    }

    #[test]
    fn test_display_not_empty() {
        assert!(!format!("{}", ShaderId::nil()).is_empty());
    }

    #[test]
    fn test_hash_set() {
        let mut set = HashSet::new();
        set.insert(ShaderId::nil());
        set.insert(ShaderId::nil());
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_clone_eq() {
        let a = ShaderId::new();
        assert_eq!(a, a.clone());
    }
}
