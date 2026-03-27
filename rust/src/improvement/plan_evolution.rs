//! Build plan template evolution — correlate plan attributes with convergence outcomes.

use serde::{Deserialize, Serialize};

/// A measurable attribute of a build plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAttribute {
    pub name: String,
    pub present: bool,
    #[serde(default)]
    pub description: String,
}

/// Convergence outcome for a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanOutcome {
    pub plan_name: String,
    #[serde(default)]
    pub attributes: Vec<PlanAttribute>,
    #[serde(default)]
    pub convergence_rounds: usize,
    #[serde(default)]
    pub escalated: bool,
}

/// Correlation between a plan attribute and convergence speed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeCorrelation {
    pub attribute_name: String,
    pub plans_with: usize,
    pub plans_without: usize,
    pub avg_rounds_with: f64,
    pub avg_rounds_without: f64,
}

impl AttributeCorrelation {
    /// Negative = attribute reduces rounds (good), positive = increases.
    pub fn impact(&self) -> f64 {
        if self.avg_rounds_without == 0.0 {
            return 0.0;
        }
        self.avg_rounds_with - self.avg_rounds_without
    }
}

/// Correlate plan attributes with convergence outcomes.
pub fn correlate_attributes(outcomes: &[PlanOutcome]) -> Vec<AttributeCorrelation> {
    let mut all_attrs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for outcome in outcomes {
        for attr in &outcome.attributes {
            all_attrs.insert(attr.name.clone());
        }
    }

    let mut correlations = Vec::new();
    for attr_name in &all_attrs {
        let with_attr: Vec<&PlanOutcome> = outcomes
            .iter()
            .filter(|o| {
                o.attributes
                    .iter()
                    .any(|a| a.name == *attr_name && a.present)
            })
            .collect();
        let without_attr: Vec<&PlanOutcome> = outcomes
            .iter()
            .filter(|o| {
                !o.attributes
                    .iter()
                    .any(|a| a.name == *attr_name && a.present)
            })
            .collect();

        let avg_with = if with_attr.is_empty() {
            0.0
        } else {
            let sum: usize = with_attr.iter().map(|o| o.convergence_rounds).sum();
            (sum as f64 / with_attr.len() as f64 * 10.0).round() / 10.0
        };
        let avg_without = if without_attr.is_empty() {
            0.0
        } else {
            let sum: usize = without_attr.iter().map(|o| o.convergence_rounds).sum();
            (sum as f64 / without_attr.len() as f64 * 10.0).round() / 10.0
        };

        correlations.push(AttributeCorrelation {
            attribute_name: attr_name.clone(),
            plans_with: with_attr.len(),
            plans_without: without_attr.len(),
            avg_rounds_with: avg_with,
            avg_rounds_without: avg_without,
        });
    }
    correlations
}

/// Suggest template improvements based on attribute correlations.
pub fn suggest_template_improvements(correlations: &[AttributeCorrelation]) -> Vec<String> {
    let mut suggestions = Vec::new();
    for corr in correlations {
        let impact = corr.impact();
        if impact < -1.0 && corr.plans_with >= 2 {
            suggestions.push(format!(
                "Make '{}' mandatory — reduces rounds by {:.1} on average",
                corr.attribute_name,
                impact.abs()
            ));
        } else if impact > 2.0 && corr.plans_without >= 2 {
            suggestions.push(format!(
                "Reconsider '{}' — increases rounds by {:.1} on average",
                corr.attribute_name, impact
            ));
        }
    }
    suggestions
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_outcome(name: &str, attrs: &[(&str, bool)], rounds: usize) -> PlanOutcome {
        PlanOutcome {
            plan_name: name.into(),
            attributes: attrs
                .iter()
                .map(|(n, p)| PlanAttribute {
                    name: n.to_string(),
                    present: *p,
                    description: String::new(),
                })
                .collect(),
            convergence_rounds: rounds,
            escalated: false,
        }
    }

    #[test]
    fn test_correlate_attributes() {
        let outcomes = vec![
            make_outcome("p1", &[("checklist", true)], 3),
            make_outcome("p2", &[("checklist", true)], 2),
            make_outcome("p3", &[("checklist", false)], 6),
        ];
        let corrs = correlate_attributes(&outcomes);
        assert_eq!(corrs.len(), 1);
        assert_eq!(corrs[0].plans_with, 2);
        assert_eq!(corrs[0].plans_without, 1);
    }

    #[test]
    fn test_suggest_improvements_mandatory() {
        let corrs = vec![AttributeCorrelation {
            attribute_name: "checklist".into(),
            plans_with: 3,
            plans_without: 3,
            avg_rounds_with: 2.0,
            avg_rounds_without: 5.0,
        }];
        let suggestions = suggest_template_improvements(&corrs);
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].contains("mandatory"));
    }

    #[test]
    fn test_suggest_improvements_reconsider() {
        let corrs = vec![AttributeCorrelation {
            attribute_name: "verbose_specs".into(),
            plans_with: 3,
            plans_without: 3,
            avg_rounds_with: 8.0,
            avg_rounds_without: 3.0,
        }];
        let suggestions = suggest_template_improvements(&corrs);
        assert_eq!(suggestions.len(), 1);
        assert!(suggestions[0].contains("Reconsider"));
    }
}
