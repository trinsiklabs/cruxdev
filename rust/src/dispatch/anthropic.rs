//! Anthropic API provider — makes real LLM calls via reqwest.
//!
//! Reads ANTHROPIC_API_KEY from environment. Supports model routing:
//! - fast: claude-haiku-4-5-20251001
//! - standard: claude-sonnet-4-6-20250514
//! - frontier: claude-opus-4-6-20250514

use super::{AuditFinding, AuditResult, EvaluationResult, FixResult, LLMDispatcher, Severity, WriteResult};
use serde::{Deserialize, Serialize};

const API_URL: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

/// Model tier for routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModelTier {
    Fast,
    Standard,
    Frontier,
}

impl ModelTier {
    pub fn model_id(&self) -> &'static str {
        match self {
            Self::Fast => "claude-haiku-4-5-20251001",
            Self::Standard => "claude-sonnet-4-6-20250514",
            Self::Frontier => "claude-opus-4-6-20250514",
        }
    }
}

/// Anthropic LLM provider.
pub struct AnthropicProvider {
    client: reqwest::Client,
    api_key: String,
    tier: ModelTier,
    max_tokens: u32,
    call_count: u64,
}

impl AnthropicProvider {
    /// Create from environment variable ANTHROPIC_API_KEY.
    pub fn from_env(tier: ModelTier) -> anyhow::Result<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow::anyhow!("ANTHROPIC_API_KEY not set"))?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
            tier,
            max_tokens: 4096,
            call_count: 0,
        })
    }

    /// Create with explicit API key.
    pub fn new(api_key: &str, tier: ModelTier) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            tier,
            max_tokens: 4096,
            call_count: 0,
        }
    }

    pub fn call_count(&self) -> u64 {
        self.call_count
    }

    async fn call_api(&mut self, system: &str, user: &str) -> anyhow::Result<String> {
        self.call_count += 1;

        let body = serde_json::json!({
            "model": self.tier.model_id(),
            "max_tokens": self.max_tokens,
            "system": system,
            "messages": [{"role": "user", "content": user}],
        });

        let resp = self.client
            .post(API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(anyhow::anyhow!("Anthropic API error {}: {}", status, text));
        }

        let parsed: serde_json::Value = serde_json::from_str(&text)?;
        let content = parsed
            .get("content")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        Ok(content)
    }
}

/// Schema validation: parse LLM JSON output, retry on failure.
pub fn validate_json<T: serde::de::DeserializeOwned>(raw: &str) -> anyhow::Result<T> {
    // Try to find JSON in the response (LLMs sometimes wrap in markdown)
    let json_str = if let Some(start) = raw.find('{') {
        if let Some(end) = raw.rfind('}') {
            &raw[start..=end]
        } else {
            raw
        }
    } else if let Some(start) = raw.find('[') {
        if let Some(end) = raw.rfind(']') {
            &raw[start..=end]
        } else {
            raw
        }
    } else {
        raw
    };

    serde_json::from_str(json_str).map_err(|e| anyhow::anyhow!("Schema validation failed: {e}"))
}

/// Validate and retry pattern: call LLM, validate output, retry on schema failure.
pub fn validate_and_retry<T, F>(
    mut call: F,
    max_retries: usize,
) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
    F: FnMut() -> anyhow::Result<String>,
{
    for attempt in 0..=max_retries {
        let raw = call()?;
        match validate_json::<T>(&raw) {
            Ok(parsed) => return Ok(parsed),
            Err(_) if attempt < max_retries => continue,
            Err(e) => return Err(anyhow::anyhow!(
                "Schema validation failed after {} attempts: {e}",
                max_retries + 1
            )),
        }
    }
    unreachable!()
}

// Note: LLMDispatcher trait uses sync methods, but Anthropic needs async.
// For now, we provide the async call_api method and sync wrappers are
// implemented at the integration layer (tokio::runtime::Handle::current().block_on).
// The trait will be made async in a future refactor.

impl LLMDispatcher for AnthropicProvider {
    fn audit(
        &mut self,
        files: &[String],
        dimensions: &[String],
        skill_context: &str,
    ) -> anyhow::Result<AuditResult> {
        let system = format!(
            "You are a code auditor. Audit the provided files on these dimensions: {}.\n\
             Respond with JSON matching this schema: \
             {{\"findings\": [{{\"id\": \"f1\", \"file\": \"path\", \"dimension\": \"dim\", \
             \"severity\": \"high|medium|low\", \"description\": \"desc\", \"suggested_fix\": \"fix\"}}]}}\n\
             {}",
            dimensions.join(", "),
            if skill_context.is_empty() { "" } else { skill_context },
        );
        let user = format!("Audit these files:\n{}", files.join("\n"));

        // Sync wrapper — use tokio runtime
        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| anyhow::anyhow!("No tokio runtime"))?;
        let response = rt.block_on(self.call_api(&system, &user))?;

        #[derive(Deserialize)]
        struct AuditResponse {
            #[serde(default)]
            findings: Vec<FindingResponse>,
        }
        #[derive(Deserialize)]
        struct FindingResponse {
            id: String,
            file: String,
            dimension: String,
            severity: String,
            description: String,
            #[serde(default)]
            suggested_fix: String,
        }

        let parsed: AuditResponse = validate_json(&response)?;

        Ok(AuditResult {
            findings: parsed.findings.into_iter().map(|f| AuditFinding {
                id: f.id,
                file: f.file,
                dimension: f.dimension,
                severity: match f.severity.as_str() {
                    "high" => Severity::High,
                    "medium" => Severity::Medium,
                    _ => Severity::Low,
                },
                description: f.description,
                suggested_fix: f.suggested_fix,
            }).collect(),
            files_audited: files.to_vec(),
            dimensions_checked: dimensions.to_vec(),
        })
    }

    fn fix(
        &mut self,
        finding_id: &str,
        finding_description: &str,
        file_path: &str,
        file_content: &str,
        _skill_context: &str,
    ) -> anyhow::Result<FixResult> {
        let system = "You are a code fixer. Apply the suggested fix. \
             Respond with JSON: {\"success\": true, \"files_modified\": [\"path\"], \"description\": \"what changed\"}";
        let user = format!(
            "Fix finding {finding_id}: {finding_description}\n\nFile: {file_path}\n```\n{}\n```",
            &file_content[..file_content.len().min(8000)]
        );

        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| anyhow::anyhow!("No tokio runtime"))?;
        let response = rt.block_on(self.call_api(system, &user))?;
        validate_json(&response)
    }

    fn evaluate_independence(
        &mut self,
        pass_a: &AuditResult,
        pass_b: &AuditResult,
    ) -> anyhow::Result<EvaluationResult> {
        let system = "You evaluate whether two audit passes are independently produced. \
             Respond with JSON: {\"independent\": true/false, \"rationale\": \"why\"}";
        let user = format!(
            "Pass A findings: {:?}\nPass B findings: {:?}\n\nAre these independently produced?",
            pass_a.findings.iter().map(|f| &f.description).collect::<Vec<_>>(),
            pass_b.findings.iter().map(|f| &f.description).collect::<Vec<_>>(),
        );

        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| anyhow::anyhow!("No tokio runtime"))?;
        let response = rt.block_on(self.call_api(system, &user))?;
        validate_json(&response)
    }

    fn write(&mut self, spec: &str, skill_context: &str) -> anyhow::Result<WriteResult> {
        let system = format!(
            "You write content based on specifications. \
             Respond with JSON: {{\"content\": \"the content\", \"files_written\": [\"paths\"], \"description\": \"what was written\"}}\n\
             {}",
            if skill_context.is_empty() { "" } else { skill_context },
        );

        let rt = tokio::runtime::Handle::try_current()
            .map_err(|_| anyhow::anyhow!("No tokio runtime"))?;
        let response = rt.block_on(self.call_api(&system, spec))?;
        validate_json(&response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_tier_ids() {
        assert!(ModelTier::Fast.model_id().contains("haiku"));
        assert!(ModelTier::Standard.model_id().contains("sonnet"));
        assert!(ModelTier::Frontier.model_id().contains("opus"));
    }

    #[test]
    fn test_validate_json_clean() {
        #[derive(Deserialize)]
        struct Test { value: i32 }
        let result: Test = validate_json(r#"{"value": 42}"#).unwrap();
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_validate_json_wrapped_in_markdown() {
        #[derive(Deserialize)]
        struct Test { value: i32 }
        let result: Test = validate_json("```json\n{\"value\": 42}\n```").unwrap();
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_validate_json_invalid() {
        #[derive(Deserialize)]
        struct Test { value: i32 }
        assert!(validate_json::<Test>("not json").is_err());
    }

    #[test]
    fn test_validate_and_retry_succeeds() {
        let mut count = 0;
        let result: serde_json::Value = validate_and_retry(
            || {
                count += 1;
                Ok(r#"{"ok": true}"#.to_string())
            },
            2,
        ).unwrap();
        assert_eq!(count, 1);
        assert_eq!(result["ok"], true);
    }

    #[test]
    fn test_validate_and_retry_fails_then_succeeds() {
        let mut count = 0;
        let result: serde_json::Value = validate_and_retry(
            || {
                count += 1;
                if count == 1 {
                    Ok("not json".to_string())
                } else {
                    Ok(r#"{"ok": true}"#.to_string())
                }
            },
            2,
        ).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_validate_and_retry_exhausted() {
        let result: anyhow::Result<serde_json::Value> = validate_and_retry(
            || Ok("invalid".to_string()),
            2,
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("3 attempts"));
    }

    #[test]
    fn test_from_env_missing_key() {
        // Don't set ANTHROPIC_API_KEY
        unsafe { std::env::remove_var("ANTHROPIC_API_KEY"); }
        assert!(AnthropicProvider::from_env(ModelTier::Standard).is_err());
    }
}
