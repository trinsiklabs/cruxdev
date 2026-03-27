//! Research convergence detection — 5-pass, novelty-scored, budget-aware.

use super::session::ResearchSession;
use serde::{Deserialize, Serialize};

/// Result of a convergence check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceCheck {
    pub converged: bool,
    pub budget_exhausted: bool,
    pub reason: String,
    pub novelty_avg: f64,
    pub coverage_ok: bool,
    pub contradictions_resolved: bool,
}

/// Calculate novelty ratio: new unique facts / total facts.
pub fn calculate_novelty(new_facts: usize, total_facts: usize) -> f64 {
    if total_facts == 0 {
        return 1.0;
    }
    new_facts as f64 / total_facts as f64
}

/// Check if a research session has converged.
pub fn check_research_convergence(
    session: &ResearchSession,
    max_searches: u32,
    min_sources_per_question: usize,
    novelty_threshold: f64,
) -> ConvergenceCheck {
    // Budget exhaustion — hard stop
    if session.total_searches >= max_searches {
        return ConvergenceCheck {
            converged: true,
            budget_exhausted: true,
            reason: format!("Budget exhausted: {}/{} searches used", session.total_searches, max_searches),
            novelty_avg: avg_novelty(&session.novelty_scores),
            coverage_ok: true,
            contradictions_resolved: true,
        };
    }

    // Coverage check: do we have enough sources per question?
    let unique_sources = session.seen_urls.len();
    let questions = session.sub_questions.len().max(1);
    let coverage_ok = unique_sources >= questions * min_sources_per_question;

    // Novelty check: are recent searches producing diminishing returns?
    let recent_novelty = if session.novelty_scores.len() >= 5 {
        avg_novelty(&session.novelty_scores[session.novelty_scores.len() - 5..])
    } else {
        avg_novelty(&session.novelty_scores)
    };
    let novelty_ok = !session.novelty_scores.is_empty() && recent_novelty < novelty_threshold;

    // Contradiction check: any contested findings without counter-evidence?
    let unresolved = session
        .findings
        .iter()
        .filter(|f| f.robustness == "contested" && f.counter_evidence.is_empty())
        .count();
    let contradictions_resolved = unresolved == 0;

    // Natural convergence: all three criteria met
    if coverage_ok && novelty_ok && contradictions_resolved {
        return ConvergenceCheck {
            converged: true,
            budget_exhausted: false,
            reason: format!(
                "Natural convergence: coverage={}/{}, novelty={:.2}<{:.2}, contradictions=0",
                unique_sources, questions * min_sources_per_question,
                recent_novelty, novelty_threshold,
            ),
            novelty_avg: recent_novelty,
            coverage_ok,
            contradictions_resolved,
        };
    }

    // Not converged — provide detailed reason
    let mut reasons = Vec::new();
    if !coverage_ok {
        reasons.push(format!(
            "coverage ({}/{})",
            unique_sources,
            questions * min_sources_per_question
        ));
    }
    if !novelty_ok {
        reasons.push(format!("novelty ({:.2} > {:.2})", recent_novelty, novelty_threshold));
    }
    if !contradictions_resolved {
        reasons.push(format!("{unresolved} unresolved contradictions"));
    }

    ConvergenceCheck {
        converged: false,
        budget_exhausted: false,
        reason: format!("Not converged: {}", reasons.join(", ")),
        novelty_avg: recent_novelty,
        coverage_ok,
        contradictions_resolved,
    }
}

fn avg_novelty(scores: &[f64]) -> f64 {
    if scores.is_empty() {
        return 1.0;
    }
    scores.iter().sum::<f64>() / scores.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::research::session::{create_session, ResearchFinding};

    #[test]
    fn test_calculate_novelty() {
        assert_eq!(calculate_novelty(5, 10), 0.5);
        assert_eq!(calculate_novelty(0, 10), 0.0);
        assert_eq!(calculate_novelty(0, 0), 1.0);
    }

    #[test]
    fn test_budget_exhaustion() {
        let mut session = create_session("test", None);
        session.total_searches = 50;
        let check = check_research_convergence(&session, 50, 3, 0.10);
        assert!(check.converged);
        assert!(check.budget_exhausted);
    }

    #[test]
    fn test_natural_convergence() {
        let mut session = create_session("test", Some(vec!["q1".into()]));
        session.seen_urls = (0..5).map(|i| format!("https://source{i}.com")).collect();
        session.novelty_scores = vec![0.1, 0.08, 0.05, 0.03, 0.02];
        session.total_searches = 10;

        let check = check_research_convergence(&session, 50, 3, 0.10);
        assert!(check.converged);
        assert!(!check.budget_exhausted);
        assert!(check.coverage_ok);
    }

    #[test]
    fn test_not_converged_low_coverage() {
        let mut session = create_session("test", Some(vec!["q1".into(), "q2".into()]));
        session.seen_urls = vec!["https://one.com".into()];
        session.novelty_scores = vec![0.05, 0.02];
        session.total_searches = 5;

        let check = check_research_convergence(&session, 50, 3, 0.10);
        assert!(!check.converged);
        assert!(!check.coverage_ok);
        assert!(check.reason.contains("coverage"));
    }

    #[test]
    fn test_not_converged_high_novelty() {
        let mut session = create_session("test", Some(vec!["q1".into()]));
        session.seen_urls = (0..5).map(|i| format!("https://s{i}.com")).collect();
        session.novelty_scores = vec![0.8, 0.7, 0.6, 0.5, 0.4];
        session.total_searches = 10;

        let check = check_research_convergence(&session, 50, 3, 0.10);
        assert!(!check.converged);
        assert!(check.reason.contains("novelty"));
    }

    #[test]
    fn test_not_converged_unresolved_contradictions() {
        let mut session = create_session("test", Some(vec!["q1".into()]));
        session.seen_urls = (0..5).map(|i| format!("https://s{i}.com")).collect();
        session.novelty_scores = vec![0.05, 0.03, 0.02, 0.01, 0.01];
        session.total_searches = 10;
        session.findings.push(ResearchFinding {
            robustness: "contested".into(),
            counter_evidence: Vec::new(), // unresolved
            ..ResearchFinding::new("f1", "claim", "https://src.com")
        });

        let check = check_research_convergence(&session, 50, 3, 0.10);
        assert!(!check.converged);
        assert!(check.reason.contains("contradictions"));
    }
}
