"""Task router — determines what Claude Code should do next.

The engine doesn't call the LLM. It tells Claude Code what to do,
and processes the results. This module maps engine state to tasks.
"""

import json
import os
import re
import time
from dataclasses import dataclass
from typing import Optional

from .convergence import (
    check_convergence,
    check_max_rounds,
    check_net_negative,
    check_timeout,
    escalate,
    record_failure,
    record_round,
)
from .persistence import save_state
from .plan_validator import validate_plan
from .state import (
    ConvergencePhase,
    ConvergenceState,
    Finding,
    FindingSeverity,
)

# Dimension sets for each convergence type
PLAN_DIMENSIONS = ["completeness", "feasibility", "risk_assessment", "dependency_ordering", "testability"]
CODE_DIMENSIONS = ["correctness", "completeness", "edge_cases", "error_handling", "security", "performance", "maintainability", "test_coverage"]
DOC_DIMENSIONS = ["accuracy", "completeness", "consistency", "clarity", "currency"]


# Task type → recommended model tier
TASK_MODEL_TIERS: dict[str, str] = {
    "audit": "standard",
    "fix": "standard",
    "test": "fast",
    "write": "standard",
}

# Phase-specific overrides
PHASE_MODEL_TIERS: dict[str, str] = {
    "plan_auditing": "fast",
    "code_auditing": "standard",
    "doc_auditing": "fast",
    "e2e_testing": "fast",
}


@dataclass
class Task:
    """A task for Claude Code to execute."""
    task_type: str  # "audit", "fix", "test", "write", "done", "escalated"
    description: str
    files: list[str]
    dimensions: list[str]
    finding: Optional[dict] = None
    test_command: Optional[list[str]] = None
    metadata: Optional[dict] = None
    recommended_tier: Optional[str] = None  # e.g., "fast", "standard", "frontier"

    def to_dict(self) -> dict:
        d = {
            "task_type": self.task_type,
            "description": self.description,
            "files": self.files,
            "dimensions": self.dimensions,
        }
        if self.finding:
            d["finding"] = self.finding
        if self.test_command:
            d["test_command"] = self.test_command
        if self.metadata:
            d["metadata"] = self.metadata
        if self.recommended_tier:
            d["recommended_tier"] = self.recommended_tier
        return d


def _detect_website(plan_file: str) -> dict:
    """Detect if the project has a website that needs convergence.

    Checks for: docs/DEPLOYMENT.md, docs/WEBSITE.md, or docs/WEBSITE_PLANNING.md.
    Returns dict with has_website, files to audit, and any site URL found.
    """
    plan_dir = os.path.dirname(os.path.abspath(plan_file))

    # Find project root (same logic as _auto_discover_docs)
    project_root = plan_dir
    for _ in range(5):
        if any(os.path.exists(os.path.join(project_root, marker))
               for marker in (".git", "pyproject.toml", "package.json")):
            break
        parent = os.path.dirname(project_root)
        if parent == project_root:
            break
        project_root = parent

    website_markers = [
        os.path.join(project_root, "docs", "DEPLOYMENT.md"),
        os.path.join(project_root, "docs", "WEBSITE.md"),
    ]

    found_files = [f for f in website_markers if os.path.exists(f)]

    if not found_files:
        return {"has_website": False, "files": [], "site_url": ""}

    # Try to extract site URL from DEPLOYMENT.md or WEBSITE.md
    site_url = ""
    for f in found_files:
        try:
            with open(f) as fh:
                content = fh.read()
                # Look for URL patterns
                import re
                urls = re.findall(r'(https?://[^\s\)\]>"\']+\.(?:io|dev|com|org|net)[^\s\)\]>"\']*)', content)
                if urls:
                    site_url = urls[0]
                    break
        except OSError:
            pass

    return {
        "has_website": True,
        "files": found_files,
        "site_url": site_url,
        "project_root": project_root,
    }


def _detect_webapp(plan_file: str) -> bool:
    """Detect if the project is a webapp (not just a static site).

    Checks for: app/ directory, API routes, Dockerfile, E2E test patterns,
    or other webapp indicators.
    """
    plan_dir = os.path.dirname(os.path.abspath(plan_file))

    project_root = plan_dir
    for _ in range(5):
        if any(os.path.exists(os.path.join(project_root, marker))
               for marker in (".git", "pyproject.toml", "package.json")):
            break
        parent = os.path.dirname(project_root)
        if parent == project_root:
            break
        project_root = parent

    webapp_markers = [
        os.path.join(project_root, "app"),
        os.path.join(project_root, "api"),
        os.path.join(project_root, "Dockerfile"),
        os.path.join(project_root, "docker-compose.yml"),
        os.path.join(project_root, "docs", "E2E_TEST_PATTERNS.md"),
        os.path.join(project_root, "docs", "UAT_TEST_PATTERNS.md"),
    ]
    return any(os.path.exists(m) for m in webapp_markers)


def _auto_discover_docs(plan_file: str) -> list[str]:
    """Auto-discover doc files from project docs/ directory.

    If a docs/ directory exists relative to the plan file's project,
    returns all .md files in it. Falls back to [plan_file].
    """
    import glob

    # Try to find docs/ relative to the plan file
    plan_dir = os.path.dirname(os.path.abspath(plan_file))

    # Walk up to find project root (look for .git, pyproject.toml, package.json)
    project_root = plan_dir
    for _ in range(5):  # max 5 levels up
        if any(os.path.exists(os.path.join(project_root, marker))
               for marker in (".git", "pyproject.toml", "package.json")):
            break
        parent = os.path.dirname(project_root)
        if parent == project_root:
            break
        project_root = parent

    docs_dir = os.path.join(project_root, "docs")
    if os.path.isdir(docs_dir):
        docs = glob.glob(os.path.join(docs_dir, "*.md"))
        # Also check README at root
        readme = os.path.join(project_root, "README.md")
        if os.path.exists(readme):
            docs.append(readme)
        if docs:
            return sorted(docs)

    return [plan_file]


def get_next_task(
    state: ConvergenceState,
    state_path: str,
    source_files: list[str] | None = None,
    doc_files: list[str] | None = None,
    test_command: list[str] | None = None,
) -> Task:
    """Determine the next task for Claude Code based on engine state."""

    # Terminal states
    if state.phase == ConvergencePhase.CONVERGED:
        return Task(
            task_type="done",
            description="Convergence complete. Two consecutive clean passes achieved.",
            files=[], dimensions=[],
            metadata={"round": state.round, "history_length": len(state.history)},
        )

    if state.phase == ConvergencePhase.ESCALATED:
        return Task(
            task_type="escalated",
            description=f"Escalated: {state.escalation_reason}",
            files=[], dimensions=[],
            metadata={"reason": state.escalation_reason, "round": state.round},
        )

    # Safety checks
    if check_timeout(state):
        escalate(state, f"{state.phase.value}_timeout")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    if check_max_rounds(state):
        escalate(state, f"{state.phase.value}_max_rounds")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    if check_net_negative(state):
        escalate(state, f"{state.phase.value}_net_negative")
        save_state(state, state_path)
        return get_next_task(state, state_path)

    # Phase-specific tasks
    # Determine model tier for this phase
    phase_tier = PHASE_MODEL_TIERS.get(state.phase.value)

    if state.phase == ConvergencePhase.PLANNING:
        if check_convergence(state):
            state.phase = ConvergencePhase.PLAN_AUDITING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        return Task(
            task_type="write",
            description="Create or refine the build plan. Follow the methodology from get_methodology().",
            recommended_tier=TASK_MODEL_TIERS.get("write"),
            files=[state.plan_file],
            dimensions=[],
        )

    if state.phase == ConvergencePhase.PLAN_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.DOC_ALIGNMENT
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        return Task(
            task_type="audit",
            description=f"Audit the plan (round {state.round}). Check for gaps, risks, missing steps.",
            files=[state.plan_file],
            dimensions=PLAN_DIMENSIONS,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.DOC_ALIGNMENT:
        # Extract alignment docs from the plan file
        alignment_docs = _extract_alignment_docs(state.plan_file)

        # No alignment docs → auto-advance (new project, nothing to align against)
        if not alignment_docs:
            state.phase = ConvergencePhase.VIABILITY
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)

        if check_convergence(state):
            state.phase = ConvergencePhase.VIABILITY
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)

        return Task(
            task_type="doc_align",
            description=(
                f"Document alignment audit (round {state.round}). "
                f"Read each alignment doc and verify the plan conforms to it. "
                f"Check: decisions, constraints, requirements, pricing rules, feature specs."
            ),
            files=[state.plan_file] + alignment_docs,
            dimensions=["doc_alignment"],
            recommended_tier="standard",
            metadata={"alignment_docs": alignment_docs},
        )

    if state.phase == ConvergencePhase.VIABILITY:
        # Auto-advance — viability is a pass-through for now
        state.phase = ConvergencePhase.EXECUTING
        save_state(state, state_path)
        return get_next_task(state, state_path, source_files, doc_files, test_command)

    if state.phase == ConvergencePhase.EXECUTING:
        # Green-field execution: parse checklist, execute items one by one
        from .checklist_parser import parse_checklist, get_next_incomplete, all_complete

        items = parse_checklist(state.plan_file)
        if not items or all_complete(items):
            # No checklist or all done → advance to code_audit
            state.phase = ConvergencePhase.CODE_AUDITING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)

        next_item = get_next_incomplete(items)
        if next_item is None:  # pragma: no cover
            state.phase = ConvergencePhase.CODE_AUDITING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)

        from .checklist_parser import completion_summary
        summary = completion_summary(items)
        return Task(
            task_type="execute",
            description=(
                f"Execute checklist item {next_item.id}: {next_item.description} "
                f"({summary['completed']}/{summary['total']} complete, "
                f"{summary['percentage']}%)"
            ),
            files=[state.plan_file],
            dimensions=[],
            recommended_tier="standard",
            test_command=test_command,
            metadata={
                "checklist_item": next_item.id,
                "phase": next_item.phase,
                "progress": summary,
            },
        )

    if state.phase == ConvergencePhase.CODE_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.DOC_AUDITING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        files = source_files or [state.plan_file]
        return Task(
            task_type="audit",
            description=f"Audit code (round {state.round}). Read the files, find issues, fix them, run tests.",
            files=files,
            dimensions=CODE_DIMENSIONS,
            test_command=test_command,
            recommended_tier=phase_tier or "standard",
        )

    if state.phase == ConvergencePhase.DOC_AUDITING:
        if check_convergence(state):
            state.phase = ConvergencePhase.WEBSITE_CONVERGENCE
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        files = doc_files or _auto_discover_docs(state.plan_file)
        return Task(
            task_type="audit",
            description=f"Audit documentation (round {state.round}). Check accuracy, completeness, consistency.",
            files=files,
            dimensions=DOC_DIMENSIONS,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.WEBSITE_CONVERGENCE:
        website_info = _detect_website(state.plan_file)
        if not website_info["has_website"]:
            # No website — skip to E2E testing
            state.phase = ConvergencePhase.E2E_TESTING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        if check_convergence(state):
            state.phase = ConvergencePhase.E2E_TESTING
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        is_webapp = _detect_webapp(state.plan_file)
        return Task(
            task_type="audit",
            description=(
                f"Website convergence (round {state.round}). The project has a website. "
                f"Audit it against the now-converged code and docs:\n"
                f"1. Update website metrics (test counts, tool counts, feature counts) to match current code\n"
                f"2. Verify all claims on the site are accurate against docs/\n"
                f"3. Check comparison pages are current against docs/COMPETITORS.md\n"
                f"4. Update any screenshots or demos if the UI changed\n"
                f"5. Deploy the updated site per docs/DEPLOYMENT.md\n"
                f"Methodology: docs/WEBSITE_PLANNING.md Phase 12 (Post-Launch Operations)\n"
                + (
                    f"6. This is a WEBAPP — also converge against docs/E2E_TEST_PATTERNS.md and docs/UAT_TEST_PATTERNS.md\n"
                    f"   Run E2E tests to verify user flows work after changes\n"
                    f"   Verify UAT acceptance criteria are still met\n"
                    if is_webapp else ""
                )
                + f"Website info: {website_info}"
            ),
            files=website_info.get("files", []),
            dimensions=["accuracy", "completeness", "metrics", "deployment"]
                + (["e2e_testing", "uat_testing"] if is_webapp else []),
            recommended_tier=phase_tier or "standard",
        )

    if state.phase == ConvergencePhase.E2E_TESTING:
        if check_convergence(state):
            state.phase = ConvergencePhase.PATTERNS_UPDATE
            state.round = 0
            state.consecutive_clean = 0
            save_state(state, state_path)
            return get_next_task(state, state_path, source_files, doc_files, test_command)
        return Task(
            task_type="test",
            description=f"Run the full test suite (round {state.round}).",
            files=[],
            dimensions=[],
            test_command=test_command,
            recommended_tier=phase_tier or "fast",
        )

    if state.phase == ConvergencePhase.PATTERNS_UPDATE:
        # Auto-advance to converged
        state.phase = ConvergencePhase.CONVERGED
        save_state(state, state_path)
        return get_next_task(state, state_path)

    # Shouldn't reach here
    return Task(  # pragma: no cover
        task_type="escalated",
        description=f"Unknown phase: {state.phase.value}",
        files=[], dimensions=[],
    )


def _extract_alignment_docs(plan_file: str) -> list[str]:
    """Extract alignment doc paths from a build plan's Document Alignment section.

    Looks for a table or list of doc paths under ## Document Alignment.
    Returns list of file paths found.
    """
    try:
        with open(plan_file) as f:
            content = f.read()
    except (FileNotFoundError, OSError):
        return []

    # Find the Document Alignment section
    match = re.search(
        r"##\s+Document Alignment\s*\n(.*?)(?=\n##\s|\Z)",
        content,
        re.DOTALL,
    )
    if not match:
        return []

    section = match.group(1)

    # Extract file paths from the section
    # Matches: paths in table cells, after | or -, or in backticks
    paths = []
    for line in section.split("\n"):
        # Table format: | ... | path/to/file.md |
        for cell_match in re.finditer(r'[\|`]\s*([a-zA-Z0-9_/.:-]+\.md)\s*[\|`]', line):
            path = cell_match.group(1).strip()
            if path and not path.startswith("---"):
                paths.append(path)
        # List format: - path/to/file.md — description
        list_match = re.match(r'\s*[-*]\s+([a-zA-Z0-9_/.:-]+\.md)', line)
        if list_match:
            paths.append(list_match.group(1))

    return list(dict.fromkeys(paths))  # Dedupe preserving order


def submit_result(
    state: ConvergenceState,
    state_path: str,
    result: dict,
) -> None:
    """Process a result from Claude Code and update engine state.

    For execution tasks, also handles checklist item completion.
    """
    task_type = result.get("task_type", "audit")

    # Handle execution phase checklist completion
    checklist_item = result.get("checklist_item")
    if checklist_item and state.phase == ConvergencePhase.EXECUTING:
        from .checklist_parser import parse_checklist, mark_complete
        items = parse_checklist(state.plan_file)
        mark_complete(items, checklist_item)
        # Note: mark_complete modifies the in-memory list but doesn't persist.
        # The plan file itself tracks completion via [x] markers.
        # The engine re-parses on each get_next_task call.

    findings_data = result.get("findings", [])

    findings = [
        Finding(
            id=f.get("id", f"f{i}"),
            file=f.get("file", "unknown"),
            dimension=f.get("dimension", "unknown"),
            severity=FindingSeverity(f.get("severity", "medium")),
            description=f.get("description", ""),
            suggested_fix=f.get("suggested_fix", ""),
            fixed=f.get("fixed", False),
        )
        for i, f in enumerate(findings_data)
    ]

    # Track failures for unfixed findings
    for f in findings:
        if not f.fixed:
            record_failure(state, f.id)

    record_round(state, findings)
    save_state(state, state_path)
