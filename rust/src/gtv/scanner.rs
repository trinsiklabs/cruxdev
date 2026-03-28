//! Claim extraction from markdown and Astro files.

use regex::Regex;

use super::{Claim, ClaimType};

/// Extract all verifiable claims from file content.
pub fn extract_claims(content: &str) -> Vec<Claim> {
    let mut claims = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let line_num = line_idx + 1;

        // Skip frontmatter delimiters, code fence markers, and HTML comments
        let trimmed = line.trim();
        if trimmed == "---" || trimmed.starts_with("```") || trimmed.starts_with("<!--") {
            continue;
        }

        claims.extend(extract_numeric(line, line_num));
        claims.extend(extract_features(line, line_num));
        claims.extend(extract_status(line, line_num));
        claims.extend(extract_paths(line, line_num));
        claims.extend(extract_urls(line, line_num));
    }

    claims
}

/// Extract numeric claims like "485 tests", "61 MCP tools", "100% coverage".
fn extract_numeric(line: &str, line_number: usize) -> Vec<Claim> {
    let re = Regex::new(
        r"(\d[\d,]*)\s*\+?\s*(?:\w+\s+){0,3}(tests?|tools?|pages?|coverage|dimensions?|blog\s*posts?|patterns?|projects?|competitors?)"
    ).unwrap();

    re.captures_iter(line)
        .map(|cap| {
            let value = cap[1].replace(',', "");
            Claim {
                claim_type: ClaimType::Numeric,
                value: format!("{} {}", value, &cap[2]),
                line_number,
                source_text: line.trim().to_string(),
            }
        })
        .collect()
}

/// Extract feature claims like "supports X", "integrates with Y".
fn extract_features(line: &str, line_number: usize) -> Vec<Claim> {
    let re = Regex::new(
        r"(?i)(supports?|routes?\s+to|integrates?\s+with|enables?|provides?)\s+([^.,;:]+)"
    ).unwrap();

    re.captures_iter(line)
        .map(|cap| Claim {
            claim_type: ClaimType::Feature,
            value: format!("{} {}", &cap[1], cap[2].trim()),
            line_number,
            source_text: line.trim().to_string(),
        })
        .collect()
}

/// Extract status claims like "active", "live", "deployed".
fn extract_status(line: &str, line_number: usize) -> Vec<Claim> {
    let re = Regex::new(
        r"(?i)\b(is\s+)?(currently\s+)?(active|live|working|running|deployed)\b"
    ).unwrap();

    re.captures_iter(line)
        .map(|cap| Claim {
            claim_type: ClaimType::Status,
            value: cap[3].to_lowercase(),
            line_number,
            source_text: line.trim().to_string(),
        })
        .collect()
}

/// Extract file/path references in backticks.
fn extract_paths(line: &str, line_number: usize) -> Vec<Claim> {
    let re = Regex::new(r"`([a-zA-Z0-9_./-]+\.[a-zA-Z]{1,10})`").unwrap();

    re.captures_iter(line)
        .filter(|cap| {
            let path = &cap[1];
            // Must look like a real path (has directory separator or is a dotfile)
            path.contains('/') || path.starts_with('.')
        })
        .map(|cap| Claim {
            claim_type: ClaimType::Path,
            value: cap[1].to_string(),
            line_number,
            source_text: line.trim().to_string(),
        })
        .collect()
}

/// Extract URL references.
fn extract_urls(line: &str, line_number: usize) -> Vec<Claim> {
    let re = Regex::new(r#"(https?://[^\s">\]]+)"#).unwrap();

    re.captures_iter(line)
        .map(|cap| Claim {
            claim_type: ClaimType::Url,
            value: cap[1].to_string(),
            line_number,
            source_text: line.trim().to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numeric_claims() {
        let claims = extract_numeric("CruxDev has 485 tests and 61 MCP tools.", 1);
        assert_eq!(claims.len(), 2);
        assert_eq!(claims[0].value, "485 tests");
        assert_eq!(claims[1].value, "61 tools");
    }

    #[test]
    fn test_extract_numeric_with_comma() {
        let claims = extract_numeric("Over 1,200 tests passing.", 1);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].value, "1200 tests");
    }

    #[test]
    fn test_extract_numeric_with_plus() {
        let claims = extract_numeric("45,000+ lines of patterns", 1);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].value, "45000 patterns");
    }

    #[test]
    fn test_extract_feature_claims() {
        let claims = extract_features("CruxDev supports automatic deployment", 1);
        assert_eq!(claims.len(), 1);
        assert!(claims[0].value.contains("supports"));
        assert!(claims[0].value.contains("automatic deployment"));
    }

    #[test]
    fn test_extract_feature_integrates() {
        let claims = extract_features("Integrates with Stripe Connect for payments", 1);
        assert_eq!(claims.len(), 1);
        assert!(claims[0].value.to_lowercase().contains("integrates with"));
    }

    #[test]
    fn test_extract_status_claims() {
        let claims = extract_status("The service is currently active and deployed.", 1);
        assert_eq!(claims.len(), 2);
        let values: Vec<&str> = claims.iter().map(|c| c.value.as_str()).collect();
        assert!(values.contains(&"active"));
        assert!(values.contains(&"deployed"));
    }

    #[test]
    fn test_extract_path_references() {
        let claims = extract_paths("See `rust/src/server.rs` for details.", 1);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].value, "rust/src/server.rs");
    }

    #[test]
    fn test_extract_path_ignores_non_paths() {
        let claims = extract_paths("Use `String` type here.", 1);
        assert_eq!(claims.len(), 0);
    }

    #[test]
    fn test_extract_url_references() {
        let claims = extract_urls("Visit https://cruxdev.dev/blog for more.", 1);
        assert_eq!(claims.len(), 1);
        assert_eq!(claims[0].value, "https://cruxdev.dev/blog");
    }

    #[test]
    fn test_extract_claims_full() {
        let content = "# CruxDev\n\n485 tests passing. See `rust/src/main.rs`.\nVisit https://cruxdev.dev\n";
        let claims = extract_claims(content);
        assert!(claims.len() >= 3);
    }

    #[test]
    fn test_extract_claims_from_astro_content() {
        let content = r#"---
layout: ../layouts/Base.astro
title: "Engine"
---
<h2>61 MCP tools</h2>
<p>Currently deployed and active.</p>
"#;
        let claims = extract_claims(content);
        let numeric: Vec<_> = claims.iter().filter(|c| c.claim_type == ClaimType::Numeric).collect();
        assert!(!numeric.is_empty());
    }

    #[test]
    fn test_skips_frontmatter_delimiters() {
        let content = "---\ntitle: Test\n---\n485 tests\n";
        let claims = extract_claims(content);
        // Should find "485 tests" but not extract from "---" lines
        let numeric: Vec<_> = claims.iter().filter(|c| c.claim_type == ClaimType::Numeric).collect();
        assert_eq!(numeric.len(), 1);
        assert_eq!(numeric[0].line_number, 4);
    }
}
