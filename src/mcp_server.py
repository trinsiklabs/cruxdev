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

QUICK START: When the user says "converge [plan]":
1. Call start_convergence(plan_file) — engine creates state, returns first task
2. Loop: call convergence_next_task(id) to get what to do next
3. Execute the task (read files, audit code, fix issues, run tests)
4. Call convergence_submit_result(id, findings_json) to report back
5. Repeat 2-4 until task_type is "done" or "escalated"
DO NOT decide when to stop — the engine decides.

For planning: call get_methodology() first, then create_plan_template(goal).
For adoption: call get_adoption_process() for step-by-step instructions.""",
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
    path = os.path.join(CRUXDEV_ROOT, "DEVELOPMENT_PATTERNS_CRUXDEV.md")
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
    path = os.path.join(CRUXDEV_ROOT, "ADOPTION_PROCESS.md")
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
    2. Code auditing (8 dimensions: correctness, security, tests, etc.)
    3. Doc auditing (5 dimensions: accuracy, completeness, etc.)
    4. E2E testing (run the test suite)
    5. Convergence (two consecutive clean passes)

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
