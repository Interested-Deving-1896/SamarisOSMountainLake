use glob::glob;
use crate::prefetch::profile::PrefetchProfile;
use crate::core::result::VumResult;

pub struct PrefetchScanner;

impl PrefetchScanner {
    pub fn new() -> Self {
        PrefetchScanner
    }

    pub fn scan(root: &str, profiles: &[PrefetchProfile]) -> VumResult<Vec<String>> {
        let mut results = Vec::new();
        for profile in profiles {
            let pattern = if profile.path_pattern.starts_with('/')
                || profile.path_pattern.starts_with(root)
            {
                profile.path_pattern.clone()
            } else {
                format!("{}/{}", root.trim_end_matches('/'), &profile.path_pattern)
            };
            let mut matched = Self::scan_pattern(root, &pattern);
            results.append(&mut matched);
        }
        results.sort();
        results.dedup();
        Ok(results)
    }

    pub fn scan_pattern(_root: &str, pattern: &str) -> Vec<String> {
        let mut results = Vec::new();
        if let Ok(entries) = glob(pattern) {
            for entry in entries.flatten() {
                if entry.is_file() {
                    results.push(entry.to_string_lossy().to_string());
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn setup_tempdir() -> TempDir {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.path().join("a.js"), "/* js */").unwrap();
        fs::write(dir.path().join("b.css"), "/* css */").unwrap();
        fs::write(sub.join("c.js"), "/* js */").unwrap();
        fs::write(sub.join("d.txt"), "text").unwrap();
        dir
    }

    #[test]
    fn test_scan_pattern_finds_files() {
        let dir = setup_tempdir();
        let pattern = format!("{}/*.js", dir.path().display());
        let results = PrefetchScanner::scan_pattern("", &pattern);
        assert_eq!(results.len(), 1);
        assert!(results[0].ends_with("a.js"));
    }

    #[test]
    fn test_scan_with_profiles() {
        let dir = setup_tempdir();
        let profiles = vec![PrefetchProfile::new("*.js", 1)];
        let results = PrefetchScanner::scan(
            &dir.path().to_string_lossy(),
            &profiles,
        )
        .unwrap();
        assert!(results.len() >= 1);
        assert!(results.iter().any(|r| r.ends_with("a.js")));
    }

    #[test]
    fn test_scan_empty_results() {
        let dir = setup_tempdir();
        let pattern = format!("{}/*.xyz", dir.path().display());
        let results = PrefetchScanner::scan_pattern("", &pattern);
        assert!(results.is_empty());
    }

    #[test]
    fn test_scan_multiple_profiles() {
        let dir = setup_tempdir();
        let profiles = vec![
            PrefetchProfile::new("*.js", 1),
            PrefetchProfile::new("*.css", 1),
        ];
        let results = PrefetchScanner::scan(
            &dir.path().to_string_lossy(),
            &profiles,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_scan_dedup() {
        let dir = setup_tempdir();
        let profiles = vec![
            PrefetchProfile::new("*.js", 1),
            PrefetchProfile::new("*.js", 1),
        ];
        let results = PrefetchScanner::scan(
            &dir.path().to_string_lossy(),
            &profiles,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
    }
}
