"""Master convergence runner — chains sub-loops through the full lifecycle.

All control flow is code. Sub-loops call the LLM dispatcher for language tasks.
"""

import time

from ..dispatch.llm import LLMDispatcher
from .code_convergence import run_code_convergence
from .convergence import advance_phase, escalate, is_terminal
from .doc_convergence import run_doc_convergence
from .e2e_convergence import run_e2e_convergence
from .persistence import save_state
from .plan_convergence import run_plan_convergence
from .state import ConvergencePhase, ConvergenceState


class ConvergenceRunner:
    """Orchestrates the full convergence lifecycle."""

    def __init__(
        self,
        state: ConvergenceState,
        llm: LLMDispatcher,
        state_path: str,
        project_dir: str = ".",
        test_command: list[str] | None = None,
        source_files: list[str] | None = None,
        doc_files: list[str] | None = None,
    ):
        self.state = state
        self.llm = llm
        self.state_path = state_path
        self.project_dir = project_dir
        self.test_command = test_command
        self.source_files = source_files
        self.doc_files = doc_files

    def run(self) -> ConvergenceState:
        """Run the full convergence lifecycle to completion."""
        while not is_terminal(self.state.phase):
            self._run_phase()
            if is_terminal(self.state.phase):
                break
            self.state.phase = advance_phase(self.state.phase)
            save_state(self.state, self.state_path)

        save_state(self.state, self.state_path)
        return self.state

    def _run_phase(self) -> None:
        """Dispatch to the appropriate sub-loop for the current phase."""
        phase = self.state.phase

        if phase == ConvergencePhase.PLANNING:
            # Planning phase — just advance (plan file already exists)
            pass

        elif phase == ConvergencePhase.PLAN_AUDITING:
            run_plan_convergence(self.state, self.llm, self.state_path)

        elif phase == ConvergencePhase.DOC_ALIGNMENT:
            # Doc alignment uses the same audit pattern as plan convergence
            # but audits the plan against external docs
            run_plan_convergence(self.state, self.llm, self.state_path)

        elif phase == ConvergencePhase.VIABILITY:
            # Viability check — advance if plan converged
            pass

        elif phase == ConvergencePhase.EXECUTING:
            # Execution phase — advance (code changes happen during audit)
            pass

        elif phase == ConvergencePhase.CODE_AUDITING:
            run_code_convergence(
                self.state,
                self.llm,
                self.state_path,
                project_dir=self.project_dir,
                test_command=self.test_command,
                files=self.source_files,
            )

        elif phase == ConvergencePhase.DOC_AUDITING:
            run_doc_convergence(
                self.state,
                self.llm,
                self.state_path,
                doc_files=self.doc_files,
            )

        elif phase == ConvergencePhase.E2E_TESTING:
            run_e2e_convergence(
                self.state,
                self.state_path,
                project_dir=self.project_dir,
                test_command=self.test_command,
            )

        elif phase == ConvergencePhase.PATTERNS_UPDATE:
            # Patterns update — advance (engine doesn't modify patterns files yet)
            pass
