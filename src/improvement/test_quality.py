"""Test quality analysis — beyond coverage.

Detects:
- Weak assertions (assert True, assert x is not None without value check)
- Tests with no assertions
- Duplicate test names
"""

from __future__ import annotations

import ast
import os
from dataclasses import dataclass


@dataclass
class TestQualityFinding:
    file: str
    test_name: str
    line: int
    category: str  # "weak_assertion", "no_assertion", "duplicate_name"
    description: str


WEAK_ASSERTIONS = {
    "assertTrue": "Consider asserting a specific value instead of True",
    "assertIsNotNone": "Consider asserting the specific expected value",
    "assertFalse": "Consider asserting a specific condition",
}


def find_weak_assertions(filepath: str) -> list[TestQualityFinding]:
    """Find tests with weak assertions."""
    try:
        with open(filepath) as f:
            source = f.read()
        tree = ast.parse(source)
    except (SyntaxError, FileNotFoundError, OSError):
        return []

    findings = []
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        if not node.name.startswith("test_"):
            continue

        for child in ast.walk(node):
            if isinstance(child, ast.Call) and isinstance(child.func, ast.Attribute):
                method = child.func.attr
                if method in WEAK_ASSERTIONS:
                    findings.append(TestQualityFinding(
                        file=filepath,
                        test_name=node.name,
                        line=child.lineno,
                        category="weak_assertion",
                        description=f"{method}() in {node.name}: {WEAK_ASSERTIONS[method]}",
                    ))

    return findings


def find_tests_without_assertions(filepath: str) -> list[TestQualityFinding]:
    """Find test functions that contain no assert statements."""
    try:
        with open(filepath) as f:
            source = f.read()
        tree = ast.parse(source)
    except (SyntaxError, FileNotFoundError, OSError):
        return []

    findings = []
    for node in ast.walk(tree):
        if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            continue
        if not node.name.startswith("test_"):
            continue

        has_assert = False
        for child in ast.walk(node):
            if isinstance(child, ast.Assert):
                has_assert = True
                break
            if isinstance(child, ast.Call) and isinstance(child.func, ast.Attribute):
                if child.func.attr.startswith("assert"):
                    has_assert = True
                    break
            # pytest.raises counts as an assertion
            if isinstance(child, ast.Call) and isinstance(child.func, ast.Attribute):
                if child.func.attr == "raises":
                    has_assert = True
                    break

        if not has_assert:
            findings.append(TestQualityFinding(
                file=filepath,
                test_name=node.name,
                line=node.lineno,
                category="no_assertion",
                description=f"Test '{node.name}' has no assertions",
            ))

    return findings


def find_duplicate_test_names(test_dir: str) -> list[TestQualityFinding]:
    """Find duplicate test function names across the test suite."""
    seen: dict[str, str] = {}  # name → first file
    findings = []

    for root, dirs, files in os.walk(test_dir):
        dirs[:] = [d for d in dirs if d != "__pycache__"]
        for name in sorted(files):
            if not name.startswith("test_") or not name.endswith(".py"):
                continue
            filepath = os.path.join(root, name)
            try:
                with open(filepath) as f:
                    tree = ast.parse(f.read())
            except (SyntaxError, OSError):
                continue

            for node in ast.walk(tree):
                if not isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    continue
                if not node.name.startswith("test_"):
                    continue
                if node.name in seen and seen[node.name] != filepath:
                    findings.append(TestQualityFinding(
                        file=filepath,
                        test_name=node.name,
                        line=node.lineno,
                        category="duplicate_name",
                        description=f"Test '{node.name}' also exists in {seen[node.name]}",
                    ))
                else:
                    seen[node.name] = filepath

    return findings


def scan_test_quality(test_dir: str) -> list[TestQualityFinding]:
    """Run all test quality checks."""
    findings = []

    for root, dirs, files in os.walk(test_dir):
        dirs[:] = [d for d in dirs if d != "__pycache__"]
        for name in sorted(files):
            if not name.startswith("test_") or not name.endswith(".py"):
                continue
            filepath = os.path.join(root, name)
            findings.extend(find_weak_assertions(filepath))
            findings.extend(find_tests_without_assertions(filepath))

    findings.extend(find_duplicate_test_names(test_dir))
    return findings
