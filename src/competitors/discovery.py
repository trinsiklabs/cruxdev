"""Competitor discovery — find competitors via structured search queries.

Takes a project description and category, generates search queries,
and returns a raw list of potential competitors with basic info.
"""

from dataclasses import dataclass, field


@dataclass
class DiscoveredCompetitor:
    """A competitor found during discovery."""
    name: str
    url: str
    description: str
    source_query: str
    confidence: float = 0.0  # 0-1, how likely this is a real competitor


@dataclass
class DiscoveryResult:
    """Result of a competitor discovery run."""
    query_terms: list[str] = field(default_factory=list)
    competitors: list[DiscoveredCompetitor] = field(default_factory=list)
    search_count: int = 0

    def deduplicated(self) -> list[DiscoveredCompetitor]:
        """Remove duplicates by URL (keep highest confidence)."""
        by_url: dict[str, DiscoveredCompetitor] = {}
        for c in self.competitors:
            normalized = c.url.rstrip("/").lower()
            if normalized not in by_url or c.confidence > by_url[normalized].confidence:
                by_url[normalized] = c
        return sorted(by_url.values(), key=lambda x: x.confidence, reverse=True)


def generate_discovery_queries(
    project_description: str,
    category: str,
    max_queries: int = 10,
) -> list[str]:
    """Generate search queries for discovering competitors.

    Returns structured queries covering:
    - Direct category searches
    - Alternative/comparison searches
    - "best X" searches
    - Feature-specific searches
    """
    queries = []

    # Direct category
    queries.append(f"{category} tools")
    queries.append(f"{category} software")
    queries.append(f"best {category} tools 2026")

    # Alternatives
    queries.append(f"{category} alternatives")
    queries.append(f"open source {category}")

    # Comparison
    queries.append(f"{category} comparison")
    queries.append(f"{category} vs")

    # Feature-based from description
    words = project_description.lower().split()
    key_terms = [w for w in words if len(w) > 4 and w not in ("which", "their", "about", "these", "those", "would")]
    if key_terms:
        queries.append(f"{key_terms[0]} {category} tool")
        if len(key_terms) > 1:
            queries.append(f"{key_terms[1]} {category}")

    return queries[:max_queries]


def parse_discovery_response(
    response_text: str,
    source_query: str,
) -> list[DiscoveredCompetitor]:
    """Parse an LLM response into discovered competitors.

    Expects the response to contain competitor entries with name, URL, description.
    Format: one competitor per line, or structured sections.
    """
    competitors = []
    lines = response_text.strip().split("\n")

    current_name = ""
    current_url = ""
    current_desc = ""

    for line in lines:
        line = line.strip()
        if not line:
            if current_name:
                competitors.append(DiscoveredCompetitor(
                    name=current_name,
                    url=current_url or f"https://{current_name.lower().replace(' ', '')}.com",
                    description=current_desc,
                    source_query=source_query,
                    confidence=0.5,
                ))
                current_name = ""
                current_url = ""
                current_desc = ""
            continue

        # Parse "Name: value" format
        if line.startswith("Name:"):
            if current_name:
                competitors.append(DiscoveredCompetitor(
                    name=current_name,
                    url=current_url or f"https://{current_name.lower().replace(' ', '')}.com",
                    description=current_desc,
                    source_query=source_query,
                    confidence=0.5,
                ))
            current_name = line.split(":", 1)[1].strip()
            current_url = ""
            current_desc = ""
        elif line.startswith("URL:"):
            current_url = line.split(":", 1)[1].strip()
            # Handle "URL: https://..." where split on first : cuts the scheme
            if current_url.startswith("//"):
                current_url = "https:" + current_url
            elif not current_url.startswith("http"):
                # Rejoin — the URL had a colon in it
                current_url = line[4:].strip()
        elif line.startswith("Description:"):
            current_desc = line.split(":", 1)[1].strip()
        elif line.startswith("- ") and ":" in line:
            # Handle "- Name: description" format
            parts = line[2:].split(":", 1)
            name = parts[0].strip()
            desc = parts[1].strip() if len(parts) > 1 else ""
            competitors.append(DiscoveredCompetitor(
                name=name,
                url=f"https://{name.lower().replace(' ', '')}.com",
                description=desc,
                source_query=source_query,
                confidence=0.3,
            ))

    # Flush last entry
    if current_name:
        competitors.append(DiscoveredCompetitor(
            name=current_name,
            url=current_url or f"https://{current_name.lower().replace(' ', '')}.com",
            description=current_desc,
            source_query=source_query,
            confidence=0.5,
        ))

    return competitors
