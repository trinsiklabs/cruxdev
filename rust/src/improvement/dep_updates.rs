//! Dependency update classification — major/minor/patch.

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Type of version update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateType {
    Patch,
    Minor,
    Major,
}

/// An outdated dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutdatedDep {
    pub name: String,
    pub current: String,
    pub latest: String,
    pub update_type: UpdateType,
}

/// Result of updating a dependency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub dep_name: String,
    pub success: bool,
    pub tests_passed: bool,
    pub message: String,
}

/// Parse a version string into (major, minor, patch).
fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let re = Regex::new(r"\d+").unwrap();
    let parts: Vec<u64> = re
        .find_iter(v)
        .take(3)
        .filter_map(|m| m.as_str().parse().ok())
        .collect();
    if parts.len() >= 3 {
        Some((parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

/// Classify version change as patch, minor, or major.
pub fn classify_update(current: &str, latest: &str) -> UpdateType {
    let curr = parse_version(current);
    let lat = parse_version(latest);

    match (curr, lat) {
        (Some((cm, _, _)), Some((lm, _, _))) if cm != lm => UpdateType::Major,
        (Some((_, cm, _)), Some((_, lm, _))) if cm != lm => UpdateType::Minor,
        (Some(_), Some(_)) => UpdateType::Patch,
        _ => UpdateType::Major,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_patch() {
        assert_eq!(classify_update("1.2.3", "1.2.4"), UpdateType::Patch);
    }

    #[test]
    fn test_classify_minor() {
        assert_eq!(classify_update("1.2.3", "1.3.0"), UpdateType::Minor);
    }

    #[test]
    fn test_classify_major() {
        assert_eq!(classify_update("1.2.3", "2.0.0"), UpdateType::Major);
    }

    #[test]
    fn test_classify_unparseable() {
        assert_eq!(classify_update("abc", "def"), UpdateType::Major);
    }

    #[test]
    fn test_outdated_dep_serde() {
        let dep = OutdatedDep {
            name: "serde".into(),
            current: "1.0.0".into(),
            latest: "1.1.0".into(),
            update_type: UpdateType::Minor,
        };
        let json = serde_json::to_string(&dep).unwrap();
        let parsed: OutdatedDep = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, "serde");
    }
}
