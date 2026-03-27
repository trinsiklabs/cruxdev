//! Meta-analysis — analyze convergence data to improve methodology.

use serde::{Deserialize, Serialize};

/// Data from a single convergence run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceDataPoint {
    pub plan_name: String,
    pub total_rounds: usize,
    pub total_findings: usize,
    pub duration_seconds: f64,
    #[serde(default)]
    pub phases_used: Vec<String>,
    #[serde(default)]
    pub escalated: bool,
}

/// An insight from meta-analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodologyInsight {
    pub element: String,
    pub correlation: String,
    pub evidence: String,
    pub suggestion: String,
}

/// Analyze convergence history for methodology improvement signals.
pub fn analyze_convergence_history(
    data_points: &[ConvergenceDataPoint],
) -> Vec<MethodologyInsight> {
    let mut insights = Vec::new();

    if data_points.is_empty() {
        return insights;
    }

    let avg_rounds =
        data_points.iter().map(|d| d.total_rounds).sum::<usize>() as f64
            / data_points.len() as f64;

    let escalated = data_points.iter().filter(|d| d.escalated).count();
    let escalation_rate = escalated as f64 / data_points.len() as f64;

    if escalation_rate > 0.3 {
        insights.push(MethodologyInsight {
            element: "max_rounds".into(),
            correlation: "negative".into(),
            evidence: format!("{:.0}% of runs escalated", escalation_rate * 100.0),
            suggestion:
                "Consider increasing max_rounds or breaking plans into smaller units".into(),
        });
    }

    if avg_rounds < 2.0 {
        insights.push(MethodologyInsight {
            element: "plan_granularity".into(),
            correlation: "positive".into(),
            evidence: format!("Average {:.1} rounds suggests plans are too simple", avg_rounds),
            suggestion: "Plans may not be challenging enough — add more audit dimensions".into(),
        });
    }

    // Findings per round
    let total_findings: usize = data_points.iter().map(|d| d.total_findings).sum();
    let total_rounds: usize = data_points.iter().map(|d| d.total_rounds).sum();
    if total_rounds > 0 {
        let findings_per_round = total_findings as f64 / total_rounds as f64;
        if findings_per_round > 10.0 {
            insights.push(MethodologyInsight {
                element: "audit_thoroughness".into(),
                correlation: "positive".into(),
                evidence: format!("{:.1} findings per round", findings_per_round),
                suggestion: "Audit dimensions are effective at finding issues".into(),
            });
        }
    }

    insights
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_data_points() {
        let insights = analyze_convergence_history(&[]);
        assert!(insights.is_empty());
    }

    #[test]
    fn test_high_escalation_rate() {
        let data = vec![
            ConvergenceDataPoint {
                plan_name: "plan1".into(),
                total_rounds: 5,
                total_findings: 10,
                duration_seconds: 100.0,
                phases_used: vec![],
                escalated: true,
            },
            ConvergenceDataPoint {
                plan_name: "plan2".into(),
                total_rounds: 5,
                total_findings: 8,
                duration_seconds: 80.0,
                phases_used: vec![],
                escalated: true,
            },
        ];
        let insights = analyze_convergence_history(&data);
        assert!(insights.iter().any(|i| i.element == "max_rounds"));
    }

    #[test]
    fn test_low_avg_rounds() {
        let data = vec![ConvergenceDataPoint {
            plan_name: "easy".into(),
            total_rounds: 1,
            total_findings: 0,
            duration_seconds: 10.0,
            phases_used: vec![],
            escalated: false,
        }];
        let insights = analyze_convergence_history(&data);
        assert!(insights.iter().any(|i| i.element == "plan_granularity"));
    }

    #[test]
    fn test_high_findings_per_round() {
        let data = vec![ConvergenceDataPoint {
            plan_name: "thorough".into(),
            total_rounds: 2,
            total_findings: 30,
            duration_seconds: 200.0,
            phases_used: vec![],
            escalated: false,
        }];
        let insights = analyze_convergence_history(&data);
        assert!(insights.iter().any(|i| i.element == "audit_thoroughness"));
    }
}
