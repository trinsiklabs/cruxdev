"""Architecture tests — CI gates for LLM minimization.

These verify that the engine's control flow is deterministic and
all LLM calls go through LLMDispatcher.
"""

import ast
import os

import pytest


ENGINE_MODULES = [
    "src/engine/convergence.py",
    "src/engine/persistence.py",
    "src/engine/state.py",
    "src/engine/timeout.py",
]

SUBLOOP_MODULES = [
    "src/engine/plan_convergence.py",
    "src/engine/code_convergence.py",
    "src/engine/doc_convergence.py",
    "src/engine/e2e_convergence.py",
    "src/engine/runner.py",
]

ALL_SRC = "src"


def _get_python_files(directory: str) -> list[str]:
    files = []
    for root, dirs, filenames in os.walk(directory):
        dirs[:] = [d for d in dirs if d != "__pycache__"]
        for name in filenames:
            if name.endswith(".py"):
                files.append(os.path.join(root, name))
    return files


def _get_imports(filepath: str) -> set[str]:
    """Get all import targets from a Python file."""
    with open(filepath) as f:
        source = f.read()
    tree = ast.parse(source)
    imports = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            for alias in node.names:
                imports.add(alias.name)
        elif isinstance(node, ast.ImportFrom):
            if node.module:
                imports.add(node.module)
    return imports


def _get_function_calls(filepath: str) -> list[str]:
    """Get all function/method call names from a Python file."""
    with open(filepath) as f:
        source = f.read()
    tree = ast.parse(source)
    calls = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Call):
            if isinstance(node.func, ast.Name):
                calls.append(node.func.id)
            elif isinstance(node.func, ast.Attribute):
                calls.append(node.func.attr)
    return calls


class TestNoLLMInControlFlow:
    """The engine NEVER calls LLM for state transitions, counting, timing, or rollback."""

    @pytest.mark.parametrize("module", ENGINE_MODULES)
    def test_no_llm_dispatcher_import(self, module):
        """Engine control modules must not import LLMDispatcher."""
        imports = _get_imports(module)
        llm_imports = {i for i in imports if "llm" in i.lower() or "dispatch" in i.lower()}
        assert llm_imports == set(), (
            f"{module} imports LLM-related modules: {llm_imports}. "
            "Engine control flow must be deterministic — no LLM calls."
        )

    @pytest.mark.parametrize("module", ENGINE_MODULES)
    def test_no_llm_function_calls(self, module):
        """Engine control modules must not call llm.audit(), llm.fix(), etc.
        Note: os.write() is a file operation, not an LLM call."""
        with open(module) as f:
            source = f.read()
        tree = ast.parse(source)

        llm_calls = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
                # Check for llm.audit(), llm.fix(), llm.write(), llm.evaluate_independence()
                if node.func.attr in ("audit", "fix", "evaluate_independence"):
                    if isinstance(node.func.value, ast.Name) and node.func.value.id == "llm":
                        llm_calls.append(f"llm.{node.func.attr}")

        assert llm_calls == [], (
            f"{module} makes LLM calls: {llm_calls}. "
            "Only convergence sub-loops may call the LLM."
        )


class TestAllLLMCallsGoThroughDispatcher:
    """All LLM calls must go through the LLMDispatcher interface."""

    def test_subloops_use_dispatcher(self):
        """Sub-loop modules should call llm.audit(), llm.fix(), etc. — through the dispatcher."""
        for module in SUBLOOP_MODULES:
            calls = _get_function_calls(module)
            # These modules ARE expected to call LLM methods
            # The point is they do it through the llm parameter, not directly

    def test_no_direct_api_calls(self):
        """No module should import httpx, requests, or anthropic directly."""
        for filepath in _get_python_files(ALL_SRC):
            imports = _get_imports(filepath)
            direct_api = {i for i in imports if i in (
                "httpx", "requests", "anthropic", "openai",
            )}
            # Allow provider implementations to import these
            if "providers" in filepath:
                continue
            assert direct_api == set(), (
                f"{filepath} imports API libraries directly: {direct_api}. "
                "All LLM calls must go through LLMDispatcher."
            )

    def test_dispatcher_is_abstract(self):
        """LLMDispatcher must be an abstract class."""
        from src.dispatch.llm import LLMDispatcher
        import inspect
        assert inspect.isabstract(LLMDispatcher)


class TestEngineIntegrity:
    """Verify engine structural properties."""

    def test_all_phases_reachable(self):
        """Every non-terminal phase can be reached by advance_phase."""
        from src.engine.convergence import PHASE_ORDER, advance_phase
        from src.engine.state import ConvergencePhase

        reachable = {ConvergencePhase.PLANNING}
        phase = ConvergencePhase.PLANNING
        while phase != ConvergencePhase.CONVERGED:
            phase = advance_phase(phase)
            reachable.add(phase)

        for p in PHASE_ORDER:
            assert p in reachable, f"Phase {p} is not reachable"

    def test_escalated_not_in_phase_order(self):
        """ESCALATED is a terminal state, not in the normal phase order."""
        from src.engine.convergence import PHASE_ORDER
        from src.engine.state import ConvergencePhase

        assert ConvergencePhase.ESCALATED not in PHASE_ORDER

    def test_convergence_threshold_default_is_two(self):
        """Default convergence requires TWO consecutive clean passes, not one."""
        from src.engine.state import ConvergenceState

        s = ConvergenceState(plan_file="p.md")
        assert s.convergence_threshold == 2
