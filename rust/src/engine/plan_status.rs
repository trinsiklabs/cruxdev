//! Plan status line management — update markdown status from engine state.

use regex::Regex;
use std::fs;

const VALID_STATUSES: &[&str] = &["NOT STARTED", "IN PROGRESS", "CONVERGED", "ESCALATED"];

/// Update the **Status:** line in a plan file. Returns true if updated.
pub fn update_plan_status(plan_file: &str, new_status: &str) -> bool {
    if !VALID_STATUSES.contains(&new_status) {
        return false;
    }
    let content = match fs::read_to_string(plan_file) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let re = Regex::new(r"(\*\*Status:\*\*\s*).+").unwrap();
    if !re.is_match(&content) {
        return false;
    }

    let updated = re.replace(&content, format!("${{1}}{new_status}")).to_string();
    if updated == content {
        return false;
    }

    fs::write(plan_file, &updated).is_ok()
}

/// Read the current **Status:** line from a plan file.
pub fn read_plan_status(plan_file: &str) -> Option<String> {
    let content = fs::read_to_string(plan_file).ok()?;
    let re = Regex::new(r"\*\*Status:\*\*\s*(.+)").unwrap();
    re.captures(&content).map(|c| c[1].trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_status() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n**Status:** NOT STARTED\n").unwrap();
        assert!(update_plan_status(path.to_str().unwrap(), "IN PROGRESS"));
        assert!(fs::read_to_string(&path).unwrap().contains("IN PROGRESS"));
    }

    #[test]
    fn update_to_converged() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n**Status:** IN PROGRESS\n").unwrap();
        assert!(update_plan_status(path.to_str().unwrap(), "CONVERGED"));
    }

    #[test]
    fn no_status_line() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\nNo status\n").unwrap();
        assert!(!update_plan_status(path.to_str().unwrap(), "IN PROGRESS"));
    }

    #[test]
    fn invalid_status() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n**Status:** NOT STARTED\n").unwrap();
        assert!(!update_plan_status(path.to_str().unwrap(), "INVALID"));
    }

    #[test]
    fn already_at_status() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n**Status:** IN PROGRESS\n").unwrap();
        assert!(!update_plan_status(path.to_str().unwrap(), "IN PROGRESS"));
    }

    #[test]
    fn read_status() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n**Status:** CONVERGED\n").unwrap();
        assert_eq!(read_plan_status(path.to_str().unwrap()), Some("CONVERGED".into()));
    }

    #[test]
    fn read_status_missing() {
        assert_eq!(read_plan_status("/nonexistent.md"), None);
    }

    #[test]
    fn file_not_found() {
        assert!(!update_plan_status("/nonexistent.md", "IN PROGRESS"));
    }
}
