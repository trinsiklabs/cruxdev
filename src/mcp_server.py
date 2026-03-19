"""CruxDev MCP server — exposes convergence engine as MCP tools.

converge() is non-blocking: spawns a background thread and returns immediately.
Poll with check_convergence_status(). Cancel with cancel_convergence().
"""

import os
import threading
import time
import uuid
from pathlib import Path

from .dispatch.llm import LLMDispatcher
from .dispatch.providers.stub import StubMode, StubProvider
from .engine.convergence import is_terminal
from .engine.persistence import load_state, save_state
from .engine.runner import ConvergenceRunner
from .engine.state import ConvergencePhase, ConvergenceState

# Active convergence runs (convergence_id → thread)
_active_runs: dict[str, threading.Thread] = {}
_state_dir: str = ""


def init(state_dir: str) -> None:
    """Initialize the MCP server with a state directory."""
    global _state_dir
    _state_dir = state_dir
    os.makedirs(state_dir, exist_ok=True)


def state_path(convergence_id: str) -> str:
    """Get the state file path for a convergence ID."""
    return os.path.join(_state_dir, f"{convergence_id}.json")


def get_provider(provider_name: str = "stub", **kwargs) -> LLMDispatcher:
    """Get an LLM provider by name. Default is stub for testing."""
    if provider_name == "stub":
        return StubProvider(mode=StubMode.CLEAN)
    if provider_name == "anthropic":
        from .dispatch.providers.anthropic import AnthropicProvider
        return AnthropicProvider(**kwargs)
    if provider_name == "ollama":
        from .dispatch.providers.ollama import OllamaProvider
        return OllamaProvider(**kwargs)
    raise ValueError(f"Unknown provider: {provider_name}")


def converge(
    plan_file: str,
    timeout_minutes: int = 120,
    provider: str = "stub",
    project_dir: str = ".",
    test_command: list[str] | None = None,
    source_files: list[str] | None = None,
    doc_files: list[str] | None = None,
) -> dict:
    """Start convergence. Returns immediately with convergence_id.

    Non-blocking — runs in background thread.
    """
    convergence_id = str(uuid.uuid4())[:8]
    state = ConvergenceState(
        plan_file=plan_file,
        deadline=time.time() + (timeout_minutes * 60),
    )
    path = state_path(convergence_id)
    save_state(state, path)

    llm = get_provider(provider)

    def run_in_background():
        runner = ConvergenceRunner(
            state,
            llm,
            path,
            project_dir=project_dir,
            test_command=test_command,
            source_files=source_files,
            doc_files=doc_files,
        )
        runner.run()

    thread = threading.Thread(target=run_in_background, daemon=True)
    thread.start()
    _active_runs[convergence_id] = thread

    return {"convergence_id": convergence_id, "status": "started"}


def check_convergence_status(convergence_id: str) -> dict:
    """Check progress of a running convergence."""
    path = state_path(convergence_id)
    state = load_state(path)
    thread = _active_runs.get(convergence_id)
    running = thread.is_alive() if thread else False

    return {
        "convergence_id": convergence_id,
        "running": running,
        "phase": state.phase.value,
        "round": state.round,
        "consecutive_clean": state.consecutive_clean,
        "total_findings": sum(len(r.findings) for r in state.history),
        "elapsed_seconds": time.time() - state.created_at,
        "escalation_reason": state.escalation_reason,
    }


def cancel_convergence(convergence_id: str) -> dict:
    """Cancel a running convergence, preserving state for resume."""
    path = state_path(convergence_id)
    state = load_state(path)
    state.phase = ConvergencePhase.ESCALATED
    state.escalation_reason = "cancelled_by_user"
    save_state(state, path)
    return {"status": "cancelled", "convergence_id": convergence_id}


def list_convergences() -> list[dict]:
    """List all convergence runs."""
    if not _state_dir or not os.path.exists(_state_dir):
        return []
    results = []
    for f in os.listdir(_state_dir):
        if f.endswith(".json"):
            cid = f[:-5]
            try:
                status = check_convergence_status(cid)
                results.append(status)
            except Exception:
                pass
    return results
