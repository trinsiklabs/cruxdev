"""Tests for doc-code drift detection."""

import os
import pytest
from src.improvement.doc_drift import (
    check_function_references,
    check_path_references,
    find_code_references,
    scan_doc,
    scan_docs_dir,
)


def test_find_code_references():
    content = "Use `src/main.py` and call `validate()` for checking."
    refs = find_code_references(content)
    assert ("src/main.py" in [r[1] for r in refs])
    assert ("validate()" in [r[1] for r in refs])


def test_find_code_references_none():
    refs = find_code_references("No code references here.")
    assert refs == []


def test_check_path_refs_exist(tmp_path):
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "main.py").write_text("x = 1")
    content = "See `src/main.py` for details."
    findings = check_path_references("doc.md", content, str(tmp_path))
    assert findings == []


def test_check_path_refs_missing(tmp_path):
    content = "See `src/deleted.py` for details."
    findings = check_path_references("doc.md", content, str(tmp_path))
    assert len(findings) == 1
    assert "deleted.py" in findings[0].description


def test_check_function_refs_exist(tmp_path):
    (tmp_path / "mod.py").write_text("def validate(): pass")
    content = "Call `validate()` to check."
    findings = check_function_references("doc.md", content, str(tmp_path))
    assert findings == []


def test_check_function_refs_missing(tmp_path):
    (tmp_path / "mod.py").write_text("def other(): pass")
    content = "Call `deleted_func()` to check."
    findings = check_function_references("doc.md", content, str(tmp_path))
    assert len(findings) == 1
    assert "deleted_func" in findings[0].description


def test_scan_doc(tmp_path):
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "exists.py").write_text("def real(): pass")

    doc = tmp_path / "README.md"
    doc.write_text("Use `src/exists.py` and `src/gone.py`. Call `real()` and `fake()`.")
    findings = scan_doc(str(doc), str(tmp_path))
    # gone.py → path drift, fake() → function drift
    assert len(findings) >= 2


def test_scan_doc_missing():
    findings = scan_doc("/nonexistent.md", ".")
    assert findings == []


def test_check_function_refs_syntax_error(tmp_path):
    (tmp_path / "broken.py").write_text("def broken(\n")
    content = "Call `mystery()` to check."
    findings = check_function_references("doc.md", content, str(tmp_path))
    assert len(findings) >= 1


def test_scan_docs_dir(tmp_path):
    docs = tmp_path / "docs"
    docs.mkdir()
    (docs / "good.md").write_text("No code refs.")
    (docs / "bad.md").write_text("See `missing.py` for details.")
    (docs / "skip.txt").write_text("Not markdown")

    findings = scan_docs_dir(str(docs), str(tmp_path))
    assert len(findings) >= 1


def test_scan_docs_dir_missing(tmp_path):
    assert scan_docs_dir(str(tmp_path / "nope"), str(tmp_path)) == []
