//! Content analysis engine — extract key points, quotes, statistics, and narrative arc
//! from markdown source content for cross-platform repurposing.

use serde::{Deserialize, Serialize};

/// A key takeaway from the source content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPoint {
    pub text: String,
    pub importance: u8, // 1-10
    pub quotable: bool,
}

/// A quotable passage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub text: String,
    pub attribution: Option<String>,
}

/// A statistic or numeric claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Statistic {
    pub claim: String,
    pub number: String,
    pub context: String,
}

/// The narrative arc of the content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeArc {
    pub hook: String,
    pub problem: String,
    pub solution: String,
    pub proof: String,
    pub cta: String,
}

/// Full analysis result from a source document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    pub title: String,
    pub word_count: usize,
    pub key_points: Vec<KeyPoint>,
    pub quotes: Vec<Quote>,
    pub statistics: Vec<Statistic>,
    pub narrative_arc: Option<NarrativeArc>,
    pub headings: Vec<String>,
    pub summary: String,
}

/// Analyze a markdown source document.
pub fn analyze(source: &str) -> ContentAnalysis {
    let title = extract_title(source);
    let headings = extract_headings(source);
    let word_count = source.split_whitespace().count();
    let key_points = extract_key_points(source);
    let quotes = extract_quotes(source);
    let statistics = extract_statistics(source);
    let narrative_arc = detect_narrative_arc(source, &headings);
    let summary = generate_summary(source, &key_points);

    ContentAnalysis {
        title,
        word_count,
        key_points,
        quotes,
        statistics,
        narrative_arc,
        headings,
        summary,
    }
}

/// Extract the title (first H1).
fn extract_title(source: &str) -> String {
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
            return trimmed.trim_start_matches("# ").to_string();
        }
    }
    "Untitled".to_string()
}

/// Extract all headings (H2 and H3).
fn extract_headings(source: &str) -> Vec<String> {
    source
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("## ") || trimmed.starts_with("### ") {
                Some(trimmed.trim_start_matches('#').trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Extract key points from the content.
/// Looks for: bold text, list items, sentences with strong language.
fn extract_key_points(source: &str) -> Vec<KeyPoint> {
    let mut points = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        // Bold text as key points
        if trimmed.contains("**") {
            let bold_parts: Vec<&str> = trimmed.split("**").collect();
            for (i, part) in bold_parts.iter().enumerate() {
                if i % 2 == 1 && !part.is_empty() && part.len() > 5 {
                    points.push(KeyPoint {
                        text: part.to_string(),
                        importance: 7,
                        quotable: part.len() < 140,
                    });
                }
            }
        }

        // List items that are substantive
        if (trimmed.starts_with("- ") || trimmed.starts_with("* "))
            && trimmed.len() > 30
            && !trimmed.contains('|') // not a table
        {
            let text = trimmed
                .trim_start_matches("- ")
                .trim_start_matches("* ")
                .to_string();
            // Skip if it's a duplicate of a bold point
            if !points.iter().any(|p| text.contains(&p.text)) {
                points.push(KeyPoint {
                    text,
                    importance: 5,
                    quotable: trimmed.len() < 160,
                });
            }
        }
    }

    // Sort by importance, keep top 10
    points.sort_by(|a, b| b.importance.cmp(&a.importance));
    points.truncate(10);
    points
}

/// Extract quotable passages.
/// Looks for: blockquotes, short impactful sentences, text in quotes.
fn extract_quotes(source: &str) -> Vec<Quote> {
    let mut quotes = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        // Blockquotes
        if trimmed.starts_with("> ") {
            let text = trimmed.trim_start_matches("> ").to_string();
            if !text.is_empty() {
                quotes.push(Quote {
                    text,
                    attribution: None,
                });
            }
        }

        // Text in quotes within paragraphs
        if trimmed.contains('"') {
            let parts: Vec<&str> = trimmed.split('"').collect();
            for (i, part) in parts.iter().enumerate() {
                if i % 2 == 1 && part.len() > 10 && part.len() < 280 {
                    quotes.push(Quote {
                        text: part.to_string(),
                        attribution: None,
                    });
                }
            }
        }
    }

    quotes.truncate(10);
    quotes
}

/// Extract statistics and numeric claims.
fn extract_statistics(source: &str) -> Vec<Statistic> {
    let mut stats = Vec::new();

    for line in source.lines() {
        let trimmed = line.trim();

        // Look for lines with numbers that aren't headings or code
        if trimmed.starts_with('#') || trimmed.starts_with("```") {
            continue;
        }

        // Find numbers with context (e.g., "113 tests", "$100/month", "21 new tests")
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            let cleaned = word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '$' && c != '%');
            if cleaned.is_empty() {
                continue;
            }

            let has_number = cleaned.chars().any(|c| c.is_ascii_digit());
            let is_meaningful = cleaned.len() > 1 || cleaned.parse::<u64>().map(|n| n > 9).unwrap_or(false);

            if has_number && is_meaningful {
                // Get surrounding context (2 words before and after)
                let start = i.saturating_sub(2);
                let end = (i + 3).min(words.len());
                let context: String = words[start..end].join(" ");

                stats.push(Statistic {
                    claim: trimmed.to_string(),
                    number: cleaned.to_string(),
                    context,
                });
                break; // one stat per line
            }
        }
    }

    stats.truncate(15);
    stats
}

/// Detect narrative arc from headings and content structure.
fn detect_narrative_arc(source: &str, headings: &[String]) -> Option<NarrativeArc> {
    if headings.len() < 3 {
        return None;
    }

    // Extract first paragraph as hook
    let paragraphs: Vec<&str> = source
        .split("\n\n")
        .filter(|p| {
            let t = p.trim();
            !t.is_empty() && !t.starts_with('#') && !t.starts_with("```") && !t.starts_with("---")
        })
        .collect();

    if paragraphs.is_empty() {
        return None;
    }

    let hook = paragraphs[0].trim().to_string();
    let problem = if paragraphs.len() > 1 {
        paragraphs[1].trim().to_string()
    } else {
        String::new()
    };
    let solution = headings.get(1).cloned().unwrap_or_default();
    let proof = headings.last().cloned().unwrap_or_default();

    // Look for CTA-like content in last paragraph
    let cta = paragraphs
        .last()
        .map(|p| p.trim().to_string())
        .unwrap_or_default();

    Some(NarrativeArc {
        hook,
        problem,
        solution,
        proof,
        cta,
    })
}

/// Generate a brief summary from key points.
fn generate_summary(source: &str, key_points: &[KeyPoint]) -> String {
    if key_points.is_empty() {
        // Fall back to first paragraph
        let first_para = source
            .split("\n\n")
            .find(|p| {
                let t = p.trim();
                !t.is_empty() && !t.starts_with('#') && !t.starts_with("```")
            })
            .unwrap_or("");

        let truncated: String = first_para.chars().take(280).collect();
        return truncated.trim().to_string();
    }

    // Combine top 3 key points
    key_points
        .iter()
        .take(3)
        .map(|kp| kp.text.as_str())
        .collect::<Vec<_>>()
        .join(". ")
}

/// Calculate Jaccard similarity between two strings (word-level).
pub fn jaccard_similarity(a: &str, b: &str) -> f64 {
    let words_a: std::collections::HashSet<&str> = a.split_whitespace().collect();
    let words_b: std::collections::HashSet<&str> = b.split_whitespace().collect();

    if words_a.is_empty() && words_b.is_empty() {
        return 1.0;
    }

    let intersection = words_a.intersection(&words_b).count();
    let union = words_a.union(&words_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_ARTICLE: &str = r#"# OneList Storage — CruxBot Gets a Persistent Brain

CruxBot's persistent state used to be scattered JSONL files in `.cruxbot/`. Observations, budget records, scripts, plans — all local, all fragile.

## The Problem

When the process died mid-write, data could corrupt. "If you wanted to query across projects, you couldn't." Every observation was trapped in a local file.

## What We Built

**Phase 1: Typed HTTP Client** — `onelist/client.rs`

- OneListClient with reqwest, default Bearer auth, agent identification headers
- Exponential backoff retry: 100ms to 200ms to 400ms, max 3 attempts
- Typed errors: Auth, NotFound, RateLimit, Server, Network

**Phase 7: Write-Ahead Log** — `onelist/wal.rs`

- When OneList is unreachable: operations append to `.cruxbot/onelist_wal.jsonl`
- Idempotent replay: UUID keys prevent duplicates on reconnect
- Circuit breaker: 3 consecutive failures triggers WAL-only mode

## The Numbers

- **914** lines of new Rust code
- **21** new tests (5 client, 8 entries, 8 WAL)
- **113** total CruxBot tests passing
- **0** failures

## What's Next

The foundation is solid. Now we build on it.
"#;

    #[test]
    fn test_extract_title() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert_eq!(analysis.title, "OneList Storage — CruxBot Gets a Persistent Brain");
    }

    #[test]
    fn test_extract_headings() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(analysis.headings.contains(&"The Problem".to_string()));
        assert!(analysis.headings.contains(&"What We Built".to_string()));
        assert!(analysis.headings.contains(&"The Numbers".to_string()));
    }

    #[test]
    fn test_word_count() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(analysis.word_count > 100);
    }

    #[test]
    fn test_extract_key_points() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(!analysis.key_points.is_empty());
        // Should find bold text
        let texts: Vec<&str> = analysis.key_points.iter().map(|kp| kp.text.as_str()).collect();
        assert!(texts.iter().any(|t| t.contains("Phase 1")));
    }

    #[test]
    fn test_extract_quotes() {
        let analysis = analyze(SAMPLE_ARTICLE);
        // Should find the quoted text
        assert!(analysis.quotes.iter().any(|q| q.text.contains("query across projects")));
    }

    #[test]
    fn test_extract_statistics() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(!analysis.statistics.is_empty());
        // Should find "914 lines", "21 new tests", "113 total"
        let numbers: Vec<&str> = analysis.statistics.iter().map(|s| s.number.as_str()).collect();
        assert!(numbers.iter().any(|n| *n == "914"));
    }

    #[test]
    fn test_narrative_arc_detected() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(analysis.narrative_arc.is_some());
        let arc = analysis.narrative_arc.unwrap();
        assert!(!arc.hook.is_empty());
        assert!(!arc.problem.is_empty());
    }

    #[test]
    fn test_summary_generated() {
        let analysis = analyze(SAMPLE_ARTICLE);
        assert!(!analysis.summary.is_empty());
    }

    #[test]
    fn test_empty_content() {
        let analysis = analyze("");
        assert_eq!(analysis.title, "Untitled");
        assert_eq!(analysis.word_count, 0);
        assert!(analysis.key_points.is_empty());
    }

    #[test]
    fn test_minimal_content() {
        let analysis = analyze("# Title\n\nJust one paragraph here.");
        assert_eq!(analysis.title, "Title");
        assert!(analysis.narrative_arc.is_none()); // too few headings
    }

    #[test]
    fn test_jaccard_similarity_identical() {
        assert!((jaccard_similarity("hello world", "hello world") - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_jaccard_similarity_different() {
        assert!(jaccard_similarity("hello world", "goodbye moon") < 0.1);
    }

    #[test]
    fn test_jaccard_similarity_partial() {
        let sim = jaccard_similarity("the quick brown fox", "the slow brown dog");
        assert!(sim > 0.2 && sim < 0.8);
    }

    #[test]
    fn test_jaccard_similarity_empty() {
        assert!((jaccard_similarity("", "") - 1.0).abs() < 0.001);
    }
}
