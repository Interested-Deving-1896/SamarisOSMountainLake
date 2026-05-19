use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(Uuid);

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for JobId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::from_str(s).map(Self)
    }
}

impl From<Uuid> for JobId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct JobPriority {
    pub level: crate::priority::level::PriorityLevel,
    pub sub_priority: u8,
}

impl JobPriority {
    pub fn new(level: crate::priority::level::PriorityLevel, sub_priority: u8) -> Self {
        Self { level, sub_priority }
    }

    pub fn score(&self) -> u32 {
        (self.level.as_u8() as u32) << 8 | (self.sub_priority as u32)
    }
}

impl Default for JobPriority {
    fn default() -> Self {
        Self {
            level: crate::priority::level::PriorityLevel::Normal,
            sub_priority: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_id_unique() {
        let a = JobId::new();
        let b = JobId::new();
        assert_ne!(a, b);
    }

    #[test]
    fn test_job_id_display_and_fromstr() {
        let id = JobId::new();
        let s = id.to_string();
        let parsed: JobId = s.parse().unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_job_priority_scoring() {
        let low = JobPriority::new(crate::priority::level::PriorityLevel::Low, 0);
        let high = JobPriority::new(crate::priority::level::PriorityLevel::High, 0);
        assert!(low.score() < high.score());
    }
}
