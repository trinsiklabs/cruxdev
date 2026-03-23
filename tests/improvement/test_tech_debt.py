"""Tests for technical debt detection."""

import pytest
from src.improvement.tech_debt import (
    DebtFinding,
    calculate_complexity,
    find_anti_patterns,
    find_complex_functions,
    scan_project,
)
import ast


def test_complexity_simple():
    code = "def f(): pass"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef))
    assert calculate_complexity(func) == 1


def test_complexity_with_if():
    code = "def f(x):\n  if x: pass"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef))
    assert calculate_complexity(func) == 2


def test_complexity_with_loop_and_except():
    code = "def f():\n  for x in y:\n    try: pass\n    except: pass"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef))
    assert calculate_complexity(func) >= 3


def test_complexity_bool_op():
    code = "def f(a,b,c):\n  if a and b or c: pass"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef))
    assert calculate_complexity(func) >= 3


def test_find_complex_functions(tmp_path):
    f = tmp_path / "complex.py"
    f.write_text("def simple(): pass\ndef complex_fn(x):\n" +
                 "\n".join(f"  if x == {i}: pass" for i in range(15)))
    findings = find_complex_functions(str(f), threshold=10)
    assert len(findings) == 1
    assert "complex_fn" in findings[0].description


def test_find_complex_functions_below_threshold(tmp_path):
    f = tmp_path / "simple.py"
    f.write_text("def f(): pass")
    assert find_complex_functions(str(f)) == []


def test_find_complex_functions_syntax_error(tmp_path):
    f = tmp_path / "bad.py"
    f.write_text("def broken(\n")
    assert find_complex_functions(str(f)) == []


def test_find_anti_patterns_bare_except(tmp_path):
    f = tmp_path / "anti.py"
    f.write_text("try:\n  pass\nexcept:\n  pass\n")
    findings = find_anti_patterns(str(f))
    assert len(findings) == 1
    assert findings[0].category == "anti_pattern"
    assert "Bare" in findings[0].description


def test_complexity_with_assert_and_with():
    code = "def f():\n  with open('x'):\n    assert True\n"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.FunctionDef))
    assert calculate_complexity(func) >= 3


def test_complexity_async():
    code = "async def f():\n  async for x in y: pass\n  async with z: pass\n"
    tree = ast.parse(code)
    func = next(n for n in ast.walk(tree) if isinstance(n, ast.AsyncFunctionDef))
    assert calculate_complexity(func) >= 3


def test_find_complex_high_severity(tmp_path):
    f = tmp_path / "very_complex.py"
    f.write_text("def mega(x):\n" + "\n".join(f"  if x == {i}: pass" for i in range(25)))
    findings = find_complex_functions(str(f), threshold=10)
    assert findings[0].severity == "high"


def test_find_anti_patterns_var_tmp(tmp_path):
    f = tmp_path / "vartmp.py"
    f.write_text('path = "/var/tmp/myfile"\n')
    findings = find_anti_patterns(str(f))
    assert len(findings) == 1


def test_find_anti_patterns_missing_file():
    assert find_anti_patterns("/nonexistent.py") == []


def test_find_anti_patterns_hardcoded_tmp(tmp_path):
    f = tmp_path / "tmp.py"
    f.write_text('path = "/tmp/myfile.txt"\n')
    findings = find_anti_patterns(str(f))
    assert len(findings) == 1
    assert "tmp" in findings[0].description.lower()


def test_find_anti_patterns_clean(tmp_path):
    f = tmp_path / "clean.py"
    f.write_text("try:\n  pass\nexcept ValueError:\n  pass\n")
    assert find_anti_patterns(str(f)) == []


def test_scan_project(tmp_path):
    src = tmp_path / "src"
    src.mkdir()
    (src / "clean.py").write_text("def f(): pass")
    (src / "dirty.py").write_text("try:\n  pass\nexcept:\n  pass\n")
    (src / "__pycache__").mkdir()
    (src / "__pycache__" / "cached.py").write_text("x")

    # Add a non-.py file that should be skipped
    (src / "README.md").write_text("not python")

    findings = scan_project(str(tmp_path))
    assert len(findings) >= 1
    # __pycache__ should be skipped
    assert all("__pycache__" not in f.file for f in findings)
    # Non-.py files should be skipped
    assert all("README" not in f.file for f in findings)
