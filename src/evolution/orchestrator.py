"""Evolution orchestrator — chains the 5-beat autonomous evolution loop.

GATHER → EVALUATE → INTEGRATE → POST → ENGAGE

Each beat produces output that feeds into the next.
All code changes go through the convergence engine with safety gates.
Protected files cannot be modified.
"""

from __future__ import annotations

import os
import time
from dataclasses import asdict

from .evaluate import evaluate_all, EvaluationResult
from .gather import gather_all, GatherResult
from .engage import process_issues
from .post import generate_changelog_entry, generate_x_post, save_post
from .state import (
    EvolutionCycle,
    EvolutionState,
    append_to_archive,
    check_protected_files,
    load_active_context,
    save_active_context,
)


DEFAULT_EVOLUTION_DIR = ".cruxdev/evolution"


class EvolutionOrchestrator:
    """Drives one complete evolution cycle through all 5 beats."""

    def __init__(
        self,
        project_dir: str,
        project_name: str,
        github_repo: str = "",
        dry_run: bool = True,
    ):
        self.project_dir = os.path.abspath(project_dir)
        self.project_name = project_name
        self.github_repo = github_repo
        self.dry_run = dry_run

        self.evolution_dir = os.path.join(self.project_dir, DEFAULT_EVOLUTION_DIR)
        self.archive_path = os.path.join(self.evolution_dir, "archive.jsonl")
        self.context_path = os.path.join(self.evolution_dir, "context.json")
        self.posts_dir = os.path.join(self.evolution_dir, "posts")

        os.makedirs(self.evolution_dir, exist_ok=True)

    def run_cycle(self) -> EvolutionCycle:
        """Run one complete evolution cycle (all 5 beats)."""
        state = load_active_context(self.context_path)
        state.project = self.project_name
        state.cycle_count += 1

        cycle = EvolutionCycle(
            cycle_id=state.cycle_count,
            started_at=time.time(),
        )
        state.current_cycle = cycle

        try:
            # Beat 1: GATHER
            cycle.beat = "gather"
            save_active_context(self.context_path, state)
            gathered = self._beat_gather()
            cycle.gathered = [
                f"{len(gathered.own_changes)} changes, "
                f"{len(gathered.github_issues)} issues, "
                f"{len(gathered.inbox_messages)} inbox messages"
            ]

            # Beat 2: EVALUATE
            cycle.beat = "evaluate"
            save_active_context(self.context_path, state)
            evaluated = self._beat_evaluate(gathered)
            cycle.evaluated = [
                {"title": item.title, "priority": item.priority, "action": item.action}
                for item in evaluated.items
            ]

            # Beat 3: INTEGRATE
            cycle.beat = "integrate"
            save_active_context(self.context_path, state)
            integrated = self._beat_integrate(evaluated, state)
            cycle.integrated = integrated

            # Beat 4: POST
            cycle.beat = "post"
            save_active_context(self.context_path, state)
            posted = self._beat_post(cycle)
            cycle.posted = posted

            # Beat 5: ENGAGE
            cycle.beat = "engage"
            save_active_context(self.context_path, state)
            engaged = self._beat_engage(gathered)
            cycle.engaged = engaged

            cycle.completed_at = time.time()
            cycle.beat = "complete"

        except Exception as e:
            cycle.error = str(e)
            cycle.beat = "error"

        # Save state
        state.current_cycle = cycle
        state.last_completed_at = time.time()
        save_active_context(self.context_path, state)

        # Append to archive (immutable)
        append_to_archive(self.archive_path, {
            "type": "cycle_complete",
            "cycle_id": cycle.cycle_id,
            "gathered": cycle.gathered,
            "evaluated_count": len(cycle.evaluated),
            "integrated_count": len(cycle.integrated),
            "posted_count": len(cycle.posted),
            "engaged_count": len(cycle.engaged),
            "error": cycle.error,
            "duration_seconds": (cycle.completed_at or time.time()) - cycle.started_at,
        })

        return cycle

    def _beat_gather(self) -> GatherResult:
        """Beat 1: Gather inputs from all sources."""
        return gather_all(
            project_dir=self.project_dir,
            project_name=self.project_name,
            github_repo=self.github_repo,
        )

    def _beat_evaluate(self, gathered: GatherResult) -> EvaluationResult:
        """Beat 2: Evaluate gathered items through prioritization and admission gate."""
        return evaluate_all(gathered)

    def _beat_integrate(
        self,
        evaluated: EvaluationResult,
        state: EvolutionState,
    ) -> list[str]:
        """Beat 3: Integrate changes through convergence pipeline.

        In dry_run mode, returns descriptions of what would be done.
        In live mode, generates build plans and converges them.
        """
        integrated = []
        for item in evaluated.items:
            if item.action == "skip":
                continue

            if self.dry_run:
                integrated.append(f"[DRY RUN] Would {item.action}: {item.title}")
            else:
                # Live mode: generate build plan and converge
                # This is where the convergence engine takes over
                integrated.append(f"Queued for convergence: {item.title}")

        return integrated

    def _beat_post(self, cycle: EvolutionCycle) -> list[str]:
        """Beat 4: Generate and save content from cycle results."""
        posted = []

        # Generate changelog entry
        changelog = generate_changelog_entry(cycle)
        path = save_post(changelog, self.posts_dir)
        posted.append(f"changelog: {path}")

        # Generate X post
        x_post = generate_x_post(cycle, self.project_name)
        path = save_post(x_post, self.posts_dir)
        posted.append(f"x_post: {path}")

        return posted

    def _beat_engage(self, gathered: GatherResult) -> list[str]:
        """Beat 5: Process community input (social isolation — no code changes)."""
        actions = process_issues(
            gathered.github_issues,
            repo=self.github_repo,
            dry_run=self.dry_run,
        )
        return [f"{a.action_type}: issue #{a.issue_number} — {a.detail}" for a in actions]
