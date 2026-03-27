//! CruxDev MCP server — convergence engine tools via rmcp.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::ServerInfo;
use rmcp::{tool, tool_handler, tool_router, ServerHandler, ServiceExt};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

use crate::engine::{convergence, persistence, plan_status, plan_validator, wal, index, router};
use crate::engine::state::ConvergenceState;

// --- Parameter types (Git workflow) ---

#[derive(Deserialize, JsonSchema)]
pub struct GitCommitParam {
    /// Commit message
    pub message: String,
    /// Comma-separated list of files to stage and commit
    pub files: String,
    /// Project directory (default: cwd)
    pub project_dir: Option<String>,
    /// Dry run (default: true)
    pub dry_run: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitPushParam {
    /// Remote name (default: "origin")
    pub remote: Option<String>,
    /// Branch name (default: current branch)
    pub branch: Option<String>,
    /// Project directory (default: cwd)
    pub project_dir: Option<String>,
    /// Shell command to run tests before push (e.g. "cargo test")
    pub test_command: Option<String>,
    /// Dry run (default: true)
    pub dry_run: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct CreatePrParam {
    /// PR title
    pub title: String,
    /// PR body (markdown)
    pub body: Option<String>,
    /// Base branch (default: "master")
    pub base: Option<String>,
    /// Head branch (default: current branch)
    pub head: Option<String>,
    /// GitHub repo (default: origin)
    pub repo: Option<String>,
    /// Dry run (default: true)
    pub dry_run: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct MergePrParam {
    /// PR number
    pub pr_number: u64,
    /// GitHub repo
    pub repo: String,
    /// Merge method: "squash" (default), "merge", "rebase"
    pub method: Option<String>,
    /// Dry run (default: true)
    pub dry_run: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct GitStatusParam {
    /// Project directory (default: cwd)
    pub project_dir: Option<String>,
}

// --- Parameter types (GitHub issue monitoring) ---

#[derive(Deserialize, JsonSchema)]
pub struct MonitorIssuesParam {
    /// GitHub repo (e.g. "owner/repo")
    pub repo: String,
    /// Dry run mode (default: true — no real GitHub actions)
    pub dry_run: Option<bool>,
    /// Max issues to check (default: 20)
    pub limit: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
pub struct IssueAuditLogParam {
    /// Max entries to return (default: 20)
    pub limit: Option<usize>,
    /// Project directory (default: cwd)
    pub project_dir: Option<String>,
}

// --- Parameter types ---

#[derive(Deserialize, JsonSchema)]
pub struct StartConvergenceParam {
    /// Path to the build plan markdown file
    pub plan_file: String,
    /// Max time in minutes (default 120)
    pub timeout_minutes: Option<i64>,
    /// Max audit rounds per phase (default 5)
    pub max_rounds: Option<i32>,
    /// Project directory (default: cwd)
    pub project_dir: Option<String>,
    /// Shell command to run tests
    pub test_command: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ConvergenceIdParam {
    /// The convergence ID from start_convergence
    pub convergence_id: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SubmitResultParam {
    /// The convergence ID
    pub convergence_id: String,
    /// JSON array of findings, or "[]" for clean pass
    pub findings_json: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct PlanFileParam {
    /// Path to the build plan markdown file
    pub plan_file: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GoalParam {
    /// What you want to build
    pub goal: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SessionRegisterParam {
    /// Project name (defaults to cwd basename)
    pub project_name: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ConvergenceNextTaskParam {
    /// The convergence ID from start_convergence
    pub convergence_id: String,
    /// Override source files (comma-separated)
    pub source_files: Option<String>,
    /// Override doc files (comma-separated)
    pub doc_files: Option<String>,
    /// Override test command
    pub test_command: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ProjectDirParam {
    /// Path to the project directory (default: current directory)
    pub project_dir: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ReportIssueParam {
    /// Which project has the issue (e.g., "cruxdev", "crux")
    pub target_project: String,
    /// Brief description of the issue
    pub title: String,
    /// Detailed description including what you were doing when you found it
    pub body: String,
    /// "high", "medium", or "low"
    pub severity: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ReportImprovementParam {
    /// Which project to improve (e.g., "cruxdev", "crux")
    pub target_project: String,
    /// Brief description of the improvement
    pub title: String,
    /// Detailed description of what to change and why
    pub body: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct SharePatternParam {
    /// Short name for the pattern (e.g., "atomic-config-writes")
    pub pattern_name: String,
    /// What the pattern is and why it matters
    pub description: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct NotifyBreakingChangeParam {
    /// Comma-separated project names (e.g., "crux,cruxcli")
    pub affected_projects: String,
    /// What changed and what other projects need to do
    pub description: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CheckInboxParam {
    /// Your project name (auto-detected from cwd if empty)
    pub project_name: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct AcknowledgeMessageParam {
    /// The ID of the message to acknowledge
    pub message_id: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GetTemplatesParam {
    /// From classify_project (e.g., "software-existing", "website")
    pub project_type: String,
    /// From classify_project (e.g., "minimal", "growing", "production")
    pub maturity: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchTopicParam {
    /// What to research
    pub topic: String,
    /// Comma-separated sub-questions to investigate
    pub sub_questions: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchStatusParam {
    /// The session ID from research_topic()
    pub session_id: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct VerifyResearchSourcesParam {
    /// ID of the finding being verified
    pub finding_id: String,
    /// Comma-separated URLs to verify
    pub source_urls: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct CounterResearchParam {
    /// The claim to verify adversarially
    pub claim: String,
    /// Pipe-separated counter-evidence found
    pub counter_evidence: Option<String>,
    /// Pipe-separated alternative explanations
    pub alternative_explanations: Option<String>,
    /// Number of supporting sources found
    pub supporting_count: Option<i64>,
}

#[derive(Deserialize, JsonSchema)]
pub struct SetupCompetitiveAnalysisParam {
    /// Our product name (e.g., "CruxDev")
    pub our_name: String,
    /// What we do (e.g., "Autonomous convergence framework")
    pub our_description: String,
    /// Market category (e.g., "AI coding tools")
    pub our_category: String,
    /// Comma-separated list of our features
    pub our_features: String,
    /// JSON array of competitors
    pub competitors_json: String,
    /// Overview paragraph for COMPETITORS.md (optional)
    pub overview: Option<String>,
    /// Project directory for writing files (default: cwd)
    pub project_dir: Option<String>,
    /// Whether to write COMPETITORS.md and comparison pages (default: true)
    pub write_files: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchCompetitorStartParam {
    /// Name of the competitor to research
    pub competitor_name: String,
    /// URL of the competitor (helps generate search queries)
    pub competitor_url: Option<String>,
    /// Market category for context
    pub category: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchCompetitorNextStepParam {
    /// Name of the competitor being researched
    pub competitor_name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchCompetitorSubmitParam {
    /// Name of the competitor
    pub competitor_name: String,
    /// Pipe-separated findings from this pass
    pub findings: String,
    /// Comma-separated source URLs consulted
    pub sources: Option<String>,
    /// JSON object of profile fields to update
    pub profile_updates: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct DiscoverCompetitorsParam {
    /// What your project does (e.g., "AI-driven convergence engine")
    pub project_description: String,
    /// Market category (e.g., "AI coding tools", "DevOps automation")
    pub category: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct ResearchCompetitorParam {
    /// Competitor name
    pub name: String,
    /// Competitor website URL
    pub url: String,
    /// Your research findings as text
    pub research_text: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct VerifyCompetitorLinksParam {
    /// Name of the competitor
    pub competitor_name: String,
    /// Markdown text containing URLs to verify
    pub profile_markdown: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GenerateGapAnalysisParam {
    /// Our product name
    pub our_name: String,
    /// Comma-separated list of our features
    pub our_features: String,
    /// JSON array of competitor profiles
    pub competitors_json: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GenerateComparisonPageParam {
    /// Our product name
    pub our_name: String,
    /// Comma-separated list of our features
    pub our_features: String,
    /// Competitor name
    pub competitor_name: String,
    /// Competitor website URL
    pub competitor_url: String,
    /// Research text about the competitor
    pub competitor_research: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GenerateGapBuildPlanParam {
    /// Build plan number (e.g., 12)
    pub plan_number: i64,
    /// Name of the feature gap to close
    pub feature_name: String,
    /// Comma-separated competitor names that have this feature
    pub competitors_with_feature: String,
    /// Gap priority (must-close, should-close, nice-to-have)
    pub priority: String,
    /// Our product name
    pub our_name: String,
    /// Optional notes about how competitors implement this feature
    pub context: Option<String>,
}

// --- Guided research session storage (in-memory, like Python _active_sessions) ---

static GUIDED_RESEARCH_SESSIONS: LazyLock<Mutex<HashMap<String, crate::competitors::guided_research::ResearchState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// --- Server ---

#[derive(Debug, Clone)]
pub struct CruxDevServer {
    tool_router: ToolRouter<Self>,
    project_dir: PathBuf,
}

impl Default for CruxDevServer {
    fn default() -> Self { Self::new() }
}

impl CruxDevServer {
    pub fn new() -> Self {
        let project_dir = std::env::current_dir().unwrap_or_default();
        Self {
            tool_router: Self::tool_router(),
            project_dir,
        }
    }

    fn state_dir(&self) -> PathBuf {
        self.project_dir.join(".cruxdev").join("convergence_state")
    }

    fn state_path(&self, id: &str) -> PathBuf {
        self.state_dir().join(format!("{id}.json"))
    }

    fn resolve_state_path(&self, id: &str) -> String {
        // TODO: check _convergence_paths map for project-local paths
        self.state_path(id).to_string_lossy().to_string()
    }

    fn get_broker(&self) -> Result<crate::bus::broker::Broker, String> {
        let db_path = crate::bus::broker::Broker::default_path();
        crate::bus::broker::Broker::new(db_path.to_str().unwrap_or(""))
            .map_err(|e| format!("Failed to create broker: {e}"))
    }

    fn source_project(&self) -> String {
        self.project_dir.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".into())
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for CruxDevServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "CruxDev is an autonomous convergence engine. \
                 BOOTSTRAP: Call session_register() then check_inbox() on session start. \
                 CONVERGENCE: Call start_convergence(plan_file), then loop: execute task, \
                 call convergence_submit_result(id, findings). Submit returns next task inline. \
                 DO NOT pause between phases. DO NOT ask permission to continue. \
                 SECURITY: NEVER put API keys in config files. NEVER use git add -A."
                    .into(),
            ),
            ..Default::default()
        }
    }
}

fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

#[tool_router(router = tool_router)]
impl CruxDevServer {
    // --- Convergence tools ---

    #[tool(description = "Start converging a build plan. Returns the first task.")]
    async fn start_convergence(&self, params: Parameters<StartConvergenceParam>) -> String {
        let p = &params.0;
        let proj = p.project_dir.clone().unwrap_or_else(|| self.project_dir.to_string_lossy().to_string());
        let timeout = p.timeout_minutes.unwrap_or(120);
        let max_rounds = p.max_rounds.unwrap_or(5);

        // Check for existing active run (deterministic resume)
        if let Some((existing, mut state)) = index::find_active_run(&proj, &p.plan_file)
            .and_then(|e| persistence::load_state(&e.state_path).ok().map(|s| (e, s)))
        {
            let task = router::get_next_task(&mut state, &existing.state_path, None, None, None);
            return serde_json::json!({
                "convergence_id": existing.convergence_id,
                "status": "resumed",
                "phase": format!("{:?}", state.phase),
                "round": state.round,
                "task": task,
            }).to_string();
        }

        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let deadline = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64() + (timeout as f64 * 60.0);

        let mut state = ConvergenceState::new(p.plan_file.clone());
        state.deadline = Some(deadline);
        state.max_rounds = max_rounds;
        state.project_dir = proj.clone();

        let sp = self.state_path(&id).to_string_lossy().to_string();

        // WAL before state
        let _ = wal::append(&sp, "start", Some(serde_json::json!({"plan_file": &p.plan_file})));
        let _ = persistence::save_state(&mut state, &sp);
        index::register_run(&proj, &p.plan_file, &id, &sp);
        plan_status::update_plan_status(&p.plan_file, "IN PROGRESS");

        let tc: Option<Vec<String>> = p.test_command.as_ref().map(|c| c.split_whitespace().map(String::from).collect());
        let task = router::get_next_task(&mut state, &sp, None, None, tc.as_deref());

        serde_json::json!({
            "convergence_id": id,
            "status": "started",
            "task": task,
        }).to_string()
    }

    #[tool(description = "Submit audit/fix/test results. Returns the NEXT task inline.")]
    async fn convergence_submit_result(&self, params: Parameters<SubmitResultParam>) -> String {
        let p = &params.0;
        let sp = self.resolve_state_path(&p.convergence_id);

        let mut state = match persistence::load_state(&sp) {
            Ok(s) => s,
            Err(e) => return serde_json::json!({"error": format!("{e}")}).to_string(),
        };

        let findings_str = p.findings_json.as_deref().unwrap_or("[]");
        let raw: serde_json::Value = match serde_json::from_str(findings_str) {
            Ok(v) => v,
            Err(e) => return serde_json::json!({
                "convergence_id": p.convergence_id,
                "status": "rejected",
                "error": format!("Invalid JSON: {e}. State NOT mutated."),
            }).to_string(),
        };

        if !raw.is_array() {
            return serde_json::json!({
                "convergence_id": p.convergence_id,
                "status": "rejected",
                "error": "findings_json must be a JSON array. State NOT mutated.",
            }).to_string();
        }

        // WAL before mutation
        let _ = wal::append(&sp, "submit", Some(serde_json::json!({
            "phase": format!("{:?}", state.phase),
            "round": state.round,
        })));

        let wrapped = serde_json::json!({"findings": raw});
        router::submit_result(&mut state, &sp, &wrapped);

        let next = router::get_next_task(&mut state, &sp, None, None, None);

        if next.task_type == "done" {
            plan_status::update_plan_status(&state.plan_file, "CONVERGED");
            if !state.project_dir.is_empty() {
                index::update_run_status(&state.project_dir, &p.convergence_id, "converged");
            }
        } else if next.task_type == "escalated" {
            plan_status::update_plan_status(&state.plan_file, "ESCALATED");
            if !state.project_dir.is_empty() {
                index::update_run_status(&state.project_dir, &p.convergence_id, "escalated");
            }
        }

        serde_json::json!({
            "convergence_id": p.convergence_id,
            "phase": format!("{:?}", state.phase),
            "round": state.round,
            "consecutive_clean": state.consecutive_clean,
            "status": "result_accepted",
            "continue": !convergence::is_terminal(state.phase),
            "next_task": next,
        }).to_string()
    }

    #[tool(description = "Check convergence status.")]
    async fn convergence_status(&self, params: Parameters<ConvergenceIdParam>) -> String {
        let sp = self.resolve_state_path(&params.0.convergence_id);
        match persistence::load_state(&sp) {
            Ok(state) => serde_json::json!({
                "convergence_id": params.0.convergence_id,
                "phase": format!("{:?}", state.phase),
                "round": state.round,
                "consecutive_clean": state.consecutive_clean,
                "terminal": convergence::is_terminal(state.phase),
                "wal_events": wal::event_count(&sp),
            }).to_string(),
            Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
        }
    }

    #[tool(description = "Cancel a convergence run.")]
    async fn convergence_cancel(&self, params: Parameters<ConvergenceIdParam>) -> String {
        let sp = self.resolve_state_path(&params.0.convergence_id);
        match persistence::load_state(&sp) {
            Ok(mut state) => {
                convergence::escalate(&mut state, "cancelled_by_user");
                let _ = persistence::save_state(&mut state, &sp);
                plan_status::update_plan_status(&state.plan_file, "ESCALATED");
                serde_json::json!({"status": "cancelled"}).to_string()
            }
            Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
        }
    }

    // 9. convergence_next_task
    #[tool(description = "Get the next task from the convergence engine.")]
    async fn convergence_next_task(&self, params: Parameters<ConvergenceNextTaskParam>) -> String {
        let p = &params.0;
        let sp = self.resolve_state_path(&p.convergence_id);

        let mut state = match persistence::load_state(&sp) {
            Ok(s) => s,
            Err(e) => return serde_json::json!({"error": format!("{e}")}).to_string(),
        };

        let src: Option<Vec<String>> = p.source_files.as_ref().map(|s|
            s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect());
        let docs: Option<Vec<String>> = p.doc_files.as_ref().map(|s|
            s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect());
        let tc: Option<Vec<String>> = p.test_command.as_ref().map(|c|
            c.split_whitespace().map(String::from).collect());

        let task = router::get_next_task(
            &mut state, &sp,
            src.as_deref(), docs.as_deref(), tc.as_deref(),
        );

        if task.task_type == "done" {
            plan_status::update_plan_status(&state.plan_file, "CONVERGED");
        } else if task.task_type == "escalated" {
            plan_status::update_plan_status(&state.plan_file, "ESCALATED");
        }

        serde_json::json!({
            "convergence_id": p.convergence_id,
            "phase": format!("{:?}", state.phase),
            "round": state.round,
            "consecutive_clean": state.consecutive_clean,
            "task": task,
        }).to_string()
    }

    // --- Planning tools ---

    #[tool(description = "Validate a build plan's structure.")]
    async fn validate_plan_structure(&self, params: Parameters<PlanFileParam>) -> String {
        let result = plan_validator::validate_plan(&params.0.plan_file);
        serde_json::json!({
            "valid": result.valid(),
            "errors": result.errors,
            "warnings": result.warnings,
        }).to_string()
    }

    // 10. create_plan_template
    #[tool(description = "Generate a build plan template — fill in the phases and checklists.")]
    async fn create_plan_template(&self, params: Parameters<GoalParam>) -> String {
        let goal = &params.0.goal;
        format!(
            "# BUILD_PLAN_NNN: {goal}\n\n\
             ## Document Alignment\n\n\
             - docs/PRODUCT_SPEC.md — product requirements\n\
             - docs/ARCHITECTURE.md — system architecture\n\n\
             ## Phase 1: Design\n\n\
             - [ ] Define requirements\n\
             - [ ] Write technical spec\n\n\
             ## Phase 2: Implementation\n\n\
             - [ ] Implement core feature\n\
             - [ ] Write tests (100% coverage)\n\n\
             ## Phase 3: Integration & Polish\n\n\
             - [ ] Integration tests\n\
             - [ ] Update documentation\n\n\
             ## Convergence Criteria\n\n\
             - All tests pass\n\
             - 100% coverage\n\
             - Two consecutive clean audit passes\n\n\
             ## Test Command\n\n\
             ```bash\n\
             python3 -m pytest tests/ -v --tb=short --cov=src --cov-fail-under=100\n\
             ```\n"
        )
    }

    // --- Bootstrap tools ---

    #[tool(description = "Get the CruxDev development methodology.")]
    async fn get_methodology(&self) -> String {
        let path = self.project_dir.join("docs").join("DEVELOPMENT_PATTERNS_CRUXDEV.md");
        std::fs::read_to_string(&path).unwrap_or_else(|_| format!("Not found: {}", path.display()))
    }

    #[tool(description = "Get the adoption process guide.")]
    async fn get_adoption_process(&self) -> String {
        let path = self.project_dir.join("docs").join("ADOPTION_PROCESS.md");
        std::fs::read_to_string(&path).unwrap_or_else(|_| format!("Not found: {}", path.display()))
    }

    // 11. install_cruxdev
    #[tool(description = "Install CruxDev MCP server into a project.")]
    async fn install_cruxdev(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let result = crate::install::install(&dir);
        result.to_string()
    }

    // 12. cruxdev_status
    #[tool(description = "Check CruxDev installation health — is everything wired and working?")]
    async fn cruxdev_status(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let report = crate::status::get_status(&dir);
        serde_json::json!({
            "healthy": report.healthy,
            "checks": report.checks.iter().map(|c| serde_json::json!({
                "name": c.name, "passed": c.passed, "message": c.message
            })).collect::<Vec<_>>(),
        }).to_string()
    }

    // --- Session bus tools ---

    #[tool(description = "Register this session with the session bus.")]
    async fn session_register(&self, params: Parameters<SessionRegisterParam>) -> String {
        let name = params.0.project_name
            .unwrap_or_else(|| self.source_project());
        let dir = self.project_dir.to_string_lossy().to_string();
        match self.get_broker() {
            Ok(broker) => {
                match broker.register_session(&name, &dir) {
                    Ok(session_id) => serde_json::json!({
                        "session_id": session_id,
                        "project": name,
                        "status": "registered",
                    }).to_string(),
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 13. session_list
    #[tool(description = "List all active CruxDev sessions across all projects.")]
    async fn session_list(&self) -> String {
        match self.get_broker() {
            Ok(broker) => {
                match broker.list_sessions(3600.0) {
                    Ok(sessions) => {
                        let n = now();
                        let list: Vec<serde_json::Value> = sessions.iter().map(|s| serde_json::json!({
                            "id": s.id,
                            "project": s.project,
                            "directory": s.directory,
                            "last_heartbeat_ago": ((n - s.last_heartbeat) * 10.0).round() / 10.0,
                        })).collect();
                        serde_json::json!(list).to_string()
                    }
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 14. report_issue
    #[tool(description = "Report an issue you've discovered to another project's session.")]
    async fn report_issue(&self, params: Parameters<ReportIssueParam>) -> String {
        let p = &params.0;
        let source = self.source_project();
        let severity = p.severity.as_deref().unwrap_or("medium");
        match self.get_broker() {
            Ok(broker) => {
                match broker.send_message("issue", &source, &p.target_project, &p.title, &p.body, severity) {
                    Ok(msg_id) => serde_json::json!({
                        "message_id": msg_id,
                        "status": "sent",
                        "from": source,
                        "to": p.target_project,
                    }).to_string(),
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 15. report_improvement
    #[tool(description = "Suggest an improvement to another project.")]
    async fn report_improvement(&self, params: Parameters<ReportImprovementParam>) -> String {
        let p = &params.0;
        let source = self.source_project();
        match self.get_broker() {
            Ok(broker) => {
                match broker.send_message("improvement", &source, &p.target_project, &p.title, &p.body, "medium") {
                    Ok(msg_id) => serde_json::json!({
                        "message_id": msg_id,
                        "status": "sent",
                        "from": source,
                        "to": p.target_project,
                    }).to_string(),
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 16. share_pattern
    #[tool(description = "Share a pattern you've learned with all other sessions.")]
    async fn share_pattern(&self, params: Parameters<SharePatternParam>) -> String {
        let p = &params.0;
        let source = self.source_project();
        match self.get_broker() {
            Ok(broker) => {
                match broker.send_message("pattern", &source, "*", &p.pattern_name, &p.description, "low") {
                    Ok(msg_id) => serde_json::json!({
                        "message_id": msg_id,
                        "status": "broadcast",
                        "from": source,
                        "pattern": p.pattern_name,
                    }).to_string(),
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 17. notify_breaking_change
    #[tool(description = "Notify other projects of a breaking change you've made.")]
    async fn notify_breaking_change(&self, params: Parameters<NotifyBreakingChangeParam>) -> String {
        let p = &params.0;
        let source = self.source_project();
        let projects: Vec<&str> = p.affected_projects.split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        match self.get_broker() {
            Ok(broker) => {
                let mut msg_ids = Vec::new();
                for proj in &projects {
                    match broker.send_message("breaking_change", &source, proj, "Breaking change", &p.description, "high") {
                        Ok(id) => msg_ids.push(id),
                        Err(e) => return serde_json::json!({"error": format!("{e}")}).to_string(),
                    }
                }
                serde_json::json!({
                    "message_ids": msg_ids,
                    "status": "sent",
                    "from": source,
                    "to": projects,
                }).to_string()
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 18. check_inbox
    #[tool(description = "Check for messages from other sessions.")]
    async fn check_inbox(&self, params: Parameters<CheckInboxParam>) -> String {
        let project = params.0.project_name.unwrap_or_else(|| self.source_project());
        match self.get_broker() {
            Ok(broker) => {
                match broker.check_inbox(&project) {
                    Ok(messages) => {
                        let n = now();
                        let list: Vec<serde_json::Value> = messages.iter().map(|m| serde_json::json!({
                            "id": m.id,
                            "type": m.msg_type,
                            "from": m.source_project,
                            "title": m.title,
                            "body": m.body,
                            "severity": m.severity,
                            "age_seconds": ((n - m.created_at) * 10.0).round() / 10.0,
                        })).collect();
                        serde_json::json!(list).to_string()
                    }
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 19. acknowledge_message
    #[tool(description = "Mark a message as handled.")]
    async fn acknowledge_message(&self, params: Parameters<AcknowledgeMessageParam>) -> String {
        match self.get_broker() {
            Ok(broker) => {
                match broker.acknowledge(&params.0.message_id) {
                    Ok(found) => serde_json::json!({
                        "message_id": params.0.message_id,
                        "acknowledged": found,
                    }).to_string(),
                    Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
                }
            }
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // --- Adoption tools ---

    // 20. classify_project
    #[tool(description = "Classify a project — determine type(s), maturity, and required templates.")]
    async fn classify_project(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let result = crate::adoption::classify::classify_project(&dir);
        serde_json::json!({
            "primary_type": result.primary_type.as_str(),
            "secondary_types": result.secondary_types.iter().map(|t| t.as_str()).collect::<Vec<_>>(),
            "maturity": result.maturity.as_str(),
            "confidence": result.confidence,
            "signals": result.signals,
        }).to_string()
    }

    // 21. inventory_project
    #[tool(description = "Inventory all project materials — documents, code, assets.")]
    async fn inventory_project(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let result = crate::adoption::inventory::inventory_project(&dir, true, true);
        let by_fmt = result.by_format();
        let fmt_counts: HashMap<String, usize> = by_fmt.iter().map(|(k, v)| (k.clone(), v.len())).collect();
        serde_json::json!({
            "total_items": result.items.len(),
            "total_size": result.total_size(),
            "by_format": fmt_counts,
            "markdown": result.to_markdown(),
        }).to_string()
    }

    // 22. get_templates
    #[tool(description = "Get required document templates for a project type and maturity.")]
    async fn get_templates(&self, params: Parameters<GetTemplatesParam>) -> String {
        let p = &params.0;
        let maturity = p.maturity.as_deref().unwrap_or("minimal");
        let ts = crate::adoption::templates::get_templates_for_type(&p.project_type, maturity);
        let by_cat = ts.by_category();
        let cat_counts: HashMap<String, usize> = by_cat.iter().map(|(k, v)| (k.clone(), v.len())).collect();
        serde_json::json!({
            "total": ts.templates.len(),
            "required": ts.required().len(),
            "templates": ts.templates.iter().map(|t| serde_json::json!({
                "category": t.category,
                "name": t.name,
                "filename": t.filename,
                "requirement": t.requirement.as_str(),
            })).collect::<Vec<_>>(),
            "by_category": cat_counts,
        }).to_string()
    }

    // 23. analyze_gaps
    #[tool(description = "Analyze gaps between project state and template requirements.")]
    async fn analyze_gaps(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let classification = crate::adoption::classify::classify_project(&dir);
        let inventory = crate::adoption::inventory::inventory_project(&dir, true, true);
        let templates = crate::adoption::templates::get_templates_for_type(
            classification.primary_type.as_str(), classification.maturity.as_str(),
        );
        let result = crate::adoption::gaps::analyze_gaps(&dir, &inventory, &templates);

        serde_json::json!({
            "project_type": classification.primary_type.as_str(),
            "maturity": classification.maturity.as_str(),
            "total_gaps": result.open_gaps().len(),
            "critical": result.critical().len(),
            "gaps": result.open_gaps().iter().map(|g| serde_json::json!({
                "name": g.template_name,
                "file": g.template_file,
                "severity": g.severity.as_str(),
                "reason": g.reason,
            })).collect::<Vec<_>>(),
            "markdown": result.to_markdown(),
        }).to_string()
    }

    // 24. gap_status
    #[tool(description = "Show current gap counts by priority for a project.")]
    async fn gap_status(&self, params: Parameters<ProjectDirParam>) -> String {
        let dir = params.0.project_dir.unwrap_or_else(|| ".".to_string());
        let classification = crate::adoption::classify::classify_project(&dir);
        let inventory = crate::adoption::inventory::inventory_project(&dir, true, true);
        let templates = crate::adoption::templates::get_templates_for_type(
            classification.primary_type.as_str(), classification.maturity.as_str(),
        );
        let result = crate::adoption::gaps::analyze_gaps(&dir, &inventory, &templates);
        let by_sev = result.by_severity();

        serde_json::json!({
            "critical": by_sev.get("critical").map(|v| v.len()).unwrap_or(0),
            "high": by_sev.get("high").map(|v| v.len()).unwrap_or(0),
            "medium": by_sev.get("medium").map(|v| v.len()).unwrap_or(0),
            "low": by_sev.get("low").map(|v| v.len()).unwrap_or(0),
            "total_open": result.open_gaps().len(),
        }).to_string()
    }

    // --- Research tools ---

    // 25. research_topic
    #[tool(description = "Start a research session on a topic.")]
    async fn research_topic(&self, params: Parameters<ResearchTopicParam>) -> String {
        let p = &params.0;
        let questions = p.sub_questions.as_ref().map(|s|
            s.split(',').map(|q| q.trim().to_string()).filter(|q| !q.is_empty()).collect::<Vec<_>>()
        );
        let session = crate::research::session::create_session(&p.topic, questions);

        serde_json::json!({
            "session_id": session.session_id,
            "topic": session.topic,
            "sub_questions": session.sub_questions,
            "current_pass": session.current_pass,
            "instructions": "Execute 5 passes: broad, academic, practitioner, contrarian, primary. Submit findings after each search.",
        }).to_string()
    }

    // 26. research_status
    #[tool(description = "Check convergence status of a research session.")]
    async fn research_status(&self, params: Parameters<ResearchStatusParam>) -> String {
        let session_id = &params.0.session_id;
        let checkpoint_dir = self.project_dir.join(".cruxdev").join("research");

        // Try to load from checkpoint
        let checkpoint_path = checkpoint_dir.join(format!("{session_id}.json"));
        if let Some(session) = crate::research::session::load_checkpoint(
            checkpoint_path.to_str().unwrap_or("")
        ) {
            return serde_json::json!({
                "session_id": session.session_id,
                "topic": session.topic,
                "sub_questions": session.sub_questions,
                "current_pass": session.current_pass,
                "total_searches": session.total_searches,
                "findings_count": session.findings.len(),
                "converged": session.converged,
                "budget_exhausted": session.budget_exhausted,
                "quality_score": session.quality_score,
                "novelty_scores": session.novelty_scores,
            }).to_string();
        }

        // Also check guided research sessions in memory
        let sessions = GUIDED_RESEARCH_SESSIONS.lock().unwrap();
        if let Some(session) = sessions.get(session_id) {
            return serde_json::json!({
                "session_id": session_id,
                "topic": session.competitor_name,
                "current_pass": session.current_pass,
                "status": if session.is_done() { "completed" } else { "in_progress" },
            }).to_string();
        }

        serde_json::json!({
            "session_id": session_id,
            "error": "Session not found. Check session_id or start a new session.",
        }).to_string()
    }

    // 27. verify_research_sources
    #[tool(description = "Run source verification pipeline on research findings.")]
    async fn verify_research_sources(&self, params: Parameters<VerifyResearchSourcesParam>) -> String {
        let p = &params.0;
        let urls: Vec<String> = p.source_urls.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .unwrap_or_default();

        let mut sources = Vec::new();
        for url in &urls {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                sources.push(serde_json::json!({
                    "url": url, "reachable": false, "status_code": serde_json::Value::Null,
                    "error": "Invalid URL scheme",
                }));
                continue;
            }
            match client.head(url).send().await {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    sources.push(serde_json::json!({
                        "url": url, "reachable": status < 400,
                        "status_code": status, "error": serde_json::Value::Null,
                    }));
                }
                Err(e) => {
                    sources.push(serde_json::json!({
                        "url": url, "reachable": false,
                        "status_code": serde_json::Value::Null,
                        "error": format!("{e}"),
                    }));
                }
            }
        }

        let reachable_count = sources.iter().filter(|s| s["reachable"] == true).count();

        serde_json::json!({
            "finding_id": p.finding_id,
            "overall_verified": reachable_count == sources.len(),
            "reachable_count": reachable_count,
            "total_sources": sources.len(),
            "sources": sources,
        }).to_string()
    }

    // 28. counter_research
    #[tool(description = "Run adversarial verification on a claim.")]
    async fn counter_research(&self, params: Parameters<CounterResearchParam>) -> String {
        let p = &params.0;
        let counter = p.counter_evidence.as_ref().map(|s|
            s.split('|').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect::<Vec<_>>()
        );
        let alts = p.alternative_explanations.as_ref().map(|s|
            s.split('|').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect::<Vec<_>>()
        );
        let supporting = p.supporting_count.unwrap_or(1) as usize;

        let result = crate::research::counter::run_counter_research(
            &p.claim, counter, alts, supporting,
        );

        serde_json::json!({
            "claim": result.original_claim,
            "robustness": result.robustness,
            "is_contested": result.is_contested(),
            "negation_queries": result.negation_queries,
            "counter_evidence": result.counter_evidence,
            "alternative_explanations": result.alternative_explanations,
        }).to_string()
    }

    // --- Competitors tools ---

    // 29. research_competitor_start
    #[tool(description = "Start guided 5-pass research on a competitor.")]
    async fn research_competitor_start(&self, params: Parameters<ResearchCompetitorStartParam>) -> String {
        let p = &params.0;
        let url = p.competitor_url.as_deref().unwrap_or("");
        let category = p.category.as_deref().unwrap_or("");

        let mut sessions = GUIDED_RESEARCH_SESSIONS.lock().unwrap();
        let key = p.competitor_name.to_lowercase();
        let is_new = !sessions.contains_key(&key);

        if is_new {
            let state = crate::competitors::guided_research::start_research(
                &p.competitor_name, url, category,
            );
            sessions.insert(key.clone(), state);
        }

        let state = sessions.get(&key).unwrap();
        let step = crate::competitors::guided_research::get_next_step(state);

        serde_json::json!({
            "session_created": is_new,
            "competitor": p.competitor_name,
            "step": step,
        }).to_string()
    }

    // 30. research_competitor_next_step
    #[tool(description = "Get the next research instruction for a competitor.")]
    async fn research_competitor_next_step(&self, params: Parameters<ResearchCompetitorNextStepParam>) -> String {
        let key = params.0.competitor_name.to_lowercase();
        let sessions = GUIDED_RESEARCH_SESSIONS.lock().unwrap();

        match sessions.get(&key) {
            Some(state) => {
                let step = crate::competitors::guided_research::get_next_step(state);
                serde_json::json!({"step": step}).to_string()
            }
            None => serde_json::json!({
                "error": format!("No research session for '{}'. Call research_competitor_start first.", params.0.competitor_name),
            }).to_string(),
        }
    }

    // 31. research_competitor_submit
    #[tool(description = "Submit findings for the current research pass.")]
    async fn research_competitor_submit(&self, params: Parameters<ResearchCompetitorSubmitParam>) -> String {
        let p = &params.0;
        let key = p.competitor_name.to_lowercase();

        let finding_list: Vec<String> = p.findings.split('|')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        let source_list: Option<Vec<String>> = p.sources.as_ref().map(|s|
            s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()
        );

        let profile_updates: Option<HashMap<String, serde_json::Value>> = p.profile_updates.as_ref().and_then(|s| {
            serde_json::from_str(s).ok()
        });

        let mut sessions = GUIDED_RESEARCH_SESSIONS.lock().unwrap();
        match sessions.get_mut(&key) {
            Some(state) => {
                let result = crate::competitors::guided_research::submit_pass_result(
                    state, finding_list, source_list, None, profile_updates,
                );
                result.to_string()
            }
            None => serde_json::json!({
                "error": format!("No research session for '{}'. Call research_competitor_start first.", p.competitor_name),
            }).to_string(),
        }
    }

    // 32. research_competitor_list
    #[tool(description = "List all active guided research sessions.")]
    async fn research_competitor_list(&self) -> String {
        let sessions = GUIDED_RESEARCH_SESSIONS.lock().unwrap();
        let list: Vec<serde_json::Value> = sessions.values().map(|s| s.to_dict()).collect();
        serde_json::json!(list).to_string()
    }

    // 33. setup_competitive_analysis
    #[tool(description = "Run complete competitive analysis in one call.")]
    async fn setup_competitive_analysis(&self, params: Parameters<SetupCompetitiveAnalysisParam>) -> String {
        let p = &params.0;
        let features: Vec<String> = p.our_features.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let raw: serde_json::Value = match serde_json::from_str(&p.competitors_json) {
            Ok(v) => v,
            Err(e) => return serde_json::json!({"error": format!("Invalid competitors_json: {e}")}).to_string(),
        };
        let competitors = crate::competitors::runner::parse_competitor_inputs(&raw);

        let result = crate::competitors::runner::setup(
            &p.our_name, &p.our_description, &p.our_category, &features, &competitors,
        );

        let mut files_written: Vec<String> = Vec::new();
        if p.write_files.unwrap_or(true) {
            let proj = p.project_dir.as_deref().unwrap_or(".");
            let proj_dir = if proj == "." {
                self.project_dir.to_string_lossy().to_string()
            } else {
                proj.to_string()
            };
            if let Ok(written) = crate::competitors::runner::write_results(&result, &proj_dir, "docs", "docs/vs") {
                files_written = written.iter().map(|p| p.to_string_lossy().to_string()).collect();
            }
        }

        serde_json::json!({
            "summary": result.summary,
            "files_written": files_written,
            "gap_analysis": result.gap_analysis,
            "discovery_queries": result.discovery_queries,
            "competitors_doc_preview": &result.competitors_doc[..result.competitors_doc.len().min(2000)],
        }).to_string()
    }

    // 34. discover_competitors
    #[tool(description = "Generate search queries for finding competitors.")]
    async fn discover_competitors(&self, params: Parameters<DiscoverCompetitorsParam>) -> String {
        let p = &params.0;
        let queries = crate::competitors::discovery::generate_discovery_queries(
            &p.project_description, &p.category, 10,
        );
        serde_json::json!({
            "queries": queries,
            "instructions": "Run each query via web search, then call research_competitor() for each result.",
        }).to_string()
    }

    // 35. research_competitor (profile parsing)
    #[tool(description = "Parse research text into a structured competitor profile.")]
    async fn research_competitor(&self, params: Parameters<ResearchCompetitorParam>) -> String {
        let p = &params.0;
        let profile = crate::competitors::research::parse_profile_response(
            &p.name, &p.url, &p.research_text,
        );
        serde_json::json!({
            "name": profile.name,
            "url": profile.url,
            "tagline": profile.tagline,
            "category": profile.category.as_str(),
            "pricing": profile.pricing,
            "tech_stack": profile.tech_stack,
            "features": profile.features.iter().map(|f| serde_json::json!({
                "name": f.name, "has": f.has_feature,
            })).collect::<Vec<_>>(),
            "strengths": profile.strengths,
            "weaknesses": profile.weaknesses,
            "differentiation": profile.differentiation,
            "markdown": profile.to_markdown(),
        }).to_string()
    }

    // 36. verify_competitor_links
    #[tool(description = "Test all URLs in a competitor profile, returns pass/fail per link.")]
    async fn verify_competitor_links(&self, params: Parameters<VerifyCompetitorLinksParam>) -> String {
        let p = &params.0;
        let urls: Vec<String> = p.profile_markdown
            .split(['(', ' ', '\n'])
            .filter(|s| s.starts_with("http://") || s.starts_with("https://"))
            .map(|s| s.trim_end_matches([')', ',', ';']).to_string())
            .collect();

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .unwrap_or_default();

        let mut links = Vec::new();
        for url in &urls {
            match client.head(url).send().await {
                Ok(resp) => {
                    let code = resp.status().as_u16();
                    links.push(serde_json::json!({
                        "url": url,
                        "status": if code < 400 { "pass" } else { "fail" },
                        "code": code,
                        "error": serde_json::Value::Null,
                    }));
                }
                Err(e) => {
                    links.push(serde_json::json!({
                        "url": url,
                        "status": "fail",
                        "code": serde_json::Value::Null,
                        "error": format!("{e}"),
                    }));
                }
            }
        }

        let passed = links.iter().filter(|l| l["status"] == "pass").count();
        serde_json::json!({
            "competitor": p.competitor_name,
            "total_links": links.len(),
            "passed": passed,
            "failed": links.len() - passed,
            "links": links,
        }).to_string()
    }

    // 37. generate_gap_analysis
    #[tool(description = "Run gap analysis comparing our features against competitors.")]
    async fn generate_gap_analysis(&self, params: Parameters<GenerateGapAnalysisParam>) -> String {
        let p = &params.0;
        let features: Vec<String> = p.our_features.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let raw: serde_json::Value = match serde_json::from_str(&p.competitors_json) {
            Ok(v) => v,
            Err(e) => return serde_json::json!({"error": format!("Invalid competitors_json: {e}")}).to_string(),
        };

        let inputs = crate::competitors::runner::parse_competitor_inputs(&raw);
        let profiles: Vec<crate::competitors::research::CompetitorProfile> = inputs.iter().map(|inp| {
            let mut profile = crate::competitors::research::CompetitorProfile::new(&inp.name, &inp.url);
            profile.category = crate::competitors::research::CompetitorCategory::from_str_loose(&inp.category);
            profile.features = inp.features.iter().map(|f| crate::competitors::research::Feature::new(f, "")).collect();
            profile
        }).collect();

        let result = crate::competitors::gap_analysis::run_gap_analysis(
            &p.our_name, &features, &profiles, None,
        );

        serde_json::json!({
            "our_name": result.our_name,
            "total_features": result.feature_matrix.len(),
            "total_gaps": result.gaps.len(),
            "must_close": result.must_close().len(),
            "should_close": result.should_close().len(),
            "gaps": result.gaps.iter().map(|g| serde_json::json!({
                "feature": g.feature_name,
                "priority": g.priority.as_str(),
                "competitors": g.competitors_with_feature,
                "status": g.status.as_str(),
            })).collect::<Vec<_>>(),
            "markdown": result.to_markdown(),
        }).to_string()
    }

    // 38. generate_comparison_page
    #[tool(description = "Generate a /vs/<competitor> comparison page for the project website.")]
    async fn generate_comparison_page(&self, params: Parameters<GenerateComparisonPageParam>) -> String {
        let p = &params.0;
        let features: Vec<String> = p.our_features.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let profile = crate::competitors::research::parse_profile_response(
            &p.competitor_name, &p.competitor_url, &p.competitor_research,
        );

        // Generate comparison content
        let slug = p.competitor_name.to_lowercase().replace(' ', "-");
        let title = format!("{} vs {}", p.our_name, p.competitor_name);

        let mut lines = vec![
            format!("---"),
            format!("title: \"{title}\""),
            format!("slug: \"/vs/{slug}\""),
            format!("---"),
            String::new(),
            format!("# {title}"),
            String::new(),
            format!("## Feature Comparison"),
            String::new(),
            format!("| Feature | {} | {} |", p.our_name, p.competitor_name),
            format!("|---|---|---|"),
        ];

        for feat in &features {
            let comp_has = profile.features.iter().any(|f| f.name.to_lowercase() == feat.to_lowercase() && f.has_feature);
            lines.push(format!("| {} | Y | {} |", feat, if comp_has { "Y" } else { "N" }));
        }
        lines.push(String::new());

        if !profile.strengths.is_empty() {
            lines.push(format!("## {} Strengths", p.competitor_name));
            for s in &profile.strengths {
                lines.push(format!("- {s}"));
            }
            lines.push(String::new());
        }

        if !profile.weaknesses.is_empty() {
            lines.push(format!("## {} Weaknesses", p.competitor_name));
            for w in &profile.weaknesses {
                lines.push(format!("- {w}"));
            }
            lines.push(String::new());
        }

        let markdown = lines.join("\n");

        serde_json::json!({
            "slug": slug,
            "title": title,
            "features_compared": features.len(),
            "markdown": markdown,
        }).to_string()
    }

    // 39. generate_gap_build_plan
    #[tool(description = "Create a build plan to close a specific competitive gap.")]
    async fn generate_gap_build_plan(&self, params: Parameters<GenerateGapBuildPlanParam>) -> String {
        let p = &params.0;
        let comps: Vec<String> = p.competitors_with_feature.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let priority_str = match p.priority.as_str() {
            "must-close" => "Must-Close",
            "should-close" => "Should-Close",
            "nice-to-have" => "Nice-To-Have",
            _ => &p.priority,
        };

        let context_str = p.context.as_deref().unwrap_or("");
        let filename = format!("BUILD_PLAN_{:03}_{}.md", p.plan_number,
            p.feature_name.to_lowercase().replace(' ', "_"));

        let content = format!(
            "# BUILD_PLAN_{:03}: {}\n\n\
             **Priority:** {priority_str}\n\
             **Gap:** {} has this feature (competitors: {})\n\n\
             ## Document Alignment\n\n\
             - docs/COMPETITORS.md — gap analysis reference\n\n\
             ## Context\n\n\
             {context_str}\n\n\
             ## Phase 1: Research & Design\n\n\
             - [ ] Analyze how competitors implement {}\n\
             - [ ] Define our approach and differentiators\n\
             - [ ] Write technical spec\n\n\
             ## Phase 2: Implementation\n\n\
             - [ ] Implement core feature\n\
             - [ ] Write tests (100% coverage)\n\
             - [ ] Integration tests\n\n\
             ## Phase 3: Documentation & Launch\n\n\
             - [ ] Update docs\n\
             - [ ] Update COMPETITORS.md gap status\n\
             - [ ] Deploy\n\n\
             ## Convergence Criteria\n\n\
             - All tests pass\n\
             - 100% coverage\n\
             - Two consecutive clean audit passes\n\
             - Gap marked as closed in COMPETITORS.md\n",
            p.plan_number, p.feature_name,
            p.feature_name, comps.join(", "),
            p.feature_name,
        );

        serde_json::json!({
            "filename": filename,
            "content": content,
        }).to_string()
    }

    // 40. monitor_issues
    #[tool(description = "Check GitHub issues, evaluate priority, and respond (dry-run by default). 5-layer prompt injection defense.")]
    async fn monitor_issues(&self, params: Parameters<MonitorIssuesParam>) -> String {
        let p = &params.0;
        let dry_run = p.dry_run.unwrap_or(true);
        let limit = p.limit.unwrap_or(20);
        let project_dir = self.project_dir.to_string_lossy().to_string();

        match crate::evolution::github::monitor_issues(&p.repo, &project_dir, dry_run, limit) {
            Ok(responses) => {
                let total = responses.len();
                let commented = responses.iter().filter(|r| r.action == "comment").count();
                serde_json::json!({
                    "repo": p.repo,
                    "dry_run": dry_run,
                    "issues_processed": total,
                    "comments_generated": commented,
                    "responses": responses,
                }).to_string()
            }
            Err(e) => serde_json::json!({"error": format!("{e}")}).to_string(),
        }
    }

    // 41. issue_audit_log
    #[tool(description = "View recent issue evaluation audit trail.")]
    async fn issue_audit_log(&self, params: Parameters<IssueAuditLogParam>) -> String {
        let p = &params.0;
        let limit = p.limit.unwrap_or(20);
        let proj = p.project_dir.as_deref().unwrap_or(".");
        let proj_dir = if proj == "." {
            self.project_dir.to_string_lossy().to_string()
        } else {
            proj.to_string()
        };
        let audit_path = format!("{proj_dir}/.cruxdev/evolution/issue_audit.jsonl");

        match std::fs::read_to_string(&audit_path) {
            Ok(content) => {
                let entries: Vec<serde_json::Value> = content
                    .lines()
                    .rev()
                    .take(limit)
                    .filter_map(|l| serde_json::from_str(l).ok())
                    .collect();
                serde_json::json!({
                    "total_entries": entries.len(),
                    "entries": entries,
                }).to_string()
            }
            Err(_) => serde_json::json!({
                "total_entries": 0,
                "entries": [],
                "note": "No audit log found. Run monitor_issues first.",
            }).to_string(),
        }
    }

    // 42. git_commit_changes
    #[tool(description = "Stage specific files and commit with safety checks. Dry-run by default. NEVER stages all files.")]
    async fn git_commit_changes(&self, params: Parameters<GitCommitParam>) -> String {
        let p = &params.0;
        let dry_run = p.dry_run.unwrap_or(true);
        let proj = p.project_dir.as_deref().unwrap_or(".");
        let proj_dir = if proj == "." {
            self.project_dir.to_string_lossy().to_string()
        } else {
            proj.to_string()
        };

        let files: Vec<String> = p.files.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Safety check
        let check = match crate::git::pre_commit_safety_check(&proj_dir, &files) {
            Ok(c) => c,
            Err(e) => return serde_json::json!({"error": e}).to_string(),
        };

        if !check.passed {
            return serde_json::json!({
                "error": "Safety check failed",
                "violations": check.violations,
            }).to_string();
        }

        if dry_run {
            return serde_json::json!({
                "dry_run": true,
                "would_commit": files,
                "message": p.message,
                "safety_check": "passed",
            }).to_string();
        }

        // Stage files
        if let Err(e) = crate::git::add(&proj_dir, &files) {
            return serde_json::json!({"error": format!("Stage failed: {e}")}).to_string();
        }

        // Commit
        match crate::git::commit(&proj_dir, &p.message) {
            Ok(result) => serde_json::json!(result).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 43. git_push_changes
    #[tool(description = "Push commits to remote with optional test gate. Dry-run by default. Never force pushes.")]
    async fn git_push_changes(&self, params: Parameters<GitPushParam>) -> String {
        let p = &params.0;
        let dry_run = p.dry_run.unwrap_or(true);
        let proj = p.project_dir.as_deref().unwrap_or(".");
        let proj_dir = if proj == "." {
            self.project_dir.to_string_lossy().to_string()
        } else {
            proj.to_string()
        };
        let remote = p.remote.as_deref().unwrap_or("origin");
        let branch = match &p.branch {
            Some(b) => b.clone(),
            None => crate::git::current_branch(&proj_dir).unwrap_or("master".into()),
        };

        // Pre-push test gate
        if let Some(cmd) = &p.test_command {
            let test_args: Vec<String> = cmd.split_whitespace().map(|s| s.to_string()).collect();
            let check = match crate::git::pre_push_test_gate(&proj_dir, &test_args) {
                Ok(c) => c,
                Err(e) => return serde_json::json!({"error": e}).to_string(),
            };
            if !check.passed {
                return serde_json::json!({
                    "error": "Pre-push test gate failed",
                    "violations": check.violations,
                }).to_string();
            }
        }

        if dry_run {
            return serde_json::json!({
                "dry_run": true,
                "would_push_to": format!("{remote}/{branch}"),
                "test_gate": "passed",
            }).to_string();
        }

        match crate::git::push(&proj_dir, remote, &branch) {
            Ok(result) => serde_json::json!(result).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 44. create_pull_request
    #[tool(description = "Create a GitHub pull request. Dry-run by default.")]
    async fn create_pull_request(&self, params: Parameters<CreatePrParam>) -> String {
        let p = &params.0;
        let dry_run = p.dry_run.unwrap_or(true);
        let base = p.base.as_deref().unwrap_or("master");
        let proj_dir = self.project_dir.to_string_lossy().to_string();
        let head = match &p.head {
            Some(h) => h.clone(),
            None => crate::git::current_branch(&proj_dir).unwrap_or("HEAD".into()),
        };
        let repo = p.repo.as_deref().unwrap_or("");

        let body = p.body.as_deref().unwrap_or("*No description provided.*");

        if dry_run {
            return serde_json::json!({
                "dry_run": true,
                "would_create_pr": {
                    "title": p.title,
                    "base": base,
                    "head": head,
                    "repo": repo,
                    "body_preview": &body[..body.len().min(200)],
                },
            }).to_string();
        }

        if repo.is_empty() {
            return serde_json::json!({"error": "repo is required for live PR creation"}).to_string();
        }

        match crate::git::create_pr(repo, &p.title, body, base, &head) {
            Ok(result) => serde_json::json!(result).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 45. merge_pull_request
    #[tool(description = "Merge a GitHub pull request. Dry-run by default. Squash merge by default.")]
    async fn merge_pull_request(&self, params: Parameters<MergePrParam>) -> String {
        let p = &params.0;
        let dry_run = p.dry_run.unwrap_or(true);
        let method = p.method.as_deref().unwrap_or("squash");

        if dry_run {
            return serde_json::json!({
                "dry_run": true,
                "would_merge": {
                    "pr_number": p.pr_number,
                    "repo": p.repo,
                    "method": method,
                },
            }).to_string();
        }

        match crate::git::merge_pr(&p.repo, p.pr_number, method) {
            Ok(true) => serde_json::json!({
                "success": true,
                "pr_number": p.pr_number,
                "method": method,
            }).to_string(),
            Ok(false) => serde_json::json!({
                "success": false,
                "error": "Merge failed — check CI status or merge conflicts",
            }).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }

    // 46. git_status_check
    #[tool(description = "Get full git status for decision-making: branch, staged, unstaged, untracked files.")]
    async fn git_status_check(&self, params: Parameters<GitStatusParam>) -> String {
        let proj = params.0.project_dir.as_deref().unwrap_or(".");
        let proj_dir = if proj == "." {
            self.project_dir.to_string_lossy().to_string()
        } else {
            proj.to_string()
        };

        match crate::git::status(&proj_dir) {
            Ok(st) => serde_json::json!({
                "branch": st.branch,
                "staged": st.staged,
                "unstaged": st.unstaged,
                "untracked": st.untracked,
                "clean": st.staged.is_empty() && st.unstaged.is_empty() && st.untracked.is_empty(),
            }).to_string(),
            Err(e) => serde_json::json!({"error": e}).to_string(),
        }
    }
}

/// Start the MCP server on stdio transport.
pub async fn run_server() {
    let server = CruxDevServer::new();
    match server.serve(rmcp::transport::io::stdio()).await {
        Ok(service) => {
            let _ = service.waiting().await;
        }
        Err(e) => {
            eprintln!("CruxDev MCP server error: {e}");
            std::process::exit(1);
        }
    }
}
