"""Beat 2: EVALUATE — gap analysis, prioritization, learnings gate.

Scores gathered items by impact and novelty.
Filters through the learnings admission gate.
"""

from __future__ import annotations

from dataclasses import dataclass, field

from .gather import GatherResult


@dataclass
class EvaluationItem:
    source: str  # "issue", "inbox", "change", "inspiration"
    title: str
    description: str
    priority: int  # 1=highest, 5=lowest
    novel: bool  # passes learnings admission gate
    action: str  # "build_plan", "fix", "investigate", "skip"


@dataclass
class EvaluationResult:
    items: list[EvaluationItem] = field(default_factory=list)
    skipped: int = 0
    actionable: int = 0


def evaluate_issue(issue: dict) -> EvaluationItem:
    """Evaluate a GitHub issue."""
    title = issue.get("title", "")
    labels = [l.get("name", "") for l in issue.get("labels", [])]

    priority = 3
    if "bug" in labels:
        priority = 1
    elif "enhancement" in labels:
        priority = 2
    elif "documentation" in labels:
        priority = 4

    return EvaluationItem(
        source="issue",
        title=title,
        description=f"GitHub issue #{issue.get('number', '?')}: {title}",
        priority=priority,
        novel=True,  # Issues are always novel (they're new input)
        action="fix" if priority <= 2 else "investigate",
    )


def evaluate_inbox_message(msg: dict) -> EvaluationItem:
    """Evaluate a session bus message."""
    severity_map = {"high": 1, "medium": 3, "low": 5}
    priority = severity_map.get(msg.get("severity", "medium"), 3)

    action_map = {
        "issue": "fix",
        "improvement": "build_plan",
        "breaking_change": "fix",
        "pattern": "investigate",
    }

    return EvaluationItem(
        source="inbox",
        title=msg.get("title", ""),
        description=f"From {msg.get('from', '?')}: {msg.get('body', '')}",
        priority=priority,
        novel=True,
        action=action_map.get(msg.get("type", ""), "investigate"),
    )


def learnings_admission_gate(item: EvaluationItem, existing_patterns: list[str]) -> bool:
    """Apply the learnings admission gate.

    A learning is only admitted if it is:
    1. Genuinely novel — not already captured
    2. Would change future behavior
    """
    # Check if title matches any existing pattern
    title_lower = item.title.lower()
    for pattern in existing_patterns:
        if pattern.lower() in title_lower or title_lower in pattern.lower():
            return False
    return True


def evaluate_all(
    gathered: GatherResult,
    existing_patterns: list[str] | None = None,
) -> EvaluationResult:
    """Evaluate all gathered items."""
    patterns = existing_patterns or []
    result = EvaluationResult()

    for issue in gathered.github_issues:
        item = evaluate_issue(issue)
        if learnings_admission_gate(item, patterns):
            result.items.append(item)
            result.actionable += 1
        else:
            result.skipped += 1

    for msg in gathered.inbox_messages:
        item = evaluate_inbox_message(msg)
        if learnings_admission_gate(item, patterns):
            result.items.append(item)
            result.actionable += 1
        else:
            result.skipped += 1

    # Sort by priority (1=highest first)
    result.items.sort(key=lambda x: x.priority)
    return result
