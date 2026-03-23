"""Tests for benchmark framework."""

import time
from src.improvement.benchmark import (
    BenchmarkResult,
    RegressionAlert,
    detect_regression,
    load_history,
    run_benchmark,
    save_result,
)


def test_run_benchmark():
    result = run_benchmark("test_op", lambda: time.sleep(0.001), iterations=3)
    assert result.name == "test_op"
    assert result.duration_ms > 0
    assert result.timestamp > 0


def test_save_and_load(tmp_path):
    results_dir = str(tmp_path / "benchmarks")
    result = BenchmarkResult(name="op1", duration_ms=10.5, timestamp=time.time())
    save_result(result, results_dir)

    history = load_history("op1", results_dir)
    assert len(history) == 1
    assert history[0].duration_ms == 10.5


def test_save_multiple(tmp_path):
    results_dir = str(tmp_path / "benchmarks")
    for i in range(5):
        save_result(
            BenchmarkResult(name="op", duration_ms=float(i), timestamp=time.time()),
            results_dir,
        )
    history = load_history("op", results_dir)
    assert len(history) == 5


def test_load_missing(tmp_path):
    assert load_history("missing", str(tmp_path)) == []


def test_load_corrupt(tmp_path):
    results_dir = str(tmp_path / "benchmarks")
    import os
    os.makedirs(results_dir)
    with open(os.path.join(results_dir, "bad.jsonl"), "w") as f:
        f.write("not json\n")
        f.write("\n")  # empty line
        f.write('{"name":"ok","duration_ms":1.0,"timestamp":1.0}\n')
    history = load_history("bad", results_dir)
    assert len(history) == 1


def test_detect_no_regression():
    current = BenchmarkResult(name="op", duration_ms=10.0, timestamp=1.0)
    history = [
        BenchmarkResult(name="op", duration_ms=9.0, timestamp=0.1),
        BenchmarkResult(name="op", duration_ms=10.0, timestamp=0.2),
        BenchmarkResult(name="op", duration_ms=11.0, timestamp=0.3),
    ]
    alert = detect_regression(current, history, threshold_pct=10.0)
    assert alert is None


def test_detect_regression():
    current = BenchmarkResult(name="op", duration_ms=20.0, timestamp=1.0)
    history = [
        BenchmarkResult(name="op", duration_ms=10.0, timestamp=0.1),
        BenchmarkResult(name="op", duration_ms=10.0, timestamp=0.2),
        BenchmarkResult(name="op", duration_ms=10.0, timestamp=0.3),
    ]
    alert = detect_regression(current, history, threshold_pct=10.0)
    assert alert is not None
    assert alert.regression_pct == 100.0


def test_detect_insufficient_history():
    current = BenchmarkResult(name="op", duration_ms=100.0, timestamp=1.0)
    history = [BenchmarkResult(name="op", duration_ms=10.0, timestamp=0.1)]
    assert detect_regression(current, history) is None


def test_detect_zero_baseline():
    current = BenchmarkResult(name="op", duration_ms=10.0, timestamp=1.0)
    history = [
        BenchmarkResult(name="op", duration_ms=0.0, timestamp=0.1),
        BenchmarkResult(name="op", duration_ms=0.0, timestamp=0.2),
        BenchmarkResult(name="op", duration_ms=0.0, timestamp=0.3),
    ]
    assert detect_regression(current, history) is None
