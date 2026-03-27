//! LLM dispatch — trait, schemas, providers (stub + Anthropic).

pub mod anthropic;

use serde::{Deserialize, Serialize};

// --- Schema types ---

/// Severity level for audit findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    High,
    Medium,
    Low,
}

/// A single audit finding from an LLM audit pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub id: String,
    pub file: String,
    pub dimension: String,
    pub severity: Severity,
    pub description: String,
    pub suggested_fix: String,
}

/// Result of an LLM audit call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResult {
    pub findings: Vec<AuditFinding>,
    pub files_audited: Vec<String>,
    pub dimensions_checked: Vec<String>,
}

/// Result of an LLM fix call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixResult {
    pub success: bool,
    pub files_modified: Vec<String>,
    pub description: String,
}

/// Result of an LLM independence evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    pub independent: bool,
    pub rationale: String,
}

/// Result of an LLM write call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteResult {
    pub content: String,
    pub files_written: Vec<String>,
    pub description: String,
}

// --- LLM Dispatcher trait ---

/// The ONLY interface through which the engine calls an LLM.
pub trait LLMDispatcher {
    fn audit(
        &mut self,
        files: &[String],
        dimensions: &[String],
        skill_context: &str,
    ) -> anyhow::Result<AuditResult>;

    fn fix(
        &mut self,
        finding_id: &str,
        finding_description: &str,
        file_path: &str,
        file_content: &str,
        skill_context: &str,
    ) -> anyhow::Result<FixResult>;

    fn evaluate_independence(
        &mut self,
        pass_a: &AuditResult,
        pass_b: &AuditResult,
    ) -> anyhow::Result<EvaluationResult>;

    fn write(&mut self, spec: &str, skill_context: &str) -> anyhow::Result<WriteResult>;
}

// --- Stub Provider ---

/// Stub mode for deterministic testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StubMode {
    Clean,
    Findings,
    Persistent,
    SchemaInvalid,
    Intermittent,
}

/// Configurable stub LLM provider for testing.
pub struct StubProvider {
    pub mode: StubMode,
    pub findings: Vec<AuditFinding>,
    call_count: u64,
}

impl StubProvider {
    pub fn new(mode: StubMode) -> Self {
        Self {
            mode,
            findings: Vec::new(),
            call_count: 0,
        }
    }

    pub fn with_findings(mut self, findings: Vec<AuditFinding>) -> Self {
        self.findings = findings;
        self
    }

    pub fn call_count(&self) -> u64 {
        self.call_count
    }
}

impl LLMDispatcher for StubProvider {
    fn audit(
        &mut self,
        files: &[String],
        dimensions: &[String],
        _skill_context: &str,
    ) -> anyhow::Result<AuditResult> {
        self.call_count += 1;

        match self.mode {
            StubMode::Clean => Ok(AuditResult {
                findings: vec![],
                files_audited: files.to_vec(),
                dimensions_checked: dimensions.to_vec(),
            }),
            StubMode::Findings | StubMode::Persistent => Ok(AuditResult {
                findings: self.findings.clone(),
                files_audited: files.to_vec(),
                dimensions_checked: dimensions.to_vec(),
            }),
            StubMode::SchemaInvalid => {
                Err(anyhow::anyhow!("Simulated schema validation failure"))
            }
            StubMode::Intermittent => {
                let findings = if self.call_count.is_multiple_of(2) {
                    vec![]
                } else {
                    self.findings.clone()
                };
                Ok(AuditResult {
                    findings,
                    files_audited: files.to_vec(),
                    dimensions_checked: dimensions.to_vec(),
                })
            }
        }
    }

    fn fix(
        &mut self,
        finding_id: &str,
        _finding_description: &str,
        file_path: &str,
        _file_content: &str,
        _skill_context: &str,
    ) -> anyhow::Result<FixResult> {
        self.call_count += 1;

        match self.mode {
            StubMode::Persistent => Ok(FixResult {
                success: false,
                files_modified: vec![],
                description: "Fix failed (persistent mode)".into(),
            }),
            StubMode::SchemaInvalid => {
                Err(anyhow::anyhow!("Simulated schema validation failure"))
            }
            _ => Ok(FixResult {
                success: true,
                files_modified: vec![file_path.to_string()],
                description: format!("Fixed {}", finding_id),
            }),
        }
    }

    fn evaluate_independence(
        &mut self,
        _pass_a: &AuditResult,
        _pass_b: &AuditResult,
    ) -> anyhow::Result<EvaluationResult> {
        self.call_count += 1;
        Ok(EvaluationResult {
            independent: true,
            rationale: "Stub: always independent".into(),
        })
    }

    fn write(&mut self, spec: &str, _skill_context: &str) -> anyhow::Result<WriteResult> {
        self.call_count += 1;
        Ok(WriteResult {
            content: format!("# Generated from spec\n{}", spec),
            files_written: vec!["output.md".into()],
            description: "Stub write".into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_clean_mode() {
        let mut stub = StubProvider::new(StubMode::Clean);
        let result = stub
            .audit(&["file.py".into()], &["style".into()], "")
            .unwrap();
        assert!(result.findings.is_empty());
        assert_eq!(stub.call_count(), 1);
    }

    #[test]
    fn test_stub_findings_mode() {
        let finding = AuditFinding {
            id: "f1".into(),
            file: "test.py".into(),
            dimension: "style".into(),
            severity: Severity::Medium,
            description: "bad style".into(),
            suggested_fix: "fix it".into(),
        };
        let mut stub = StubProvider::new(StubMode::Findings).with_findings(vec![finding]);
        let result = stub.audit(&[], &[], "").unwrap();
        assert_eq!(result.findings.len(), 1);
    }

    #[test]
    fn test_stub_fix() {
        let mut stub = StubProvider::new(StubMode::Clean);
        let result = stub.fix("f1", "desc", "file.py", "content", "").unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_stub_persistent_fix_fails() {
        let mut stub = StubProvider::new(StubMode::Persistent);
        let result = stub.fix("f1", "desc", "file.py", "content", "").unwrap();
        assert!(!result.success);
    }

    #[test]
    fn test_stub_evaluate_independence() {
        let mut stub = StubProvider::new(StubMode::Clean);
        let a = AuditResult {
            findings: vec![],
            files_audited: vec![],
            dimensions_checked: vec![],
        };
        let result = stub.evaluate_independence(&a, &a).unwrap();
        assert!(result.independent);
    }

    #[test]
    fn test_stub_write() {
        let mut stub = StubProvider::new(StubMode::Clean);
        let result = stub.write("build a thing", "").unwrap();
        assert!(result.content.contains("Generated from spec"));
    }

    #[test]
    fn test_schema_invalid_audit() {
        let mut stub = StubProvider::new(StubMode::SchemaInvalid);
        assert!(stub.audit(&[], &[], "").is_err());
    }

    #[test]
    fn test_audit_result_serde() {
        let result = AuditResult {
            findings: vec![AuditFinding {
                id: "f1".into(),
                file: "test.py".into(),
                dimension: "style".into(),
                severity: Severity::High,
                description: "issue".into(),
                suggested_fix: "fix".into(),
            }],
            files_audited: vec!["test.py".into()],
            dimensions_checked: vec!["style".into()],
        };
        let json = serde_json::to_string(&result).unwrap();
        let parsed: AuditResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.findings.len(), 1);
    }
}
