//! Convergence index — maps plan files to active convergence runs.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub plan_file: String,
    pub convergence_id: String,
    pub status: String,
    pub started_at: f64,
    pub state_path: String,
}

fn index_path(project_dir: &str) -> PathBuf {
    Path::new(project_dir).join(".cruxdev").join("convergence_index.json")
}

pub fn load_index(project_dir: &str) -> Vec<IndexEntry> {
    let path = index_path(project_dir);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_index(project_dir: &str, entries: &[IndexEntry]) {
    let path = index_path(project_dir);
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let data = serde_json::to_string_pretty(entries).unwrap_or_default();
    let tmp = format!("{}.tmp", path.display());
    if fs::write(&tmp, &data).is_ok() {
        let _ = fs::rename(&tmp, &path);
    }
}

/// Find an active run for a plan file.
pub fn find_active_run(project_dir: &str, plan_file: &str) -> Option<IndexEntry> {
    let entries = load_index(project_dir);
    let norm = fs::canonicalize(plan_file).unwrap_or_else(|_| PathBuf::from(plan_file));
    entries.into_iter().find(|e| {
        let entry_norm = fs::canonicalize(&e.plan_file).unwrap_or_else(|_| PathBuf::from(&e.plan_file));
        entry_norm == norm && e.status == "active"
    })
}

/// Register a new convergence run.
pub fn register_run(project_dir: &str, plan_file: &str, convergence_id: &str, state_path: &str) -> IndexEntry {
    let mut entries = load_index(project_dir);
    let entry = IndexEntry {
        plan_file: plan_file.to_string(),
        convergence_id: convergence_id.to_string(),
        status: "active".to_string(),
        started_at: now(),
        state_path: state_path.to_string(),
    };
    entries.push(entry.clone());
    save_index(project_dir, &entries);
    entry
}

/// Update a run's status.
pub fn update_run_status(project_dir: &str, convergence_id: &str, new_status: &str) -> bool {
    let mut entries = load_index(project_dir);
    if let Some(entry) = entries.iter_mut().find(|e| e.convergence_id == convergence_id) {
        entry.status = new_status.to_string();
        save_index(project_dir, &entries);
        true
    } else {
        false
    }
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_index() {
        let dir = tempfile::tempdir().unwrap();
        assert!(load_index(dir.path().to_str().unwrap()).is_empty());
    }

    #[test]
    fn register_and_load() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path().to_str().unwrap();
        register_run(d, "plan.md", "abc", "/state.json");
        let entries = load_index(d);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].convergence_id, "abc");
    }

    #[test]
    fn find_active() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path().to_str().unwrap();
        let plan = dir.path().join("plan.md");
        fs::write(&plan, "# Plan").unwrap();
        register_run(d, plan.to_str().unwrap(), "abc", "/s.json");

        let found = find_active_run(d, plan.to_str().unwrap());
        assert!(found.is_some());
        assert_eq!(found.unwrap().convergence_id, "abc");
    }

    #[test]
    fn find_active_not_found() {
        let dir = tempfile::tempdir().unwrap();
        assert!(find_active_run(dir.path().to_str().unwrap(), "nope.md").is_none());
    }

    #[test]
    fn update_status() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path().to_str().unwrap();
        register_run(d, "plan.md", "abc", "/s.json");
        assert!(update_run_status(d, "abc", "converged"));

        let entries = load_index(d);
        assert_eq!(entries[0].status, "converged");
    }

    #[test]
    fn update_status_not_found() {
        let dir = tempfile::tempdir().unwrap();
        assert!(!update_run_status(dir.path().to_str().unwrap(), "nope", "done"));
    }

    #[test]
    fn find_ignores_completed() {
        let dir = tempfile::tempdir().unwrap();
        let d = dir.path().to_str().unwrap();
        let plan = dir.path().join("plan.md");
        fs::write(&plan, "# Plan").unwrap();
        register_run(d, plan.to_str().unwrap(), "abc", "/s.json");
        update_run_status(d, "abc", "converged");

        assert!(find_active_run(d, plan.to_str().unwrap()).is_none());
    }
}
