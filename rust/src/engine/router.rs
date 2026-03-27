//! Task router — determines what the LLM should do next.
//! Maps engine state to tasks. The engine doesn't call the LLM.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

use super::checklist;
use super::convergence::*;
use super::persistence::save_state;
use super::state::*;

// Dimension sets
pub const PLAN_DIMENSIONS: &[&str] = &["completeness", "feasibility", "risk_assessment", "dependency_ordering", "testability"];
pub const CODE_DIMENSIONS: &[&str] = &["correctness", "completeness", "edge_cases", "error_handling", "security", "performance", "maintainability", "test_coverage"];
pub const DOC_DIMENSIONS: &[&str] = &["accuracy", "completeness", "consistency", "clarity", "currency"];
pub const FORM_DIMENSIONS: &[&str] = &["layout", "labels", "validation", "errors", "accessibility", "mobile", "cta", "trust", "performance"];
pub const METRICS_DIMENSIONS: &[&str] = &["coverage", "collection", "actionability", "thresholds", "freshness", "anti_gaming", "accessibility"];
pub const DASHBOARD_DIMENSIONS: &[&str] = &["hierarchy", "density", "visualization", "color", "real_time", "mobile", "accessibility", "performance", "actionability"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_type: String,
    pub description: String,
    pub files: Vec<String>,
    pub dimensions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_command: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_tier: Option<String>,
}

impl Task {
    fn done() -> Self {
        Self {
            task_type: "done".into(),
            description: "Convergence complete — two consecutive independent clean passes achieved.".into(),
            files: vec![], dimensions: vec![], finding: None, test_command: None,
            metadata: None, recommended_tier: None,
        }
    }

    fn escalated(reason: &str, round: i32) -> Self {
        Self {
            task_type: "escalated".into(),
            description: format!("Convergence escalated: {reason}"),
            files: vec![], dimensions: vec![], finding: None, test_command: None,
            metadata: Some(json!({"reason": reason, "round": round})),
            recommended_tier: None,
        }
    }
}

/// Find project root by walking up from plan file.
fn find_project_root(plan_file: &str) -> PathBuf {
    let plan_dir = Path::new(plan_file).parent().unwrap_or(Path::new("."));
    let mut root = plan_dir.to_path_buf();
    let markers = [".git", "pyproject.toml", "package.json", "Cargo.toml"];

    for _ in 0..5 {
        if markers.iter().any(|m| root.join(m).exists()) {
            return root;
        }
        match root.parent() {
            Some(p) if p != root => root = p.to_path_buf(),
            _ => break,
        }
    }
    root
}

/// Auto-discover doc files from project docs/ directory.
fn auto_discover_docs(plan_file: &str) -> Vec<String> {
    let root = find_project_root(plan_file);
    let docs_dir = root.join("docs");

    if !docs_dir.is_dir() {
        return vec![plan_file.to_string()];
    }

    let mut docs: Vec<String> = std::fs::read_dir(&docs_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    let readme = root.join("README.md");
    if readme.exists() {
        docs.push(readme.to_string_lossy().to_string());
    }

    if docs.is_empty() {
        return vec![plan_file.to_string()];
    }
    docs.sort();
    docs
}

/// Detect if project has a website.
fn detect_website(plan_file: &str) -> (bool, Vec<String>, String) {
    let root = find_project_root(plan_file);
    let markers = ["docs/DEPLOYMENT.md", "docs/WEBSITE.md"];
    let found: Vec<String> = markers.iter()
        .map(|m| root.join(m))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    if found.is_empty() {
        return (false, vec![], String::new());
    }

    // Try to extract site URL
    let re = regex::Regex::new(r"(https?://[^\s\)\]>]+\.(?:io|dev|com|org|net)[^\s\)\]]*)").unwrap();
    let url = found.iter()
        .filter_map(|f| std::fs::read_to_string(f).ok())
        .find_map(|content| re.find(&content).map(|m| m.as_str().to_string()))
        .unwrap_or_default();

    (true, found, url)
}

/// Detect if project is a webapp (not just static site).
fn detect_webapp(plan_file: &str) -> bool {
    let root = find_project_root(plan_file);
    let markers = ["app", "api", "Dockerfile", "docker-compose.yml",
                   "docs/E2E_TEST_PATTERNS.md", "docs/UAT_TEST_PATTERNS.md"];
    markers.iter().any(|m| root.join(m).exists())
}

/// Main dispatch: determine the next task based on engine state.
pub fn get_next_task(
    state: &mut ConvergenceState,
    state_path: &str,
    source_files: Option<&[String]>,
    doc_files: Option<&[String]>,
    test_command: Option<&[String]>,
) -> Task {
    // Terminal states
    if state.phase == ConvergencePhase::Converged {
        return Task::done();
    }
    if state.phase == ConvergencePhase::Escalated {
        return Task::escalated(
            state.escalation_reason.as_deref().unwrap_or("unknown"),
            state.round,
        );
    }

    // Safety checks
    if check_max_rounds(state) {
        let reason = format!("{:?}_max_rounds", state.phase);
        escalate(state, &reason);
        let _ = save_state(state, state_path);
        return get_next_task(state, state_path, source_files, doc_files, test_command);
    }

    if check_net_negative(state) {
        let reason = format!("{:?}_net_negative", state.phase);
        escalate(state, &reason);
        let _ = save_state(state, state_path);
        return get_next_task(state, state_path, source_files, doc_files, test_command);
    }

    match state.phase {
        ConvergencePhase::Planning => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::PlanAuditing);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            Task {
                task_type: "write".into(),
                description: "Create or refine the build plan.".into(),
                files: vec![state.plan_file.clone()],
                dimensions: vec![], finding: None, test_command: None,
                metadata: None, recommended_tier: Some("standard".into()),
            }
        }

        ConvergencePhase::PlanAuditing => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::DocAlignment);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            Task {
                task_type: "audit".into(),
                description: format!("Audit the plan (round {}). Check for gaps, risks, missing steps.", state.round),
                files: vec![state.plan_file.clone()],
                dimensions: PLAN_DIMENSIONS.iter().map(|s| s.to_string()).collect(),
                finding: None, test_command: None, metadata: None,
                recommended_tier: Some("fast".into()),
            }
        }

        ConvergencePhase::DocAlignment => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::Viability);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            Task {
                task_type: "doc_align".into(),
                description: format!("Document alignment audit (round {}).", state.round),
                files: vec![state.plan_file.clone()],
                dimensions: vec!["doc_alignment".into()],
                finding: None, test_command: None, metadata: None,
                recommended_tier: Some("standard".into()),
            }
        }

        ConvergencePhase::Viability => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::Executing);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            Task {
                task_type: "audit".into(),
                description: format!("Viability check (round {}).", state.round),
                files: vec![state.plan_file.clone()],
                dimensions: vec!["viability".into()],
                finding: None, test_command: None, metadata: None,
                recommended_tier: Some("standard".into()),
            }
        }

        ConvergencePhase::Executing => {
            let items = checklist::parse_checklist(&state.plan_file);
            if items.is_empty() || checklist::all_complete(&items) {
                advance_to(state, state_path, ConvergencePhase::CodeAuditing);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            if let Some(next) = checklist::get_next_incomplete(&items) {
                let (total, done, pct) = checklist::completion_summary(&items);
                Task {
                    task_type: "execute".into(),
                    description: format!("Execute checklist item {}: {} ({}/{} complete, {:.1}%)",
                        next.id, next.description, done, total, pct),
                    files: vec![state.plan_file.clone()],
                    dimensions: vec![], finding: None,
                    test_command: test_command.map(|tc| tc.to_vec()),
                    metadata: Some(json!({
                        "checklist_item": next.id,
                        "phase": next.phase,
                        "progress": {"total": total, "completed": done, "percentage": pct},
                    })),
                    recommended_tier: Some("standard".into()),
                }
            } else {
                advance_to(state, state_path, ConvergencePhase::CodeAuditing);
                get_next_task(state, state_path, source_files, doc_files, test_command)
            }
        }

        ConvergencePhase::CodeAuditing => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::DocAuditing);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            let files = source_files.map(|f| f.to_vec())
                .unwrap_or_else(|| vec![state.plan_file.clone()]);
            Task {
                task_type: "audit".into(),
                description: format!("Audit code (round {}).", state.round),
                files,
                dimensions: CODE_DIMENSIONS.iter().map(|s| s.to_string()).collect(),
                finding: None,
                test_command: test_command.map(|tc| tc.to_vec()),
                metadata: None,
                recommended_tier: Some("standard".into()),
            }
        }

        ConvergencePhase::DocAuditing => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::WebsiteConvergence);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            let files = doc_files.map(|f| f.to_vec())
                .unwrap_or_else(|| auto_discover_docs(&state.plan_file));
            Task {
                task_type: "audit".into(),
                description: format!("Audit documentation (round {}).", state.round),
                files,
                dimensions: DOC_DIMENSIONS.iter().map(|s| s.to_string()).collect(),
                finding: None, test_command: None, metadata: None,
                recommended_tier: Some("fast".into()),
            }
        }

        ConvergencePhase::WebsiteConvergence => {
            let (has_website, website_files, site_url) = detect_website(&state.plan_file);
            if !has_website {
                advance_to(state, state_path, ConvergencePhase::E2eTesting);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::E2eTesting);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            let is_webapp = detect_webapp(&state.plan_file);
            let mut dims = vec!["accuracy".into(), "completeness".into(), "metrics".into(), "deployment".into()];
            if is_webapp {
                dims.push("e2e_testing".into());
                dims.push("uat_testing".into());
            }
            // Check for forms — add form audit dimensions if detected
            let form_dir = if state.project_dir.is_empty() {
                std::path::Path::new(&state.plan_file)
                    .parent()
                    .unwrap_or(std::path::Path::new("."))
                    .to_string_lossy()
                    .to_string()
            } else {
                state.project_dir.clone()
            };
            if super::form_detect::project_has_forms(&form_dir) {
                for dim in FORM_DIMENSIONS {
                    dims.push((*dim).into());
                }
            }
            Task {
                task_type: "audit".into(),
                description: format!("Website convergence (round {}). URL: {}", state.round, site_url),
                files: website_files,
                dimensions: dims,
                finding: None, test_command: None, metadata: None,
                recommended_tier: Some("standard".into()),
            }
        }

        ConvergencePhase::E2eTesting => {
            if check_convergence(state) {
                advance_to(state, state_path, ConvergencePhase::PatternsUpdate);
                return get_next_task(state, state_path, source_files, doc_files, test_command);
            }
            Task {
                task_type: "test".into(),
                description: format!("Run the full test suite (round {}).", state.round),
                files: vec![], dimensions: vec![], finding: None,
                test_command: test_command.map(|tc| tc.to_vec()),
                metadata: None,
                recommended_tier: Some("fast".into()),
            }
        }

        ConvergencePhase::PatternsUpdate => {
            if check_convergence(state) {
                state.phase = ConvergencePhase::Converged;
                let _ = save_state(state, state_path);
                return Task::done();
            }
            Task {
                task_type: "write".into(),
                description: "Capture novel learnings. Check inbox.".into(),
                files: vec![], dimensions: vec![], finding: None,
                test_command: None, metadata: None,
                recommended_tier: Some("fast".into()),
            }
        }

        ConvergencePhase::Converged => Task::done(),
        ConvergencePhase::Escalated => Task::escalated(
            state.escalation_reason.as_deref().unwrap_or("unknown"),
            state.round,
        ),
    }
}

fn advance_to(state: &mut ConvergenceState, state_path: &str, next: ConvergencePhase) {
    state.phase = next;
    state.round = 0;
    state.consecutive_clean = 0;
    let _ = save_state(state, state_path);
}

/// Process submitted results — record round, handle checklist completion.
pub fn submit_result(state: &mut ConvergenceState, state_path: &str, result: &Value) {
    // Handle checklist completion
    if let Some(item_id) = result.get("checklist_item")
        .and_then(|v| v.as_str())
        .filter(|_| state.phase == ConvergencePhase::Executing)
    {
        checklist::mark_complete_in_file(&state.plan_file, item_id);
    }

    // Extract findings
    let findings: Vec<Finding> = result.get("findings")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter().filter_map(|f| {
                Some(Finding {
                    id: f.get("id")?.as_str()?.to_string(),
                    file: f.get("file")?.as_str()?.to_string(),
                    dimension: f.get("dimension")?.as_str()?.to_string(),
                    severity: match f.get("severity")?.as_str()? {
                        "high" => FindingSeverity::High,
                        "medium" => FindingSeverity::Medium,
                        _ => FindingSeverity::Low,
                    },
                    description: f.get("description")?.as_str()?.to_string(),
                    suggested_fix: f.get("suggested_fix").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    fixed: f.get("fixed").and_then(|v| v.as_bool()).unwrap_or(false),
                })
            }).collect()
        })
        .unwrap_or_default();

    record_round(state, findings);
    let _ = save_state(state, state_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::persistence;

    fn setup(tmp: &Path, plan_content: &str) -> (ConvergenceState, String, String) {
        let plan = tmp.join("plan.md");
        std::fs::write(&plan, plan_content).unwrap();
        let state_path = tmp.join("state.json");
        let mut state = ConvergenceState::new(plan.to_str().unwrap().to_string());
        persistence::save_state(&mut state, state_path.to_str().unwrap()).unwrap();
        (state, plan.to_str().unwrap().to_string(), state_path.to_str().unwrap().to_string())
    }

    #[test]
    fn planning_phase_returns_write() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "write");
    }

    #[test]
    fn converged_returns_done() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::Converged;
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "done");
    }

    #[test]
    fn escalated_returns_escalated() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::Escalated;
        state.escalation_reason = Some("timeout".into());
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "escalated");
    }

    #[test]
    fn executing_returns_checklist_item() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n## Phase 1: Build\n- [ ] 1.1 First\n- [ ] 1.2 Second\n");
        state.phase = ConvergencePhase::Executing;
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "execute");
        assert!(task.description.contains("1.1"));
    }

    #[test]
    fn executing_all_complete_advances() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n## Phase 1\n- [x] 1.1 Done\n- [x] 1.2 Done\n");
        state.phase = ConvergencePhase::Executing;
        persistence::save_state(&mut state, &sp).unwrap();
        let _task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(state.phase, ConvergencePhase::CodeAuditing);
    }

    #[test]
    fn max_rounds_escalates() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::CodeAuditing;
        state.round = 10;
        state.max_rounds = 5;
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "escalated");
    }

    #[test]
    fn doc_auditing_auto_discovers() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs").join("API.md"), "# API").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::DocAuditing;
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert!(task.files.iter().any(|f| f.contains("API.md")));
    }

    #[test]
    fn website_convergence_skips_when_no_website() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::WebsiteConvergence;
        persistence::save_state(&mut state, &sp).unwrap();
        let _task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(state.phase, ConvergencePhase::E2eTesting);
    }

    #[test]
    fn website_convergence_audits_when_present() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs").join("DEPLOYMENT.md"), "# Deploy to https://site.dev").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::WebsiteConvergence;
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "audit");
        assert!(task.description.contains("Website"));
    }

    #[test]
    fn website_convergence_includes_form_dimensions_when_forms_detected() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".git")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs").join("DEPLOYMENT.md"), "# Deploy to https://site.dev").unwrap();
        // Create a file with a form
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join("src").join("contact.html"), "<form action='/submit'><input type='text'/></form>").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::WebsiteConvergence;
        state.project_dir = dir.path().to_str().unwrap().to_string();
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert_eq!(task.task_type, "audit");
        // Should include form dimensions
        assert!(task.dimensions.contains(&"layout".to_string()), "should have form layout dimension: {:?}", task.dimensions);
        assert!(task.dimensions.contains(&"validation".to_string()), "should have form validation dimension");
        assert!(task.dimensions.contains(&"accessibility".to_string()), "should have form accessibility dimension");
    }

    #[test]
    fn submit_result_records_round() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        submit_result(&mut state, &sp, &json!({"findings": []}));
        assert_eq!(state.round, 1);
        assert_eq!(state.consecutive_clean, 1);
    }

    #[test]
    fn submit_result_marks_checklist() {
        let dir = tempfile::tempdir().unwrap();
        let (mut state, plan, sp) = setup(dir.path(), "# Plan\n## Phase 1\n- [ ] 1.1 Build it\n");
        state.phase = ConvergencePhase::Executing;
        submit_result(&mut state, &sp, &json!({"checklist_item": "1.1", "findings": []}));
        let content = std::fs::read_to_string(&plan).unwrap();
        assert!(content.contains("- [x] 1.1"));
    }
}
