//! GitHub Release creation and changelog generation.

use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseResult {
    pub success: bool,
    pub url: String,
    pub tag: String,
    pub error: String,
}

/// Get the latest git tag (semver).
pub fn get_latest_tag(project_dir: &str) -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .current_dir(project_dir)
        .output()
        .ok()?;

    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// Generate release notes from git log since last tag.
pub fn generate_release_notes(project_dir: &str, plan_name: &str) -> String {
    let since = get_latest_tag(project_dir)
        .map(|t| format!("{t}..HEAD"))
        .unwrap_or_else(|| "HEAD~20..HEAD".to_string());

    let output = Command::new("git")
        .args(["log", "--oneline", &since])
        .current_dir(project_dir)
        .output();

    let commits = output
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();

    let mut notes = vec![format!("## {plan_name}"), String::new()];

    for line in commits.lines().take(20) {
        // Strip hash prefix, keep message
        let msg = line.split_once(' ').map(|(_, m)| m).unwrap_or(line);
        notes.push(format!("- {msg}"));
    }

    notes.join("\n")
}

/// Create a GitHub release via `gh` CLI.
pub fn create_release(
    repo: &str,
    tag: &str,
    title: &str,
    body: &str,
    project_dir: &str,
) -> ReleaseResult {
    // Create tag first
    let _ = Command::new("git")
        .args(["tag", tag])
        .current_dir(project_dir)
        .output();

    let output = Command::new("gh")
        .args([
            "release", "create", tag,
            "--repo", repo,
            "--title", title,
            "--notes", body,
        ])
        .current_dir(project_dir)
        .output();

    match output {
        Ok(o) if o.status.success() => ReleaseResult {
            success: true,
            url: String::from_utf8_lossy(&o.stdout).trim().to_string(),
            tag: tag.to_string(),
            error: String::new(),
        },
        Ok(o) => ReleaseResult {
            success: false,
            url: String::new(),
            tag: tag.to_string(),
            error: String::from_utf8_lossy(&o.stderr).trim().to_string(),
        },
        Err(e) => ReleaseResult {
            success: false,
            url: String::new(),
            tag: tag.to_string(),
            error: format!("{e}"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_release_notes_format() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        // Init repo with commits
        Command::new("git").args(["init"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.email", "t@t.com"]).current_dir(path).output().unwrap();
        Command::new("git").args(["config", "user.name", "T"]).current_dir(path).output().unwrap();
        std::fs::write(dir.path().join("f.txt"), "x").unwrap();
        Command::new("git").args(["add", "."]).current_dir(path).output().unwrap();
        Command::new("git").args(["commit", "-m", "Initial commit"]).current_dir(path).output().unwrap();

        let notes = generate_release_notes(path, "BP022");
        assert!(notes.contains("## BP022"));
    }

    #[test]
    fn test_get_latest_tag_none() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().to_str().unwrap();
        Command::new("git").args(["init"]).current_dir(path).output().unwrap();
        assert!(get_latest_tag(path).is_none());
    }
}
