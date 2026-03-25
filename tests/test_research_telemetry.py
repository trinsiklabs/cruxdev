"""Tests for research telemetry module."""

import os

from src.research.telemetry import (
    ImprovementSignal,
    SearchMetric,
    SessionMetrics,
    detect_signals,
    load_telemetry,
    save_telemetry,
)


class TestSessionMetrics:
    def test_avg_novelty(self):
        m = SessionMetrics("s1", "topic", novelty_trajectory=[0.8, 0.4, 0.2])
        assert abs(m.avg_novelty - 0.4667) < 0.01

    def test_avg_novelty_empty(self):
        m = SessionMetrics("s1", "topic")
        assert m.avg_novelty == 0.0

    def test_novelty_declining_true(self):
        m = SessionMetrics("s1", "t", novelty_trajectory=[0.9, 0.8, 0.7, 0.3, 0.2, 0.1])
        assert m.novelty_declining is True

    def test_novelty_declining_false(self):
        m = SessionMetrics("s1", "t", novelty_trajectory=[0.1, 0.2, 0.3, 0.8, 0.9])
        assert m.novelty_declining is False

    def test_novelty_declining_too_few(self):
        m = SessionMetrics("s1", "t", novelty_trajectory=[0.5, 0.3])
        assert m.novelty_declining is False


class TestDetectSignals:
    def test_premature_convergence(self):
        m = SessionMetrics("s1", "t", total_searches=3, convergence_reason="natural_convergence")
        signals = detect_signals(m)
        assert any(s.signal_type == "premature_convergence" for s in signals)

    def test_budget_exhausted(self):
        m = SessionMetrics("s1", "t", convergence_reason="budget_exhausted")
        signals = detect_signals(m)
        assert any(s.signal_type == "budget_exhausted" for s in signals)

    def test_low_diversity(self):
        m = SessionMetrics("s1", "t", total_searches=10, unique_sources=2)
        signals = detect_signals(m)
        assert any(s.signal_type == "low_diversity" for s in signals)

    def test_non_declining_novelty(self):
        m = SessionMetrics("s1", "t", novelty_trajectory=[0.1, 0.2, 0.3, 0.4, 0.5])
        signals = detect_signals(m)
        assert any(s.signal_type == "non_declining_novelty" for s in signals)

    def test_no_signals_healthy(self):
        m = SessionMetrics(
            "s1", "t",
            total_searches=20,
            unique_sources=10,
            convergence_reason="natural_convergence",
            novelty_trajectory=[0.9, 0.7, 0.5, 0.3, 0.1, 0.05],
        )
        signals = detect_signals(m)
        assert len(signals) == 0


class TestSaveLoadTelemetry:
    def test_save_and_load(self, tmp_path):
        metrics = SessionMetrics(
            session_id="s1",
            topic="test topic",
            total_searches=5,
            total_findings=10,
            unique_sources=3,
            novelty_trajectory=[0.8, 0.5, 0.2],
            final_quality_score=0.85,
            convergence_reason="natural_convergence",
            duration_seconds=120.0,
            search_metrics=[
                SearchMetric(query="test query", findings_count=3, novelty_score=0.8),
            ],
        )
        path = save_telemetry(metrics, str(tmp_path / "telemetry"))
        assert os.path.exists(path)

        loaded = load_telemetry(path)
        assert loaded is not None
        assert loaded.session_id == "s1"
        assert loaded.topic == "test topic"
        assert loaded.total_searches == 5
        assert len(loaded.search_metrics) == 1
        assert loaded.search_metrics[0].query == "test query"

    def test_load_nonexistent(self):
        assert load_telemetry("/nonexistent/path.json") is None

    def test_load_invalid_json(self, tmp_path):
        path = tmp_path / "bad.json"
        path.write_text("not json")
        assert load_telemetry(str(path)) is None

    def test_atomic_write(self, tmp_path):
        metrics = SessionMetrics("s1", "t")
        save_telemetry(metrics, str(tmp_path / "telemetry"))
        files = os.listdir(tmp_path / "telemetry")
        assert not any(f.endswith(".tmp") for f in files)
