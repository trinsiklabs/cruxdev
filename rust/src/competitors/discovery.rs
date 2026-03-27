//! Competitor discovery — find competitors via structured search queries.
//!
//! Takes a project description and category, generates search queries,
//! and returns a raw list of potential competitors with basic info.

use serde::{Deserialize, Serialize};

/// A competitor found during discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscoveredCompetitor {
    pub name: String,
    pub url: String,
    pub description: String,
    pub source_query: String,
    /// 0.0-1.0, how likely this is a real competitor
    pub confidence: f64,
}

impl DiscoveredCompetitor {
    pub fn new(name: &str, url: &str, description: &str, source_query: &str, confidence: f64) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            description: description.to_string(),
            source_query: source_query.to_string(),
            confidence,
        }
    }
}

/// Result of a competitor discovery run.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub query_terms: Vec<String>,
    pub competitors: Vec<DiscoveredCompetitor>,
    pub search_count: usize,
}

impl DiscoveryResult {
    /// Remove duplicates by URL (keep highest confidence).
    pub fn deduplicated(&self) -> Vec<DiscoveredCompetitor> {
        let mut by_url: std::collections::HashMap<String, DiscoveredCompetitor> =
            std::collections::HashMap::new();

        for c in &self.competitors {
            let normalized = c.url.trim_end_matches('/').to_lowercase();
            let dominated = by_url
                .get(&normalized)
                .map(|existing| c.confidence > existing.confidence)
                .unwrap_or(true);
            if dominated {
                by_url.insert(normalized, c.clone());
            }
        }

        let mut result: Vec<DiscoveredCompetitor> = by_url.into_values().collect();
        result.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        result
    }
}

/// Generate search queries for discovering competitors.
///
/// Returns structured queries covering: direct category, alternatives,
/// comparisons, and feature-specific searches.
pub fn generate_discovery_queries(
    project_description: &str,
    category: &str,
    max_queries: usize,
) -> Vec<String> {
    let mut queries = Vec::new();

    // Direct category
    queries.push(format!("{category} tools"));
    queries.push(format!("{category} software"));
    queries.push(format!("best {category} tools 2026"));

    // Alternatives
    queries.push(format!("{category} alternatives"));
    queries.push(format!("open source {category}"));

    // Comparison
    queries.push(format!("{category} comparison"));
    queries.push(format!("{category} vs"));

    // Feature-based from description
    let stopwords = ["which", "their", "about", "these", "those", "would"];
    let words_lower = project_description.to_lowercase();
    let key_terms: Vec<&str> = words_lower
        .split_whitespace()
        .filter(|w| w.len() > 4 && !stopwords.contains(w))
        .collect();

    if !key_terms.is_empty() {
        queries.push(format!("{} {category} tool", key_terms[0]));
        if key_terms.len() > 1 {
            queries.push(format!("{} {category}", key_terms[1]));
        }
    }

    queries.truncate(max_queries);
    queries
}

/// Parse an LLM response into discovered competitors.
///
/// Expects the response to contain competitor entries with name, URL, description.
/// Format: one competitor per line, or structured sections.
pub fn parse_discovery_response(
    response_text: &str,
    source_query: &str,
) -> Vec<DiscoveredCompetitor> {
    let mut competitors = Vec::new();
    let lines: Vec<&str> = response_text.trim().split('\n').collect();

    let mut current_name = String::new();
    let mut current_url = String::new();
    let mut current_desc = String::new();

    let flush = |name: &str, url: &str, desc: &str, query: &str, confidence: f64| -> DiscoveredCompetitor {
        let final_url = if url.is_empty() {
            format!("https://{}.com", name.to_lowercase().replace(' ', ""))
        } else {
            url.to_string()
        };
        DiscoveredCompetitor::new(name, &final_url, desc, query, confidence)
    };

    for line in &lines {
        let line = line.trim();
        if line.is_empty() {
            if !current_name.is_empty() {
                competitors.push(flush(&current_name, &current_url, &current_desc, source_query, 0.5));
                current_name.clear();
                current_url.clear();
                current_desc.clear();
            }
            continue;
        }

        if line.starts_with("Name:") {
            if !current_name.is_empty() {
                competitors.push(flush(&current_name, &current_url, &current_desc, source_query, 0.5));
            }
            current_name = line.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
            current_url.clear();
            current_desc.clear();
        } else if let Some(rest) = line.strip_prefix("URL:") {
            let raw = rest.trim();
            if raw.starts_with("//") {
                current_url = format!("https:{raw}");
            } else if !raw.starts_with("http") {
                // Rejoin -- the URL had a colon in it
                current_url = rest.trim().to_string();
            } else {
                current_url = raw.to_string();
            }
        } else if line.starts_with("Description:") {
            current_desc = line.split_once(':').map(|x| x.1).unwrap_or("").trim().to_string();
        } else if line.starts_with("- ") && line.contains(':') {
            let rest = &line[2..];
            let parts: Vec<&str> = rest.splitn(2, ':').collect();
            let name = parts[0].trim();
            let desc = if parts.len() > 1 { parts[1].trim() } else { "" };
            competitors.push(DiscoveredCompetitor::new(
                name,
                &format!("https://{}.com", name.to_lowercase().replace(' ', "")),
                desc,
                source_query,
                0.3,
            ));
        }
    }

    // Flush last entry
    if !current_name.is_empty() {
        competitors.push(flush(&current_name, &current_url, &current_desc, source_query, 0.5));
    }

    competitors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_discovery_queries_basic() {
        let queries = generate_discovery_queries("An AI coding assistant", "AI coding", 10);
        assert!(queries.len() <= 10);
        assert!(queries.contains(&"AI coding tools".to_string()));
        assert!(queries.contains(&"AI coding software".to_string()));
        assert!(queries.contains(&"AI coding alternatives".to_string()));
    }

    #[test]
    fn test_generate_discovery_queries_max_limit() {
        let queries = generate_discovery_queries("test description words here", "testing", 3);
        assert_eq!(queries.len(), 3);
    }

    #[test]
    fn test_parse_discovery_response_name_url_format() {
        let response = "Name: Cursor\nURL: https://cursor.sh\nDescription: AI code editor\n";
        let competitors = parse_discovery_response(response, "test query");
        assert_eq!(competitors.len(), 1);
        assert_eq!(competitors[0].name, "Cursor");
        assert_eq!(competitors[0].url, "https://cursor.sh");
        assert_eq!(competitors[0].description, "AI code editor");
        assert_eq!(competitors[0].confidence, 0.5);
    }

    #[test]
    fn test_parse_discovery_response_dash_format() {
        let response = "- Copilot: AI pair programmer\n- Cody: AI coding assistant\n";
        let competitors = parse_discovery_response(response, "search");
        assert_eq!(competitors.len(), 2);
        assert_eq!(competitors[0].name, "Copilot");
        assert_eq!(competitors[0].confidence, 0.3);
    }

    #[test]
    fn test_parse_discovery_response_multiple_entries() {
        let response = "Name: Alpha\nURL: https://alpha.com\nDescription: First\n\nName: Beta\nURL: https://beta.com\nDescription: Second\n";
        let competitors = parse_discovery_response(response, "q");
        assert_eq!(competitors.len(), 2);
        assert_eq!(competitors[0].name, "Alpha");
        assert_eq!(competitors[1].name, "Beta");
    }

    #[test]
    fn test_deduplicated_keeps_highest_confidence() {
        let result = DiscoveryResult {
            query_terms: vec![],
            competitors: vec![
                DiscoveredCompetitor::new("Test", "https://test.com", "desc1", "q1", 0.3),
                DiscoveredCompetitor::new("Test", "https://test.com", "desc2", "q2", 0.8),
                DiscoveredCompetitor::new("Test", "https://TEST.COM/", "desc3", "q3", 0.5),
            ],
            search_count: 3,
        };
        let deduped = result.deduplicated();
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].confidence, 0.8);
    }

    #[test]
    fn test_parse_discovery_response_url_with_scheme() {
        let response = "Name: Test\nURL: //example.com\nDescription: test\n";
        let competitors = parse_discovery_response(response, "q");
        assert_eq!(competitors[0].url, "https://example.com");
    }
}
