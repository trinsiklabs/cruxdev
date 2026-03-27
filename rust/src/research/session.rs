//! Research session — state tracking, checkpointing, recovery.
//!
//! Each research session tracks: topic, sub-questions, convergence state,
//! findings, sources fetched, quality scores.

use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single research finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchFinding {
    pub id: String,
    pub content: String,
    pub source_url: String,
    pub quality_score: f64,
    /// robust, moderate, fragile, contested
    pub robustness: String,
    /// Which pass found this (1-5)
    pub pass_found: u32,
    pub counter_evidence: Vec<String>,
    pub tags: Vec<String>,
}

impl ResearchFinding {
    pub fn new(id: &str, content: &str, source_url: &str) -> Self {
        Self {
            id: id.to_string(),
            content: content.to_string(),
            source_url: source_url.to_string(),
            quality_score: 0.0,
            robustness: "moderate".to_string(),
            pass_found: 1,
            counter_evidence: Vec::new(),
            tags: Vec::new(),
        }
    }
}

/// A research session tracking state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSession {
    pub session_id: String,
    pub topic: String,
    pub sub_questions: Vec<String>,
    pub findings: Vec<ResearchFinding>,
    pub seen_urls: Vec<String>,
    pub total_searches: u32,
    pub novelty_scores: Vec<f64>,
    pub current_pass: u32,
    pub converged: bool,
    pub budget_exhausted: bool,
    pub quality_score: f64,
    pub started_at: f64,
    pub completed_at: Option<f64>,
}

/// Create a new research session.
pub fn create_session(topic: &str, sub_questions: Option<Vec<String>>) -> ResearchSession {
    let short_id = &Uuid::new_v4().to_string()[..8];
    ResearchSession {
        session_id: short_id.to_string(),
        topic: topic.to_string(),
        sub_questions: sub_questions.unwrap_or_default(),
        findings: Vec::new(),
        seen_urls: Vec::new(),
        total_searches: 0,
        novelty_scores: Vec::new(),
        current_pass: 1,
        converged: false,
        budget_exhausted: false,
        quality_score: 0.0,
        started_at: 0.0,
        completed_at: None,
    }
}

/// Save session state to disk for crash recovery.
pub fn save_checkpoint(session: &ResearchSession, checkpoint_dir: &str) -> Result<String> {
    fs::create_dir_all(checkpoint_dir)?;
    let path = Path::new(checkpoint_dir).join(format!("{}.json", session.session_id));
    let data = serde_json::to_string_pretty(session)?;
    fs::write(&path, data)?;
    Ok(path.to_string_lossy().to_string())
}

/// Load a session from checkpoint.
pub fn load_checkpoint(checkpoint_path: &str) -> Option<ResearchSession> {
    let path = Path::new(checkpoint_path);
    if !path.exists() {
        return None;
    }
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

/// Find the most recent checkpoint for a topic.
pub fn find_latest_checkpoint(checkpoint_dir: &str, topic: &str) -> Option<String> {
    let dir = Path::new(checkpoint_dir);
    if !dir.is_dir() {
        return None;
    }

    let mut candidates: Vec<(f64, String)> = Vec::new();
    for entry in fs::read_dir(dir).ok()? {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let path_str = path.to_string_lossy().to_string();
        if let Some(session) = load_checkpoint(&path_str)
            && session.topic == topic && !session.converged {
                candidates.push((session.started_at, path_str));
        }
    }

    if candidates.is_empty() {
        return None;
    }
    candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    Some(candidates[0].1.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let session = create_session("AI coding tools", None);
        assert_eq!(session.topic, "AI coding tools");
        assert_eq!(session.session_id.len(), 8);
        assert_eq!(session.current_pass, 1);
        assert!(!session.converged);
    }

    #[test]
    fn test_create_session_with_sub_questions() {
        let session = create_session("topic", Some(vec!["q1".to_string(), "q2".to_string()]));
        assert_eq!(session.sub_questions.len(), 2);
    }

    #[test]
    fn test_save_and_load_checkpoint() {
        let dir = tempfile::tempdir().unwrap();
        let session = create_session("test topic", None);
        let path = save_checkpoint(&session, dir.path().to_str().unwrap()).unwrap();

        let loaded = load_checkpoint(&path).unwrap();
        assert_eq!(loaded.topic, "test topic");
        assert_eq!(loaded.session_id, session.session_id);
    }

    #[test]
    fn test_load_checkpoint_missing_file() {
        let result = load_checkpoint("/nonexistent/path.json");
        assert!(result.is_none());
    }

    #[test]
    fn test_find_latest_checkpoint() {
        let dir = tempfile::tempdir().unwrap();
        let dir_str = dir.path().to_str().unwrap();

        let mut s1 = create_session("target topic", None);
        s1.started_at = 100.0;
        save_checkpoint(&s1, dir_str).unwrap();

        let mut s2 = create_session("target topic", None);
        s2.started_at = 200.0;
        save_checkpoint(&s2, dir_str).unwrap();

        let mut s3 = create_session("other topic", None);
        s3.started_at = 300.0;
        save_checkpoint(&s3, dir_str).unwrap();

        let found = find_latest_checkpoint(dir_str, "target topic").unwrap();
        let loaded = load_checkpoint(&found).unwrap();
        assert_eq!(loaded.started_at, 200.0);
    }

    #[test]
    fn test_find_latest_checkpoint_nonexistent_dir() {
        let result = find_latest_checkpoint("/nonexistent/dir", "topic");
        assert!(result.is_none());
    }
}
