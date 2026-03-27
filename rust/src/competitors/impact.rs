//! Competitive impact detection — classify whether a build plan changes the competitive landscape.

use std::fs;
use serde::{Deserialize, Serialize};

/// Type of competitive impact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompetitiveImpact {
    /// New capability no competitor has
    Differentiator,
    /// Closes a gap from gap analysis
    GapClosure,
    /// Achieves parity with a competitor on a feature
    Parity,
    /// No competitive relevance
    None,
}

impl CompetitiveImpact {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Differentiator => "differentiator",
            Self::GapClosure => "gap_closure",
            Self::Parity => "parity",
            Self::None => "none",
        }
    }
}

/// Result of impact classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactResult {
    pub impact: CompetitiveImpact,
    pub reason: String,
    pub affected_gaps: Vec<String>,
    pub affected_competitors: Vec<String>,
    pub recommended_actions: Vec<String>,
}

/// Classify the competitive impact of a build plan.
pub fn classify_impact(plan_content: &str, gap_features: &[String], competitor_features: &[String]) -> ImpactResult {
    let lower = plan_content.to_lowercase();

    // Check for gap closure — does the plan mention any known gap features?
    let mut closed_gaps = Vec::new();
    for gap in gap_features {
        if lower.contains(&gap.to_lowercase()) {
            closed_gaps.push(gap.clone());
        }
    }

    if !closed_gaps.is_empty() {
        return ImpactResult {
            impact: CompetitiveImpact::GapClosure,
            reason: format!("Closes gaps: {}", closed_gaps.join(", ")),
            affected_gaps: closed_gaps,
            affected_competitors: Vec::new(),
            recommended_actions: vec![
                "Update gap status in COMPETITORS.md".into(),
                "Refresh gap analysis".into(),
                "Regenerate affected vs/ pages".into(),
                "Update feature matrix".into(),
            ],
        };
    }

    // Check for parity — does the plan implement something a competitor already has?
    let mut parity_features = Vec::new();
    for feat in competitor_features {
        if lower.contains(&feat.to_lowercase()) {
            parity_features.push(feat.clone());
        }
    }

    if !parity_features.is_empty() {
        return ImpactResult {
            impact: CompetitiveImpact::Parity,
            reason: format!("Achieves parity on: {}", parity_features.join(", ")),
            affected_gaps: Vec::new(),
            affected_competitors: Vec::new(),
            recommended_actions: vec![
                "Update feature matrix in COMPETITORS.md".into(),
                "Regenerate affected vs/ pages".into(),
            ],
        };
    }

    // Check for differentiator keywords
    let differentiator_signals = [
        "new capability", "no competitor", "first to", "unique", "differentiator",
        "moat", "nobody else", "competitive advantage",
    ];

    for signal in &differentiator_signals {
        if lower.contains(signal) {
            return ImpactResult {
                impact: CompetitiveImpact::Differentiator,
                reason: format!("Contains differentiator signal: '{signal}'"),
                affected_gaps: Vec::new(),
                affected_competitors: Vec::new(),
                recommended_actions: vec![
                    "Add to 'Our moat vs them' in COMPETITORS.md".into(),
                    "Regenerate all vs/ pages".into(),
                    "Update llms.txt with new capability".into(),
                    "Generate social post about competitive advantage".into(),
                ],
            };
        }
    }

    // Check for capability keywords that likely change competitive position
    let capability_signals = [
        "integration", "platform", "multi-platform", "new mcp tool", "new dimension",
        "methodology", "pattern", "audit", "safety gate", "autonomous",
    ];

    for signal in &capability_signals {
        if lower.contains(signal) {
            return ImpactResult {
                impact: CompetitiveImpact::Differentiator,
                reason: format!("New capability: '{signal}'"),
                affected_gaps: Vec::new(),
                affected_competitors: Vec::new(),
                recommended_actions: vec![
                    "Evaluate if competitors have this".into(),
                    "Update COMPETITORS.md moat section if unique".into(),
                    "Update feature matrix".into(),
                ],
            };
        }
    }

    ImpactResult {
        impact: CompetitiveImpact::None,
        reason: "No competitive impact detected".into(),
        affected_gaps: Vec::new(),
        affected_competitors: Vec::new(),
        recommended_actions: Vec::new(),
    }
}

/// Scan all converged build plans for competitive impact.
pub fn scan_plans_for_impact(
    plans_dir: &str,
    gap_features: &[String],
    competitor_features: &[String],
) -> Vec<(String, ImpactResult)> {
    let mut results = Vec::new();

    let entries = match fs::read_dir(plans_dir) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.to_string_lossy().ends_with(".md") {
            continue;
        }

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        // Only check converged plans
        if !content.contains("CONVERGED") {
            continue;
        }

        let impact = classify_impact(&content, gap_features, competitor_features);
        if impact.impact != CompetitiveImpact::None {
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            results.push((name, impact));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_closure_detection() {
        let plan = "This plan implements multi-platform binaries for ARM and Intel.";
        let gaps = vec!["multi-platform binaries".into()];
        let result = classify_impact(plan, &gaps, &[]);
        assert_eq!(result.impact, CompetitiveImpact::GapClosure);
        assert!(result.affected_gaps.contains(&"multi-platform binaries".to_string()));
    }

    #[test]
    fn test_differentiator_detection() {
        let plan = "This adds a unique capability that no competitor has.";
        let result = classify_impact(plan, &[], &[]);
        assert_eq!(result.impact, CompetitiveImpact::Differentiator);
    }

    #[test]
    fn test_parity_detection() {
        let plan = "Implement GitHub Actions support for CI/CD.";
        let competitor_feats = vec!["GitHub Actions".into()];
        let result = classify_impact(plan, &[], &competitor_feats);
        assert_eq!(result.impact, CompetitiveImpact::Parity);
    }

    #[test]
    fn test_no_impact() {
        let plan = "Fix a typo in the README.";
        let result = classify_impact(plan, &[], &[]);
        assert_eq!(result.impact, CompetitiveImpact::None);
    }

    #[test]
    fn test_capability_signal() {
        let plan = "Add new MCP tool for build freshness checking.";
        let result = classify_impact(plan, &[], &[]);
        assert_eq!(result.impact, CompetitiveImpact::Differentiator);
    }

    #[test]
    fn test_scan_plans() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("BUILD_PLAN_001.md"),
            "**Status:** CONVERGED\n\nThis adds a new integration platform.",
        ).unwrap();
        fs::write(
            dir.path().join("BUILD_PLAN_002.md"),
            "**Status:** NOT STARTED\n\nFix typo.",
        ).unwrap();

        let results = scan_plans_for_impact(dir.path().to_str().unwrap(), &[], &[]);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1.impact, CompetitiveImpact::Differentiator);
    }
}
