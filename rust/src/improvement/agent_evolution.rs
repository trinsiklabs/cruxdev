//! Agent evolution — maintain and evolve agent configuration variants.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Protected invariants that cannot be modified by evolution.
pub const PROTECTED_INVARIANTS: &[&str] = &[
    "convergence_threshold",
    "safety_pipeline",
    "human_escalation",
    "test_requirement",
    "max_rounds_floor",
];

/// An agent configuration variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub fitness: f64,
}

impl AgentConfig {
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.params.get(key)
    }
}

/// A population of agent config variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Population {
    #[serde(default)]
    pub variants: Vec<AgentConfig>,
    #[serde(default = "default_max_size")]
    pub max_size: usize,
}

fn default_max_size() -> usize {
    10
}

impl Default for Population {
    fn default() -> Self {
        Self {
            variants: Vec::new(),
            max_size: 10,
        }
    }
}

impl Population {
    pub fn best(&self) -> Option<&AgentConfig> {
        self.variants
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
    }

    pub fn add(&mut self, variant: AgentConfig) -> bool {
        if self.variants.len() >= self.max_size {
            return false;
        }
        self.variants.push(variant);
        true
    }

    pub fn select(&mut self, keep_count: usize) -> &[AgentConfig] {
        self.variants
            .sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        self.variants.truncate(keep_count);
        &self.variants
    }
}

/// Check that protected invariants are not weakened. Returns list of violations.
pub fn validate_invariants(config: &AgentConfig) -> Vec<String> {
    let mut violations = Vec::new();

    if let Some(val) = config.get("convergence_threshold")
        && let Some(n) = val.as_i64()
        && n < 2
    {
        violations.push(format!("convergence_threshold={} < minimum 2", n));
    }

    if let Some(val) = config.get("max_rounds_floor")
        && let Some(n) = val.as_i64()
        && n < 3
    {
        violations.push(format!("max_rounds_floor={} < minimum 3", n));
    }

    if let Some(val) = config.get("safety_pipeline")
        && val == &serde_json::Value::Bool(false)
    {
        violations.push("safety_pipeline cannot be disabled".into());
    }

    if let Some(val) = config.get("human_escalation")
        && val == &serde_json::Value::Bool(false)
    {
        violations.push("human_escalation cannot be disabled".into());
    }

    violations
}

/// Create a mutated variant from a base config. Returns None if mutation violates invariants.
pub fn mutate_config(
    base: &AgentConfig,
    mutations: HashMap<String, serde_json::Value>,
    new_id: &str,
) -> Option<AgentConfig> {
    let mut new_params = base.params.clone();
    new_params.extend(mutations);

    let variant = AgentConfig {
        id: new_id.to_string(),
        name: format!("{}_mutated", base.name),
        params: new_params,
        fitness: 0.0,
    };

    let violations = validate_invariants(&variant);
    if violations.is_empty() {
        Some(variant)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_invariants_ok() {
        let config = AgentConfig {
            id: "c1".into(),
            name: "test".into(),
            params: HashMap::from([
                ("convergence_threshold".into(), serde_json::json!(3)),
                ("safety_pipeline".into(), serde_json::json!(true)),
            ]),
            fitness: 0.0,
        };
        assert!(validate_invariants(&config).is_empty());
    }

    #[test]
    fn test_validate_invariants_violations() {
        let config = AgentConfig {
            id: "c1".into(),
            name: "test".into(),
            params: HashMap::from([
                ("convergence_threshold".into(), serde_json::json!(1)),
                ("safety_pipeline".into(), serde_json::json!(false)),
            ]),
            fitness: 0.0,
        };
        let violations = validate_invariants(&config);
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_mutate_config_valid() {
        let base = AgentConfig {
            id: "base".into(),
            name: "original".into(),
            params: HashMap::from([("convergence_threshold".into(), serde_json::json!(3))]),
            fitness: 1.0,
        };
        let mutations = HashMap::from([("new_param".into(), serde_json::json!("value"))]);
        let result = mutate_config(&base, mutations, "m1");
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "original_mutated");
    }

    #[test]
    fn test_mutate_config_violating() {
        let base = AgentConfig {
            id: "base".into(),
            name: "original".into(),
            params: HashMap::new(),
            fitness: 1.0,
        };
        let mutations =
            HashMap::from([("convergence_threshold".into(), serde_json::json!(0))]);
        assert!(mutate_config(&base, mutations, "m1").is_none());
    }

    #[test]
    fn test_population_add_and_best() {
        let mut pop = Population::default();
        pop.add(AgentConfig {
            id: "a".into(),
            name: "a".into(),
            params: HashMap::new(),
            fitness: 0.5,
        });
        pop.add(AgentConfig {
            id: "b".into(),
            name: "b".into(),
            params: HashMap::new(),
            fitness: 0.9,
        });
        assert_eq!(pop.best().unwrap().id, "b");
    }

    #[test]
    fn test_population_select() {
        let mut pop = Population::default();
        for i in 0..5 {
            pop.add(AgentConfig {
                id: format!("v{}", i),
                name: format!("v{}", i),
                params: HashMap::new(),
                fitness: i as f64,
            });
        }
        pop.select(2);
        assert_eq!(pop.variants.len(), 2);
        assert_eq!(pop.variants[0].fitness, 4.0);
    }
}
