//! Convergence parameter tuning from historical data.

use crate::engine::state::RoundResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Safety floors — never go below these.
pub const MIN_CONVERGENCE_THRESHOLD: i32 = 2;
pub const MIN_MAX_ROUNDS: i32 = 3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningRecommendation {
    pub parameter: String,
    pub current_value: f64,
    pub recommended_value: f64,
    pub rationale: String,
}

/// Calculate how many rounds a history took (returns total length).
pub fn analyze_rounds_to_convergence(history: &[RoundResult]) -> usize {
    history.len()
}

/// Recommend max_rounds based on historical convergence data.
pub fn recommend_max_rounds(
    histories: &[Vec<RoundResult>],
    current_max: i32,
) -> TuningRecommendation {
    if histories.is_empty() {
        return TuningRecommendation {
            parameter: "max_rounds".into(),
            current_value: current_max as f64,
            recommended_value: current_max as f64,
            rationale: "No historical data — using default".into(),
        };
    }

    let rounds_needed: Vec<usize> = histories
        .iter()
        .map(|h| analyze_rounds_to_convergence(h))
        .collect();
    let avg = rounds_needed.iter().sum::<usize>() as f64 / rounds_needed.len() as f64;
    let max_needed = *rounds_needed.iter().max().unwrap_or(&(current_max as usize));

    let recommended = std::cmp::max(MIN_MAX_ROUNDS, max_needed as i32 + 1);

    TuningRecommendation {
        parameter: "max_rounds".into(),
        current_value: current_max as f64,
        recommended_value: recommended as f64,
        rationale: format!(
            "Avg rounds: {:.1}, max needed: {}, recommended: {} (with buffer)",
            avg, max_needed, recommended
        ),
    }
}

/// Rank audit dimensions by how many issues they find.
pub fn rank_dimensions_by_yield(histories: &[Vec<RoundResult>]) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for history in histories {
        for r in history {
            for f in &r.findings {
                *counts.entry(f.dimension.clone()).or_default() += 1;
            }
        }
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    ranked
}

/// Validate that convergence threshold meets the safety floor.
pub fn validate_convergence_threshold(threshold: i32) -> TuningRecommendation {
    if threshold < MIN_CONVERGENCE_THRESHOLD {
        TuningRecommendation {
            parameter: "convergence_threshold".into(),
            current_value: threshold as f64,
            recommended_value: MIN_CONVERGENCE_THRESHOLD as f64,
            rationale: format!(
                "Threshold {} is below safety floor of {}",
                threshold, MIN_CONVERGENCE_THRESHOLD
            ),
        }
    } else {
        TuningRecommendation {
            parameter: "convergence_threshold".into(),
            current_value: threshold as f64,
            recommended_value: threshold as f64,
            rationale: format!("Threshold {} meets safety floor", threshold),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::state::{ConvergencePhase, Finding, FindingSeverity, RoundResult};

    fn make_round(findings_count: usize) -> RoundResult {
        let findings: Vec<Finding> = (0..findings_count)
            .map(|i| Finding {
                id: format!("f{}", i),
                file: "test.py".into(),
                dimension: "correctness".into(),
                severity: FindingSeverity::Medium,
                description: "test".into(),
                suggested_fix: "fix".into(),
                fixed: false,
            })
            .collect();
        RoundResult {
            round: 1,
            phase: ConvergencePhase::CodeAuditing,
            findings,
            findings_fixed: 0,
            timestamp: 0.0,
        }
    }

    #[test]
    fn test_recommend_max_rounds_no_history() {
        let rec = recommend_max_rounds(&[], 5);
        assert_eq!(rec.recommended_value, 5.0);
    }

    #[test]
    fn test_recommend_max_rounds_with_history() {
        let histories = vec![
            vec![make_round(1), make_round(0)],
            vec![make_round(1), make_round(1), make_round(0)],
        ];
        let rec = recommend_max_rounds(&histories, 5);
        // max needed = 3, recommended = 4
        assert_eq!(rec.recommended_value, 4.0);
    }

    #[test]
    fn test_validate_threshold_below_floor() {
        let rec = validate_convergence_threshold(1);
        assert_eq!(rec.recommended_value, MIN_CONVERGENCE_THRESHOLD as f64);
    }

    #[test]
    fn test_validate_threshold_ok() {
        let rec = validate_convergence_threshold(3);
        assert_eq!(rec.recommended_value, 3.0);
    }

    #[test]
    fn test_rank_dimensions() {
        let mut r1 = make_round(2);
        r1.findings[0].dimension = "style".into();
        r1.findings[1].dimension = "correctness".into();
        let mut r2 = make_round(1);
        r2.findings[0].dimension = "style".into();
        let ranked = rank_dimensions_by_yield(&[vec![r1, r2]]);
        assert_eq!(ranked[0].0, "style");
        assert_eq!(ranked[0].1, 2);
    }
}
