//! Audit dimension prioritization — high-yield dimensions first.

use crate::engine::state::RoundResult;
use std::collections::HashMap;

/// Reorder dimensions by finding frequency (high-yield first).
///
/// All dimensions in default_order are preserved. Dimensions not seen
/// in history retain their original relative order.
pub fn rank_by_findings(history: &[RoundResult], default_order: &[String]) -> Vec<String> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for r in history {
        for f in &r.findings {
            *counts.entry(&f.dimension).or_default() += 1;
        }
    }

    let mut seen: Vec<(&str, usize)> = default_order
        .iter()
        .filter(|d| counts.get(d.as_str()).copied().unwrap_or(0) > 0)
        .map(|d| (d.as_str(), *counts.get(d.as_str()).unwrap()))
        .collect();
    seen.sort_by(|a, b| b.1.cmp(&a.1));

    let unseen: Vec<&str> = default_order
        .iter()
        .filter(|d| counts.get(d.as_str()).copied().unwrap_or(0) == 0)
        .map(|d| d.as_str())
        .collect();

    let mut result: Vec<String> = seen.into_iter().map(|(d, _)| d.to_string()).collect();
    result.extend(unseen.iter().map(|d| d.to_string()));
    result
}

/// Get finding counts per dimension.
pub fn get_dimension_stats(history: &[RoundResult]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for r in history {
        for f in &r.findings {
            *counts.entry(f.dimension.clone()).or_default() += 1;
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::state::{ConvergencePhase, Finding, FindingSeverity};

    fn make_round(dims: &[&str]) -> RoundResult {
        RoundResult {
            round: 1,
            phase: ConvergencePhase::CodeAuditing,
            findings: dims
                .iter()
                .map(|d| Finding {
                    id: "f1".into(),
                    file: "test.py".into(),
                    dimension: d.to_string(),
                    severity: FindingSeverity::Medium,
                    description: "test".into(),
                    suggested_fix: "fix".into(),
                    fixed: false,
                })
                .collect(),
            findings_fixed: 0,
            timestamp: 0.0,
        }
    }

    #[test]
    fn test_rank_by_findings() {
        let history = vec![
            make_round(&["style", "correctness"]),
            make_round(&["style"]),
        ];
        let default_order: Vec<String> = vec![
            "correctness".into(),
            "style".into(),
            "docs".into(),
        ];
        let ranked = rank_by_findings(&history, &default_order);
        assert_eq!(ranked[0], "style");
        assert_eq!(ranked[1], "correctness");
        assert_eq!(ranked[2], "docs");
    }

    #[test]
    fn test_get_dimension_stats() {
        let history = vec![make_round(&["a", "b", "a"])];
        let stats = get_dimension_stats(&history);
        assert_eq!(stats["a"], 2);
        assert_eq!(stats["b"], 1);
    }

    #[test]
    fn test_rank_empty_history() {
        let default_order: Vec<String> = vec!["a".into(), "b".into()];
        let ranked = rank_by_findings(&[], &default_order);
        assert_eq!(ranked, default_order);
    }
}
