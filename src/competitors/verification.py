"""URL and claim verification for competitor profiles.

Tests all URLs in a competitor profile, returns pass/fail per link.
Validates claims against source data.
"""

import re
from dataclasses import dataclass, field


@dataclass
class LinkCheckResult:
    """Result of checking a single URL."""
    url: str
    status: str  # "ok", "broken", "timeout", "skipped"
    status_code: int = 0
    error: str = ""


@dataclass
class VerificationResult:
    """Result of verifying a competitor profile."""
    competitor_name: str
    links_checked: list[LinkCheckResult] = field(default_factory=list)
    claims_verified: int = 0
    claims_unverified: int = 0

    @property
    def all_links_ok(self) -> bool:
        return all(r.status == "ok" for r in self.links_checked)

    @property
    def broken_links(self) -> list[LinkCheckResult]:
        return [r for r in self.links_checked if r.status == "broken"]

    def to_dict(self) -> dict:
        return {
            "competitor": self.competitor_name,
            "links_total": len(self.links_checked),
            "links_ok": sum(1 for r in self.links_checked if r.status == "ok"),
            "links_broken": len(self.broken_links),
            "claims_verified": self.claims_verified,
            "claims_unverified": self.claims_unverified,
        }


def extract_urls(text: str) -> list[str]:
    """Extract all URLs from text."""
    pattern = r'https?://[^\s\)\]>"\']+'
    return list(dict.fromkeys(re.findall(pattern, text)))  # deduplicate, preserve order


def check_url(url: str, timeout: int = 10) -> LinkCheckResult:
    """Check if a URL is reachable.

    Uses urllib to avoid adding requests as a dependency.
    Returns LinkCheckResult with status.
    """
    import urllib.request
    import urllib.error

    try:
        req = urllib.request.Request(url, method="HEAD")
        req.add_header("User-Agent", "CruxDev-Verification/1.0")
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            return LinkCheckResult(url=url, status="ok", status_code=resp.status)
    except urllib.error.HTTPError as e:
        # Some sites block HEAD, try GET for 405
        if e.code == 405:
            try:
                req = urllib.request.Request(url, method="GET")
                req.add_header("User-Agent", "CruxDev-Verification/1.0")
                with urllib.request.urlopen(req, timeout=timeout) as resp:
                    return LinkCheckResult(url=url, status="ok", status_code=resp.status)
            except Exception:
                pass
        return LinkCheckResult(url=url, status="broken", status_code=e.code, error=str(e))
    except TimeoutError:
        return LinkCheckResult(url=url, status="timeout", error="Connection timed out")
    except Exception as e:
        return LinkCheckResult(url=url, status="broken", error=str(e))


def verify_profile_links(
    competitor_name: str,
    profile_markdown: str,
    check_fn=None,
) -> VerificationResult:
    """Verify all links in a competitor profile.

    Args:
        competitor_name: Name of the competitor
        profile_markdown: Markdown text containing URLs
        check_fn: Optional function to check URLs (for testing). Defaults to check_url.
    """
    checker = check_fn or check_url
    urls = extract_urls(profile_markdown)
    result = VerificationResult(competitor_name=competitor_name)

    for url in urls:
        link_result = checker(url)
        result.links_checked.append(link_result)

    return result
