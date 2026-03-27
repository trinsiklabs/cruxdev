//! README auto-optimization — update badges, verify sections.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// README health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadmeHealth {
    pub exists: bool,
    pub has_quick_start: bool,
    pub has_demo: bool,
    pub has_badges: bool,
    pub has_installation: bool,
    pub has_features: bool,
    pub test_count_current: bool,
    pub tool_count_current: bool,
    pub suggestions: Vec<String>,
}

/// Check README health and suggest improvements.
pub fn check_readme(project_dir: &str, current_test_count: usize, current_tool_count: usize) -> ReadmeHealth {
    let readme_path = Path::new(project_dir).join("README.md");
    let content = match fs::read_to_string(&readme_path) {
        Ok(c) => c,
        Err(_) => {
            return ReadmeHealth {
                exists: false,
                has_quick_start: false,
                has_demo: false,
                has_badges: false,
                has_installation: false,
                has_features: false,
                test_count_current: false,
                tool_count_current: false,
                suggestions: vec!["README.md does not exist — create one".into()],
            };
        }
    };

    let lower = content.to_lowercase();
    let mut suggestions = Vec::new();

    let has_quick_start = lower.contains("quick start") || lower.contains("quickstart") || lower.contains("getting started");
    if !has_quick_start {
        suggestions.push("Add a Quick Start section (30-second getting started)".into());
    }

    let has_demo = content.contains(".gif") || content.contains(".mp4") || content.contains(".webm")
        || lower.contains("demo") || content.contains("asciinema") || content.contains("asciicast");
    if !has_demo {
        suggestions.push("Add a demo GIF or asciinema recording".into());
    }

    let has_badges = content.contains("shields.io") || content.contains("badge") || content.contains("![");
    if !has_badges {
        suggestions.push("Add badges (tests, coverage, license, latest release)".into());
    }

    let has_installation = lower.contains("install") || lower.contains("setup") || lower.contains("cargo install");
    if !has_installation {
        suggestions.push("Add an Installation section".into());
    }

    let has_features = lower.contains("features") || lower.contains("what it does") || lower.contains("capabilities");
    if !has_features {
        suggestions.push("Add a Features section".into());
    }

    let test_count_str = format!("{current_test_count}");
    let test_count_current = content.contains(&test_count_str);
    if !test_count_current && current_test_count > 0 {
        suggestions.push(format!("Update test count to {current_test_count}"));
    }

    let tool_count_str = format!("{current_tool_count}");
    let tool_count_current = content.contains(&tool_count_str);
    if !tool_count_current && current_tool_count > 0 {
        suggestions.push(format!("Update tool count to {current_tool_count}"));
    }

    ReadmeHealth {
        exists: true,
        has_quick_start,
        has_demo,
        has_badges,
        has_installation,
        has_features,
        test_count_current,
        tool_count_current,
        suggestions,
    }
}

/// Update a specific number in README (e.g., test count, tool count).
pub fn update_readme_number(
    project_dir: &str,
    old_value: &str,
    new_value: &str,
) -> Result<bool, String> {
    let readme_path = Path::new(project_dir).join("README.md");
    let content = fs::read_to_string(&readme_path).map_err(|e| format!("{e}"))?;

    if !content.contains(old_value) {
        return Ok(false);
    }

    let updated = content.replace(old_value, new_value);
    fs::write(&readme_path, updated).map_err(|e| format!("{e}"))?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_readme_missing() {
        let dir = tempfile::tempdir().unwrap();
        let health = check_readme(dir.path().to_str().unwrap(), 368, 46);
        assert!(!health.exists);
        assert!(!health.suggestions.is_empty());
    }

    #[test]
    fn test_check_readme_minimal() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "# My Project\n\nA project.\n").unwrap();
        let health = check_readme(dir.path().to_str().unwrap(), 100, 10);
        assert!(health.exists);
        assert!(!health.has_quick_start);
        assert!(!health.has_demo);
        assert!(health.suggestions.len() >= 3);
    }

    #[test]
    fn test_check_readme_complete() {
        let dir = tempfile::tempdir().unwrap();
        let content = "# Project\n\n![badge](https://shields.io/test)\n\n## Quick Start\n\nRun it.\n\n## Features\n\n- 368 tests\n- 46 tools\n\n## Installation\n\n`cargo install`\n\n![demo](demo.gif)\n";
        fs::write(dir.path().join("README.md"), content).unwrap();
        let health = check_readme(dir.path().to_str().unwrap(), 368, 46);
        assert!(health.has_quick_start);
        assert!(health.has_demo);
        assert!(health.has_badges);
        assert!(health.has_installation);
        assert!(health.has_features);
        assert!(health.test_count_current);
        assert!(health.tool_count_current);
    }

    #[test]
    fn test_update_readme_number() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "We have 300 tests and growing.").unwrap();
        let updated = update_readme_number(dir.path().to_str().unwrap(), "300", "368").unwrap();
        assert!(updated);
        let content = fs::read_to_string(dir.path().join("README.md")).unwrap();
        assert!(content.contains("368"));
    }
}
