"""Tests for test quality analysis."""

import os
import pytest
from src.improvement.test_quality import (
    find_duplicate_test_names,
    find_tests_without_assertions,
    find_weak_assertions,
    scan_test_quality,
)


def test_weak_assertion_assertTrue(tmp_path):
    f = tmp_path / "test_example.py"
    f.write_text("def test_foo(self):\n  self.assertTrue(True)\n")
    findings = find_weak_assertions(str(f))
    assert len(findings) == 1
    assert "assertTrue" in findings[0].description


def test_weak_assertion_clean(tmp_path):
    f = tmp_path / "test_clean.py"
    f.write_text("def test_foo():\n  assert x == 42\n")
    findings = find_weak_assertions(str(f))
    assert findings == []


def test_no_assertion_detected(tmp_path):
    f = tmp_path / "test_empty.py"
    f.write_text("def test_nothing():\n  x = 1 + 1\n  print(x)\n")
    findings = find_tests_without_assertions(str(f))
    assert len(findings) == 1
    assert "no assertions" in findings[0].description.lower()


def test_assertion_present(tmp_path):
    f = tmp_path / "test_good.py"
    f.write_text("def test_good():\n  assert 1 == 1\n")
    findings = find_tests_without_assertions(str(f))
    assert findings == []


def test_pytest_raises_counts(tmp_path):
    f = tmp_path / "test_raises.py"
    f.write_text("def test_raises():\n  pytest.raises(ValueError)\n")
    findings = find_tests_without_assertions(str(f))
    assert findings == []


def test_duplicate_names(tmp_path):
    (tmp_path / "test_a.py").write_text("def test_duplicate(): pass\n")
    (tmp_path / "test_b.py").write_text("def test_duplicate(): pass\n")
    findings = find_duplicate_test_names(str(tmp_path))
    assert len(findings) == 1
    assert "duplicate" in findings[0].description.lower()


def test_no_duplicates(tmp_path):
    (tmp_path / "test_a.py").write_text("def test_alpha(): pass\n")
    (tmp_path / "test_b.py").write_text("def test_beta(): pass\n")
    findings = find_duplicate_test_names(str(tmp_path))
    assert findings == []


def test_scan_test_quality(tmp_path):
    (tmp_path / "test_mixed.py").write_text(
        "def test_weak(self):\n  self.assertTrue(True)\n"
        "def test_empty():\n  x = 1\n"
        "def test_good():\n  assert x == 1\n"
    )
    findings = scan_test_quality(str(tmp_path))
    assert len(findings) >= 2  # weak assertion + no assertion


def test_assertIsNotNone_weak(tmp_path):
    f = tmp_path / "test_weak2.py"
    f.write_text("def test_foo(self):\n  self.assertIsNotNone(x)\n")
    findings = find_weak_assertions(str(f))
    assert len(findings) == 1


def test_non_test_function_skipped(tmp_path):
    f = tmp_path / "test_skip.py"
    f.write_text("def helper():\n  self.assertTrue(True)\ndef test_real():\n  assert True\n")
    findings = find_weak_assertions(str(f))
    assert findings == []


def test_assert_method_counts(tmp_path):
    f = tmp_path / "test_method.py"
    f.write_text("def test_with_assert_method():\n  self.assertEqual(1, 1)\n")
    findings = find_tests_without_assertions(str(f))
    assert findings == []


def test_duplicate_syntax_error(tmp_path):
    (tmp_path / "test_bad.py").write_text("def test_broken(\n")
    findings = find_duplicate_test_names(str(tmp_path))
    assert findings == []


def test_non_test_file_in_dir(tmp_path):
    (tmp_path / "helper.py").write_text("def helper(): pass\n")
    (tmp_path / "test_real.py").write_text("def test_ok():\n  assert True\n")
    findings = scan_test_quality(str(tmp_path))
    assert all("helper" not in f.file for f in findings)
    dup = find_duplicate_test_names(str(tmp_path))
    assert all("helper" not in f.file for f in dup)


def test_non_test_func_in_duplicate_check(tmp_path):
    (tmp_path / "test_helpers.py").write_text(
        "def helper(): pass\ndef test_actual():\n  assert True\n"
    )
    findings = find_duplicate_test_names(str(tmp_path))
    assert findings == []


def test_no_assert_skips_non_test(tmp_path):
    f = tmp_path / "test_mixed2.py"
    f.write_text("def setup():\n  x = 1\ndef test_real():\n  assert True\n")
    findings = find_tests_without_assertions(str(f))
    assert findings == []


def test_syntax_error_skipped(tmp_path):
    (tmp_path / "test_bad.py").write_text("def test_broken(\n")
    assert find_weak_assertions(str(tmp_path / "test_bad.py")) == []
    assert find_tests_without_assertions(str(tmp_path / "test_bad.py")) == []
