"""Research telemetry — track quality metrics across sessions.

Monitors novelty trajectories, source diversity, cost per quality point,
and detects improvement signals (premature convergence, budget exhaustion).
"""

from __future__ import annotations

import json
import os
import time
from dataclasses import asdict, dataclass, field


@dataclass
class SearchMetric:
    """Metrics for a single search operation."""
    query: str
    findings_count: int
    novelty_score: float
    timestamp: float = field(default_factory=time.time)


@dataclass
class SessionMetrics:
    """Aggregated metrics for a research session."""
    session_id: str
    topic: str
    total_searches: int = 0
    total_findings: int = 0
    unique_sources: int = 0
    novelty_trajectory: list[float] = field(default_factory=list)
    final_quality_score: float = 0.0
    convergence_reason: str = ""
    duration_seconds: float = 0.0
    search_metrics: list[SearchMetric] = field(default_factory=list)

    @property
    def avg_novelty(self) -> float:
        if not self.novelty_trajectory:
            return 0.0
        return sum(self.novelty_trajectory) / len(self.novelty_trajectory)

    @property
    def novelty_declining(self) -> bool:
        """Check if novelty is declining over time (good sign)."""
        if len(self.novelty_trajectory) < 3:
            return False
        first_half = self.novelty_trajectory[:len(self.novelty_trajectory) // 2]
        second_half = self.novelty_trajectory[len(self.novelty_trajectory) // 2:]
        return (sum(second_half) / len(second_half)) < (sum(first_half) / len(first_half))


@dataclass
class ImprovementSignal:
    """A detected improvement opportunity."""
    signal_type: str  # premature_convergence, budget_exhausted, low_diversity, stale_sources
    description: str
    severity: str = "medium"


def detect_signals(metrics: SessionMetrics) -> list[ImprovementSignal]:
    """Detect improvement signals from session metrics.

    Identifies:
    - Premature convergence (converged too early)
    - Budget exhaustion (ran out of budget before convergence)
    - Low source diversity
    - Non-declining novelty (may be searching wrong terms)
    """
    signals = []

    # Premature convergence: very few searches but marked as converged
    if metrics.convergence_reason == "natural_convergence" and metrics.total_searches < 5:
        signals.append(ImprovementSignal(
            signal_type="premature_convergence",
            description=f"Converged after only {metrics.total_searches} searches — may need more exploration",
            severity="high",
        ))

    # Budget exhausted
    if metrics.convergence_reason == "budget_exhausted":
        signals.append(ImprovementSignal(
            signal_type="budget_exhausted",
            description="Hit search budget without natural convergence — consider increasing budget",
            severity="medium",
        ))

    # Low source diversity
    if metrics.total_searches > 0 and metrics.unique_sources < 3:
        signals.append(ImprovementSignal(
            signal_type="low_diversity",
            description=f"Only {metrics.unique_sources} unique sources across {metrics.total_searches} searches",
            severity="medium",
        ))

    # Non-declining novelty
    if metrics.novelty_trajectory and not metrics.novelty_declining and len(metrics.novelty_trajectory) >= 5:
        signals.append(ImprovementSignal(
            signal_type="non_declining_novelty",
            description="Novelty is not declining — search queries may be too varied",
            severity="low",
        ))

    return signals


def save_telemetry(metrics: SessionMetrics, telemetry_dir: str) -> str:
    """Save session telemetry to disk."""
    os.makedirs(telemetry_dir, exist_ok=True)
    path = os.path.join(telemetry_dir, f"{metrics.session_id}.json")

    data = asdict(metrics)
    tmp_path = path + ".tmp"
    with open(tmp_path, "w") as f:
        json.dump(data, f, indent=2)
    os.replace(tmp_path, path)
    return path


def load_telemetry(telemetry_path: str) -> SessionMetrics | None:
    """Load session telemetry from disk."""
    try:
        with open(telemetry_path) as f:
            data = json.load(f)
        search_metrics = [SearchMetric(**sm) for sm in data.pop("search_metrics", [])]
        metrics = SessionMetrics(**data)
        metrics.search_metrics = search_metrics
        return metrics
    except (FileNotFoundError, json.JSONDecodeError, TypeError):
        return None
