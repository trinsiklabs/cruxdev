//! Prompt A/B testing — compare prompt variants across scored dimensions.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A prompt variant to test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptVariant {
    pub id: String,
    pub name: String,
    pub content: String,
    #[serde(default)]
    pub scores: HashMap<String, f64>,
}

/// Result of an A/B test between two prompt variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestResult {
    pub variant_a: PromptVariant,
    pub variant_b: PromptVariant,
    pub winner: String,
    #[serde(default)]
    pub dimensions: HashMap<String, String>,
    #[serde(default)]
    pub safe_to_deploy: bool,
}

impl ABTestResult {
    pub fn summary(&self) -> String {
        if self.winner == "tie" {
            return "Tie — no significant difference".to_string();
        }
        let winner_name = if self.winner == "a" {
            &self.variant_a.name
        } else {
            &self.variant_b.name
        };
        format!("{} wins", winner_name)
    }
}

/// Compare two prompt variants across all scored dimensions.
///
/// Rules:
/// - Variant wins a dimension if its score is higher
/// - Overall winner is the variant that wins more dimensions
/// - NOT safe to deploy if challenger (b) loses on any critical dimension
pub fn compare_variants(
    variant_a: PromptVariant,
    variant_b: PromptVariant,
    critical_dimensions: &[String],
) -> ABTestResult {
    let critical: HashSet<&str> = critical_dimensions.iter().map(|s| s.as_str()).collect();
    let all_dims: HashSet<&str> = variant_a
        .scores
        .keys()
        .chain(variant_b.scores.keys())
        .map(|s| s.as_str())
        .collect();

    let mut dimension_results = HashMap::new();
    let mut a_wins = 0u32;
    let mut b_wins = 0u32;
    let mut b_regresses_critical = false;

    for dim in &all_dims {
        let score_a = variant_a.scores.get(*dim).copied().unwrap_or(0.0);
        let score_b = variant_b.scores.get(*dim).copied().unwrap_or(0.0);

        if score_b > score_a {
            dimension_results.insert(dim.to_string(), "b".to_string());
            b_wins += 1;
        } else if score_a > score_b {
            dimension_results.insert(dim.to_string(), "a".to_string());
            a_wins += 1;
            if critical.contains(dim) {
                b_regresses_critical = true;
            }
        } else {
            dimension_results.insert(dim.to_string(), "tie".to_string());
        }
    }

    let winner = if a_wins > b_wins {
        "a"
    } else if b_wins > a_wins {
        "b"
    } else {
        "tie"
    };

    ABTestResult {
        variant_a,
        variant_b,
        winner: winner.to_string(),
        dimensions: dimension_results,
        safe_to_deploy: winner == "b" && !b_regresses_critical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_variant(name: &str, scores: &[(&str, f64)]) -> PromptVariant {
        PromptVariant {
            id: name.to_string(),
            name: name.to_string(),
            content: "test".to_string(),
            scores: scores.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        }
    }

    #[test]
    fn test_compare_b_wins() {
        let a = make_variant("baseline", &[("accuracy", 0.8), ("speed", 0.7)]);
        let b = make_variant("challenger", &[("accuracy", 0.9), ("speed", 0.9)]);
        let result = compare_variants(a, b, &[]);
        assert_eq!(result.winner, "b");
        assert!(result.safe_to_deploy);
    }

    #[test]
    fn test_compare_a_wins() {
        let a = make_variant("baseline", &[("accuracy", 0.9), ("speed", 0.9)]);
        let b = make_variant("challenger", &[("accuracy", 0.7), ("speed", 0.7)]);
        let result = compare_variants(a, b, &[]);
        assert_eq!(result.winner, "a");
        assert!(!result.safe_to_deploy);
    }

    #[test]
    fn test_compare_tie() {
        let a = make_variant("baseline", &[("accuracy", 0.8)]);
        let b = make_variant("challenger", &[("accuracy", 0.8)]);
        let result = compare_variants(a, b, &[]);
        assert_eq!(result.winner, "tie");
    }

    #[test]
    fn test_critical_dimension_regression() {
        let a = make_variant("baseline", &[("accuracy", 0.9), ("speed", 0.5)]);
        let b = make_variant("challenger", &[("accuracy", 0.7), ("speed", 0.9)]);
        let result = compare_variants(a, b, &["accuracy".to_string()]);
        // b wins on speed, a wins on accuracy (critical) — not safe
        assert!(!result.safe_to_deploy);
    }

    #[test]
    fn test_summary() {
        let a = make_variant("baseline", &[("x", 0.5)]);
        let b = make_variant("challenger", &[("x", 0.9)]);
        let result = compare_variants(a, b, &[]);
        assert_eq!(result.summary(), "challenger wins");
    }
}
