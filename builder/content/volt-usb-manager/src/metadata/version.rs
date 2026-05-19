use crate::core::result::VumResult;
use crate::core::error::VumError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub const CURRENT: Self = Self {
        major: 1,
        minor: 0,
        patch: 0,
    };

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn from_string(s: &str) -> VumResult<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(VumError::InvalidConfig(format!(
                "Invalid version format: {}",
                s
            )));
        }
        let major = parts[0]
            .parse()
            .map_err(|_| VumError::InvalidConfig(format!("Invalid major version: {}", parts[0])))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| VumError::InvalidConfig(format!("Invalid minor version: {}", parts[1])))?;
        let patch = parts[2]
            .parse()
            .map_err(|_| VumError::InvalidConfig(format!("Invalid patch version: {}", parts[2])))?;
        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::CURRENT
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        let v = Version::CURRENT;
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 0);
        assert_eq!(v.patch, 0);
    }

    #[test]
    fn test_to_string() {
        let v = Version {
            major: 2,
            minor: 3,
            patch: 4,
        };
        assert_eq!(v.to_string(), "2.3.4");
    }

    #[test]
    fn test_from_string() {
        let v = Version::from_string("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_from_string_wrong_parts() {
        assert!(Version::from_string("1.2").is_err());
        assert!(Version::from_string("1.2.3.4").is_err());
    }

    #[test]
    fn test_from_string_non_numeric() {
        assert!(Version::from_string("a.b.c").is_err());
    }

    #[test]
    fn test_from_string_empty() {
        assert!(Version::from_string("").is_err());
    }

    #[test]
    fn test_display() {
        let v = Version {
            major: 0,
            minor: 9,
            patch: 1,
        };
        assert_eq!(format!("{}", v), "0.9.1");
    }

    #[test]
    fn test_roundtrip() {
        let v = Version {
            major: 3,
            minor: 14,
            patch: 159,
        };
        let s = v.to_string();
        let restored = Version::from_string(&s).unwrap();
        assert_eq!(v, restored);
    }

    #[test]
    fn test_default_is_current() {
        assert_eq!(Version::default(), Version::CURRENT);
    }
}
