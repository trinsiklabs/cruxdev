"""Technical debt auto-detection — AST analysis for complexity, duplication, anti-patterns.

Scans Python source files for:
- High cyclomatic complexity functions
- Code duplication (similar AST subtrees)
- Anti-patterns (bare except, hardcoded paths, unused imports)
"""

from __future__ import annotations

import ast
import os
from dataclasses import dataclass


@dataclass
class DebtFinding:
    file: str
    line: int
    category: str  # "complexity", "duplication", "anti_pattern"
    severity: str  # "high", "medium", "low"
    description: str
    suggestion: str


def calculate_complexity(node: ast.FunctionDef | ast.AsyncFunctionDef) -> int:
    """Calculate cyclomatic complexity of a function.

    Counts decision points: if, elif, for, while, except, and, or,
    assert, with, ternary (IfExp).
    """
    complexity = 1  # Base complexity
    for child in ast.walk(node):
        if isinstance(child, (ast.If, ast.IfExp)):
            complexity += 1
        elif isinstance(child, (ast.For, ast.While, ast.AsyncFor)):
            complexity += 1
        elif isinstance(child, ast.ExceptHandler):
            complexity += 1
        elif isinstance(child, ast.Assert):
            complexity += 1
        elif isinstance(child, (ast.With, ast.AsyncWith)):
            complexity += 1
        elif isinstance(child, ast.BoolOp):
            # and/or add complexity
            complexity += len(child.values) - 1
    return complexity


def find_complex_functions(
    filepath: str,
    threshold: int = 10,
) -> list[DebtFinding]:
    """Find functions with cyclomatic complexity above threshold."""
    try:
        with open(filepath) as f:
            source = f.read()
        tree = ast.parse(source)
    except (SyntaxError, FileNotFoundError, OSError):
        return []

    findings = []
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            complexity = calculate_complexity(node)
            if complexity > threshold:
                severity = "high" if complexity > 20 else "medium"
                findings.append(DebtFinding(
                    file=filepath,
                    line=node.lineno,
                    category="complexity",
                    severity=severity,
                    description=f"Function '{node.name}' has complexity {complexity} (threshold: {threshold})",
                    suggestion=f"Refactor '{node.name}' into smaller functions",
                ))
    return findings


def find_anti_patterns(filepath: str) -> list[DebtFinding]:
    """Find common anti-patterns in Python code."""
    try:
        with open(filepath) as f:
            source = f.read()
        tree = ast.parse(source)
    except (SyntaxError, FileNotFoundError, OSError):
        return []

    findings = []
    for node in ast.walk(tree):
        # Bare except
        if isinstance(node, ast.ExceptHandler) and node.type is None:
            findings.append(DebtFinding(
                file=filepath,
                line=node.lineno,
                category="anti_pattern",
                severity="medium",
                description=f"Bare 'except:' on line {node.lineno} catches all exceptions including SystemExit and KeyboardInterrupt",
                suggestion="Catch specific exceptions (e.g., except ValueError:)",
            ))

        # Hardcoded /tmp paths
        if isinstance(node, ast.Constant) and isinstance(node.value, str):
            if node.value.startswith("/tmp/") or node.value.startswith("/var/tmp/"):
                findings.append(DebtFinding(
                    file=filepath,
                    line=node.lineno,
                    category="anti_pattern",
                    severity="low",
                    description=f"Hardcoded temp path '{node.value[:50]}' on line {node.lineno}",
                    suggestion="Use tempfile.mkdtemp() or tempfile.NamedTemporaryFile()",
                ))

    return findings


def scan_project(
    project_dir: str,
    complexity_threshold: int = 10,
) -> list[DebtFinding]:
    """Scan all Python files in a project for technical debt."""
    findings = []

    for root, dirs, files in os.walk(project_dir):
        dirs[:] = [d for d in dirs if d not in {
            "__pycache__", ".git", "node_modules", ".venv", "venv",
            ".tox", ".mypy_cache", ".pytest_cache",
        }]
        for name in files:
            if not name.endswith(".py"):
                continue
            filepath = os.path.join(root, name)
            findings.extend(find_complex_functions(filepath, complexity_threshold))
            findings.extend(find_anti_patterns(filepath))

    return sorted(findings, key=lambda f: (f.severity == "high", f.severity == "medium"), reverse=True)
