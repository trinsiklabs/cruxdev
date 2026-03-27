//! Write-ahead log — append-only JSONL, fsync'd before state mutation.

use anyhow::Result;
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Derive WAL path from state path (.json → .wal).
fn wal_path(state_path: &str) -> String {
    if let Some(base) = state_path.strip_suffix(".json") {
        format!("{base}.wal")
    } else {
        format!("{state_path}.wal")
    }
}

/// Append an event to the WAL. fsync for durability.
pub fn append(state_path: &str, event_type: &str, details: Option<serde_json::Value>) -> Result<()> {
    let path = wal_path(state_path);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut entry = json!({
        "timestamp": now(),
        "event_type": event_type,
    });
    if let Some(serde_json::Value::Object(map)) = details {
        for (k, v) in map {
            entry[&k] = v;
        }
    }

    let mut line = serde_json::to_string(&entry)?;
    line.push('\n');

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    file.write_all(line.as_bytes())?;
    file.sync_all()?; // fsync via std
    Ok(())
}

/// Read all WAL events.
pub fn read(state_path: &str) -> Vec<serde_json::Value> {
    let path = wal_path(state_path);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Count WAL events without loading them all.
pub fn event_count(state_path: &str) -> usize {
    let path = wal_path(state_path);
    match fs::read_to_string(&path) {
        Ok(c) => c.lines().filter(|l| !l.trim().is_empty()).count(),
        Err(_) => 0,
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
    fn append_and_read() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json").to_str().unwrap().to_string();

        append(&state_path, "start", Some(json!({"plan": "test.md"}))).unwrap();
        let events = read(&state_path);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0]["event_type"], "start");
        assert_eq!(events[0]["plan"], "test.md");
    }

    #[test]
    fn multiple_appends() {
        let dir = tempfile::tempdir().unwrap();
        let sp = dir.path().join("s.json").to_str().unwrap().to_string();

        append(&sp, "start", None).unwrap();
        append(&sp, "submit", Some(json!({"findings": 3}))).unwrap();
        append(&sp, "phase_change", None).unwrap();

        assert_eq!(read(&sp).len(), 3);
        assert_eq!(event_count(&sp), 3);
    }

    #[test]
    fn read_nonexistent() {
        assert!(read("/nonexistent/state.json").is_empty());
    }

    #[test]
    fn event_count_nonexistent() {
        assert_eq!(event_count("/nonexistent/s.json"), 0);
    }

    #[test]
    fn wal_path_derivation() {
        assert_eq!(wal_path("abc.json"), "abc.wal");
        assert_eq!(wal_path("path/to/state.json"), "path/to/state.wal");
        assert_eq!(wal_path("no_ext"), "no_ext.wal");
    }
}
