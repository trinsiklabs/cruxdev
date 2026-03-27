//! Research quality scoring — 3-tier system.
//!
//! Tier 1: Fast gate (deterministic, no LLM)
//! Tier 2: Standard score (6 dimensions)
//! Tier 3: Deep score (full rubric, high-quality only)

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

/// Quality assessment result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityResult {
    pub tier: u8,
    /// 0.0-1.0
    pub score: f64,
    pub passed: bool,
    pub details: serde_json::Value,
}

const BOILERPLATE_WORDS: &[&str] = &[
    "subscribe",
    "click here",
    "newsletter",
    "cookie",
    "advertisement",
    "sign up",
    "unsubscribe",
];

fn boilerplate_set() -> HashSet<&'static str> {
    BOILERPLATE_WORDS.iter().copied().collect()
}

/// Tier 1: Deterministic quality check. No LLM needed.
///
/// Checks: content length, real sentences, information density, boilerplate.
/// Score < 0.3 -> REJECT.
pub fn fast_gate(content: &str) -> QualityResult {
    if content.trim().len() < 200 {
        return QualityResult {
            tier: 1,
            score: 0.0,
            passed: false,
            details: serde_json::json!({"reason": "too_short"}),
        };
    }

    // Count real sentences (split on sentence-ending punctuation, keep those > 20 chars)
    let sentences: Vec<&str> = content
        .split(['.', '?', '!'])
        .map(|s| s.trim())
        .filter(|s| s.len() > 20)
        .collect();

    if sentences.len() < 3 {
        return QualityResult {
            tier: 1,
            score: 0.1,
            passed: false,
            details: serde_json::json!({"reason": "few_sentences", "count": sentences.len()}),
        };
    }

    // Information density
    let content_lower = content.to_lowercase();
    let words: Vec<&str> = content_lower.split_whitespace().collect();
    let unique_words: HashSet<&str> = words.iter().copied().collect();
    let density = if !words.is_empty() {
        unique_words.len() as f64 / words.len() as f64
    } else {
        0.0
    };

    if density < 0.3 {
        return QualityResult {
            tier: 1,
            score: 0.2,
            passed: false,
            details: serde_json::json!({"reason": "low_density", "density": round3(density)}),
        };
    }

    // Boilerplate penalty
    let bp_set = boilerplate_set();
    let boilerplate_count = words.iter().filter(|w| bp_set.contains(**w)).count();
    let boilerplate_ratio = if !words.is_empty() {
        boilerplate_count as f64 / words.len() as f64
    } else {
        0.0
    };

    let score = (density + (1.0 - boilerplate_ratio)).min(1.0);
    let passed = score >= 0.3;

    QualityResult {
        tier: 1,
        score: round3(score),
        passed,
        details: serde_json::json!({
            "chars": content.len(),
            "sentences": sentences.len(),
            "density": round3(density),
            "boilerplate_ratio": round3(boilerplate_ratio),
        }),
    }
}

/// Tier 2: 6-dimension binary check.
///
/// Score = count of True / 6. Threshold: >= 0.5 to proceed.
pub fn standard_score(
    has_authority: bool,
    has_recency: bool,
    has_citations: bool,
    has_relevance: bool,
    has_counter_evidence: bool,
    has_consistency: bool,
) -> QualityResult {
    let dimensions = [
        has_authority,
        has_recency,
        has_citations,
        has_relevance,
        has_counter_evidence,
        has_consistency,
    ];
    let score = dimensions.iter().filter(|&&d| d).count() as f64 / 6.0;

    QualityResult {
        tier: 2,
        score: round3(score),
        passed: score >= 0.5,
        details: serde_json::json!({
            "authority": has_authority,
            "recency": has_recency,
            "citations": has_citations,
            "relevance": has_relevance,
            "counter_evidence": has_counter_evidence,
            "consistency": has_consistency,
        }),
    }
}

/// Tier 3: Full rubric for high-quality findings.
///
/// Each dimension 0-25 points. Total 0-100.
pub fn deep_score(source_quality: u32, coverage: u32, synthesis: u32, reliability: u32) -> QualityResult {
    let total = source_quality + coverage + synthesis + reliability;
    let score = total as f64 / 100.0;

    QualityResult {
        tier: 3,
        score: round3(score),
        passed: score >= 0.7,
        details: serde_json::json!({
            "source_quality": source_quality,
            "coverage": coverage,
            "synthesis": synthesis,
            "reliability": reliability,
            "total": total,
        }),
    }
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_gate_too_short() {
        let result = fast_gate("short text");
        assert!(!result.passed);
        assert_eq!(result.score, 0.0);
        assert_eq!(result.details["reason"], "too_short");
    }

    #[test]
    fn test_fast_gate_few_sentences() {
        // Long enough but no real sentences
        let content = "a ".repeat(200);
        let result = fast_gate(&content);
        assert!(!result.passed);
    }

    #[test]
    fn test_fast_gate_passes_good_content() {
        let content = "This is a comprehensive research document about artificial intelligence and machine learning. \
                       It covers the fundamental concepts that drive modern AI systems forward. \
                       The analysis includes multiple perspectives from different researchers in the field. \
                       Various approaches to solving complex problems are discussed in detail throughout. \
                       Performance benchmarks show significant improvements over baseline methods used previously.";
        let result = fast_gate(content);
        assert!(result.passed);
        assert!(result.score >= 0.3);
    }

    #[test]
    fn test_standard_score_passes() {
        let result = standard_score(true, true, true, true, false, false);
        assert!(result.passed);
        assert!(result.score >= 0.5);
        assert_eq!(result.tier, 2);
    }

    #[test]
    fn test_standard_score_fails() {
        let result = standard_score(true, false, false, false, false, false);
        assert!(!result.passed);
    }

    #[test]
    fn test_deep_score_passes() {
        let result = deep_score(20, 20, 20, 20);
        assert!(result.passed);
        assert_eq!(result.score, 0.8);
        assert_eq!(result.tier, 3);
    }

    #[test]
    fn test_deep_score_fails() {
        let result = deep_score(10, 10, 10, 10);
        assert!(!result.passed);
        assert_eq!(result.score, 0.4);
    }

    #[test]
    fn test_deep_score_exact_threshold() {
        let result = deep_score(18, 18, 18, 16);
        assert!(result.passed); // 70/100 = 0.7
    }
}
