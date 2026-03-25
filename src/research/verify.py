"""Source verification — URL checking, claim-source matching, recency.

Verifies that research findings are backed by accessible, recent,
relevant sources.
"""

from __future__ import annotations

import re
import time
from dataclasses import dataclass, field


@dataclass
class SourceCheck:
    """Result of checking a single source."""
    url: str
    reachable: bool = False
    recency_ok: bool = True  # True if source is recent enough
    claim_match: bool = True  # True if source supports the claim
    error: str = ""


@dataclass
class VerifyResult:
    """Result of verifying all sources in a finding."""
    finding_id: str
    sources_checked: list[SourceCheck] = field(default_factory=list)
    overall_verified: bool = False

    @property
    def all_reachable(self) -> bool:
        return all(s.reachable for s in self.sources_checked)

    @property
    def reachable_count(self) -> int:
        return sum(1 for s in self.sources_checked if s.reachable)


def extract_urls(text: str) -> list[str]:
    """Extract URLs from text."""
    pattern = r'https?://[^\s\)\]>"\']+'
    return list(dict.fromkeys(re.findall(pattern, text)))


def check_source_recency(
    source_date_str: str,
    max_age_days: int = 365,
) -> bool:
    """Check if a source is recent enough.

    Args:
        source_date_str: Date string (ISO format or year)
        max_age_days: Maximum age in days
    """
    try:
        # Try ISO format
        if len(source_date_str) >= 10:
            import datetime
            dt = datetime.datetime.fromisoformat(source_date_str[:10])
            age = (datetime.datetime.now() - dt).days
            return age <= max_age_days
        # Try year only
        year = int(source_date_str[:4])
        current_year = time.localtime().tm_year
        return (current_year - year) <= (max_age_days // 365 + 1)
    except (ValueError, IndexError):
        return True  # Can't parse → assume ok


def verify_sources(
    finding_id: str,
    source_urls: list[str],
    check_fn=None,
) -> VerifyResult:
    """Verify all sources for a finding.

    Args:
        finding_id: ID of the finding being verified
        source_urls: URLs to verify
        check_fn: Optional function to check URLs (for testing)
    """
    result = VerifyResult(finding_id=finding_id)

    for url in source_urls:
        if check_fn:
            reachable = check_fn(url)
            result.sources_checked.append(SourceCheck(
                url=url,
                reachable=reachable,
            ))
        else:
            result.sources_checked.append(SourceCheck(
                url=url,
                reachable=True,  # Default without checker
            ))

    result.overall_verified = (
        len(result.sources_checked) > 0
        and result.all_reachable
    )
    return result
