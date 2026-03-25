"""Tests for competitor discovery module."""

from src.competitors.discovery import (
    DiscoveredCompetitor,
    DiscoveryResult,
    generate_discovery_queries,
    parse_discovery_response,
)


class TestGenerateDiscoveryQueries:
    def test_generates_queries_from_category(self):
        queries = generate_discovery_queries("AI coding tool", "AI coding")
        assert any("AI coding" in q for q in queries)
        assert len(queries) > 0

    def test_includes_alternatives_query(self):
        queries = generate_discovery_queries("desc", "testing")
        assert any("alternatives" in q for q in queries)

    def test_includes_comparison_query(self):
        queries = generate_discovery_queries("desc", "testing")
        assert any("comparison" in q for q in queries)

    def test_includes_best_query(self):
        queries = generate_discovery_queries("desc", "testing")
        assert any("best" in q for q in queries)

    def test_includes_open_source_query(self):
        queries = generate_discovery_queries("desc", "testing")
        assert any("open source" in q for q in queries)

    def test_includes_feature_based_query(self):
        queries = generate_discovery_queries("convergence driven development", "testing")
        assert any("convergence" in q for q in queries)

    def test_max_queries_limit(self):
        queries = generate_discovery_queries("long description with many words here", "testing", max_queries=3)
        assert len(queries) <= 3

    def test_filters_short_words(self):
        queries = generate_discovery_queries("a the is of", "testing")
        # Short words should be filtered, so no feature-based queries
        assert all("testing" in q or "best" in q or "open source" in q for q in queries)


class TestParseDiscoveryResponse:
    def test_parse_name_url_description_format(self):
        response = """Name: CompetitorA
URL: https://competitora.com
Description: A great tool for testing

Name: CompetitorB
URL: https://competitorb.io
Description: Another testing tool"""

        results = parse_discovery_response(response, "test query")
        assert len(results) == 2
        assert results[0].name == "CompetitorA"
        assert results[0].url == "https://competitora.com"
        assert results[0].description == "A great tool for testing"
        assert results[1].name == "CompetitorB"
        assert results[1].url == "https://competitorb.io"

    def test_parse_list_format(self):
        response = """- ToolA: A coding assistant
- ToolB: A testing framework"""

        results = parse_discovery_response(response, "query")
        assert len(results) == 2
        assert results[0].name == "ToolA"
        assert results[0].description == "A coding assistant"
        assert results[0].confidence == 0.3  # lower confidence for list format

    def test_source_query_preserved(self):
        response = "Name: Test\nURL: https://test.com\nDescription: test"
        results = parse_discovery_response(response, "my query")
        assert results[0].source_query == "my query"

    def test_empty_response(self):
        results = parse_discovery_response("", "query")
        assert len(results) == 0

    def test_url_without_scheme(self):
        response = "Name: Test\nURL: test.com\nDescription: test"
        results = parse_discovery_response(response, "q")
        assert len(results) == 1
        # Should handle gracefully
        assert results[0].url == "test.com"

    def test_missing_url_generates_default(self):
        response = "Name: My Tool\nDescription: great\n"
        results = parse_discovery_response(response, "q")
        assert results[0].url == "https://mytool.com"

    def test_multiple_name_entries_flush_previous(self):
        response = "Name: First\nURL: https://first.com\nDescription: one\nName: Second\nURL: https://second.com\nDescription: two"
        results = parse_discovery_response(response, "q")
        assert len(results) == 2
        assert results[0].name == "First"
        assert results[1].name == "Second"

    def test_url_with_double_slash_prefix(self):
        response = "Name: Test\nURL: //example.com/path\nDescription: test"
        results = parse_discovery_response(response, "q")
        assert results[0].url == "https://example.com/path"


class TestDiscoveryResult:
    def test_deduplicated_by_url(self):
        result = DiscoveryResult(competitors=[
            DiscoveredCompetitor("A", "https://a.com", "first", "q1", 0.8),
            DiscoveredCompetitor("A copy", "https://a.com/", "second", "q2", 0.3),
            DiscoveredCompetitor("B", "https://b.com", "third", "q1", 0.5),
        ])
        deduped = result.deduplicated()
        assert len(deduped) == 2
        # Higher confidence kept
        assert deduped[0].name == "A"
        assert deduped[0].confidence == 0.8

    def test_deduplicated_sorted_by_confidence(self):
        result = DiscoveryResult(competitors=[
            DiscoveredCompetitor("Low", "https://low.com", "", "q", 0.2),
            DiscoveredCompetitor("High", "https://high.com", "", "q", 0.9),
        ])
        deduped = result.deduplicated()
        assert deduped[0].name == "High"
        assert deduped[1].name == "Low"

    def test_empty_result(self):
        result = DiscoveryResult()
        assert result.deduplicated() == []
        assert result.search_count == 0
