"""Tests for dependency update system."""

from src.improvement.dep_updates import (
    UpdateType,
    classify_update,
    OutdatedDep,
    UpdateResult,
)


def test_classify_patch():
    assert classify_update("1.0.0", "1.0.1") == UpdateType.PATCH


def test_classify_minor():
    assert classify_update("1.0.0", "1.1.0") == UpdateType.MINOR


def test_classify_major():
    assert classify_update("1.0.0", "2.0.0") == UpdateType.MAJOR


def test_classify_no_version():
    assert classify_update("unknown", "also_unknown") == UpdateType.MAJOR


def test_outdated_dep_dataclass():
    dep = OutdatedDep(name="foo", current="1.0", latest="2.0", update_type=UpdateType.MAJOR)
    assert dep.name == "foo"


def test_update_result_dataclass():
    dep = OutdatedDep(name="bar", current="1.0", latest="1.1", update_type=UpdateType.MINOR)
    result = UpdateResult(dep=dep, success=True, tests_passed=True, message="ok")
    assert result.success is True
