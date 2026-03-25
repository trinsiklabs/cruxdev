"""Tests for competitor gap analysis module."""

from src.competitors.gap_analysis import (
    FeatureGap,
    FeatureMatrixEntry,
    GapAnalysisResult,
    build_feature_matrix,
    classify_gaps,
    run_gap_analysis,
)
from src.competitors.research import CompetitorProfile, Feature


class TestBuildFeatureMatrix:
    def test_our_features_marked(self):
        matrix = build_feature_matrix("Us", ["A", "B"], [])
        assert len(matrix) == 2
        assert all(e.our_status for e in matrix)

    def test_competitor_features_added(self):
        profile = CompetitorProfile(
            name="Comp", url="https://comp.com",
            features=[Feature("C", "desc", True)],
        )
        matrix = build_feature_matrix("Us", ["A"], [profile])
        assert len(matrix) == 2
        c_entry = next(e for e in matrix if e.feature.lower() == "c")
        assert c_entry.our_status is False
        assert c_entry.competitor_status["Comp"] is True

    def test_overlapping_features(self):
        profile = CompetitorProfile(
            name="Comp", url="https://comp.com",
            features=[Feature("a", "desc", True)],  # same as ours (case-insensitive)
        )
        matrix = build_feature_matrix("Us", ["A"], [profile])
        assert len(matrix) == 1  # merged
        entry = matrix[0]
        assert entry.our_status is True
        assert entry.competitor_status["Comp"] is True

    def test_multiple_competitors(self):
        profiles = [
            CompetitorProfile(name="A", url="u", features=[Feature("X", "", True)]),
            CompetitorProfile(name="B", url="u", features=[Feature("X", "", True), Feature("Y", "", True)]),
        ]
        matrix = build_feature_matrix("Us", [], profiles)
        x_entry = next(e for e in matrix if e.feature.lower() == "x")
        assert x_entry.competitor_status["A"] is True
        assert x_entry.competitor_status["B"] is True

    def test_sorted_by_feature_name(self):
        profiles = [CompetitorProfile(name="C", url="u", features=[Feature("Z", "", True), Feature("A", "", True)])]
        matrix = build_feature_matrix("Us", ["M"], profiles)
        names = [e.feature.lower() for e in matrix]
        assert names == sorted(names)


class TestClassifyGaps:
    def test_must_close_two_official(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=False,
            competitor_status={"A": True, "B": True},
        )]
        gaps = classify_gaps(matrix, ["A", "B"])
        assert len(gaps) == 1
        assert gaps[0].priority == "must-close"

    def test_should_close_one_official(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=False,
            competitor_status={"A": True, "C": True},
        )]
        gaps = classify_gaps(matrix, ["A"])
        assert gaps[0].priority == "should-close"

    def test_nice_to_have_no_official(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=False,
            competitor_status={"C": True},
        )]
        gaps = classify_gaps(matrix, ["A", "B"])
        assert gaps[0].priority == "nice-to-have"

    def test_no_gap_when_we_have_feature(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=True,
            competitor_status={"A": True},
        )]
        gaps = classify_gaps(matrix, ["A"])
        assert len(gaps) == 0

    def test_no_gap_when_nobody_has_feature(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=False,
            competitor_status={"A": False},
        )]
        gaps = classify_gaps(matrix, ["A"])
        assert len(gaps) == 0

    def test_competitors_with_feature_populated(self):
        matrix = [FeatureMatrixEntry(
            feature="X", our_status=False,
            competitor_status={"A": True, "B": False, "C": True},
        )]
        gaps = classify_gaps(matrix, ["A", "C"])
        assert set(gaps[0].competitors_with_feature) == {"A", "C"}


class TestGapAnalysisResult:
    def test_must_close_filter(self):
        result = GapAnalysisResult(our_name="Us", gaps=[
            FeatureGap("A", ["X"], "must-close"),
            FeatureGap("B", ["Y"], "should-close"),
            FeatureGap("C", ["Z"], "must-close"),
        ])
        assert len(result.must_close) == 2

    def test_should_close_filter(self):
        result = GapAnalysisResult(our_name="Us", gaps=[
            FeatureGap("A", ["X"], "should-close"),
        ])
        assert len(result.should_close) == 1

    def test_open_gaps_filter(self):
        result = GapAnalysisResult(our_name="Us", gaps=[
            FeatureGap("A", ["X"], "must-close", status="open"),
            FeatureGap("B", ["Y"], "must-close", status="closed"),
        ])
        assert len(result.open_gaps) == 1

    def test_to_markdown(self):
        result = GapAnalysisResult(
            our_name="Us",
            feature_matrix=[
                FeatureMatrixEntry("F1", True, {"Comp": True}),
                FeatureMatrixEntry("F2", False, {"Comp": True}),
            ],
            gaps=[
                FeatureGap("F2", ["Comp"], "must-close", rationale="Important feature"),
            ],
        )
        md = result.to_markdown()
        assert "## Gap Analysis" in md
        assert "| F1 | Y | Y |" in md
        assert "| F2 | N | Y |" in md
        assert "Must-Close" in md
        assert "**F2**" in md
        assert "Important feature" in md

    def test_to_markdown_empty(self):
        result = GapAnalysisResult(our_name="Us")
        md = result.to_markdown()
        assert "## Gap Analysis" in md


class TestRunGapAnalysis:
    def test_full_analysis(self):
        profiles = [
            CompetitorProfile(name="A", url="u", features=[
                Feature("shared", "", True),
                Feature("unique_a", "", True),
            ]),
            CompetitorProfile(name="B", url="u", features=[
                Feature("shared", "", True),
                Feature("unique_b", "", True),
            ]),
        ]
        result = run_gap_analysis("Us", ["shared", "our_only"], profiles)
        assert result.our_name == "Us"
        assert len(result.feature_matrix) > 0
        # "shared" should not be a gap (we have it)
        gap_names = [g.feature_name for g in result.gaps]
        assert "shared" not in gap_names

    def test_defaults_all_as_official(self):
        profiles = [
            CompetitorProfile(name="A", url="u", features=[Feature("X", "", True)]),
            CompetitorProfile(name="B", url="u", features=[Feature("X", "", True)]),
        ]
        result = run_gap_analysis("Us", [], profiles)
        # Both are official by default, X should be must-close
        x_gap = next(g for g in result.gaps if g.feature_name.lower() == "x")
        assert x_gap.priority == "must-close"

    def test_custom_official_list(self):
        profiles = [
            CompetitorProfile(name="A", url="u", features=[Feature("X", "", True)]),
            CompetitorProfile(name="B", url="u", features=[Feature("X", "", True)]),
        ]
        result = run_gap_analysis("Us", [], profiles, official_competitors=["A"])
        x_gap = next(g for g in result.gaps if g.feature_name.lower() == "x")
        assert x_gap.priority == "should-close"  # only 1 official has it
