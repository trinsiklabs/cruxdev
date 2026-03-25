"""Tests for source verification module."""

from src.research.verify import (
    SourceCheck,
    VerifyResult,
    check_source_recency,
    extract_urls,
    verify_sources,
)


class TestExtractUrls:
    def test_extracts_urls(self):
        urls = extract_urls("See https://example.com and http://test.io/page")
        assert len(urls) == 2

    def test_deduplicates(self):
        urls = extract_urls("https://a.com https://a.com")
        assert len(urls) == 1

    def test_no_urls(self):
        assert extract_urls("no links") == []


class TestCheckSourceRecency:
    def test_recent_iso_date(self):
        assert check_source_recency("2026-01-01") is True

    def test_old_iso_date(self):
        assert check_source_recency("2020-01-01", max_age_days=365) is False

    def test_recent_year(self):
        assert check_source_recency("2025") is True

    def test_old_year(self):
        assert check_source_recency("2015", max_age_days=365) is False

    def test_invalid_date(self):
        # Can't parse → assume ok
        assert check_source_recency("not-a-date") is True

    def test_empty_string(self):
        assert check_source_recency("") is True


class TestVerifyResult:
    def test_all_reachable(self):
        result = VerifyResult("f1", [
            SourceCheck("https://a.com", reachable=True),
            SourceCheck("https://b.com", reachable=True),
        ])
        assert result.all_reachable is True
        assert result.reachable_count == 2

    def test_some_unreachable(self):
        result = VerifyResult("f1", [
            SourceCheck("https://a.com", reachable=True),
            SourceCheck("https://b.com", reachable=False),
        ])
        assert result.all_reachable is False
        assert result.reachable_count == 1


class TestVerifySources:
    def test_with_checker(self):
        result = verify_sources(
            "f1",
            ["https://good.com", "https://bad.com"],
            check_fn=lambda url: "good" in url,
        )
        assert len(result.sources_checked) == 2
        assert result.sources_checked[0].reachable is True
        assert result.sources_checked[1].reachable is False
        assert result.overall_verified is False  # not all reachable

    def test_all_reachable(self):
        result = verify_sources(
            "f1",
            ["https://a.com", "https://b.com"],
            check_fn=lambda url: True,
        )
        assert result.overall_verified is True

    def test_no_sources(self):
        result = verify_sources("f1", [])
        assert result.overall_verified is False

    def test_default_check(self):
        result = verify_sources("f1", ["https://a.com"])
        assert result.sources_checked[0].reachable is True  # default = True
        assert result.overall_verified is True
