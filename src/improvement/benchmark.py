"""Performance benchmark framework — track operations over time.

Defines benchmarks, runs them, stores results, detects regressions.
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import asdict, dataclass
from pathlib import Path


@dataclass
class BenchmarkResult:
    name: str
    duration_ms: float
    timestamp: float
    commit: str = ""
    metadata: dict | None = None


@dataclass
class RegressionAlert:
    benchmark: str
    baseline_ms: float
    current_ms: float
    regression_pct: float
    threshold_pct: float


def run_benchmark(name: str, fn: callable, iterations: int = 5) -> BenchmarkResult:
    """Run a benchmark function multiple times and return median duration."""
    durations = []
    for _ in range(iterations):
        start = time.perf_counter()
        fn()
        elapsed = (time.perf_counter() - start) * 1000  # ms
        durations.append(elapsed)

    durations.sort()
    median = durations[len(durations) // 2]

    return BenchmarkResult(
        name=name,
        duration_ms=round(median, 3),
        timestamp=time.time(),
    )


def save_result(result: BenchmarkResult, results_dir: str) -> str:
    """Append benchmark result to JSONL file."""
    os.makedirs(results_dir, exist_ok=True)
    filepath = os.path.join(results_dir, f"{result.name}.jsonl")
    with open(filepath, "a") as f:
        f.write(json.dumps(asdict(result)) + "\n")
    return filepath


def load_history(name: str, results_dir: str, max_entries: int = 100) -> list[BenchmarkResult]:
    """Load benchmark history from JSONL file."""
    filepath = os.path.join(results_dir, f"{name}.jsonl")
    if not os.path.exists(filepath):
        return []

    results = []
    with open(filepath) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                data = json.loads(line)
                results.append(BenchmarkResult(**{
                    k: v for k, v in data.items() if k in BenchmarkResult.__dataclass_fields__
                }))
            except (json.JSONDecodeError, TypeError):
                continue

    return results[-max_entries:]


def detect_regression(
    current: BenchmarkResult,
    history: list[BenchmarkResult],
    threshold_pct: float = 10.0,
    min_history: int = 3,
) -> RegressionAlert | None:
    """Detect if current result is a regression vs historical baseline.

    Uses median of historical results as baseline.
    """
    if len(history) < min_history:
        return None

    durations = sorted(r.duration_ms for r in history)
    baseline = durations[len(durations) // 2]

    if baseline == 0:
        return None

    regression_pct = ((current.duration_ms - baseline) / baseline) * 100

    if regression_pct > threshold_pct:
        return RegressionAlert(
            benchmark=current.name,
            baseline_ms=baseline,
            current_ms=current.duration_ms,
            regression_pct=round(regression_pct, 1),
            threshold_pct=threshold_pct,
        )

    return None
