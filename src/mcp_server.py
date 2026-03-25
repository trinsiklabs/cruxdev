"""CruxDev MCP server — exposes convergence engine to Claude Code.

Claude Code drives the convergence loop by calling these tools.
The engine owns all state, counters, timeouts, and termination logic.
Claude Code owns the LLM reasoning (auditing, fixing, writing).

Run with: python3 -m src.mcp_server
"""

import json
import os
import time
import uuid

from mcp.server.fastmcp import FastMCP

from .dispatch.llm import LLMDispatcher
from .dispatch.providers.stub import StubMode, StubProvider
from .engine.convergence import is_terminal
from .engine.persistence import load_state, save_state
from .engine.plan_validator import get_plan_template, validate_plan
from .engine.runner import ConvergenceRunner
from .engine.state import ConvergencePhase, ConvergenceState
from .engine.task_router import Task, get_next_task, submit_result

# --- Configuration ---

CRUXDEV_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
STATE_DIR = os.path.join(CRUXDEV_ROOT, ".cruxdev", "convergence_state")

# --- Server ---

mcp = FastMCP(
    "cruxdev",
    instructions="""CruxDev is an autonomous convergence engine. It drives code through
audit-fix-re-audit loops until two consecutive independent clean passes are achieved.

BOOTSTRAP: On EVERY session start, BEFORE doing anything else:
1. Call session_register() to join the session bus (auto-detects project name from cwd)
2. Call check_inbox() to process messages from other sessions
3. If a crux MCP is available, call crux.restore_context() to load session state
These steps are MANDATORY. Do not skip them. Do not wait for the user to ask.

CONVERGENCE: When the user says "converge [plan]":
1. Call start_convergence(plan_file) — engine creates state, returns first task
2. Loop: call convergence_next_task(id) to get what to do next
3. Execute the task (read files, audit code, fix issues, run tests)
4. Call convergence_submit_result(id, findings_json) to report back
5. Repeat 2-4 until task_type is "done" or "escalated"
DO NOT decide when to stop — the engine decides.

RESEARCH: ALL research activities MUST follow the 5-pass iterative deepening system:
1. Broad search — establish the landscape
2. Academic/authoritative — find evidence with specifics
3. Practitioner/user — find real-world experience
4. Contrarian/adversarial — find counter-evidence (NOT OPTIONAL)
5. Primary sources — official docs, original data
Research converges when novelty drops below 10% across 5 consecutive searches.
Counter-research is MANDATORY for all claims. Every citation must be verified.
Call get_methodology() for the full methodology document.

SESSION STATE: After every significant action (file edit, decision, task completion),
call crux.update_session() with working_on, add_decision, add_file as appropriate.
This keeps session state always current so context can be restored at any time
without advance notice. This is NOT optional.

POST-EXECUTION: After EVERY convergence, you MUST:
1. Audit all docs against code (two clean passes)
2. Update website metrics if applicable
3. Deploy if applicable
4. Check inbox for messages from other sessions
5. Capture learnings if novel

ADOPTION: Call get_adoption_process() for step-by-step instructions.
PLANNING: Call create_plan_template(goal) to scaffold a build plan.""",
)

os.makedirs(STATE_DIR, exist_ok=True)


def _state_path(convergence_id: str) -> str:
    return os.path.join(STATE_DIR, f"{convergence_id}.json")


# --- Bootstrap tools ---


@mcp.tool()
def get_methodology() -> str:
    """Get the CruxDev development methodology — read this before planning or converging.

    Returns the full DEVELOPMENT_PATTERNS_CRUXDEV.md document containing:
    - The autonomous lifecycle (brainstorming → plan → converge → report)
    - Safety gates (build/test gate, coverage gate, 3-failure rollback, 15-min timeout)
    - Convergence rules (two consecutive independent clean passes)
    - Audit dimensions for code (8) and documentation (5)

    Call this when:
    - Starting a new project or feature
    - Before creating a build plan
    - When you need to understand CruxDev's methodology
    """
    path = os.path.join(CRUXDEV_ROOT, "docs", "DEVELOPMENT_PATTERNS_CRUXDEV.md")
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return "DEVELOPMENT_PATTERNS_CRUXDEV.md not found at " + path


@mcp.tool()
def get_adoption_process() -> str:
    """Get step-by-step instructions for adopting a project into CruxDev.

    Returns ADOPTION_PROCESS.md — a complete guide covering:
    - Installing Crux (intelligence layer)
    - Installing CruxDev (convergence engine)
    - Configuring test commands and coverage enforcement
    - Creating the first build plan
    - Running the first convergence

    Call this when:
    - Setting up CruxDev on a new project for the first time
    - The user asks how to get started with CruxDev
    - You need the adoption checklist
    """
    path = os.path.join(CRUXDEV_ROOT, "docs", "ADOPTION_PROCESS.md")
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return "ADOPTION_PROCESS.md not found at " + path


@mcp.tool()
def install_cruxdev(project_dir: str = ".") -> str:
    """Install CruxDev MCP server into a project.

    Adds the cruxdev MCP server to the project's .claude/mcp.json.
    Preserves any existing MCP servers (like Crux).
    Creates .cruxdev/ directory for convergence state.

    After installation, restart Claude Code to activate the new tools.

    Args:
        project_dir: Path to the project to install into (default: current directory)
    """
    from .install import install
    result = install(project_dir)
    return json.dumps(result, indent=2)


# --- Planning tools ---


@mcp.tool()
def create_plan_template(goal: str) -> str:
    """Generate a build plan template — fill in the phases and checklists.

    Returns a markdown plan skeleton with:
    - Title and goal from your input
    - Phase structure with numbered checklists
    - Test command placeholder
    - Convergence criteria section

    After filling in the template, call validate_plan_structure() to check
    the plan has everything the convergence engine needs.

    Args:
        goal: What you want to build (e.g., "Migrate auth to OAuth2",
              "Add WebSocket support", "Refactor database layer")
    """
    return get_plan_template(goal)


@mcp.tool()
def validate_plan_structure(plan_file: str) -> str:
    """Check if a build plan has the structure the engine needs to converge it.

    Validates (errors = must fix):
    - Has a title (# heading)
    - Has checklist items (- [ ] task)
    - Has a Document Alignment section (lists product docs the plan must conform to)
    - Not too short (> 50 chars)

    Warns (should fix):
    - No numbered phases (## Phase N)
    - No test command references
    - No convergence criteria

    Returns JSON with {valid: bool, errors: [...], warnings: [...]}.
    Fix all errors before calling start_convergence().

    Args:
        plan_file: Absolute path to the build plan markdown file
    """
    result = validate_plan(plan_file)
    return json.dumps(result.to_dict(), indent=2)


# --- Convergence tools ---


@mcp.tool()
def start_convergence(
    plan_file: str,
    timeout_minutes: int = 120,
    max_rounds: int = 5,
    source_files: str = "",
    doc_files: str = "",
    test_command: str = "",
) -> str:
    """Start converging a build plan. Returns the first task for you to execute.

    This begins the convergence loop. The engine will guide you through:
    1. Plan auditing (is the plan complete and feasible?)
    2. Document alignment (does the plan conform to product docs and decisions?)
    3. Execution (green-field: build each checklist item, write code + tests)
    4. Code auditing (8 dimensions: correctness, security, tests, etc.)
    5. Doc auditing (5 dimensions: accuracy, completeness, etc.)
    6. E2E testing (run the test suite)
    7. Convergence (two consecutive clean passes)

    After calling this, enter the convergence loop:
    - Read the returned task
    - Execute it (audit files, fix issues, run tests)
    - Call convergence_submit_result() with your findings
    - Call convergence_next_task() to get the next task
    - Repeat until task_type is "done" or "escalated"

    Args:
        plan_file: Path to the build plan markdown file
        timeout_minutes: Max time for entire convergence (default 120 = 2 hours)
        max_rounds: Max audit rounds per phase before escalation (default 5)
        source_files: Comma-separated source files to audit (e.g. "src/main.py,src/util.py")
        doc_files: Comma-separated doc files to audit (e.g. "README.md,CHANGELOG.md")
        test_command: Shell command to run tests (e.g. "python3 -m pytest tests/ -v")
    """
    convergence_id = str(uuid.uuid4())[:8]
    state = ConvergenceState(
        plan_file=plan_file,
        deadline=time.time() + (timeout_minutes * 60),
        max_rounds=max_rounds,
    )
    path = _state_path(convergence_id)
    save_state(state, path)

    src = [f.strip() for f in source_files.split(",") if f.strip()] if source_files else None
    docs = [f.strip() for f in doc_files.split(",") if f.strip()] if doc_files else None
    test_cmd = test_command.split() if test_command else None

    task = get_next_task(state, path, src, docs, test_cmd)

    return json.dumps({
        "convergence_id": convergence_id,
        "status": "started",
        "task": task.to_dict(),
    }, indent=2)


@mcp.tool()
def convergence_next_task(
    convergence_id: str,
    source_files: str = "",
    doc_files: str = "",
    test_command: str = "",
) -> str:
    """Get the next task from the convergence engine.

    Call this after submitting results from the previous task.
    The engine checks convergence state (rounds, clean passes, timeouts)
    and returns one of:
    - "audit": Read and audit files on specific dimensions
    - "doc_align": Verify plan conforms to product docs
    - "execute": Build a checklist item (green-field: write code + tests)
    - "fix": Fix a specific finding
    - "test": Run the test suite
    - "write": Create or update a file
    - "done": Convergence complete — two clean passes achieved!
    - "escalated": Engine stopped — timeout, max rounds, or repeated failures

    When you get "done" or "escalated", the loop is over.

    Args:
        convergence_id: The ID returned by start_convergence()
        source_files: Override source files (comma-separated)
        doc_files: Override doc files (comma-separated)
        test_command: Override test command
    """
    path = _state_path(convergence_id)
    state = load_state(path)

    src = [f.strip() for f in source_files.split(",") if f.strip()] if source_files else None
    docs = [f.strip() for f in doc_files.split(",") if f.strip()] if doc_files else None
    test_cmd = test_command.split() if test_command else None

    task = get_next_task(state, path, src, docs, test_cmd)

    return json.dumps({
        "convergence_id": convergence_id,
        "phase": state.phase.value,
        "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "task": task.to_dict(),
    }, indent=2)


@mcp.tool()
def convergence_submit_result(
    convergence_id: str,
    findings_json: str = "[]",
) -> str:
    """Report your audit/fix/test results back to the convergence engine.

    The engine processes your findings, updates counters, and checks:
    - Did this round have zero findings? (increments clean pass counter)
    - Have we hit two consecutive clean passes? (convergence!)
    - Are findings increasing round over round? (net-negative → escalate)
    - Has the same finding failed 3 times? (rollback → escalate)

    For a CLEAN PASS (no issues found): pass "[]"
    For FINDINGS: pass a JSON array like:
    [{"id": "f1", "file": "src/main.py", "dimension": "correctness",
      "severity": "high", "description": "Off-by-one in loop",
      "suggested_fix": "Change < to <=", "fixed": true}]

    Set "fixed": true if you already fixed it, false if not.

    Args:
        convergence_id: The ID from start_convergence()
        findings_json: JSON array of findings, or "[]" for clean pass
    """
    path = _state_path(convergence_id)
    state = load_state(path)

    try:
        findings = json.loads(findings_json)
    except json.JSONDecodeError:
        findings = []

    submit_result(state, path, {"findings": findings})

    return json.dumps({
        "convergence_id": convergence_id,
        "phase": state.phase.value,
        "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "status": "result_accepted",
    }, indent=2)


@mcp.tool()
def convergence_status(convergence_id: str) -> str:
    """Check the current status of a convergence run.

    Returns: phase, round number, consecutive clean passes,
    total findings, total fixed, elapsed time, and whether
    the convergence is terminal (done or escalated).
    """
    path = _state_path(convergence_id)
    state = load_state(path)

    return json.dumps({
        "convergence_id": convergence_id,
        "phase": state.phase.value,
        "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "total_findings": sum(len(r.findings) for r in state.history),
        "total_fixed": sum(r.findings_fixed for r in state.history),
        "elapsed_seconds": round(time.time() - state.created_at, 1),
        "escalation_reason": state.escalation_reason,
        "terminal": is_terminal(state.phase),
    }, indent=2)


@mcp.tool()
def convergence_cancel(convergence_id: str) -> str:
    """Cancel a convergence run. State is preserved — you can review what happened."""
    path = _state_path(convergence_id)
    state = load_state(path)
    state.phase = ConvergencePhase.ESCALATED
    state.escalation_reason = "cancelled_by_user"
    save_state(state, path)
    return json.dumps({
        "status": "cancelled",
        "convergence_id": convergence_id,
    })


# --- Status tool ---


@mcp.tool()
def cruxdev_status(project_dir: str = ".") -> str:
    """Check CruxDev installation health — is everything wired and working?

    Runs health checks: MCP server, tools, state directory, config, Python version,
    dependencies, methodology docs, slash commands, active convergences.

    Returns a structured report with pass/fail for each check.
    """
    from .status import get_status
    report = get_status(project_dir)
    result = {
        "healthy": report.healthy,
        "checks": [{"name": c.name, "passed": c.passed, "message": c.message} for c in report.checks],
        "warnings": [{"name": w.name, "message": w.message} for w in report.warnings],
        "active_convergences": report.active_convergences,
        "versions": report.versions,
    }
    return json.dumps(result, indent=2)


# --- Session bus tools ---


def _get_broker():  # pragma: no cover — monkeypatched in tests
    from .bus.broker import Broker
    return Broker()


@mcp.tool()
def session_register(project_name: str = "") -> str:
    """Register this session with the CruxDev session bus.

    Call this at the start of a session so other sessions can discover you
    and send you messages. Returns your session ID.

    Args:
        project_name: Name of the project (defaults to current directory name)
    """
    if not project_name:
        project_name = os.path.basename(os.getcwd())
    broker = _get_broker()
    session_id = broker.register_session(project_name, os.getcwd())
    return json.dumps({
        "session_id": session_id,
        "project": project_name,
        "status": "registered",
    })


@mcp.tool()
def session_list() -> str:
    """List all active CruxDev sessions across all projects.

    Shows what sessions are running, which projects they're in,
    and when they last checked in.
    """
    broker = _get_broker()
    sessions = broker.list_sessions()
    return json.dumps([
        {
            "id": s.id,
            "project": s.project,
            "directory": s.directory,
            "last_heartbeat_ago": round(time.time() - s.last_heartbeat, 1),
        }
        for s in sessions
    ], indent=2)


@mcp.tool()
def report_issue(
    target_project: str,
    title: str,
    body: str,
    severity: str = "medium",
) -> str:
    """Report an issue you've discovered to another project's session.

    Use this when you find a bug, limitation, or problem in another
    project's code or tools while working on your project.

    Args:
        target_project: Which project has the issue (e.g., "cruxdev", "crux")
        title: Brief description of the issue
        body: Detailed description including what you were doing when you found it
        severity: "high", "medium", or "low"
    """
    broker = _get_broker()
    # Determine source project from cwd
    source = os.path.basename(os.getcwd())
    msg_id = broker.report_issue(source, target_project, title, body, severity)
    return json.dumps({
        "message_id": msg_id,
        "status": "sent",
        "from": source,
        "to": target_project,
    })


@mcp.tool()
def report_improvement(
    target_project: str,
    title: str,
    body: str,
) -> str:
    """Suggest an improvement to another project.

    Use this when you think of a feature, optimization, or enhancement
    that would help another project in the ecosystem.

    Args:
        target_project: Which project to improve (e.g., "cruxdev", "crux")
        title: Brief description of the improvement
        body: Detailed description of what to change and why
    """
    broker = _get_broker()
    source = os.path.basename(os.getcwd())
    msg_id = broker.report_improvement(source, target_project, title, body)
    return json.dumps({
        "message_id": msg_id,
        "status": "sent",
        "from": source,
        "to": target_project,
    })


@mcp.tool()
def share_pattern(
    pattern_name: str,
    description: str,
) -> str:
    """Share a pattern you've learned with all other sessions.

    Use this when you discover a useful pattern, convention, or technique
    that other projects in the ecosystem should know about.

    Args:
        pattern_name: Short name for the pattern (e.g., "atomic-config-writes")
        description: What the pattern is and why it matters
    """
    broker = _get_broker()
    source = os.path.basename(os.getcwd())
    msg_id = broker.share_pattern(source, pattern_name, description)
    return json.dumps({
        "message_id": msg_id,
        "status": "broadcast",
        "from": source,
        "pattern": pattern_name,
    })


@mcp.tool()
def notify_breaking_change(
    affected_projects: str,
    description: str,
) -> str:
    """Notify other projects of a breaking change you've made.

    Use this when you rename, remove, or change the behavior of something
    that other projects depend on.

    Args:
        affected_projects: Comma-separated project names (e.g., "crux,cruxcli")
        description: What changed and what other projects need to do
    """
    broker = _get_broker()
    source = os.path.basename(os.getcwd())
    projects = [p.strip() for p in affected_projects.split(",") if p.strip()]
    msg_ids = broker.notify_breaking_change(source, projects, description)
    return json.dumps({
        "message_ids": msg_ids,
        "status": "sent",
        "from": source,
        "to": projects,
    })


@mcp.tool()
def check_inbox(project_name: str = "") -> str:
    """Check for messages from other sessions.

    Returns issues, improvements, patterns, and breaking changes
    reported by other projects. Call this periodically and after convergence.

    Args:
        project_name: Your project name (auto-detected from cwd if empty)
    """
    broker = _get_broker()
    project = project_name or os.path.basename(os.getcwd())
    messages = broker.check_inbox(project)
    return json.dumps([
        {
            "id": m.id,
            "type": m.type,
            "from": m.source_project,
            "title": m.title,
            "body": m.body,
            "severity": m.severity,
            "age_seconds": round(time.time() - m.created_at, 1),
        }
        for m in messages
    ], indent=2)


@mcp.tool()
def acknowledge_message(message_id: str) -> str:
    """Mark a message as handled.

    Call this after you've acted on an issue, improvement, pattern,
    or breaking change notification.

    Args:
        message_id: The ID of the message to acknowledge
    """
    broker = _get_broker()
    found = broker.acknowledge(message_id)
    return json.dumps({
        "message_id": message_id,
        "acknowledged": found,
    })


# --- Adoption tools ---


@mcp.tool()
def classify_project(project_dir: str = ".") -> str:
    """Classify a project — determine type(s), maturity, and required templates.

    Scans the project directory for known patterns (code, configs, docs)
    and returns the best-fit classification with confidence score.

    Args:
        project_dir: Path to the project to classify (default: current directory)
    """
    from .adoption.classify import classify_project as _classify
    result = _classify(project_dir)
    return json.dumps({
        "primary_type": result.primary_type,
        "secondary_types": result.secondary_types,
        "maturity": result.maturity,
        "confidence": result.confidence,
        "signals": result.signals,
    }, indent=2)


@mcp.tool()
def inventory_project(project_dir: str = ".") -> str:
    """Inventory all project materials — documents, code, assets.

    Scans the project and produces a structured inventory with
    format detection and size tracking.

    Args:
        project_dir: Path to the project to inventory
    """
    from .adoption.inventory import inventory_project as _inventory
    result = _inventory(project_dir)
    return json.dumps({
        "total_items": len(result.items),
        "total_size": result.total_size,
        "by_format": {fmt: len(items) for fmt, items in result.by_format.items()},
        "markdown": result.to_markdown(),
    }, indent=2)


@mcp.tool()
def get_templates(
    project_type: str,
    maturity: str = "minimal",
) -> str:
    """Get required document templates for a project type and maturity.

    Returns templates organized by category with requirement levels.

    Args:
        project_type: From classify_project (e.g., "software-existing", "website")
        maturity: From classify_project (e.g., "minimal", "growing", "production")
    """
    from .adoption.templates import get_templates_for_type
    ts = get_templates_for_type(project_type, maturity)
    return json.dumps({
        "total": len(ts.templates),
        "required": len(ts.required),
        "templates": [
            {"category": t.category, "name": t.name, "filename": t.filename, "requirement": t.requirement}
            for t in ts.templates
        ],
        "by_category": {cat: len(items) for cat, items in ts.by_category.items()},
    }, indent=2)


@mcp.tool()
def analyze_gaps(project_dir: str = ".") -> str:
    """Analyze gaps between project state and template requirements.

    Runs classification, inventory, and template matching to find
    missing or stub documents. Returns GAPS.md content.

    Args:
        project_dir: Path to the project to analyze
    """
    from .adoption.classify import classify_project as _classify
    from .adoption.gaps import analyze_gaps as _analyze
    from .adoption.inventory import inventory_project as _inventory
    from .adoption.templates import get_templates_for_type

    classification = _classify(project_dir)
    inventory = _inventory(project_dir)
    templates = get_templates_for_type(classification.primary_type, classification.maturity)
    result = _analyze(project_dir, inventory, templates)

    return json.dumps({
        "project_type": classification.primary_type,
        "maturity": classification.maturity,
        "total_gaps": len(result.open_gaps),
        "critical": len(result.critical),
        "gaps": [
            {"name": g.template_name, "file": g.template_file,
             "severity": g.severity, "reason": g.reason}
            for g in result.open_gaps
        ],
        "markdown": result.to_markdown(),
    }, indent=2)


@mcp.tool()
def gap_status(project_dir: str = ".") -> str:
    """Show current gap counts by priority for a project.

    Quick summary of how many gaps exist at each severity level.

    Args:
        project_dir: Path to the project
    """
    from .adoption.classify import classify_project as _classify
    from .adoption.gaps import analyze_gaps as _analyze
    from .adoption.inventory import inventory_project as _inventory
    from .adoption.templates import get_templates_for_type

    classification = _classify(project_dir)
    inventory = _inventory(project_dir)
    templates = get_templates_for_type(classification.primary_type, classification.maturity)
    result = _analyze(project_dir, inventory, templates)

    by_sev = result.by_severity
    return json.dumps({
        "critical": len(by_sev.get("critical", [])),
        "high": len(by_sev.get("high", [])),
        "medium": len(by_sev.get("medium", [])),
        "low": len(by_sev.get("low", [])),
        "total_open": len(result.open_gaps),
    }, indent=2)


# --- Research tools ---


@mcp.tool()
def research_topic(
    topic: str,
    sub_questions: str = "",
) -> str:
    """Start a research session on a topic.

    Creates a research session with the 5-pass methodology:
    1. Broad search, 2. Academic, 3. Practitioner, 4. Contrarian, 5. Primary.

    Returns the session ID and initial search queries.

    Args:
        topic: What to research
        sub_questions: Comma-separated sub-questions to investigate
    """
    from .research.session import create_session
    from .research.counter import generate_negation_queries

    questions = [q.strip() for q in sub_questions.split(",") if q.strip()] if sub_questions else []
    session = create_session(topic, questions)

    return json.dumps({
        "session_id": session.session_id,
        "topic": session.topic,
        "sub_questions": session.sub_questions,
        "current_pass": session.current_pass,
        "instructions": "Execute 5 passes: broad, academic, practitioner, contrarian, primary. Submit findings after each search.",
    }, indent=2)


@mcp.tool()
def research_status(session_id: str) -> str:
    """Check convergence status of a research session.

    Args:
        session_id: The session ID from research_topic()
    """
    from .research.session import ResearchSession
    from .research.convergence import check_research_convergence

    # Build a minimal session to check convergence
    # In practice, the session state would be loaded from checkpoint
    return json.dumps({
        "session_id": session_id,
        "instructions": "Load session from checkpoint or track state in conversation. Use counter_research() for adversarial verification.",
    }, indent=2)


@mcp.tool()
def verify_research_sources(
    finding_id: str,
    source_urls: str,
) -> str:
    """Run source verification pipeline on research findings.

    Checks URL reachability for all sources.

    Args:
        finding_id: ID of the finding being verified
        source_urls: Comma-separated URLs to verify
    """
    from .research.verify import verify_sources
    from .mcp_normalize import to_string_list

    urls = to_string_list(source_urls)
    result = verify_sources(finding_id, urls)
    return json.dumps({
        "finding_id": result.finding_id,
        "overall_verified": result.overall_verified,
        "reachable_count": result.reachable_count,
        "total_sources": len(result.sources_checked),
        "sources": [
            {"url": s.url, "reachable": s.reachable, "error": s.error}
            for s in result.sources_checked
        ],
    }, indent=2)


@mcp.tool()
def counter_research(
    claim: str,
    counter_evidence: str = "",
    alternative_explanations: str = "",
    supporting_count: int = 1,
) -> str:
    """Run adversarial verification on a claim.

    Generates negation queries and assesses robustness based on
    counter-evidence vs supporting evidence.

    Args:
        claim: The claim to verify adversarially
        counter_evidence: Pipe-separated counter-evidence found
        alternative_explanations: Pipe-separated alternative explanations
        supporting_count: Number of supporting sources found
    """
    from .research.counter import run_counter_research
    from .mcp_normalize import to_int, to_pipe_list

    counter = to_pipe_list(counter_evidence)
    alts = to_pipe_list(alternative_explanations)

    result = run_counter_research(claim, counter, alts, to_int(supporting_count, 1))
    return json.dumps({
        "claim": result.original_claim,
        "robustness": result.robustness,
        "is_contested": result.is_contested,
        "negation_queries": result.negation_queries,
        "counter_evidence": result.counter_evidence,
        "alternative_explanations": result.alternative_explanations,
    }, indent=2)


# --- Competitors tools ---


@mcp.tool()
def discover_competitors(
    project_description: str,
    category: str,
) -> str:
    """Generate search queries and parse discovery results for finding competitors.

    Returns structured discovery queries based on the project description and category.
    Use these queries with web search, then feed results back through this tool.

    Args:
        project_description: What your project does (e.g., "AI-driven convergence engine")
        category: Market category (e.g., "AI coding tools", "DevOps automation")
    """
    from .competitors.discovery import generate_discovery_queries
    queries = generate_discovery_queries(project_description, category)
    return json.dumps({
        "queries": queries,
        "instructions": "Run each query via web search, then call research_competitor() for each result.",
    }, indent=2)


@mcp.tool()
def research_competitor(
    name: str,
    url: str,
    research_text: str,
) -> str:
    """Parse research text into a structured competitor profile.

    Feed in your research findings (from web search, docs, etc.) and get back
    a structured profile with features, strengths, weaknesses, pricing, etc.

    Args:
        name: Competitor name
        url: Competitor website URL
        research_text: Your research findings as text (tagline, features, pricing, etc.)
    """
    from .competitors.research import parse_profile_response
    profile = parse_profile_response(name, url, research_text)
    return json.dumps({
        "name": profile.name,
        "url": profile.url,
        "tagline": profile.tagline,
        "category": profile.category,
        "pricing": profile.pricing,
        "tech_stack": profile.tech_stack,
        "features": [{"name": f.name, "has": f.has_feature} for f in profile.features],
        "strengths": profile.strengths,
        "weaknesses": profile.weaknesses,
        "differentiation": profile.differentiation,
        "markdown": profile.to_markdown(),
    }, indent=2)


@mcp.tool()
def verify_competitor_links(
    competitor_name: str,
    profile_markdown: str,
) -> str:
    """Test all URLs in a competitor profile, returns pass/fail per link.

    Checks every URL found in the profile markdown for reachability.

    Args:
        competitor_name: Name of the competitor
        profile_markdown: Markdown text containing URLs to verify
    """
    from .competitors.verification import verify_profile_links
    result = verify_profile_links(competitor_name, profile_markdown)
    return json.dumps({
        **result.to_dict(),
        "links": [
            {"url": r.url, "status": r.status, "code": r.status_code, "error": r.error}
            for r in result.links_checked
        ],
    }, indent=2)


@mcp.tool()
def generate_gap_analysis(
    our_name: str,
    our_features: str,
    competitors_json: str,
) -> str:
    """Run gap analysis comparing our features against competitors.

    Builds a feature matrix and classifies gaps by priority:
    - must-close: 2+ official competitors have it
    - should-close: 1 official competitor has it
    - nice-to-have: only non-official competitors have it

    Args:
        our_name: Our product name
        our_features: Comma-separated list of our features
        competitors_json: JSON array of competitor profiles (from research_competitor)
    """
    from .competitors.gap_analysis import run_gap_analysis
    from .competitors.research import CompetitorProfile, Feature
    from .mcp_normalize import normalize_competitors, to_string_list

    features = to_string_list(our_features)
    normalized = normalize_competitors(competitors_json)

    profiles = []
    for c in normalized:
        profiles.append(CompetitorProfile(
            name=c["name"],
            url=c["url"],
            category=c["category"],
            features=[Feature(f["name"], "", f["has"]) for f in c["features"]],
        ))

    result = run_gap_analysis(our_name, features, profiles)
    return json.dumps({
        "our_name": result.our_name,
        "total_features": len(result.feature_matrix),
        "total_gaps": len(result.gaps),
        "must_close": len(result.must_close),
        "should_close": len(result.should_close),
        "gaps": [
            {"feature": g.feature_name, "priority": g.priority,
             "competitors": g.competitors_with_feature, "status": g.status}
            for g in result.gaps
        ],
        "markdown": result.to_markdown(),
    }, indent=2)


@mcp.tool()
def generate_comparison_page(
    our_name: str,
    our_features: str,
    competitor_name: str,
    competitor_url: str,
    competitor_research: str,
) -> str:
    """Generate a /vs/<competitor> comparison page for the project website.

    Produces markdown with frontmatter, feature comparison table,
    strengths/weaknesses, and pricing comparison.

    Args:
        our_name: Our product name
        our_features: Comma-separated list of our features
        competitor_name: Competitor name
        competitor_url: Competitor website URL
        competitor_research: Research text about the competitor
    """
    from .competitors.comparison_page import generate_comparison_content
    from .competitors.research import parse_profile_response
    from .mcp_normalize import to_string_list

    features = to_string_list(our_features)
    profile = parse_profile_response(competitor_name, competitor_url, competitor_research)
    page = generate_comparison_content(our_name, features, profile)
    return json.dumps({
        "slug": page.slug,
        "title": page.title,
        "features_compared": page.features_compared,
        "markdown": page.to_markdown(),
    }, indent=2)


@mcp.tool()
def generate_gap_build_plan(
    plan_number: int,
    feature_name: str,
    competitors_with_feature: str,
    priority: str,
    our_name: str,
    context: str = "",
) -> str:
    """Create a build plan to close a specific competitive gap.

    Generates a complete build plan following CruxDev template with
    document alignment, checklists, convergence criteria, etc.

    Args:
        plan_number: Build plan number (e.g., 12)
        feature_name: Name of the feature gap to close
        competitors_with_feature: Comma-separated competitor names that have this feature
        priority: Gap priority (must-close, should-close, nice-to-have)
        our_name: Our product name
        context: Optional notes about how competitors implement this feature
    """
    from .competitors.build_plan_generator import generate_gap_plan
    from .competitors.gap_analysis import FeatureGap
    from .mcp_normalize import to_int, to_string_list

    plan_number = to_int(plan_number, 1)
    comps = to_string_list(competitors_with_feature)
    gap = FeatureGap(
        feature_name=feature_name,
        competitors_with_feature=comps,
        priority=priority,
    )
    plan = generate_gap_plan(plan_number, gap, our_name, context)
    return json.dumps({
        "filename": plan.filename,
        "content": plan.content,
    }, indent=2)


# --- Legacy direct-execution support ---


def init(state_dir: str) -> None:
    """Initialize with custom state directory (for testing)."""
    global STATE_DIR
    STATE_DIR = state_dir
    os.makedirs(state_dir, exist_ok=True)


# Keep old function signatures for backward compat with tests
_active_runs: dict = {}


def get_provider(provider_name: str = "stub", **kwargs) -> LLMDispatcher:
    """Get an LLM provider by name."""
    if provider_name == "stub":
        return StubProvider(mode=StubMode.CLEAN)
    if provider_name == "anthropic":
        from .dispatch.providers.anthropic import AnthropicProvider
        return AnthropicProvider(**kwargs)
    if provider_name == "ollama":
        from .dispatch.providers.ollama import OllamaProvider
        return OllamaProvider(**kwargs)
    raise ValueError(f"Unknown provider: {provider_name}")


def state_path(convergence_id: str) -> str:
    return _state_path(convergence_id)


def converge(plan_file: str, timeout_minutes: int = 120, provider: str = "stub",
             project_dir: str = ".", test_command: list[str] | None = None,
             source_files: list[str] | None = None, doc_files: list[str] | None = None) -> dict:
    """Direct convergence (background thread). For standalone/testing use."""
    import threading
    convergence_id = str(uuid.uuid4())[:8]
    state = ConvergenceState(plan_file=plan_file, deadline=time.time() + (timeout_minutes * 60))
    path = _state_path(convergence_id)
    save_state(state, path)
    llm = get_provider(provider)

    def run():
        runner = ConvergenceRunner(state, llm, path, project_dir=project_dir,
                                   test_command=test_command, source_files=source_files, doc_files=doc_files)
        runner.run()

    thread = threading.Thread(target=run, daemon=True)
    thread.start()
    _active_runs[convergence_id] = thread
    return {"convergence_id": convergence_id, "status": "started"}


def check_convergence_status(convergence_id: str) -> dict:
    path = _state_path(convergence_id)
    state = load_state(path)
    thread = _active_runs.get(convergence_id)
    running = thread.is_alive() if thread else False
    return {
        "convergence_id": convergence_id, "running": running,
        "phase": state.phase.value, "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "total_findings": sum(len(r.findings) for r in state.history),
        "elapsed_seconds": time.time() - state.created_at,
        "escalation_reason": state.escalation_reason,
    }


def cancel_convergence(convergence_id: str) -> dict:
    path = _state_path(convergence_id)
    state = load_state(path)
    state.phase = ConvergencePhase.ESCALATED
    state.escalation_reason = "cancelled_by_user"
    save_state(state, path)
    return {"status": "cancelled", "convergence_id": convergence_id}


def list_convergences() -> list[dict]:
    if not STATE_DIR or not os.path.exists(STATE_DIR):
        return []
    results = []
    for f in os.listdir(STATE_DIR):
        if f.endswith(".json"):
            cid = f[:-5]
            try:
                results.append(check_convergence_status(cid))
            except Exception:
                pass
    return results


# --- Entry point ---

if __name__ == "__main__":
    mcp.run()
