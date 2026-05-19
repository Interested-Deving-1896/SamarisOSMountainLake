use glob::Pattern;

pub struct PrefetchProfile {
    pub path_pattern: String,
    pub priority: u8,
    pub max_size_kb: u64,
    pub pin: bool,
    pub compression: bool,
}

impl PrefetchProfile {
    pub fn new(pattern: &str, priority: u8) -> Self {
        PrefetchProfile {
            path_pattern: pattern.to_string(),
            priority,
            max_size_kb: 10240,
            pin: false,
            compression: true,
        }
    }

    pub fn matches(&self, path: &str) -> bool {
        if let Ok(pattern) = Pattern::new(&self.path_pattern) {
            pattern.matches(path)
        } else {
            let simplified = self.path_pattern.replace("*", "");
            path.contains(&simplified)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_profile() {
        let profile = PrefetchProfile::new("*.js", 5);
        assert_eq!(profile.path_pattern, "*.js");
        assert_eq!(profile.priority, 5);
        assert_eq!(profile.max_size_kb, 10240);
        assert!(!profile.pin);
        assert!(profile.compression);
    }

    #[test]
    fn test_matches_glob() {
        let profile = PrefetchProfile::new("*.js", 1);
        assert!(profile.matches("/path/to/app.js"));
        assert!(profile.matches("app.js"));
        assert!(!profile.matches("app.ts"));
        assert!(!profile.matches("app.jsx"));
    }

    #[test]
    fn test_matches_directory_pattern() {
        let profile = PrefetchProfile::new("/opt/volt/*/index.html", 10);
        assert!(profile.matches("/opt/volt/desktop/index.html"));
        assert!(profile.matches("/opt/volt/mobile/index.html"));
        assert!(!profile.matches("/opt/volt/desktop/other.html"));
    }

    #[test]
    fn test_matches_no_wildcard() {
        let profile = PrefetchProfile::new("exact.txt", 1);
        assert!(profile.matches("exact.txt"));
        assert!(!profile.matches("notexact.txt"));
    }

    #[test]
    fn test_custom_max_size() {
        let mut profile = PrefetchProfile::new("*.bin", 3);
        profile.max_size_kb = 500;
        assert_eq!(profile.max_size_kb, 500);
    }
}
