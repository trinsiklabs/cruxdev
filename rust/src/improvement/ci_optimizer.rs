//! CI/CD self-optimization — analyze workflow timing, identify improvements.

use serde::{Deserialize, Serialize};

/// A single CI workflow step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub duration_seconds: f64,
    #[serde(default)]
    pub cacheable: bool,
    #[serde(default)]
    pub parallelizable: bool,
}

/// A suggested CI optimization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub step_name: String,
    pub suggestion_type: String,
    pub description: String,
    #[serde(default)]
    pub estimated_savings_seconds: f64,
}

/// Result of CI workflow analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIAnalysis {
    pub total_duration: f64,
    pub steps: Vec<WorkflowStep>,
    pub suggestions: Vec<OptimizationSuggestion>,
}

impl CIAnalysis {
    pub fn potential_savings(&self) -> f64 {
        self.suggestions
            .iter()
            .map(|s| s.estimated_savings_seconds)
            .sum()
    }
}

/// Cache-related keywords in step names.
const CACHE_KEYWORDS: &[&str] = &["install", "dependencies", "setup"];

/// Analyze a CI workflow for optimization opportunities.
pub fn analyze_workflow(steps: Vec<WorkflowStep>) -> CIAnalysis {
    let total_duration: f64 = steps.iter().map(|s| s.duration_seconds).sum();
    let mut suggestions = Vec::new();

    for step in &steps {
        let name_lower = step.name.to_lowercase();

        // Cacheable detection
        if CACHE_KEYWORDS.iter().any(|kw| name_lower.contains(kw)) && !step.cacheable {
            suggestions.push(OptimizationSuggestion {
                step_name: step.name.clone(),
                suggestion_type: "cache".into(),
                description: format!("Cache {} to avoid re-running", step.name),
                estimated_savings_seconds: step.duration_seconds * 0.8,
            });
        }

        // Parallelizable detection
        if step.parallelizable && step.duration_seconds > 30.0 {
            suggestions.push(OptimizationSuggestion {
                step_name: step.name.clone(),
                suggestion_type: "parallelize".into(),
                description: format!("Run {} in parallel with other steps", step.name),
                estimated_savings_seconds: step.duration_seconds * 0.5,
            });
        }

        // Slow step warning
        if step.duration_seconds > 120.0 {
            suggestions.push(OptimizationSuggestion {
                step_name: step.name.clone(),
                suggestion_type: "optimize".into(),
                description: format!(
                    "{} takes {:.0}s — investigate optimization",
                    step.name, step.duration_seconds
                ),
                estimated_savings_seconds: step.duration_seconds * 0.3,
            });
        }
    }

    CIAnalysis {
        total_duration,
        steps,
        suggestions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_empty_workflow() {
        let analysis = analyze_workflow(vec![]);
        assert_eq!(analysis.total_duration, 0.0);
        assert!(analysis.suggestions.is_empty());
    }

    #[test]
    fn test_analyze_cacheable_step() {
        let steps = vec![WorkflowStep {
            name: "Install dependencies".into(),
            duration_seconds: 45.0,
            cacheable: false,
            parallelizable: false,
        }];
        let analysis = analyze_workflow(steps);
        assert_eq!(analysis.suggestions.len(), 1);
        assert_eq!(analysis.suggestions[0].suggestion_type, "cache");
    }

    #[test]
    fn test_analyze_parallelizable_step() {
        let steps = vec![WorkflowStep {
            name: "Run unit tests".into(),
            duration_seconds: 60.0,
            cacheable: false,
            parallelizable: true,
        }];
        let analysis = analyze_workflow(steps);
        assert_eq!(analysis.suggestions.len(), 1);
        assert_eq!(analysis.suggestions[0].suggestion_type, "parallelize");
    }

    #[test]
    fn test_analyze_slow_step() {
        let steps = vec![WorkflowStep {
            name: "Build project".into(),
            duration_seconds: 200.0,
            cacheable: false,
            parallelizable: false,
        }];
        let analysis = analyze_workflow(steps);
        assert_eq!(analysis.suggestions.len(), 1);
        assert_eq!(analysis.suggestions[0].suggestion_type, "optimize");
    }

    #[test]
    fn test_potential_savings() {
        let analysis = CIAnalysis {
            total_duration: 300.0,
            steps: vec![],
            suggestions: vec![
                OptimizationSuggestion {
                    step_name: "a".into(),
                    suggestion_type: "cache".into(),
                    description: "".into(),
                    estimated_savings_seconds: 30.0,
                },
                OptimizationSuggestion {
                    step_name: "b".into(),
                    suggestion_type: "parallelize".into(),
                    description: "".into(),
                    estimated_savings_seconds: 20.0,
                },
            ],
        };
        assert_eq!(analysis.potential_savings(), 50.0);
    }
}
