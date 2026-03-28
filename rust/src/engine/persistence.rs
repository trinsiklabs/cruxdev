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

/// Save a checkpoint after each convergence round.
/// Checkpoints are named: <state_path>.checkpoint.<round>
pub fn save_checkpoint(state: &ConvergenceState, state_path: &str) -> Result<()> {
    let checkpoint_dir = format!("{}.checkpoints", state_path);
    fs::create_dir_all(&checkpoint_dir)?;

    let checkpoint_path = format!("{}/round_{:03}.json", checkpoint_dir, state.round);
    let data = serde_json::to_string_pretty(state)?;
    let tmp = format!("{}.tmp", checkpoint_path);
    let mut f = fs::File::create(&tmp)?;
    f.write_all(data.as_bytes())?;
    f.sync_all()?;
    fs::rename(&tmp, &checkpoint_path)?;

    // Rotate: keep last 3 checkpoints
    let mut checkpoints: Vec<_> = fs::read_dir(&checkpoint_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    checkpoints.sort_by_key(|e| e.file_name());
    if checkpoints.len() > 3 {
        for old in &checkpoints[..checkpoints.len() - 3] {
            let _ = fs::remove_file(old.path());
        }
    }

    Ok(())
}

/// Find the latest checkpoint for a state path.
pub fn find_latest_checkpoint(state_path: &str) -> Option<ConvergenceState> {
    let checkpoint_dir = format!("{}.checkpoints", state_path);
    let mut checkpoints: Vec<_> = fs::read_dir(&checkpoint_dir).ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    checkpoints.sort_by_key(|e| e.file_name());

    let latest = checkpoints.last()?;
    let data = fs::read_to_string(latest.path()).ok()?;
    serde_json::from_str(&data).ok()
}

/// Count checkpoints for a state path.
pub fn checkpoint_count(state_path: &str) -> usize {
    let checkpoint_dir = format!("{}.checkpoints", state_path);
    fs::read_dir(&checkpoint_dir)
        .map(|rd| rd.filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
            .count())
        .unwrap_or(0)
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

    #[test]
    fn checkpoint_save_and_find() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json");
        let sp = state_path.to_str().unwrap();

        let mut state = ConvergenceState::new("plan.md".into());
        state.round = 3;
        save_checkpoint(&state, sp).unwrap();

        let loaded = find_latest_checkpoint(sp).unwrap();
        assert_eq!(loaded.round, 3);
        assert_eq!(checkpoint_count(sp), 1);
    }

    #[test]
    fn checkpoint_rotation_keeps_3() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json");
        let sp = state_path.to_str().unwrap();

        let mut state = ConvergenceState::new("plan.md".into());
        for round in 1..=5 {
            state.round = round;
            save_checkpoint(&state, sp).unwrap();
        }

        assert_eq!(checkpoint_count(sp), 3);
        let latest = find_latest_checkpoint(sp).unwrap();
        assert_eq!(latest.round, 5);
    }

    #[test]
    fn no_checkpoint_returns_none() {
        assert!(find_latest_checkpoint("/nonexistent/state.json").is_none());
        assert_eq!(checkpoint_count("/nonexistent/state.json"), 0);
    }
}
