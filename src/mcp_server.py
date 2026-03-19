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

mcp = FastMCP("cruxdev")

os.makedirs(STATE_DIR, exist_ok=True)


def _state_path(convergence_id: str) -> str:
    return os.path.join(STATE_DIR, f"{convergence_id}.json")


# --- Bootstrap tools ---


@mcp.tool()
def get_methodology() -> str:
    """Get the CruxDev development methodology.

    Returns the full development patterns document that guides
    how to plan, execute, and converge work autonomously.
    """
    path = os.path.join(CRUXDEV_ROOT, "DEVELOPMENT_PATTERNS_CRUXDEV.md")
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return "DEVELOPMENT_PATTERNS_CRUXDEV.md not found at " + path


@mcp.tool()
def get_adoption_process() -> str:
    """Get the CruxDev adoption process for new projects.

    Returns step-by-step instructions for adopting any project
    into the Crux/CruxDev ecosystem.
    """
    path = os.path.join(CRUXDEV_ROOT, "ADOPTION_PROCESS.md")
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return "ADOPTION_PROCESS.md not found at " + path


# --- Planning tools ---


@mcp.tool()
def create_plan_template(goal: str) -> str:
    """Generate a structured build plan template for a given goal.

    Returns a markdown plan with phases, checklists, test commands,
    and convergence criteria — ready for you to fill in.
    """
    return get_plan_template(goal)


@mcp.tool()
def validate_plan_structure(plan_file: str) -> str:
    """Validate a build plan's structure for convergence.

    Checks that the plan has: title, phases, checklists, test commands,
    convergence criteria. Returns errors and warnings.
    Does NOT evaluate plan quality — that's your job.
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
    """Start a convergence run. Returns convergence_id and first task.

    The engine creates state and tells you what to do first.
    Call get_next_task() after completing each task.
    Call submit_result() to report findings.

    Args:
        plan_file: Path to the build plan markdown file
        timeout_minutes: Max time for entire convergence (default 120)
        max_rounds: Max audit rounds per phase (default 5)
        source_files: Comma-separated list of source files to audit
        doc_files: Comma-separated list of doc files to audit
        test_command: Test command (e.g. "python3 -m pytest tests/")
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
    """Get the next task the engine wants you to do.

    Call this after completing a task and submitting results.
    The engine decides what's next based on convergence state.
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
    """Submit audit/fix/test results back to the engine.

    Pass findings as a JSON array:
    [{"id": "f1", "file": "a.py", "dimension": "correctness",
      "severity": "high", "description": "...", "suggested_fix": "...",
      "fixed": true}]

    For clean passes (no findings), pass "[]".
    The engine updates state, checks convergence, and tells you what's next.
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
    """Check the current status of a convergence run."""
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
    """Cancel a convergence run, preserving state."""
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
