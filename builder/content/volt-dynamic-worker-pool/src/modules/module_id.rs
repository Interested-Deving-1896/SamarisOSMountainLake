use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn orbit() -> Self {
        Self("orbit".to_string())
    }

    pub fn desktop() -> Self {
        Self("desktop".to_string())
    }

    pub fn kernel_a() -> Self {
        Self("kernel_a".to_string())
    }

    pub fn kernel_b() -> Self {
        Self("kernel_b".to_string())
    }

    pub fn vrm() -> Self {
        Self("vrm".to_string())
    }

    pub fn vum() -> Self {
        Self("vum".to_string())
    }

    pub fn vgm() -> Self {
        Self("vgm".to_string())
    }

    pub fn electron() -> Self {
        Self("electron".to_string())
    }

    pub fn background() -> Self {
        Self("background".to_string())
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl Default for ModuleId {
    fn default() -> Self {
        Self::background()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_as_str() {
        let id = ModuleId::new("test_module");
        assert_eq!(id.as_str(), "test_module");
    }

    #[test]
    fn test_named_constructors() {
        assert_eq!(ModuleId::orbit().as_str(), "orbit");
        assert_eq!(ModuleId::desktop().as_str(), "desktop");
        assert_eq!(ModuleId::kernel_a().as_str(), "kernel_a");
        assert_eq!(ModuleId::kernel_b().as_str(), "kernel_b");
        assert_eq!(ModuleId::vrm().as_str(), "vrm");
        assert_eq!(ModuleId::vum().as_str(), "vum");
        assert_eq!(ModuleId::vgm().as_str(), "vgm");
        assert_eq!(ModuleId::electron().as_str(), "electron");
        assert_eq!(ModuleId::background().as_str(), "background");
    }

    #[test]
    fn test_display() {
        let id = ModuleId::new("display_test");
        assert_eq!(format!("{}", id), "display_test");
    }

    #[test]
    fn test_default_is_background() {
        let default_id: ModuleId = Default::default();
        assert_eq!(default_id, ModuleId::background());
    }

    #[test]
    fn test_equality_and_hash() {
        use std::collections::HashSet;
        let a = ModuleId::new("same");
        let b = ModuleId::new("same");
        let c = ModuleId::new("different");
        assert_eq!(a, b);
        assert_ne!(a, c);

        let mut set = HashSet::new();
        set.insert(a.clone());
        set.insert(b);
        set.insert(c);
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let id = ModuleId::orbit();
        let json = serde_json::to_string(&id).unwrap();
        let deserialized: ModuleId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, deserialized);
    }
}
