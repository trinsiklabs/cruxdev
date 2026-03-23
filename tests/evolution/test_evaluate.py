"""Tests for Beat 2: Evaluate."""

from src.evolution.evaluate import (
    EvaluationItem,
    evaluate_all,
    evaluate_inbox_message,
    evaluate_issue,
    learnings_admission_gate,
)
from src.evolution.gather import GatherResult


def test_evaluate_issue_bug():
    item = evaluate_issue({"number": 1, "title": "App crashes", "labels": [{"name": "bug"}]})
    assert item.priority == 1
    assert item.action == "fix"


def test_evaluate_issue_enhancement():
    item = evaluate_issue({"number": 2, "title": "Add feature", "labels": [{"name": "enhancement"}]})
    assert item.priority == 2
    assert item.action == "fix"  # Priority ≤ 2 maps to "fix"


def test_evaluate_issue_docs():
    item = evaluate_issue({"number": 3, "title": "Fix docs", "labels": [{"name": "documentation"}]})
    assert item.priority == 4


def test_evaluate_issue_no_labels():
    item = evaluate_issue({"number": 4, "title": "Something"})
    assert item.priority == 3


def test_evaluate_inbox_high():
    item = evaluate_inbox_message({"title": "Bug", "severity": "high", "type": "issue", "from": "crux", "body": "details"})
    assert item.priority == 1
    assert item.action == "fix"


def test_evaluate_inbox_improvement():
    item = evaluate_inbox_message({"title": "Idea", "severity": "medium", "type": "improvement", "from": "crux", "body": "details"})
    assert item.action == "build_plan"


def test_evaluate_inbox_breaking():
    item = evaluate_inbox_message({"title": "API changed", "severity": "high", "type": "breaking_change", "from": "crux", "body": "details"})
    assert item.action == "fix"


def test_evaluate_inbox_pattern():
    item = evaluate_inbox_message({"title": "New pattern", "severity": "low", "type": "pattern", "from": "crux", "body": "details"})
    assert item.action == "investigate"


def test_learnings_gate_novel():
    item = EvaluationItem(source="issue", title="New bug", description="d", priority=1, novel=True, action="fix")
    assert learnings_admission_gate(item, ["Old pattern", "Different thing"]) is True


def test_learnings_gate_duplicate():
    item = EvaluationItem(source="issue", title="Old pattern", description="d", priority=1, novel=True, action="fix")
    assert learnings_admission_gate(item, ["Old pattern"]) is False


def test_evaluate_all():
    gathered = GatherResult(
        github_issues=[
            {"number": 1, "title": "Bug", "labels": [{"name": "bug"}]},
            {"number": 2, "title": "Feature", "labels": []},
        ],
        inbox_messages=[
            {"title": "Issue from crux", "severity": "high", "type": "issue", "from": "crux", "body": "details"},
        ],
    )
    result = evaluate_all(gathered)
    assert result.actionable == 3
    assert result.items[0].priority <= result.items[-1].priority  # Sorted


def test_evaluate_all_with_filter():
    gathered = GatherResult(
        github_issues=[
            {"number": 1, "title": "Already known bug", "labels": [{"name": "bug"}]},
        ],
    )
    result = evaluate_all(gathered, existing_patterns=["Already known bug"])
    assert result.actionable == 0
    assert result.skipped == 1


def test_evaluate_all_inbox_skipped():
    gathered = GatherResult(
        inbox_messages=[
            {"title": "Known pattern", "severity": "low", "type": "pattern", "from": "crux", "body": "d"},
        ],
    )
    result = evaluate_all(gathered, existing_patterns=["Known pattern"])
    assert result.skipped == 1
    assert result.actionable == 0


def test_evaluate_all_empty():
    result = evaluate_all(GatherResult())
    assert result.actionable == 0
    assert result.items == []
