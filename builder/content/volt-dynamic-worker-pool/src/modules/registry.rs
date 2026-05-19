use std::collections::HashMap;

use crate::core::error::WorkerPoolError;
use crate::core::result::WorkerPoolResult;
use crate::modules::module_id::ModuleId;
use crate::modules::module_profile::ModuleProfile;

#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    modules: HashMap<ModuleId, ModuleProfile>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, profile: ModuleProfile) -> WorkerPoolResult<()> {
        let id = profile.module_id.clone();
        if self.modules.contains_key(&id) {
            return Err(WorkerPoolError::ModuleAlreadyRegistered(id.to_string()));
        }
        self.modules.insert(id, profile);
        Ok(())
    }

    pub fn is_registered(&self, module_id: &ModuleId) -> bool {
        self.modules.contains_key(module_id)
    }

    pub fn get(&self, module_id: &ModuleId) -> Option<&ModuleProfile> {
        self.modules.get(module_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ModuleId, &ModuleProfile)> {
        self.modules.iter()
    }

    pub fn len(&self) -> usize {
        self.modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
