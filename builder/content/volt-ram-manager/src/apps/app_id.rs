use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppId(pub u64);

impl AppId {
    pub fn new(id: u64) -> Self {
        AppId(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for AppId {
    fn from(id: u64) -> Self {
        AppId(id)
    }
}

impl fmt::Display for AppId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppId({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_id_creation() {
        let id = AppId::new(42);
        assert_eq!(id.as_u64(), 42);
    }

    #[test]
    fn test_app_id_from_u64() {
        let id: AppId = 100u64.into();
        assert_eq!(id.0, 100);
    }

    #[test]
    fn test_app_id_equality() {
        let a = AppId(1);
        let b = AppId(1);
        let c = AppId(2);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_app_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(AppId(1));
        set.insert(AppId(1));
        set.insert(AppId(2));
        assert_eq!(set.len(), 2);
    }
}
