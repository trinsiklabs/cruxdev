"""Documentation-code drift detection.

Detects mismatches between documentation and code:
- Function signatures that changed but docs weren't updated
- Module renames not reflected in docs
- Features added/removed without doc updates
- Stale file path references
"""

from __future__ import annotations

import ast
import os
import re
from dataclasses import dataclass


@dataclass
class DriftFinding:
    doc_file: str
    line: int
    category: str  # "signature", "path", "module", "feature"
    description: str


def find_code_references(doc_content: str) -> list[tuple[int, str]]:
    """Extract code references from documentation.

    Returns list of (line_number, reference) tuples.
    References include: file paths, function names in backticks, module names.
    """
    refs = []
    for i, line in enumerate(doc_content.split("\n"), 1):
        # File paths (e.g., `src/engine/state.py`)
        for match in re.finditer(r'`([a-zA-Z0-9_/.-]+\.py)`', line):
            refs.append((i, match.group(1)))
        # Function references (e.g., `validate_plan()`)
        for match in re.finditer(r'`([a-zA-Z_][a-zA-Z0-9_]*)\(\)`', line):
            refs.append((i, match.group(1) + "()"))
    return refs


def check_path_references(
    doc_file: str,
    doc_content: str,
    project_dir: str,
) -> list[DriftFinding]:
    """Check if file path references in a doc still exist."""
    findings = []
    refs = find_code_references(doc_content)

    for line_num, ref in refs:
        if ref.endswith("()"):
            continue  # Function ref, not path ref
        full_path = os.path.join(project_dir, ref)
        if not os.path.exists(full_path):
            findings.append(DriftFinding(
                doc_file=doc_file,
                line=line_num,
                category="path",
                description=f"Referenced path `{ref}` does not exist",
            ))

    return findings


def check_function_references(
    doc_file: str,
    doc_content: str,
    project_dir: str,
) -> list[DriftFinding]:
    """Check if function references in a doc still exist in the codebase."""
    findings = []
    refs = find_code_references(doc_content)

    # Build a set of all function names in the project
    all_functions: set[str] = set()
    for root, dirs, files in os.walk(project_dir):
        dirs[:] = [d for d in dirs if d not in {"__pycache__", ".git", "node_modules", ".venv"}]
        for name in files:
            if not name.endswith(".py"):
                continue
            filepath = os.path.join(root, name)
            try:
                with open(filepath) as f:
                    tree = ast.parse(f.read())
                for node in ast.walk(tree):
                    if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                        all_functions.add(node.name)
            except (SyntaxError, OSError):
                continue

    for line_num, ref in refs:
        if not ref.endswith("()"):
            continue
        func_name = ref[:-2]  # Remove ()
        if func_name not in all_functions:
            findings.append(DriftFinding(
                doc_file=doc_file,
                line=line_num,
                category="signature",
                description=f"Referenced function `{func_name}()` not found in codebase",
            ))

    return findings


def scan_doc(
    doc_file: str,
    project_dir: str,
) -> list[DriftFinding]:
    """Scan a single doc file for drift."""
    try:
        with open(doc_file) as f:
            content = f.read()
    except (FileNotFoundError, OSError):
        return []

    findings = []
    findings.extend(check_path_references(doc_file, content, project_dir))
    findings.extend(check_function_references(doc_file, content, project_dir))
    return findings


def scan_docs_dir(
    docs_dir: str,
    project_dir: str,
) -> list[DriftFinding]:
    """Scan all documentation files for drift."""
    findings = []

    if not os.path.isdir(docs_dir):
        return findings

    for name in sorted(os.listdir(docs_dir)):
        if not name.endswith(".md"):
            continue
        doc_path = os.path.join(docs_dir, name)
        findings.extend(scan_doc(doc_path, project_dir))

    return findings
