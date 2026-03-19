"""Dependency graph — code-maintained import graph for Python projects.

Uses Python's ast module to parse imports. The engine (not LLM) decides
what files to include in each audit call.
"""

import ast
import os
from pathlib import Path


class DependencyGraph:
    """Import-based dependency graph for Python projects."""

    def __init__(self, project_root: str):
        self.root = project_root
        self.edges: dict[str, set[str]] = {}  # file → files it imports

    def build(self) -> None:
        """Parse all Python source files, build import graph."""
        self.edges.clear()
        for py_file in self._find_python_files():
            self.edges[py_file] = self._parse_imports(py_file)

    def update(self, changed_file: str) -> None:
        """Re-parse one file, update edges."""
        rel = os.path.relpath(changed_file, self.root)
        self.edges[rel] = self._parse_imports(rel)

    def impact_set(self, changed_files: list[str]) -> set[str]:
        """Given changes, what other files might be affected?
        Returns transitive dependents (files that import the changed files)."""
        affected = set(changed_files)
        reverse = self._reverse_edges()
        queue = list(changed_files)

        while queue:
            current = queue.pop(0)
            for dependent in reverse.get(current, set()):
                if dependent not in affected:
                    affected.add(dependent)
                    queue.append(dependent)

        return affected

    def audit_context(
        self, target_files: list[str], token_budget: int
    ) -> list[str]:
        """Given files to audit and a token budget,
        return the optimal set of context files.
        Estimates ~50 tokens per line."""
        context = list(target_files)
        remaining_budget = token_budget

        # Subtract target file tokens
        for f in target_files:
            remaining_budget -= self._estimate_tokens(f)

        # Add dependencies of target files, sorted by relevance
        deps = set()
        for f in target_files:
            deps.update(self.edges.get(f, set()))
        deps -= set(target_files)

        for dep in sorted(deps):
            cost = self._estimate_tokens(dep)
            if remaining_budget >= cost:
                context.append(dep)
                remaining_budget -= cost

        return context

    def assign_scopes(self, n_agents: int) -> list[set[str]]:
        """Partition files into N non-overlapping scopes with minimal
        cross-scope dependencies."""
        all_files = sorted(self.edges.keys())
        if n_agents <= 0 or len(all_files) == 0:
            return []

        # Simple round-robin partition (good enough for v1)
        scopes: list[set[str]] = [set() for _ in range(n_agents)]
        for i, f in enumerate(all_files):
            scopes[i % n_agents].add(f)

        return [s for s in scopes if s]  # Remove empty scopes

    def _find_python_files(self) -> list[str]:
        """Find all .py files relative to project root."""
        files = []
        for root, dirs, filenames in os.walk(self.root):
            # Skip common non-source directories
            dirs[:] = [d for d in dirs if d not in {
                "__pycache__", ".git", "node_modules", ".venv", "venv",
                ".tox", ".mypy_cache", ".pytest_cache",
            }]
            for name in filenames:
                if name.endswith(".py"):
                    full = os.path.join(root, name)
                    rel = os.path.relpath(full, self.root)
                    files.append(rel)
        return sorted(files)

    def _parse_imports(self, rel_path: str) -> set[str]:
        """Parse a Python file and extract import targets as relative paths."""
        full_path = os.path.join(self.root, rel_path)
        try:
            with open(full_path) as f:
                source = f.read()
            tree = ast.parse(source)
        except (SyntaxError, FileNotFoundError, OSError):
            return set()

        imports = set()
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    resolved = self._resolve_module(alias.name)
                    if resolved:
                        imports.add(resolved)
            elif isinstance(node, ast.ImportFrom):
                if node.module:
                    resolved = self._resolve_module(node.module)
                    if resolved:
                        imports.add(resolved)

        return imports

    def _resolve_module(self, module_name: str) -> str | None:
        """Try to resolve a module name to a relative file path."""
        parts = module_name.split(".")
        # Try as package (dir/__init__.py) or module (file.py)
        candidates = [
            os.path.join(*parts) + ".py",
            os.path.join(*parts, "__init__.py"),
        ]
        for candidate in candidates:
            full = os.path.join(self.root, candidate)
            if os.path.exists(full):
                return candidate
        return None

    def _reverse_edges(self) -> dict[str, set[str]]:
        """Build reverse dependency map: file → files that import it."""
        reverse: dict[str, set[str]] = {}
        for src, deps in self.edges.items():
            for dep in deps:
                reverse.setdefault(dep, set()).add(src)
        return reverse

    def _estimate_tokens(self, rel_path: str) -> int:
        """Estimate token count for a file (~50 tokens per line)."""
        full = os.path.join(self.root, rel_path)
        try:
            with open(full) as f:
                return sum(50 for _ in f)
        except (FileNotFoundError, OSError):
            return 0
