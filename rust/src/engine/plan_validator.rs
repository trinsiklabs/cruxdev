//! Plan validator — structural checks on build plan markdown.

use regex::Regex;
use std::fs;

#[derive(Debug, Default)]
pub struct PlanValidation {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl PlanValidation {
    pub fn valid(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validate a build plan's structure.
pub fn validate_plan(plan_path: &str) -> PlanValidation {
    let mut result = PlanValidation::default();

    let content = match fs::read_to_string(plan_path) {
        Ok(c) => c,
        Err(_) => {
            result.errors.push(format!("Plan file not found: {plan_path}"));
            return result;
        }
    };

    if !Regex::new(r"(?m)^#\s+").unwrap().is_match(&content) {
        result.errors.push("Plan must have a title (# heading)".into());
    }

    if !Regex::new(r"(?mi)^##\s+.*(?:Phase|Step)\s+\d").unwrap().is_match(&content) {
        result.warnings.push("Plan has no numbered phases/steps".into());
    }

    if !Regex::new(r"(?m)^\s*-\s*\[\s*[xX ]?\s*\]").unwrap().is_match(&content) {
        result.errors.push("Plan must have checklist items (- [ ] item)".into());
    }

    if !Regex::new(r"(?i)test|pytest|bun test|npm test|cargo test").unwrap().is_match(&content) {
        result.warnings.push("Plan does not reference test commands".into());
    }

    if !Regex::new(r"(?i)converge|convergence|clean pass|coverage").unwrap().is_match(&content) {
        result.warnings.push("Plan does not reference convergence criteria".into());
    }

    if !Regex::new(r"(?i)##\s+Document Alignment").unwrap().is_match(&content) {
        result.warnings.push("Plan has no '## Document Alignment' section".into());
    }

    if content.trim().len() < 50 {
        result.errors.push("Plan is too short (< 50 characters)".into());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_plan() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\n## Phase 1: Build\n- [ ] 1.1 Task\n## Document Alignment\n- d.md\ncargo test\nconvergence\n").unwrap();
        let result = validate_plan(path.to_str().unwrap());
        assert!(result.valid());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn missing_file() {
        let result = validate_plan("/nonexistent.md");
        assert!(!result.valid());
    }

    #[test]
    fn missing_title() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "No heading here just text with enough characters to pass length check and a checklist\n- [ ] 1.1 task\n").unwrap();
        let result = validate_plan(path.to_str().unwrap());
        assert!(!result.valid());
    }

    #[test]
    fn missing_checklist() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# Plan\nNo checklist items here just enough text to pass the length check for validation purposes.\n").unwrap();
        let result = validate_plan(path.to_str().unwrap());
        assert!(!result.valid());
    }

    #[test]
    fn too_short() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plan.md");
        fs::write(&path, "# P\n- [ ] 1.1 x").unwrap();
        let result = validate_plan(path.to_str().unwrap());
        assert!(!result.valid());
    }
}
