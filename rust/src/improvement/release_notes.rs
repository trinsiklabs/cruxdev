//! Release notes generation — parse commit messages, categorize, generate markdown.

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Structured commit info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub category: String,
}

/// Release notes container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotes {
    pub version: String,
    #[serde(default)]
    pub commits: Vec<CommitInfo>,
}

/// Category patterns for commit classification.
const CATEGORY_PATTERNS: &[(&str, &str)] = &[
    (r"^feat", "feature"),
    (r"^fix", "fix"),
    (r"^refactor", "refactor"),
    (r"^docs?", "docs"),
    (r"^test", "test"),
    (r"^chore|^ci|^build", "chore"),
    (r"(?i)breaking", "breaking"),
];

/// Categorize a commit message using conventional commit patterns.
pub fn categorize_commit(message: &str) -> String {
    let first_line = message.lines().next().unwrap_or("").to_lowercase();
    let trimmed = first_line.trim();
    for &(pattern, category) in CATEGORY_PATTERNS {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(trimmed) {
            return category.to_string();
        }
    }
    "chore".to_string()
}

/// Parse a git log output string (format: "hash|message") into CommitInfo list.
pub fn parse_git_log_output(output: &str) -> Vec<CommitInfo> {
    let mut commits = Vec::new();
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() || !line.contains('|') {
            continue;
        }
        if let Some((hash, message)) = line.split_once('|') {
            let hash = hash.trim().to_string();
            let message = message.trim().to_string();
            let category = categorize_commit(&message);
            commits.push(CommitInfo {
                hash,
                message,
                category,
            });
        }
    }
    commits
}

/// Section ordering for release notes.
const SECTION_ORDER: &[(&str, &str)] = &[
    ("breaking", "Breaking Changes"),
    ("feature", "Features"),
    ("fix", "Bug Fixes"),
    ("refactor", "Refactoring"),
    ("docs", "Documentation"),
    ("test", "Tests"),
    ("chore", "Maintenance"),
];

/// Generate markdown release notes from categorized commits.
pub fn generate_release_notes(version: &str, commits: &[CommitInfo]) -> String {
    if commits.is_empty() {
        return format!("# {}\n\nNo changes.\n", version);
    }

    let mut sections: Vec<(&str, &str, Vec<&CommitInfo>)> = SECTION_ORDER
        .iter()
        .map(|&(key, title)| (key, title, Vec::new()))
        .collect();

    for commit in commits {
        for section in &mut sections {
            if section.0 == commit.category {
                section.2.push(commit);
            }
        }
    }

    let mut lines = vec![format!("# {}", version), String::new()];
    for (_, title, section_commits) in &sections {
        if section_commits.is_empty() {
            continue;
        }
        lines.push(format!("## {}", title));
        lines.push(String::new());
        for c in section_commits {
            let short_hash = if c.hash.len() >= 7 {
                &c.hash[..7]
            } else {
                &c.hash
            };
            lines.push(format!("- {} (`{}`)", c.message, short_hash));
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorize_feature() {
        assert_eq!(categorize_commit("feat: add login"), "feature");
    }

    #[test]
    fn test_categorize_fix() {
        assert_eq!(categorize_commit("fix: null pointer"), "fix");
    }

    #[test]
    fn test_categorize_breaking() {
        assert_eq!(categorize_commit("BREAKING: remove old API"), "breaking");
    }

    #[test]
    fn test_categorize_default() {
        assert_eq!(categorize_commit("update readme"), "chore");
    }

    #[test]
    fn test_parse_git_log_output() {
        let output = "abc1234|feat: add feature\ndef5678|fix: bug fix\n";
        let commits = parse_git_log_output(output);
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].category, "feature");
        assert_eq!(commits[1].category, "fix");
    }

    #[test]
    fn test_generate_release_notes_empty() {
        let notes = generate_release_notes("1.0.0", &[]);
        assert!(notes.contains("No changes"));
    }

    #[test]
    fn test_generate_release_notes_with_commits() {
        let commits = vec![
            CommitInfo {
                hash: "abc1234567".into(),
                message: "Add login".into(),
                category: "feature".into(),
            },
            CommitInfo {
                hash: "def5678901".into(),
                message: "Fix crash".into(),
                category: "fix".into(),
            },
        ];
        let notes = generate_release_notes("2.0.0", &commits);
        assert!(notes.contains("# 2.0.0"));
        assert!(notes.contains("## Features"));
        assert!(notes.contains("## Bug Fixes"));
    }
}
