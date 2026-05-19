use crate::core::result::VumResult;
use crate::prefetch::warmup::WarmupPlan;
use crate::prefetch::scanner::PrefetchScanner;

pub struct BootAssetPrefetcher;

impl BootAssetPrefetcher {
    pub fn new() -> Self {
        BootAssetPrefetcher
    }

    pub fn discover_assets(root: &str) -> Vec<String> {
        let patterns = Self::default_boot_assets();
        let mut assets = Vec::new();

        for pattern in &patterns {
            let full_pattern = if pattern.starts_with('/') {
                pattern.clone()
            } else {
                format!("{}/{}", root.trim_end_matches('/'), pattern)
            };
            let mut found = PrefetchScanner::scan_pattern(root, &full_pattern);
            assets.append(&mut found);
        }

        assets.sort();
        assets.dedup();
        assets
    }

    pub fn prefetch(assets: &[String], max_bytes: u64) -> VumResult<WarmupPlan> {
        let mut plan = WarmupPlan::new();
        let mut accumulated: u64 = 0;

        for asset in assets {
            if accumulated >= max_bytes {
                break;
            }

            let metadata = std::fs::metadata(asset)?;
            let size = metadata.len();

            if accumulated + size > max_bytes {
                let remaining = max_bytes.saturating_sub(accumulated);
                plan.add_file(asset, remaining, false);
                accumulated += remaining;
                break;
            }

            let is_pin = asset.contains("brand/") || asset.contains("index.html");
            plan.add_file(asset, size, is_pin);
            accumulated += size;
        }

        plan.prefetched_bytes = accumulated;
        Ok(plan)
    }

    pub fn default_boot_assets() -> Vec<String> {
        vec![
            "index.html".to_string(),
            "assets/*.js".to_string(),
            "assets/*.css".to_string(),
            "wallpapers/*.png".to_string(),
            "brand/*.png".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn setup_boot_dir() -> TempDir {
        let dir = TempDir::new().unwrap();
        fs::create_dir_all(dir.path().join("assets")).unwrap();
        fs::create_dir_all(dir.path().join("wallpapers")).unwrap();
        fs::create_dir_all(dir.path().join("brand")).unwrap();

        fs::write(dir.path().join("index.html"), "<html></html>").unwrap();
        fs::write(dir.path().join("assets/app.js"), "// js").unwrap();
        fs::write(dir.path().join("assets/app.css"), "/* css */").unwrap();
        fs::write(dir.path().join("assets/lib.js"), "// lib").unwrap();
        fs::write(dir.path().join("wallpapers/desktop.png"), "png").unwrap();
        fs::write(dir.path().join("brand/logo.png"), "png").unwrap();
        dir
    }

    #[test]
    fn test_default_boot_assets() {
        let assets = BootAssetPrefetcher::default_boot_assets();
        assert_eq!(assets.len(), 5);
        assert!(assets.contains(&"index.html".to_string()));
        assert!(assets.contains(&"assets/*.js".to_string()));
        assert!(assets.contains(&"brand/*.png".to_string()));
    }

    #[test]
    fn test_discover_assets() {
        let dir = setup_boot_dir();
        let root = dir.path().to_string_lossy().to_string();
        let assets = BootAssetPrefetcher::discover_assets(&root);
        assert!(assets.iter().any(|a| a.ends_with("index.html")));
        assert!(assets.iter().any(|a| a.ends_with("app.js")));
        assert!(assets.iter().any(|a| a.ends_with("logo.png")));
        assert!(assets.iter().any(|a| a.ends_with("desktop.png")));
    }

    #[test]
    fn test_prefetch_plan() {
        let dir = setup_boot_dir();
        let root = dir.path().to_string_lossy().to_string();
        let assets = BootAssetPrefetcher::discover_assets(&root);
        let plan = BootAssetPrefetcher::prefetch(&assets, 10 * 1024 * 1024).unwrap();
        assert!(!plan.files.is_empty());
        assert!(plan.total_bytes > 0);
        assert!(plan.prefetched_bytes > 0);
    }

    #[test]
    fn test_prefetch_max_bytes() {
        let dir = setup_boot_dir();
        let root = dir.path().to_string_lossy().to_string();
        let assets = BootAssetPrefetcher::discover_assets(&root);
        let plan = BootAssetPrefetcher::prefetch(&assets, 1).unwrap();
        assert!(plan.prefetched_bytes <= 1);
    }

    #[test]
    fn test_prefetch_pins_brand_assets() {
        let dir = setup_boot_dir();
        let root = dir.path().to_string_lossy().to_string();
        let assets = BootAssetPrefetcher::discover_assets(&root);
        let plan = BootAssetPrefetcher::prefetch(&assets, 10 * 1024 * 1024).unwrap();
        let has_pinned = plan.pinned.iter().any(|p| p.contains("brand/"));
        assert!(has_pinned);
    }
}
