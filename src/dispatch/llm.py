"""LLM Dispatcher — the ONLY way the engine calls an LLM.

Grep for LLMDispatcher to verify LLM scope.
All implementations must validate output against schemas.
"""

from abc import ABC, abstractmethod

from .schema import AuditResult, EvaluationResult, FixResult, WriteResult


class LLMDispatcher(ABC):
    """Abstract interface for LLM calls.

    The engine calls these methods only from convergence sub-loops
    (plan_convergence, code_convergence, doc_convergence).
    Never from state.py, convergence.py, timeout.py, or persistence.py.
    """

    @abstractmethod
    def audit(
        self,
        files: list[str],
        dimensions: list[str],
        skill_context: str,
    ) -> AuditResult:
        """Ask LLM to audit files. Returns schema-validated findings."""

    @abstractmethod
    def fix(
        self,
        finding_id: str,
        finding_description: str,
        file_path: str,
        file_content: str,
        skill_context: str,
    ) -> FixResult:
        """Ask LLM to generate a fix. Returns schema-validated result."""

    @abstractmethod
    def evaluate_independence(
        self,
        pass_a: AuditResult,
        pass_b: AuditResult,
    ) -> EvaluationResult:
        """Ask LLM if two audit passes are independent.
        Called AFTER code-level structural check. LLM is secondary only."""

    @abstractmethod
    def write(self, spec: str, skill_context: str) -> WriteResult:
        """Ask LLM to write code or documentation."""
