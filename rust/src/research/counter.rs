//! Counter-research — adversarial verification of claims.
//!
//! Generates negation queries, finds alternative explanations,
//! and checks for replication failures. Mandatory for all research.

use serde::{Deserialize, Serialize};

/// Result of counter-research on a claim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterResult {
    pub original_claim: String,
    pub negation_queries: Vec<String>,
    pub counter_evidence: Vec<String>,
    pub alternative_explanations: Vec<String>,
    /// robust, moderate, fragile, contested
    pub robustness: String,
}

impl CounterResult {
    pub fn is_contested(&self) -> bool {
        self.robustness == "contested"
    }

    pub fn has_counter_evidence(&self) -> bool {
        !self.counter_evidence.is_empty()
    }
}

/// Generate search queries to find evidence against a claim.
///
/// Strategies: direct negation, failure reports, alternative framing, criticism.
pub fn generate_negation_queries(claim: &str) -> Vec<String> {
    vec![
        format!("{claim} not true"),
        format!("{claim} wrong"),
        format!("{claim} problems"),
        format!("{claim} limitations"),
        format!("{claim} criticism"),
    ]
}

/// Assess claim robustness based on evidence balance.
///
/// Returns: robust, moderate, fragile, or contested.
pub fn assess_robustness(counter_count: usize, alternative_count: usize, supporting_count: usize) -> &'static str {
    if supporting_count == 0 {
        return "fragile";
    }

    let total_against = counter_count + alternative_count;
    let total = total_against + supporting_count;
    let ratio = if total > 0 {
        total_against as f64 / total as f64
    } else {
        0.0
    };

    if ratio >= 0.5 {
        "contested"
    } else if ratio >= 0.3 {
        "fragile"
    } else if ratio >= 0.1 {
        "moderate"
    } else {
        "robust"
    }
}

/// Run counter-research on a claim.
pub fn run_counter_research(
    claim: &str,
    counter_evidence: Option<Vec<String>>,
    alternative_explanations: Option<Vec<String>>,
    supporting_count: usize,
) -> CounterResult {
    let counter = counter_evidence.unwrap_or_default();
    let alternatives = alternative_explanations.unwrap_or_default();
    let queries = generate_negation_queries(claim);
    let robustness = assess_robustness(counter.len(), alternatives.len(), supporting_count);

    CounterResult {
        original_claim: claim.to_string(),
        negation_queries: queries,
        counter_evidence: counter,
        alternative_explanations: alternatives,
        robustness: robustness.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_negation_queries() {
        let queries = generate_negation_queries("AI improves productivity");
        assert_eq!(queries.len(), 5);
        assert!(queries[0].contains("not true"));
        assert!(queries[4].contains("criticism"));
    }

    #[test]
    fn test_assess_robustness_robust() {
        assert_eq!(assess_robustness(0, 0, 10), "robust");
    }

    #[test]
    fn test_assess_robustness_moderate() {
        assert_eq!(assess_robustness(1, 0, 5), "moderate");
    }

    #[test]
    fn test_assess_robustness_fragile_no_support() {
        assert_eq!(assess_robustness(0, 0, 0), "fragile");
    }

    #[test]
    fn test_assess_robustness_fragile_high_ratio() {
        assert_eq!(assess_robustness(3, 0, 7), "fragile");
    }

    #[test]
    fn test_assess_robustness_contested() {
        assert_eq!(assess_robustness(5, 0, 5), "contested");
    }

    #[test]
    fn test_run_counter_research() {
        let result = run_counter_research(
            "Rust is faster than C",
            Some(vec!["Benchmark shows C wins in X scenario".to_string()]),
            None,
            3,
        );
        assert_eq!(result.original_claim, "Rust is faster than C");
        assert!(result.has_counter_evidence());
        assert_eq!(result.negation_queries.len(), 5);
    }

    #[test]
    fn test_run_counter_research_no_evidence() {
        let result = run_counter_research("some claim", None, None, 5);
        assert!(!result.has_counter_evidence());
        assert_eq!(result.robustness, "robust");
        assert!(!result.is_contested());
    }

    #[test]
    fn test_counter_result_contested() {
        let result = run_counter_research(
            "claim",
            Some(vec!["e1".to_string(), "e2".to_string(), "e3".to_string()]),
            Some(vec!["a1".to_string(), "a2".to_string()]),
            1,
        );
        assert!(result.is_contested());
    }
}
