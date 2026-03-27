//! Atomic state persistence — write-then-rename for crash safety.

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

use super::state::ConvergenceState;

/// Atomic write of convergence state to disk.
pub fn save_state(state: &mut ConvergenceState, path: &str) -> Result<()> {
    state.updated_at = now();
    let data = serde_json::to_string_pretty(state)?;
    let parent = Path::new(path)
        .parent()
        .context("state path has no parent")?;
    fs::create_dir_all(parent)?;

    let tmp = format!("{}.tmp", path);
    let mut f = fs::File::create(&tmp)?;
    f.write_all(data.as_bytes())?;
    f.sync_all()?;
    fs::rename(&tmp, path)?;
    Ok(())
}

/// Load convergence state from disk.
pub fn load_state(path: &str) -> Result<ConvergenceState> {
    let data = fs::read_to_string(path)?;
    let state: ConvergenceState = serde_json::from_str(&data)?;
    Ok(state)
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
    fn save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let path_str = path.to_str().unwrap();

        let mut state = ConvergenceState::new("plan.md".into());
        state.round = 3;
        state.consecutive_clean = 1;
        save_state(&mut state, path_str).unwrap();

        let loaded = load_state(path_str).unwrap();
        assert_eq!(loaded.plan_file, "plan.md");
        assert_eq!(loaded.round, 3);
        assert_eq!(loaded.consecutive_clean, 1);
    }

    #[test]
    fn save_creates_parent_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("deep").join("nested").join("state.json");
        let path_str = path.to_str().unwrap();

        let mut state = ConvergenceState::new("p.md".into());
        save_state(&mut state, path_str).unwrap();
        assert!(path.exists());
    }

    #[test]
    fn save_updates_timestamp() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let path_str = path.to_str().unwrap();

        let mut state = ConvergenceState::new("p.md".into());
        let before = state.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        save_state(&mut state, path_str).unwrap();
        assert!(state.updated_at > before);
    }

    #[test]
    fn load_nonexistent_fails() {
        assert!(load_state("/nonexistent/state.json").is_err());
    }

    #[test]
    fn no_tmp_file_remains() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let path_str = path.to_str().unwrap();

        let mut state = ConvergenceState::new("p.md".into());
        save_state(&mut state, path_str).unwrap();

        let files: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().to_string())
            .collect();
        assert!(!files.iter().any(|f| f.ends_with(".tmp")));
    }
}
