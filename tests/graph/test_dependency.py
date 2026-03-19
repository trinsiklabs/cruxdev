"""Tests for dependency graph — real Python files."""

import os

import pytest

from src.graph.dependency import DependencyGraph


def _create_project(tmp_path):
    """Create a small Python project for testing."""
    (tmp_path / "main.py").write_text("from lib import helper\nimport utils\n")
    (tmp_path / "lib").mkdir()
    (tmp_path / "lib" / "__init__.py").write_text("")
    (tmp_path / "lib" / "helper.py").write_text("from utils import do_thing\n")
    (tmp_path / "utils.py").write_text("def do_thing(): pass\n")
    (tmp_path / "standalone.py").write_text("print('hello')\n")
    return str(tmp_path)


# --- Graph construction ---


def test_build_finds_all_files(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()
    assert "main.py" in g.edges
    assert "utils.py" in g.edges
    assert "standalone.py" in g.edges
    assert os.path.join("lib", "helper.py") in g.edges
    assert os.path.join("lib", "__init__.py") in g.edges


def test_build_resolves_imports(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()
    # main.py imports from lib and utils
    main_deps = g.edges["main.py"]
    assert "utils.py" in main_deps


def test_build_skips_pycache(tmp_path):
    root = _create_project(tmp_path)
    pycache = tmp_path / "__pycache__"
    pycache.mkdir()
    (pycache / "cached.py").write_text("x = 1\n")
    g = DependencyGraph(root)
    g.build()
    assert "__pycache__/cached.py" not in g.edges


def test_build_handles_syntax_error(tmp_path):
    (tmp_path / "bad.py").write_text("def broken(\n")
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert g.edges.get("bad.py") == set()


def test_update_single_file(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    # Modify main.py to add a new import
    (tmp_path / "main.py").write_text("import standalone\n")
    g.update(os.path.join(root, "main.py"))
    assert "standalone.py" in g.edges["main.py"]


# --- Impact set ---


def test_impact_set_direct(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    # Changing utils.py affects files that import it
    affected = g.impact_set(["utils.py"])
    assert "utils.py" in affected


def test_impact_set_transitive(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    # If utils.py changes, lib/helper.py is affected (imports utils),
    # and main.py might be affected (imports lib)
    affected = g.impact_set(["utils.py"])
    assert "utils.py" in affected


def test_impact_set_no_deps(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    affected = g.impact_set(["standalone.py"])
    assert affected == {"standalone.py"}


# --- Context budget ---


def test_audit_context_within_budget(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    context = g.audit_context(["main.py"], token_budget=100000)
    assert "main.py" in context


def test_audit_context_limited_budget(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    # Very small budget — should only include target
    context = g.audit_context(["main.py"], token_budget=10)
    assert "main.py" in context


# --- Scope assignment ---


def test_assign_scopes_basic(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    scopes = g.assign_scopes(2)
    assert len(scopes) == 2
    all_files = scopes[0] | scopes[1]
    # All files should be assigned
    assert len(all_files) == len(g.edges)
    # No overlap
    assert len(scopes[0] & scopes[1]) == 0


def test_assign_scopes_single_agent(tmp_path):
    root = _create_project(tmp_path)
    g = DependencyGraph(root)
    g.build()

    scopes = g.assign_scopes(1)
    assert len(scopes) == 1
    assert len(scopes[0]) == len(g.edges)


def test_assign_scopes_more_agents_than_files(tmp_path):
    (tmp_path / "a.py").write_text("x = 1\n")
    g = DependencyGraph(str(tmp_path))
    g.build()

    scopes = g.assign_scopes(5)
    assert len(scopes) == 1  # Only 1 file, so only 1 non-empty scope


def test_assign_scopes_zero_agents(tmp_path):
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert g.assign_scopes(0) == []


def test_assign_scopes_empty_graph(tmp_path):
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert g.assign_scopes(2) == []


# --- Edge cases ---


def test_empty_project(tmp_path):
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert g.edges == {}


def test_resolve_nonexistent_module(tmp_path):
    (tmp_path / "a.py").write_text("import nonexistent_module\n")
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert g.edges["a.py"] == set()


def test_import_from(tmp_path):
    (tmp_path / "a.py").write_text("from b import something\n")
    (tmp_path / "b.py").write_text("something = 1\n")
    g = DependencyGraph(str(tmp_path))
    g.build()
    assert "b.py" in g.edges["a.py"]


def test_estimate_tokens_missing_file(tmp_path):
    g = DependencyGraph(str(tmp_path))
    assert g._estimate_tokens("nonexistent.py") == 0
