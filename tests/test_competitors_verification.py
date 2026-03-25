"""Tests for competitor verification module."""

from unittest.mock import patch, MagicMock
import urllib.error

from src.competitors.verification import (
    LinkCheckResult,
    VerificationResult,
    check_url,
    extract_urls,
    verify_profile_links,
)


class TestExtractUrls:
    def test_extract_https_urls(self):
        text = "Visit https://example.com and https://test.io/page for more."
        urls = extract_urls(text)
        assert "https://example.com" in urls
        assert "https://test.io/page" in urls

    def test_extract_http_urls(self):
        text = "Old site at http://legacy.com/docs"
        urls = extract_urls(text)
        assert "http://legacy.com/docs" in urls

    def test_deduplicates(self):
        text = "https://a.com and https://a.com again"
        urls = extract_urls(text)
        assert len(urls) == 1

    def test_no_urls(self):
        assert extract_urls("no links here") == []

    def test_urls_in_markdown(self):
        text = "[link](https://example.com) and **[other](https://other.com)**"
        urls = extract_urls(text)
        assert len(urls) == 2

    def test_url_with_path_and_query(self):
        text = "https://api.example.com/v2/docs?page=1"
        urls = extract_urls(text)
        assert "https://api.example.com/v2/docs?page=1" in urls


class TestVerificationResult:
    def test_all_links_ok(self):
        result = VerificationResult(
            competitor_name="Test",
            links_checked=[
                LinkCheckResult("https://a.com", "ok", 200),
                LinkCheckResult("https://b.com", "ok", 200),
            ]
        )
        assert result.all_links_ok is True
        assert len(result.broken_links) == 0

    def test_broken_links_detected(self):
        result = VerificationResult(
            competitor_name="Test",
            links_checked=[
                LinkCheckResult("https://a.com", "ok", 200),
                LinkCheckResult("https://broken.com", "broken", 404, "Not Found"),
            ]
        )
        assert result.all_links_ok is False
        assert len(result.broken_links) == 1
        assert result.broken_links[0].url == "https://broken.com"

    def test_to_dict(self):
        result = VerificationResult(
            competitor_name="Test",
            links_checked=[
                LinkCheckResult("https://a.com", "ok", 200),
                LinkCheckResult("https://b.com", "broken", 404),
            ],
            claims_verified=3,
            claims_unverified=1,
        )
        d = result.to_dict()
        assert d["competitor"] == "Test"
        assert d["links_total"] == 2
        assert d["links_ok"] == 1
        assert d["links_broken"] == 1
        assert d["claims_verified"] == 3
        assert d["claims_unverified"] == 1

    def test_empty_result(self):
        result = VerificationResult(competitor_name="Empty")
        assert result.all_links_ok is True
        assert len(result.broken_links) == 0


class TestVerifyProfileLinks:
    def test_with_mock_checker(self):
        def mock_check(url):
            if "broken" in url:
                return LinkCheckResult(url, "broken", 404, "Not Found")
            return LinkCheckResult(url, "ok", 200)

        md = "Visit https://good.com and https://broken.com/page"
        result = verify_profile_links("TestComp", md, check_fn=mock_check)
        assert result.competitor_name == "TestComp"
        assert len(result.links_checked) == 2
        assert result.links_checked[0].status == "ok"
        assert result.links_checked[1].status == "broken"

    def test_no_urls_in_profile(self):
        result = verify_profile_links("NoLinks", "No URLs here", check_fn=lambda u: None)
        assert len(result.links_checked) == 0

    def test_timeout_handling(self):
        def timeout_check(url):
            return LinkCheckResult(url, "timeout", error="timed out")

        result = verify_profile_links("Slow", "https://slow.com", check_fn=timeout_check)
        assert result.links_checked[0].status == "timeout"
        assert result.all_links_ok is False


class TestCheckUrl:
    @patch("urllib.request.urlopen")
    def test_ok_response(self, mock_urlopen):
        mock_resp = MagicMock()
        mock_resp.status = 200
        mock_resp.__enter__ = lambda s: s
        mock_resp.__exit__ = MagicMock(return_value=False)
        mock_urlopen.return_value = mock_resp

        result = check_url("https://example.com")
        assert result.status == "ok"
        assert result.status_code == 200

    @patch("urllib.request.urlopen")
    def test_http_error(self, mock_urlopen):
        mock_urlopen.side_effect = urllib.error.HTTPError(
            "https://broken.com", 404, "Not Found", {}, None
        )
        result = check_url("https://broken.com")
        assert result.status == "broken"
        assert result.status_code == 404

    @patch("urllib.request.urlopen")
    def test_http_405_retries_get(self, mock_urlopen):
        # First call (HEAD) raises 405, second call (GET) succeeds
        mock_resp = MagicMock()
        mock_resp.status = 200
        mock_resp.__enter__ = lambda s: s
        mock_resp.__exit__ = MagicMock(return_value=False)

        mock_urlopen.side_effect = [
            urllib.error.HTTPError("url", 405, "Method Not Allowed", {}, None),
            mock_resp,
        ]
        result = check_url("https://example.com")
        assert result.status == "ok"
        assert result.status_code == 200

    @patch("urllib.request.urlopen")
    def test_http_405_retry_also_fails(self, mock_urlopen):
        mock_urlopen.side_effect = [
            urllib.error.HTTPError("url", 405, "Method Not Allowed", {}, None),
            urllib.error.HTTPError("url", 500, "Server Error", {}, None),
        ]
        result = check_url("https://example.com")
        assert result.status == "broken"
        assert result.status_code == 405

    @patch("urllib.request.urlopen")
    def test_timeout_error(self, mock_urlopen):
        mock_urlopen.side_effect = TimeoutError("Connection timed out")
        result = check_url("https://slow.com")
        assert result.status == "timeout"

    @patch("urllib.request.urlopen")
    def test_generic_exception(self, mock_urlopen):
        mock_urlopen.side_effect = ConnectionError("Connection refused")
        result = check_url("https://down.com")
        assert result.status == "broken"
        assert "Connection refused" in result.error
