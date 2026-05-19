use std::collections::HashMap;
use std::path::Path;

use crate::core::error::{Result, TesseractError};

pub struct AssetCache {
    assets: HashMap<String, Vec<u8>>,
    total_bytes: usize,
}

impl AssetCache {
    pub fn precache(root: &str) -> Result<Self> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            return Err(TesseractError::System(format!("asset root not found: {root}")));
        }

        let mut assets = HashMap::new();
        let mut total_bytes = 0usize;

        let patterns = &[
            "app/index.html",
            "app/assets/index-*.js",
            "app/assets/index-*.css",
            "app/wallpapers/*.png",
            "app/brand/*.png",
            "app/assets/*.png",
            "app/sounds-login.mp3",
        ];

        for pattern in patterns {
            let full_pattern = root_path.join(pattern);
            let pattern_str = full_pattern.to_string_lossy().to_string();

            match glob::glob(&pattern_str) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        if entry.is_file() {
                            match std::fs::read(&entry) {
                                Ok(data) => {
                                    let rel_path = entry
                                        .strip_prefix(root_path)
                                        .unwrap_or(&entry)
                                        .to_string_lossy()
                                        .to_string();
                                    total_bytes += data.len();
                                    assets.insert(rel_path, data);
                                }
                                Err(e) => {
                                    tracing::debug!("asset read error {}: {e}", entry.display());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("glob pattern {pattern}: {e}");
                }
            }
        }

        if assets.is_empty() {
            tracing::warn!("No assets cached from {root} — UI will load from disk");
        }

        Ok(Self { assets, total_bytes })
    }

    pub fn get(&self, path: &str) -> Option<&[u8]> {
        self.assets.get(path).map(|v| v.as_slice())
    }

    pub fn contains(&self, path: &str) -> bool {
        self.assets.contains_key(path)
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn is_empty(&self) -> bool {
        self.assets.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.assets.keys()
    }
}
