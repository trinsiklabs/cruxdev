//! Convergence report generator — human-readable summaries of convergence runs.

use super::state::ConvergenceState;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConvergenceReport {
    pub plan_file: String,
    pub status: String,
    pub total_rounds: i32,
    pub phases_traversed: Vec<String>,
    pub total_findings: usize,
    pub total_fixed: usize,
    pub findings_by_dimension: Vec<(String, usize)>,
    pub timeline: Vec<RoundSummary>,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoundSummary {
    pub round: i32,
    pub phase: String,
    pub findings_count: usize,
    pub findings_fixed: usize,
    pub dimensions_with_findings: Vec<String>,
}

/// Generate a convergence report from state.
pub fn generate_report(state: &ConvergenceState) -> ConvergenceReport {
    let mut phases = Vec::new();
    let mut findings_by_dim: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut timeline = Vec::new();
    let mut total_findings = 0;
    let mut total_fixed = 0;

    for round in &state.history {
        let phase_name = format!("{:?}", round.phase);
        if !phases.contains(&phase_name) {
            phases.push(phase_name.clone());
        }

        let mut dims_with_findings = Vec::new();
        for finding in &round.findings {
            *findings_by_dim.entry(finding.dimension.clone()).or_insert(0) += 1;
            if !dims_with_findings.contains(&finding.dimension) {
                dims_with_findings.push(finding.dimension.clone());
            }
        }

        total_findings += round.findings.len();
        total_fixed += round.findings_fixed;

        timeline.push(RoundSummary {
            round: round.round,
            phase: phase_name,
            findings_count: round.findings.len(),
            findings_fixed: round.findings_fixed,
            dimensions_with_findings: dims_with_findings,
        });
    }

    let mut sorted_dims: Vec<(String, usize)> = findings_by_dim.into_iter().collect();
    sorted_dims.sort_by(|a, b| b.1.cmp(&a.1));

    let status = format!("{:?}", state.phase);
    let duration = state.updated_at - state.created_at;

    ConvergenceReport {
        plan_file: state.plan_file.clone(),
        status,
        total_rounds: state.round,
        phases_traversed: phases,
        total_findings,
        total_fixed,
        findings_by_dimension: sorted_dims,
        timeline,
        duration_seconds: duration,
    }
}

/// Generate a markdown report.
pub fn to_markdown(report: &ConvergenceReport) -> String {
    let mut lines = vec![
        format!("# Convergence Report: {}", report.plan_file),
        String::new(),
        format!("**Status:** {}", report.status),
        format!("**Rounds:** {}", report.total_rounds),
        format!("**Duration:** {:.1}s", report.duration_seconds),
        format!("**Findings:** {} discovered, {} fixed", report.total_findings, report.total_fixed),
        String::new(),
        "## Phases Traversed".into(),
        String::new(),
    ];

    for (i, phase) in report.phases_traversed.iter().enumerate() {
        lines.push(format!("{}. {}", i + 1, phase));
    }

    if !report.findings_by_dimension.is_empty() {
        lines.push(String::new());
        lines.push("## Findings by Dimension".into());
        lines.push(String::new());
        lines.push("| Dimension | Count |".into());
        lines.push("|---|---|".into());
        for (dim, count) in &report.findings_by_dimension {
            lines.push(format!("| {} | {} |", dim, count));
        }
    }

    if !report.timeline.is_empty() {
        lines.push(String::new());
        lines.push("## Timeline".into());
        lines.push(String::new());
        for round in &report.timeline {
            let dims = if round.dimensions_with_findings.is_empty() {
                "clean".into()
            } else {
                round.dimensions_with_findings.join(", ")
            };
            lines.push(format!(
                "- **Round {}** ({}): {} findings, {} fixed [{}]",
                round.round, round.phase, round.findings_count, round.findings_fixed, dims
            ));
        }
    }

    lines.join("\n")
}

/// Generate a one-line summary for a round.
pub fn round_summary_line(round: &RoundSummary) -> String {
    if round.findings_count == 0 {
        format!("Round {}: clean pass ({})", round.round, round.phase)
    } else {
        format!(
            "Round {}: {} findings in {}, {} fixed ({})",
            round.round,
            round.findings_count,
            round.dimensions_with_findings.join(", "),
            round.findings_fixed,
            round.phase,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::state::*;

    #[test]
    fn test_empty_report() {
        let state = ConvergenceState::new("plan.md".into());
        let report = generate_report(&state);
        assert_eq!(report.total_rounds, 0);
        assert_eq!(report.total_findings, 0);
        assert!(report.phases_traversed.is_empty());
    }

    #[test]
    fn test_report_with_history() {
        let mut state = ConvergenceState::new("plan.md".into());
        state.round = 2;
        state.history.push(RoundResult {
            round: 1,
            phase: ConvergencePhase::CodeAuditing,
            findings: vec![Finding {
                id: "1".into(),
                file: "main.rs".into(),
                dimension: "security".into(),
                severity: FindingSeverity::High,
                description: "SQL injection".into(),
                suggested_fix: "use parameterized".into(),
                fixed: true,
            }],
            findings_fixed: 1,
            timestamp: 100.0,
        });
        state.history.push(RoundResult {
            round: 2,
            phase: ConvergencePhase::CodeAuditing,
            findings: vec![],
            findings_fixed: 0,
            timestamp: 200.0,
        });

        let report = generate_report(&state);
        assert_eq!(report.total_findings, 1);
        assert_eq!(report.total_fixed, 1);
        assert_eq!(report.timeline.len(), 2);
        assert_eq!(report.findings_by_dimension[0], ("security".into(), 1));
    }

    #[test]
    fn test_markdown_output() {
        let state = ConvergenceState::new("plan.md".into());
        let report = generate_report(&state);
        let md = to_markdown(&report);
        assert!(md.contains("# Convergence Report"));
        assert!(md.contains("**Status:**"));
    }

    #[test]
    fn test_round_summary_clean() {
        let round = RoundSummary {
            round: 3,
            phase: "CodeAuditing".into(),
            findings_count: 0,
            findings_fixed: 0,
            dimensions_with_findings: vec![],
        };
        assert_eq!(round_summary_line(&round), "Round 3: clean pass (CodeAuditing)");
    }
}
