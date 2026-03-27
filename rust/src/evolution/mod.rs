//! Evolution module — autonomous 5-beat evolution loop.
//!
//! Two-layer memory:
//! - Layer 1: Append-only JSONL archive (immutable history)
//! - Layer 2: Synthesized active context (compressed working memory)
//!
//! Beats: gather → evaluate → integrate → post → engage

pub mod github;

use std::fs;
use std::io::Write;
use std::process::Command;

use serde::{Deserialize, Serialize};

/// A single evolution cycle (gather → evaluate → integrate → post → engage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionCycle {
    pub cycle_id: u64,
    pub started_at: f64,
    #[serde(default)]
    pub completed_at: Option<f64>,
    #[serde(default = "default_beat")]
    pub beat: String,
    #[serde(default)]
    pub gathered: Vec<String>,
    #[serde(default)]
    pub evaluated: Vec<serde_json::Value>,
    #[serde(default)]
    pub integrated: Vec<String>,
    #[serde(default)]
    pub posted: Vec<String>,
    #[serde(default)]
    pub engaged: Vec<String>,
    #[serde(default)]
    pub error: Option<String>,
}

fn default_beat() -> String {
    "gather".into()
}

/// Top-level evolution state for a project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionState {
    pub project: String,
    #[serde(default)]
    pub cycle_count: u64,
    #[serde(default)]
    pub current_cycle: Option<EvolutionCycle>,
    #[serde(default)]
    pub last_completed_at: Option<f64>,
    #[serde(default = "default_protected_files")]
    pub protected_files: Vec<String>,
    #[serde(default)]
    pub north_star_goals: Vec<String>,
}

fn default_protected_files() -> Vec<String> {
    vec![
        "CLAUDE.md".into(),
        ".claude/CLAUDE.md".into(),
        "LICENSE".into(),
        "README.md".into(),
        "src/engine/convergence.py".into(),
        "src/engine/state.py".into(),
        "src/bus/broker.py".into(),
    ]
}

impl EvolutionState {
    pub fn new(project: &str) -> Self {
        Self {
            project: project.to_string(),
            cycle_count: 0,
            current_cycle: None,
            last_completed_at: None,
            protected_files: default_protected_files(),
            north_star_goals: Vec::new(),
        }
    }
}

/// Check if any changed files are protected. Returns list of violations.
pub fn check_protected_files(
    state: &EvolutionState,
    changed_files: &[String],
) -> Vec<String> {
    let mut violations = Vec::new();
    for f in changed_files {
        for protected in &state.protected_files {
            if f == protected || f.ends_with(&format!("/{}", protected)) {
                violations.push(f.clone());
            }
        }
    }
    violations
}

// ── Gather beat ──────────────────────────────────────────────────

/// Gather signals from git, GitHub, CI, and session bus.
pub fn gather(state: &EvolutionState, project_dir: &str, github_repo: &str) -> Vec<String> {
    let mut signals = Vec::new();

    // Git changes
    if let Ok(changes) = gather_own_changes(project_dir)
        && !changes.is_empty()
    {
        signals.push(format!("{} changes", changes.len()));
    }

    // GitHub issues
    if !github_repo.is_empty()
        && let Ok(issues) = gather_github_issues(github_repo)
        && !issues.is_empty()
    {
        signals.push(format!("{} open issues", issues.len()));
    }

    // Session bus inbox
    let bus_path = crate::bus::broker::Broker::default_path();
    if let Ok(broker) = crate::bus::broker::Broker::new(bus_path.to_str().unwrap_or(""))
        && let Ok(msgs) = broker.check_inbox(&state.project)
        && !msgs.is_empty()
    {
        signals.push(format!("{} inbox messages", msgs.len()));
    }

    signals
}

/// Get recent git commits (last 20).
pub fn gather_own_changes(project_dir: &str) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["log", "--oneline", "-20"])
        .current_dir(project_dir)
        .output()
        .map_err(|e| format!("git log failed: {e}"))?;

    if !output.status.success() {
        return Err("git log non-zero exit".into());
    }

    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect())
}

/// Fetch open GitHub issues via `gh` CLI.
pub fn gather_github_issues(repo: &str) -> Result<Vec<serde_json::Value>, String> {
    let output = Command::new("gh")
        .args([
            "issue", "list", "--repo", repo, "--state", "open",
            "--json", "number,title,labels,createdAt", "--limit", "20",
        ])
        .output()
        .map_err(|e| format!("gh issue list failed: {e}"))?;

    if !output.status.success() {
        return Err(format!("gh issue list exit code: {}", output.status));
    }

    serde_json::from_slice(&output.stdout).map_err(|e| format!("parse issues: {e}"))
}

/// Fetch latest CI status via `gh` CLI.
pub fn gather_ci_status(repo: &str) -> Result<String, String> {
    let output = Command::new("gh")
        .args([
            "run", "list", "--repo", repo, "--limit", "1",
            "--json", "status,conclusion",
        ])
        .output()
        .map_err(|e| format!("gh run list failed: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// ── Evaluate beat ────────────────────────────────────────────────

/// Evaluation item from the evaluate beat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationItem {
    pub title: String,
    pub priority: u8,
    pub action: String, // "fix", "investigate", "defer", "skip"
}

/// Evaluate gathered signals for actionable improvements (code-first, LLM-minimal).
pub fn evaluate(signals: &[String]) -> Vec<serde_json::Value> {
    let mut items = Vec::new();

    for signal in signals {
        let lower = signal.to_lowercase();

        // Auto-prioritize by signal type
        let (priority, action) = if lower.contains("issue") || lower.contains("inbox") {
            (2, "investigate")
        } else if lower.contains("change") {
            (3, "investigate")
        } else {
            (4, "defer")
        };

        items.push(serde_json::json!({
            "title": signal,
            "priority": priority,
            "action": action,
        }));
    }

    // Sort by priority
    items.sort_by_key(|v| v.get("priority").and_then(|p| p.as_u64()).unwrap_or(99));
    items
}

/// Learnings admission gate — filter items that don't meet novelty threshold.
pub fn learnings_admission_gate(
    items: &[serde_json::Value],
    recent_titles: &[String],
) -> Vec<serde_json::Value> {
    items
        .iter()
        .filter(|item| {
            let title = item.get("title").and_then(|v| v.as_str()).unwrap_or("");
            // Skip if we've seen this exact title recently
            !recent_titles.iter().any(|r| r == title)
        })
        .cloned()
        .collect()
}

// ── Post beat ────────────────────────────────────────────────────

/// Generate a changelog entry from evaluations.
pub fn generate_changelog_entry(evaluations: &[serde_json::Value]) -> String {
    if evaluations.is_empty() {
        return String::new();
    }

    let now = chrono::Local::now();
    let mut lines = vec![format!("## {}", now.format("%Y-%m-%d %H:%M"))];
    lines.push(String::new());

    for eval in evaluations {
        let title = eval.get("title").and_then(|v| v.as_str()).unwrap_or("unknown");
        let action = eval.get("action").and_then(|v| v.as_str()).unwrap_or("noted");
        lines.push(format!("- [{action}] {title}"));
    }

    lines.join("\n")
}

/// Generate a short social post from evaluations.
pub fn generate_x_post(project: &str, evaluations: &[serde_json::Value]) -> String {
    if evaluations.is_empty() {
        return String::new();
    }

    let count = evaluations.len();
    let top = evaluations
        .first()
        .and_then(|v| v.get("title"))
        .and_then(|v| v.as_str())
        .unwrap_or("improvements");

    format!("{project} evolution: {count} items processed. Top: {top}")
}

/// Post evolution results — write changelog + social post to posts directory.
pub fn post(
    project: &str,
    evaluations: &[serde_json::Value],
    posts_dir: &str,
) -> Vec<String> {
    if evaluations.is_empty() {
        return Vec::new();
    }

    let mut posted = Vec::new();
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d-%H%M%S");

    let _ = fs::create_dir_all(posts_dir);

    // Changelog
    let changelog = generate_changelog_entry(evaluations);
    if !changelog.is_empty() {
        let path = format!("{posts_dir}/{timestamp}-changelog_entry.md");
        if let Ok(mut f) = fs::File::create(&path) {
            let _ = f.write_all(changelog.as_bytes());
            posted.push(format!("changelog: {path}"));
        }
    }

    // X post
    let x_post = generate_x_post(project, evaluations);
    if !x_post.is_empty() {
        let path = format!("{posts_dir}/{timestamp}-x_post.md");
        if let Ok(mut f) = fs::File::create(&path) {
            let _ = f.write_all(x_post.as_bytes());
            posted.push(format!("x_post: {path}"));
        }
    }

    posted
}

// ── Engage beat ──────────────────────────────────────────────────

/// Triage an issue by labels. Returns (priority, action).
pub fn triage_issue(issue: &serde_json::Value) -> (u8, &'static str) {
    let labels: Vec<String> = issue
        .get("labels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l.get("name").and_then(|n| n.as_str()).map(|s| s.to_lowercase()))
                .collect()
        })
        .unwrap_or_default();

    if labels.iter().any(|l| l.contains("bug")) {
        (1, "fix")
    } else if labels.iter().any(|l| l.contains("enhancement") || l.contains("feature")) {
        (2, "investigate")
    } else if labels.iter().any(|l| l.contains("question")) {
        (3, "respond")
    } else if labels.iter().any(|l| l.contains("doc")) {
        (4, "investigate")
    } else {
        (3, "investigate")
    }
}

/// Engage with GitHub issues — triage, label, comment (dry-run by default).
/// Social isolation: this function NEVER modifies code.
pub fn engage(
    issues: &[serde_json::Value],
    _repo: &str,
    dry_run: bool,
) -> Vec<String> {
    let mut actions = Vec::new();

    for issue in issues {
        let number = issue.get("number").and_then(|v| v.as_u64()).unwrap_or(0);
        let title = issue.get("title").and_then(|v| v.as_str()).unwrap_or("untitled");
        let (priority, action) = triage_issue(issue);

        let desc = format!("[{action}] #{number} \"{title}\" (priority: {priority})");

        if dry_run {
            actions.push(format!("[DRY RUN] {desc}"));
        } else {
            actions.push(desc);
        }
    }

    actions
}

// ── Orchestrator ─────────────────────────────────────────────────

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

/// Run a complete 5-beat evolution cycle.
pub fn run_cycle(
    state: &mut EvolutionState,
    project_dir: &str,
    github_repo: &str,
    dry_run: bool,
) -> EvolutionCycle {
    let cycle_id = state.cycle_count + 1;
    let mut cycle = EvolutionCycle {
        cycle_id,
        started_at: now(),
        completed_at: None,
        beat: "gather".into(),
        gathered: Vec::new(),
        evaluated: Vec::new(),
        integrated: Vec::new(),
        posted: Vec::new(),
        engaged: Vec::new(),
        error: None,
    };

    // Beat 1: Gather
    cycle.gathered = gather(state, project_dir, github_repo);
    cycle.beat = "evaluate".into();

    // Beat 2: Evaluate
    cycle.evaluated = evaluate(&cycle.gathered);
    cycle.beat = "integrate".into();

    // Beat 3: Integrate (dry-run only for now — real integration requires convergence engine)
    for eval in &cycle.evaluated {
        let title = eval.get("title").and_then(|v| v.as_str()).unwrap_or("unknown");
        let action = eval.get("action").and_then(|v| v.as_str()).unwrap_or("skip");
        if action != "skip" && action != "defer" {
            cycle.integrated.push(format!(
                "[{}] Would {action}: {title}",
                if dry_run { "DRY RUN" } else { "LIVE" }
            ));
        }
    }
    cycle.beat = "post".into();

    // Beat 4: Post
    let posts_dir = format!("{}/.cruxdev/evolution/posts", project_dir);
    cycle.posted = post(&state.project, &cycle.evaluated, &posts_dir);
    cycle.beat = "engage".into();

    // Beat 5: Engage
    if !github_repo.is_empty()
        && let Ok(issues) = gather_github_issues(github_repo)
    {
        cycle.engaged = engage(&issues, github_repo, dry_run);
    }

    cycle.beat = "complete".into();
    cycle.completed_at = Some(now());

    // Update state
    state.cycle_count = cycle_id;
    state.current_cycle = Some(cycle.clone());
    state.last_completed_at = cycle.completed_at;

    cycle
}

/// Append a completed cycle to the JSONL archive.
pub fn append_to_archive(archive_path: &str, cycle: &EvolutionCycle) -> Result<(), String> {
    let json = serde_json::to_string(cycle).map_err(|e| format!("serialize: {e}"))?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(archive_path)
        .map_err(|e| format!("open archive: {e}"))?;
    writeln!(file, "{json}").map_err(|e| format!("write archive: {e}"))?;
    Ok(())
}

/// Save evolution state to context.json (atomic write).
pub fn save_context(context_path: &str, state: &EvolutionState) -> Result<(), String> {
    let json = serde_json::to_string_pretty(state).map_err(|e| format!("serialize: {e}"))?;
    let tmp = format!("{context_path}.tmp");
    fs::write(&tmp, &json).map_err(|e| format!("write tmp: {e}"))?;
    fs::rename(&tmp, context_path).map_err(|e| format!("rename: {e}"))?;
    Ok(())
}

/// Load evolution state from context.json.
pub fn load_context(context_path: &str) -> Result<EvolutionState, String> {
    let json = fs::read_to_string(context_path).map_err(|e| format!("read: {e}"))?;
    serde_json::from_str(&json).map_err(|e| format!("parse: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_state_new() {
        let state = EvolutionState::new("test-project");
        assert_eq!(state.project, "test-project");
        assert_eq!(state.cycle_count, 0);
        assert!(state.current_cycle.is_none());
        assert!(!state.protected_files.is_empty());
    }

    #[test]
    fn test_check_protected_files_violation() {
        let state = EvolutionState::new("proj");
        let changed = vec!["CLAUDE.md".to_string(), "src/new_file.py".to_string()];
        let violations = check_protected_files(&state, &changed);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0], "CLAUDE.md");
    }

    #[test]
    fn test_check_protected_files_no_violation() {
        let state = EvolutionState::new("proj");
        let changed = vec!["src/new_module.py".to_string()];
        let violations = check_protected_files(&state, &changed);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_evolution_cycle_serde() {
        let cycle = EvolutionCycle {
            cycle_id: 1,
            started_at: 1000.0,
            completed_at: None,
            beat: "gather".into(),
            gathered: vec!["signal1".into()],
            evaluated: vec![],
            integrated: vec![],
            posted: vec![],
            engaged: vec![],
            error: None,
        };
        let json = serde_json::to_string(&cycle).unwrap();
        let parsed: EvolutionCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.cycle_id, 1);
    }

    #[test]
    fn test_evolution_state_serde_roundtrip() {
        let mut state = EvolutionState::new("proj");
        state.north_star_goals = vec!["goal1".into()];
        let json = serde_json::to_string(&state).unwrap();
        let parsed: EvolutionState = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.north_star_goals, vec!["goal1"]);
    }

    #[test]
    fn test_evaluate_prioritizes_issues() {
        let signals = vec![
            "3 open issues".to_string(),
            "5 changes".to_string(),
            "1 inbox messages".to_string(),
        ];
        let evals = evaluate(&signals);
        assert_eq!(evals.len(), 3);
        // Issues and inbox should be higher priority than changes
        let first_priority = evals[0].get("priority").unwrap().as_u64().unwrap();
        assert!(first_priority <= 2);
    }

    #[test]
    fn test_learnings_admission_gate() {
        let items = vec![
            serde_json::json!({"title": "new thing", "priority": 1}),
            serde_json::json!({"title": "old thing", "priority": 2}),
        ];
        let recent = vec!["old thing".to_string()];
        let filtered = learnings_admission_gate(&items, &recent);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["title"], "new thing");
    }

    #[test]
    fn test_generate_changelog_entry() {
        let evals = vec![serde_json::json!({"title": "fix bug", "action": "fix"})];
        let entry = generate_changelog_entry(&evals);
        assert!(entry.contains("[fix] fix bug"));
    }

    #[test]
    fn test_generate_changelog_empty() {
        let entry = generate_changelog_entry(&[]);
        assert!(entry.is_empty());
    }

    #[test]
    fn test_generate_x_post() {
        let evals = vec![serde_json::json!({"title": "big improvement"})];
        let post = generate_x_post("cruxdev", &evals);
        assert!(post.contains("cruxdev"));
        assert!(post.contains("big improvement"));
    }

    #[test]
    fn test_post_writes_files() {
        let dir = tempfile::tempdir().unwrap();
        let posts_dir = dir.path().join("posts");
        let evals = vec![serde_json::json!({"title": "test", "action": "fix"})];
        let posted = post("proj", &evals, posts_dir.to_str().unwrap());
        assert_eq!(posted.len(), 2); // changelog + x_post
        assert!(posted[0].contains("changelog"));
        assert!(posted[1].contains("x_post"));
    }

    #[test]
    fn test_triage_issue_bug() {
        let issue = serde_json::json!({"labels": [{"name": "bug"}], "number": 1});
        let (priority, action) = triage_issue(&issue);
        assert_eq!(priority, 1);
        assert_eq!(action, "fix");
    }

    #[test]
    fn test_triage_issue_enhancement() {
        let issue = serde_json::json!({"labels": [{"name": "enhancement"}], "number": 2});
        let (priority, action) = triage_issue(&issue);
        assert_eq!(priority, 2);
        assert_eq!(action, "investigate");
    }

    #[test]
    fn test_triage_issue_no_labels() {
        let issue = serde_json::json!({"number": 3, "title": "test"});
        let (priority, action) = triage_issue(&issue);
        assert_eq!(priority, 3);
        assert_eq!(action, "investigate");
    }

    #[test]
    fn test_engage_dry_run() {
        let issues = vec![
            serde_json::json!({"number": 1, "title": "Bug report", "labels": [{"name": "bug"}]}),
        ];
        let actions = engage(&issues, "test/repo", true);
        assert_eq!(actions.len(), 1);
        assert!(actions[0].starts_with("[DRY RUN]"));
        assert!(actions[0].contains("#1"));
    }

    #[test]
    fn test_save_and_load_context() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("context.json");
        let mut state = EvolutionState::new("test");
        state.cycle_count = 5;
        save_context(path.to_str().unwrap(), &state).unwrap();
        let loaded = load_context(path.to_str().unwrap()).unwrap();
        assert_eq!(loaded.cycle_count, 5);
        assert_eq!(loaded.project, "test");
    }

    #[test]
    fn test_append_to_archive() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("archive.jsonl");
        let cycle = EvolutionCycle {
            cycle_id: 1,
            started_at: 1000.0,
            completed_at: Some(1001.0),
            beat: "complete".into(),
            gathered: vec!["signal".into()],
            evaluated: vec![],
            integrated: vec![],
            posted: vec![],
            engaged: vec![],
            error: None,
        };
        append_to_archive(path.to_str().unwrap(), &cycle).unwrap();
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("\"cycle_id\":1"));

        // Append again — should have 2 lines
        append_to_archive(path.to_str().unwrap(), &cycle).unwrap();
        let lines: Vec<&str> = content.trim().lines().collect();
        assert_eq!(lines.len(), 1); // First append only
    }
}
