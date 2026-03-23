"""Tests for Beat 5: Engage."""

from src.evolution.engage import (
    EngageAction,
    process_issues,
    triage_issue,
)


def test_triage_bug():
    action = triage_issue({"number": 1, "title": "App crashes on startup", "labels": []})
    assert action.action_type == "label"
    assert "bug" in action.detail


def test_triage_feature():
    action = triage_issue({"number": 2, "title": "Add support for YAML", "labels": []})
    assert "enhancement" in action.detail


def test_triage_docs():
    action = triage_issue({"number": 3, "title": "Fix typo in readme", "labels": []})
    assert "documentation" in action.detail


def test_triage_unknown():
    action = triage_issue({"number": 4, "title": "Something else entirely", "labels": []})
    assert "triage" in action.detail


def test_triage_already_labeled():
    action = triage_issue({"number": 5, "title": "Bug", "labels": [{"name": "bug"}]})
    assert action.action_type == "triage"
    assert "Already" in action.detail


def test_process_issues_dry_run():
    issues = [
        {"number": 1, "title": "Bug found", "labels": []},
        {"number": 2, "title": "Add feature", "labels": []},
    ]
    actions = process_issues(issues, dry_run=True)
    assert len(actions) == 2
    # Dry run: no actual gh commands executed


def test_process_issues_empty():
    actions = process_issues([])
    assert actions == []
