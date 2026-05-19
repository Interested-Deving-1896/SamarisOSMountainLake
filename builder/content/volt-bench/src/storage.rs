use std::path::PathBuf;
use crate::errors::BenchError;

const DEFAULT_STORAGE_PATH: &str = "/var/lib/samaris/bench";
const MAX_HISTORY: usize = 100;

pub struct Storage {
    path: PathBuf,
}

impl Storage {
    pub fn new(path: Option<&str>) -> Self {
        let p = path
            .map(|s| s.to_string())
            .or_else(|| std::env::var("BENCH_STORAGE_PATH").ok())
            .unwrap_or_else(|| DEFAULT_STORAGE_PATH.to_string());
        Self {
            path: PathBuf::from(p),
        }
    }

    pub fn ensure_dirs(&self) -> Result<(), BenchError> {
        std::fs::create_dir_all(self.path.join("baselines"))?;
        std::fs::create_dir_all(self.path.join("export"))?;
        Ok(())
    }

    pub fn latest_path(&self) -> PathBuf {
        self.path.join("latest.json")
    }

    pub fn history_path(&self) -> PathBuf {
        self.path.join("history.json")
    }

    pub fn optimizer_input_path(&self) -> PathBuf {
        self.path.join("optimizer-input.json")
    }

    pub fn read_latest(&self) -> Result<String, BenchError> {
        Ok(std::fs::read_to_string(self.latest_path())?)
    }

    pub fn write_latest(&self, data: &str) -> Result<(), BenchError> {
        self.ensure_dirs()?;
        Ok(std::fs::write(self.latest_path(), data)?)
    }

    pub fn read_history(&self) -> Result<String, BenchError> {
        let path = self.history_path();
        if path.exists() {
            Ok(std::fs::read_to_string(path)?)
        } else {
            Ok(r#"{"entries":[],"max_entries":100}"#.to_string())
        }
    }

    pub fn append_history(&self, entry: &str) -> Result<(), BenchError> {
        self.ensure_dirs()?;
        let current = self.read_history()?;
        let mut history: serde_json::Value = serde_json::from_str(&current)?;
        let entries = history["entries"].as_array_mut()
            .ok_or_else(|| BenchError::StorageError("Invalid history format".into()))?;
        let new_entry: serde_json::Value = serde_json::from_str(entry)?;
        entries.push(new_entry);
        while entries.len() > MAX_HISTORY {
            entries.remove(0);
        }
        history["max_entries"] = serde_json::json!(MAX_HISTORY);
        Ok(std::fs::write(self.history_path(), serde_json::to_string_pretty(&history)?)?)
    }

    pub fn write_optimizer_input(&self, data: &str) -> Result<(), BenchError> {
        self.ensure_dirs()?;
        Ok(std::fs::write(self.optimizer_input_path(), data)?)
    }

    pub fn baseline_dir(&self) -> PathBuf {
        self.path.join("baselines")
    }

    pub fn list_baselines(&self) -> Result<Vec<String>, BenchError> {
        let dir = self.baseline_dir();
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut baselines = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.path().extension().map(|e| e == "json").unwrap_or(false) {
                if let Some(name) = entry.file_name().to_str() {
                    baselines.push(name.to_string());
                }
            }
        }
        Ok(baselines)
    }

    pub fn import_baseline(&self, name: &str, data: &str) -> Result<(), BenchError> {
        self.ensure_dirs()?;
        let safe_name = name.replace('/', "_").replace('\\', "_");
        Ok(std::fs::write(self.baseline_dir().join(safe_name), data)?)
    }
}
