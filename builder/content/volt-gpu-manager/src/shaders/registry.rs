use dashmap::DashMap;
use crate::shaders::shader_id::ShaderId;
use crate::core::error::VgmError;
use crate::core::result::VgmResult;

pub struct ShaderRegistry {
    entries: DashMap<String, ShaderId>,
}

impl ShaderRegistry {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn register(&self, name: &str, id: ShaderId) -> VgmResult<()> {
        if name.is_empty() {
            return Err(VgmError::InvalidConfig(
                "Shader name cannot be empty".into(),
            ));
        }
        if self.entries.contains_key(name) {
            return Err(VgmError::ResourceAlreadyExists(format!(
                "Shader '{}' is already registered",
                name
            )));
        }
        self.entries.insert(name.to_string(), id);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<ShaderId> {
        self.entries.get(name).map(|e| *e)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    pub fn unregister(&self, name: &str) {
        self.entries.remove(name);
    }
}

impl Default for ShaderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_lookup() {
        let reg = ShaderRegistry::new();
        let id = ShaderId::new();
        assert!(reg.register("vs_main", id).is_ok());
        assert_eq!(reg.lookup("vs_main"), Some(id));
    }

    #[test]
    fn test_register_duplicate_fails() {
        let reg = ShaderRegistry::new();
        let id = ShaderId::new();
        reg.register("dup", id).unwrap();
        assert!(reg.register("dup", ShaderId::new()).is_err());
    }

    #[test]
    fn test_register_empty_name_fails() {
        let reg = ShaderRegistry::new();
        assert!(reg.register("", ShaderId::new()).is_err());
    }

    #[test]
    fn test_contains() {
        let reg = ShaderRegistry::new();
        assert!(!reg.contains("missing"));
        reg.register("present", ShaderId::new()).unwrap();
        assert!(reg.contains("present"));
    }

    #[test]
    fn test_unregister() {
        let reg = ShaderRegistry::new();
        reg.register("tmp", ShaderId::new()).unwrap();
        assert!(reg.contains("tmp"));
        reg.unregister("tmp");
        assert!(!reg.contains("tmp"));
    }

    #[test]
    fn test_lookup_missing_returns_none() {
        let reg = ShaderRegistry::new();
        assert_eq!(reg.lookup("nope"), None);
    }
}
