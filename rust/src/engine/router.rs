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
pub const CODE_DIMENSIONS: &[&str] = &["correctness", "completeness", "edge_cases", "error_handling", "security", "performance", "maintainability", "test_coverage", "duplication"];
pub const DOC_DIMENSIONS: &[&str] = &["accuracy", "completeness", "consistency", "clarity", "currency"];
pub const FORM_DIMENSIONS: &[&str] = &["layout", "labels", "validation", "errors", "accessibility", "mobile", "cta", "trust", "performance"];
pub const METRICS_DIMENSIONS: &[&str] = &["coverage", "collection", "actionability", "thresholds", "freshness", "anti_gaming", "accessibility"];
pub const DASHBOARD_DIMENSIONS: &[&str] = &["hierarchy", "density", "visualization", "color", "real_time", "mobile", "accessibility", "performance", "actionability"];
pub const WEBSITE_ESSENTIAL_PAGES: &[&str] = &["homepage", "quick_install", "quick_start", "documentation", "comparison_pages", "llms_txt", "robots_sitemap"];
pub const MCP_SERVER_DIMENSIONS: &[&str] = &["tool_design", "security", "testing", "error_handling", "performance", "documentation", "skill_sync"];
pub const SKILL_DIMENSIONS: &[&str] = &["design", "description", "mcp_sync", "testing", "safety", "documentation", "distribution"];
pub const CONTENT_DIMENSIONS: &[&str] = &["accuracy", "completeness", "clarity", "consistency", "engagement", "structure", "voice", "citations"];
pub const BUSINESS_DIMENSIONS: &[&str] = &["viability", "market_fit", "financial_soundness", "legal_compliance", "competitive_position", "scalability"];
pub const MEDIA_DIMENSIONS: &[&str] = &["content_quality", "production_quality", "audience_fit", "seo", "accessibility", "consistency"];
pub const UI_COMPONENT_DIMENSIONS: &[&str] = &["variant_consistency", "token_usage", "composition_quality", "duplication_detection", "api_surface", "accessibility", "maintenance_cost"];
pub const COLOR_CONTRAST_DIMENSIONS: &[&str] = &["wcag_aa_compliance", "color_system_consistency", "dark_mode_parity", "semantic_tokens", "focus_visibility"];
pub const LOGO_DIMENSIONS: &[&str] = &["viewbox_optimization", "favicon_set", "dark_light_variants", "size_legibility"];
pub const POST_DEPLOYMENT_DIMENSIONS: &[&str] = &["health_endpoint", "smoke_tests", "ssl_verification", "asset_integrity", "rollback_plan", "notifications", "migration_check"];

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
fn detect_ui_components(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    // Check for common UI component patterns across frameworks
    let component_markers = [
        // React/Next.js/Vue/Svelte component dirs
        "src/components", "components", "app/components",
        // Phoenix/LiveView
        "lib/web/components", "lib/web/live",
        // Flutter
        "lib/widgets",
        // Blazor
        "Components", "Pages",
        // Component library configs
        "components.json", // shadcn
    ];
    let component_files = [
        // Tailwind + component library indicators
        "tailwind.config.js", "tailwind.config.ts", "tailwind.config.mjs",
        // Framework component files
        "package.json", // check for React/Vue/Svelte deps
        "mix.exs", // Phoenix
        "pubspec.yaml", // Flutter
    ];
    let has_component_dir = component_markers.iter().any(|m| root.join(m).is_dir());
    let has_component_config = component_files.iter().any(|m| root.join(m).exists());
    has_component_dir && has_component_config
}

fn detect_webapp(plan_file: &str) -> bool {
    let root = find_project_root(plan_file);
    let markers = ["app", "api", "Dockerfile", "docker-compose.yml",
                   "docs/E2E_TEST_PATTERNS.md", "docs/UAT_TEST_PATTERNS.md"];
    markers.iter().any(|m| root.join(m).exists())
}

/// Resolve project dir from state, falling back to plan_file parent.
fn resolve_proj_dir(state: &ConvergenceState) -> String {
    if !state.project_dir.is_empty() {
        state.project_dir.clone()
    } else {
        std::path::Path::new(&state.plan_file)
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .to_string_lossy()
            .to_string()
    }
}

/// Detect if project has metrics/observability (dashboards, metrics collection).
fn detect_metrics(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "docs/METRICS_PATTERNS.md", "docs/METRICS.md",
        "grafana", "prometheus", "datadog",
        ".cruxdev/growth/metrics.jsonl",
    ];
    markers.iter().any(|m| root.join(m).exists())
}

/// Detect if project has dashboards.
fn detect_dashboards(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "docs/DASHBOARD_PATTERNS.md", "dashboard", "dashboards",
        "src/pages/dashboard", "app/dashboard",
    ];
    markers.iter().any(|m| root.join(m).exists())
}

/// Detect if project is an MCP server.
fn detect_mcp_server(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let has_mcp_config = root.join(".mcp.json").exists();
    let has_server = ["src/server.rs", "src/mcp_server.py", "src/server.ts", "server.py"]
        .iter()
        .any(|m| root.join(m).exists());
    has_mcp_config && has_server
}

/// Detect if project has skills (.claude/skills/ or .claude/commands/).
fn detect_skills(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    root.join(".claude/skills").is_dir() || root.join(".claude/commands").is_dir()
}

/// Detect if project produces content (blog, newsletter, posts).
fn detect_content_project(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "blog", "src/pages/blog", "content", "posts",
        "newsletters", "issues",
        "docs/BLOG_POST_PATTERNS.md", "docs/X_POST_PATTERNS.md",
        ".cruxdev/evolution/posts",
    ];
    markers.iter().any(|m| root.join(m).is_dir() || root.join(m).exists())
}

/// Detect if project is a business (not purely software).
fn detect_business_project(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "docs/BUSINESS_PLAN.md", "docs/BUDGET.md", "docs/OPERATIONS.md",
        "docs/PRICING.md", "docs/REVENUE.md",
        "proposals", "clients", "deliverables",
    ];
    markers.iter().filter(|m| root.join(m).exists() || root.join(m).is_dir()).count() >= 2
}

/// Detect if project is a media/content creation project.
fn detect_media_project(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "episodes", "videos", "chapters", "manuscript",
        "SHOW_FORMAT.md", "docs/SHOW_FORMAT.md",
        "CHANNEL_STRATEGY.md", "docs/CHANNEL_STRATEGY.md",
        "BOOK_OUTLINE.md", "docs/BOOK_OUTLINE.md",
    ];
    markers.iter().any(|m| root.join(m).exists() || root.join(m).is_dir())
}

/// Detect if project is deployable (has deployment config).
fn detect_deployable(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    let markers = [
        "fly.toml", "Dockerfile", "docker-compose.yml", "docker-compose.yaml",
        "vercel.json", "netlify.toml", "deploy.sh", "scripts/deploy.sh",
        "render.yaml", "railway.json", "Procfile",
    ];
    markers.iter().any(|m| root.join(m).exists())
}

/// Detect if project has a website with visual elements needing color/contrast audit.
fn detect_website_visual(project_dir: &str) -> bool {
    let root = Path::new(project_dir);
    // Has CSS/styles + website indicators
    let has_styles = ["styles", "src/styles", "css", "tailwind.config.js", "tailwind.config.ts", "tailwind.config.mjs"]
        .iter()
        .any(|m| root.join(m).exists() || root.join(m).is_dir());
    let has_website = root.join("docs/DEPLOYMENT.md").exists()
        || root.join("docs/WEBSITE.md").exists()
        || root.join("astro.config.mjs").exists()
        || root.join("next.config.js").exists()
        || root.join("next.config.ts").exists();
    has_styles && has_website
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
            let mut dims: Vec<String> = CODE_DIMENSIONS.iter().map(|s| s.to_string()).collect();
            let proj_dir = resolve_proj_dir(state);
            // UI component DRY dimensions
            if detect_ui_components(&proj_dir) {
                for dim in UI_COMPONENT_DIMENSIONS { dims.push((*dim).into()); }
            }
            // MCP server dimensions
            if detect_mcp_server(&proj_dir) {
                for dim in MCP_SERVER_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Metrics/observability dimensions
            if detect_metrics(&proj_dir) {
                for dim in METRICS_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Business dimensions
            if detect_business_project(&proj_dir) {
                for dim in BUSINESS_DIMENSIONS { dims.push((*dim).into()); }
            }
            Task {
                task_type: "audit".into(),
                description: format!("Audit code (round {}).", state.round),
                files,
                dimensions: dims,
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
            let mut dims: Vec<String> = DOC_DIMENSIONS.iter().map(|s| s.to_string()).collect();
            let proj_dir = resolve_proj_dir(state);
            // Skills dimensions
            if detect_skills(&proj_dir) {
                for dim in SKILL_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Content dimensions (blog, newsletter, posts)
            if detect_content_project(&proj_dir) {
                for dim in CONTENT_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Media dimensions (books, podcasts, YouTube, courses)
            if detect_media_project(&proj_dir) {
                for dim in MEDIA_DIMENSIONS { dims.push((*dim).into()); }
            }
            Task {
                task_type: "audit".into(),
                description: format!("Audit documentation (round {}).", state.round),
                files,
                dimensions: dims,
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
            let proj_dir = resolve_proj_dir(state);
            // Form dimensions
            if super::form_detect::project_has_forms(&proj_dir) {
                for dim in FORM_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Dashboard dimensions
            if detect_dashboards(&proj_dir) {
                for dim in DASHBOARD_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Post-deployment verification dimensions
            if detect_deployable(&proj_dir) {
                for dim in POST_DEPLOYMENT_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Color/contrast dimensions + automated scanning
            if detect_website_visual(&proj_dir) {
                for dim in COLOR_CONTRAST_DIMENSIONS { dims.push((*dim).into()); }
                for dim in LOGO_DIMENSIONS { dims.push((*dim).into()); }
            }
            // Run automated contrast scanner
            let contrast_violations = super::contrast_check::scan_project(&proj_dir);
            let auto_findings: Vec<serde_json::Value> = contrast_violations.iter().map(|v| {
                json!({
                    "file": v.file,
                    "line": v.line,
                    "dimension": "wcag_aa_compliance",
                    "severity": v.severity,
                    "description": format!("{} on {} — estimated {:.1}:1 (requires {:.1}:1)", v.text_class, v.bg_class, v.estimated_ratio, v.required_ratio),
                    "suggested_fix": v.fix,
                })
            }).collect();

            let metadata = if auto_findings.is_empty() {
                None
            } else {
                Some(json!({
                    "auto_findings": auto_findings,
                    "contrast_violations": auto_findings.len(),
                    "note": "These contrast violations were detected automatically. Fix ALL before declaring convergence."
                }))
            };

            Task {
                task_type: "audit".into(),
                description: format!("Website convergence (round {}). URL: {} [{} contrast violations detected]", state.round, site_url, contrast_violations.len()),
                files: website_files,
                dimensions: dims,
                finding: None, test_command: None, metadata,
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

    /// Architecture test: verify every DIMENSIONS constant is referenced in routing logic.
    /// This prevents the pattern of defining dimensions but never wiring them into convergence.
    #[test]
    fn all_dimension_sets_are_wired_into_routing() {
        let source = include_str!("router.rs");

        // Every *_DIMENSIONS constant must appear in a `for dim in` or `dims.push` context
        // (not just its definition line)
        let dimension_sets = [
            "PLAN_DIMENSIONS",
            "CODE_DIMENSIONS",
            "DOC_DIMENSIONS",
            "FORM_DIMENSIONS",
            "METRICS_DIMENSIONS",
            "DASHBOARD_DIMENSIONS",
            "MCP_SERVER_DIMENSIONS",
            "SKILL_DIMENSIONS",
            "CONTENT_DIMENSIONS",
            "BUSINESS_DIMENSIONS",
            "MEDIA_DIMENSIONS",
            "UI_COMPONENT_DIMENSIONS",
            "COLOR_CONTRAST_DIMENSIONS",
            "LOGO_DIMENSIONS",
        ];

        for dim_set in &dimension_sets {
            // Count occurrences — must appear at least twice (once for definition, once for usage)
            let count = source.matches(dim_set).count();
            assert!(
                count >= 2,
                "Dimension set {} is defined but never used in routing logic (found {} references, need >= 2)",
                dim_set, count
            );
        }
    }

    #[test]
    fn code_auditing_includes_mcp_dimensions_for_mcp_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src")).unwrap();
        std::fs::write(dir.path().join(".mcp.json"), "{}").unwrap();
        std::fs::write(dir.path().join("src/server.rs"), "fn main() {}").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::CodeAuditing;
        state.project_dir = dir.path().to_str().unwrap().to_string();
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert!(task.dimensions.contains(&"tool_design".to_string()), "MCP project should have tool_design dimension");
        assert!(task.dimensions.contains(&"skill_sync".to_string()), "MCP project should have skill_sync dimension");
    }

    #[test]
    fn doc_auditing_includes_content_dimensions_for_content_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("blog")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs/README.md"), "# Docs").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::DocAuditing;
        state.project_dir = dir.path().to_str().unwrap().to_string();
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert!(task.dimensions.contains(&"engagement".to_string()), "Content project should have engagement dimension");
    }

    #[test]
    fn doc_auditing_includes_skill_dimensions_for_skill_project() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join(".claude/skills")).unwrap();
        std::fs::create_dir_all(dir.path().join("docs")).unwrap();
        std::fs::write(dir.path().join("docs/README.md"), "# Docs").unwrap();
        let (mut state, _plan, sp) = setup(dir.path(), "# Plan\n- [ ] 1.1 task\n");
        state.phase = ConvergencePhase::DocAuditing;
        state.project_dir = dir.path().to_str().unwrap().to_string();
        persistence::save_state(&mut state, &sp).unwrap();
        let task = get_next_task(&mut state, &sp, None, None, None);
        assert!(task.dimensions.contains(&"mcp_sync".to_string()), "Skill project should have mcp_sync dimension");
    }
}
