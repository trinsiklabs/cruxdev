//! Convergence engine state — enums, structs, pure data.
//! All state is deterministic — no async, no I/O.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Convergence phases in execution order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConvergencePhase {
    Planning,
    PatternAssessment,
    PatternOrchestration,
    PlanAuditing,
    DocAlignment,
    Viability,
    Executing,
    CodeAuditing,
    DocAuditing,
    WebsiteConvergence,
    E2eTesting,
    PatternsUpdate,
    Converged,
    Escalated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub file: String,
    pub dimension: String,
    pub severity: FindingSeverity,
    pub description: String,
    pub suggested_fix: String,
    #[serde(default)]
    pub fixed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundResult {
    pub round: i32,
    pub phase: ConvergencePhase,
    pub findings: Vec<Finding>,
    pub findings_fixed: usize,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunResult {
    pub passed: bool,
    pub total: usize,
    pub failures: usize,
    pub coverage: Option<f64>,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub duration_seconds: f64,
}

/// Current convergence protocol version.
pub const PROTOCOL_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceState {
    pub plan_file: String,
    #[serde(default = "default_protocol_version")]
    pub protocol_version: String,
    #[serde(default = "default_phase")]
    pub phase: ConvergencePhase,
    #[serde(default)]
    pub round: i32,
    #[serde(default = "default_max_rounds")]
    pub max_rounds: i32,
    #[serde(default)]
    pub consecutive_clean: i32,
    #[serde(default = "default_threshold")]
    pub convergence_threshold: i32,
    #[serde(default)]
    pub failures: HashMap<String, i32>,
    #[serde(default = "default_max_failures")]
    pub max_failures: i32,
    pub deadline: Option<f64>,
    #[serde(default = "default_timeout")]
    pub timeout_per_task: f64,
    #[serde(default)]
    pub history: Vec<RoundResult>,
    pub escalation_reason: Option<String>,
    #[serde(default = "now")]
    pub created_at: f64,
    #[serde(default = "now")]
    pub updated_at: f64,
    #[serde(default)]
    pub project_dir: String,
}

fn default_protocol_version() -> String { PROTOCOL_VERSION.to_string() }
fn default_phase() -> ConvergencePhase { ConvergencePhase::Planning }
fn default_max_rounds() -> i32 { 5 }
fn default_threshold() -> i32 { 2 }
fn default_max_failures() -> i32 { 3 }
fn default_timeout() -> f64 { 900.0 }
fn now() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}

impl ConvergenceState {
    pub fn new(plan_file: String) -> Self {
        let t = now();
        Self {
            plan_file,
            protocol_version: PROTOCOL_VERSION.to_string(),
            phase: ConvergencePhase::Planning,
            round: 0,
            max_rounds: 5,
            consecutive_clean: 0,
            convergence_threshold: 2,
            failures: HashMap::new(),
            max_failures: 3,
            deadline: None,
            timeout_per_task: 900.0,
            history: Vec::new(),
            escalation_reason: None,
            created_at: t,
            updated_at: t,
            project_dir: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_serializes() {
        let json = serde_json::to_string(&ConvergencePhase::Planning).unwrap();
        assert_eq!(json, "\"planning\"");
    }

    #[test]
    fn phase_deserializes() {
        let phase: ConvergencePhase = serde_json::from_str("\"code_auditing\"").unwrap();
        assert_eq!(phase, ConvergencePhase::CodeAuditing);
    }

    #[test]
    fn all_phases_exist() {
        let phases = vec![
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
            ConvergencePhase::Escalated,
        ];
        assert_eq!(phases.len(), 12);
    }

    #[test]
    fn state_new() {
        let state = ConvergenceState::new("plan.md".to_string());
        assert_eq!(state.plan_file, "plan.md");
        assert_eq!(state.phase, ConvergencePhase::Planning);
        assert_eq!(state.round, 0);
        assert_eq!(state.max_rounds, 5);
        assert_eq!(state.convergence_threshold, 2);
        assert!(state.created_at > 0.0);
    }

    #[test]
    fn state_serializes_roundtrip() {
        let state = ConvergenceState::new("test.md".to_string());
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: ConvergenceState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.plan_file, "test.md");
        assert_eq!(deserialized.phase, ConvergencePhase::Planning);
    }

    #[test]
    fn finding_serializes() {
        let f = Finding {
            id: "f1".to_string(),
            file: "main.rs".to_string(),
            dimension: "correctness".to_string(),
            severity: FindingSeverity::High,
            description: "bug".to_string(),
            suggested_fix: "fix it".to_string(),
            fixed: false,
        };
        let json = serde_json::to_string(&f).unwrap();
        assert!(json.contains("\"high\""));
    }

    #[test]
    fn severity_values() {
        assert_eq!(
            serde_json::to_string(&FindingSeverity::High).unwrap(),
            "\"high\""
        );
        assert_eq!(
            serde_json::to_string(&FindingSeverity::Medium).unwrap(),
            "\"medium\""
        );
        assert_eq!(
            serde_json::to_string(&FindingSeverity::Low).unwrap(),
            "\"low\""
        );
    }
}
