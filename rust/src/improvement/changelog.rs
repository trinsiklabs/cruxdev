//! Living changelog — auto-generated from commit history.

use super::release_notes::CommitInfo;

/// Section ordering for changelog generation.
const SECTION_ORDER: &[(&str, &str)] = &[
    ("breaking", "Breaking Changes"),
    ("feature", "Features"),
    ("fix", "Bug Fixes"),
    ("refactor", "Refactoring"),
    ("docs", "Documentation"),
    ("test", "Tests"),
    ("chore", "Maintenance"),
];

/// Generate a full changelog string from commits.
pub fn generate_changelog(commits: &[CommitInfo], version: &str) -> String {
    if commits.is_empty() {
        return format!("# Changelog\n\n## {}\n\nNo changes recorded.\n", version);
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

    let mut lines = vec![
        "# Changelog".to_string(),
        String::new(),
        format!("## {}", version),
        String::new(),
    ];

    for (_, title, section_commits) in &sections {
        if section_commits.is_empty() {
            continue;
        }
        lines.push(format!("### {}", title));
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
    fn test_generate_changelog_empty() {
        let result = generate_changelog(&[], "1.0.0");
        assert!(result.contains("No changes recorded"));
    }

    #[test]
    fn test_generate_changelog_with_commits() {
        let commits = vec![
            CommitInfo {
                hash: "abc1234567".into(),
                message: "Add new feature".into(),
                category: "feature".into(),
            },
            CommitInfo {
                hash: "def5678901".into(),
                message: "Fix bug".into(),
                category: "fix".into(),
            },
        ];
        let result = generate_changelog(&commits, "2.0.0");
        assert!(result.contains("## 2.0.0"));
        assert!(result.contains("### Features"));
        assert!(result.contains("Add new feature"));
        assert!(result.contains("### Bug Fixes"));
        assert!(result.contains("`abc1234`"));
    }

    #[test]
    fn test_generate_changelog_section_ordering() {
        let commits = vec![
            CommitInfo {
                hash: "aaa1111111".into(),
                message: "Chore task".into(),
                category: "chore".into(),
            },
            CommitInfo {
                hash: "bbb2222222".into(),
                message: "Breaking change".into(),
                category: "breaking".into(),
            },
        ];
        let result = generate_changelog(&commits, "3.0.0");
        let breaking_pos = result.find("Breaking Changes").unwrap();
        let chore_pos = result.find("Maintenance").unwrap();
        assert!(breaking_pos < chore_pos);
    }
}
