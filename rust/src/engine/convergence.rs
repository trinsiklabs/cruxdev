//! Core convergence loop logic — all deterministic, no async, no LLM.

use super::state::*;

/// Phase execution order (excludes terminal states).
pub const PHASE_ORDER: &[ConvergencePhase] = &[
    ConvergencePhase::Planning,
    ConvergencePhase::PlanAuditing,
    ConvergencePhase::DocAlignment,
    ConvergencePhase::Viability,
    ConvergencePhase::Executing,
    ConvergencePhase::CodeAuditing,
    ConvergencePhase::DocAuditing,
    ConvergencePhase::WebsiteConvergence,
    ConvergencePhase::E2eTesting,
    ConvergencePhase::PatternsUpdate,
    ConvergencePhase::Converged,
];

/// Move to the next phase in the sequence.
pub fn advance_phase(current: ConvergencePhase) -> ConvergencePhase {
    let idx = PHASE_ORDER.iter().position(|&p| p == current);
    match idx {
        Some(i) if i + 1 < PHASE_ORDER.len() => PHASE_ORDER[i + 1],
        _ => current,
    }
}

/// Check if phase is terminal.
pub fn is_terminal(phase: ConvergencePhase) -> bool {
    matches!(phase, ConvergencePhase::Converged | ConvergencePhase::Escalated)
}

/// Two consecutive clean passes.
pub fn check_convergence(state: &ConvergenceState) -> bool {
    state.consecutive_clean >= state.convergence_threshold
}

/// Check if max rounds exceeded for the current phase.
pub fn check_max_rounds(state: &ConvergenceState) -> bool {
    state.round >= state.max_rounds
}

/// Compare last 3 rounds — if findings increasing twice consecutively, net negative.
pub fn check_net_negative(state: &ConvergenceState) -> bool {
    if state.history.len() < 3 {
        return false;
    }
    let n = state.history.len();
    let last = state.history[n - 1].findings.len();
    let prev = state.history[n - 2].findings.len();
    let prev_prev = state.history[n - 3].findings.len();
    last > prev && prev > prev_prev
}

/// Record a round's results.
pub fn record_round(state: &mut ConvergenceState, findings: Vec<Finding>) {
    let result = RoundResult {
        round: state.round,
        phase: state.phase,
        findings_fixed: findings.iter().filter(|f| f.fixed).count(),
        findings,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64(),
    };
    let clean = result.findings.is_empty();
    state.history.push(result);

    if clean {
        state.consecutive_clean += 1;
    } else {
        state.consecutive_clean = 0;
    }
    state.round += 1;
}

/// Move to escalated state.
pub fn escalate(state: &mut ConvergenceState, reason: &str) {
    state.phase = ConvergencePhase::Escalated;
    state.escalation_reason = Some(reason.to_string());
}

/// Check if same task has failed too many times.
pub fn should_rollback(state: &ConvergenceState, task_id: &str) -> bool {
    state.failures.get(task_id).copied().unwrap_or(0) >= state.max_failures
}

/// Record a task failure.
pub fn record_failure(state: &mut ConvergenceState, task_id: &str) {
    let count = state.failures.entry(task_id.to_string()).or_insert(0);
    *count += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_order_length() {
        assert_eq!(PHASE_ORDER.len(), 11);
        assert_eq!(PHASE_ORDER[0], ConvergencePhase::Planning);
        assert_eq!(PHASE_ORDER[10], ConvergencePhase::Converged);
    }

    #[test]
    fn advance_phase_normal() {
        assert_eq!(advance_phase(ConvergencePhase::Planning), ConvergencePhase::PlanAuditing);
        assert_eq!(advance_phase(ConvergencePhase::CodeAuditing), ConvergencePhase::DocAuditing);
    }

    #[test]
    fn advance_phase_at_end() {
        assert_eq!(advance_phase(ConvergencePhase::Converged), ConvergencePhase::Converged);
    }

    #[test]
    fn advance_phase_escalated_stays() {
        assert_eq!(advance_phase(ConvergencePhase::Escalated), ConvergencePhase::Escalated);
    }

    #[test]
    fn is_terminal_true() {
        assert!(is_terminal(ConvergencePhase::Converged));
        assert!(is_terminal(ConvergencePhase::Escalated));
    }

    #[test]
    fn is_terminal_false() {
        assert!(!is_terminal(ConvergencePhase::Planning));
        assert!(!is_terminal(ConvergencePhase::CodeAuditing));
    }

    #[test]
    fn check_convergence_threshold() {
        let mut state = ConvergenceState::new("p.md".into());
        assert!(!check_convergence(&state));
        state.consecutive_clean = 1;
        assert!(!check_convergence(&state));
        state.consecutive_clean = 2;
        assert!(check_convergence(&state));
    }

    #[test]
    fn check_max_rounds_hit() {
        let mut state = ConvergenceState::new("p.md".into());
        state.max_rounds = 5;
        state.round = 4;
        assert!(!check_max_rounds(&state));
        state.round = 5;
        assert!(check_max_rounds(&state));
    }

    #[test]
    fn net_negative_detection() {
        let mut state = ConvergenceState::new("p.md".into());
        // Need 3 rounds with increasing findings
        let f = |n: usize| -> Vec<Finding> {
            (0..n).map(|i| Finding {
                id: format!("f{i}"),
                file: "a.rs".into(),
                dimension: "correctness".into(),
                severity: FindingSeverity::Medium,
                description: "issue".into(),
                suggested_fix: "fix".into(),
                fixed: false,
            }).collect()
        };
        record_round(&mut state, f(1));
        record_round(&mut state, f(2));
        record_round(&mut state, f(3));
        assert!(check_net_negative(&state));
    }

    #[test]
    fn net_negative_not_enough_rounds() {
        let state = ConvergenceState::new("p.md".into());
        assert!(!check_net_negative(&state));
    }

    #[test]
    fn record_round_clean() {
        let mut state = ConvergenceState::new("p.md".into());
        record_round(&mut state, vec![]);
        assert_eq!(state.consecutive_clean, 1);
        assert_eq!(state.round, 1);
    }

    #[test]
    fn record_round_with_findings() {
        let mut state = ConvergenceState::new("p.md".into());
        state.consecutive_clean = 1;
        record_round(&mut state, vec![Finding {
            id: "f1".into(), file: "a.rs".into(), dimension: "x".into(),
            severity: FindingSeverity::Low, description: "d".into(),
            suggested_fix: "s".into(), fixed: false,
        }]);
        assert_eq!(state.consecutive_clean, 0);
    }

    #[test]
    fn escalate_sets_reason() {
        let mut state = ConvergenceState::new("p.md".into());
        escalate(&mut state, "timeout");
        assert_eq!(state.phase, ConvergencePhase::Escalated);
        assert_eq!(state.escalation_reason.as_deref(), Some("timeout"));
    }

    #[test]
    fn rollback_tracking() {
        let mut state = ConvergenceState::new("p.md".into());
        assert!(!should_rollback(&state, "t1"));
        record_failure(&mut state, "t1");
        record_failure(&mut state, "t1");
        assert!(!should_rollback(&state, "t1"));
        record_failure(&mut state, "t1");
        assert!(should_rollback(&state, "t1"));
    }
}
