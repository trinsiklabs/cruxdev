"""Ollama provider — calls local Ollama instance for audit/fix/evaluate/write.

No API key required. HTTP-level timeout. Schema validation via validate_and_retry.
"""

import json
from typing import Optional

import httpx

from ..llm import LLMDispatcher
from ..schema import AuditResult, EvaluationResult, FixResult, WriteResult
from ..validation import validate_and_retry
from .anthropic import _audit_system_prompt, _audit_user_prompt, _extract_json, _fix_system_prompt, _fix_user_prompt, _write_system_prompt

DEFAULT_MODEL = "llama3.2"
DEFAULT_BASE_URL = "http://localhost:11434"
DEFAULT_TIMEOUT = 300.0  # Ollama can be slow


class OllamaProvider(LLMDispatcher):
    """Ollama local model provider."""

    def __init__(
        self,
        model: str = DEFAULT_MODEL,
        base_url: str = DEFAULT_BASE_URL,
        timeout: float = DEFAULT_TIMEOUT,
        max_retries: int = 2,
    ):
        self.model = model
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.max_retries = max_retries
        self._client = httpx.Client(timeout=timeout)

    def _call(
        self,
        system: str,
        user: str,
        error_feedback: Optional[str] = None,
    ) -> str:
        prompt = user
        if error_feedback:
            prompt += f"\n\nPrevious error: {error_feedback}\nPlease fix and respond with valid JSON."

        response = self._client.post(
            f"{self.base_url}/api/generate",
            json={
                "model": self.model,
                "system": system,
                "prompt": prompt,
                "stream": False,
                "format": "json",
            },
        )
        response.raise_for_status()
        data = response.json()
        return _extract_json(data.get("response", ""))

    def audit(self, files: list[str], dimensions: list[str], skill_context: str) -> AuditResult:
        system = _audit_system_prompt(dimensions, skill_context)
        user = _audit_user_prompt(files)

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, AuditResult, self.max_retries)

    def fix(self, finding_id: str, finding_description: str, file_path: str, file_content: str, skill_context: str) -> FixResult:
        system = _fix_system_prompt(skill_context)
        user = _fix_user_prompt(finding_id, finding_description, file_path, file_content)

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, FixResult, self.max_retries)

    def evaluate_independence(self, pass_a: AuditResult, pass_b: AuditResult) -> EvaluationResult:
        system = "You evaluate whether two audit passes were independent. Respond with JSON: {\"independent\": bool, \"rationale\": str}"
        user = (
            f"Pass A: {len(pass_a.findings)} findings on {pass_a.files_audited}\n"
            f"Pass B: {len(pass_b.findings)} findings on {pass_b.files_audited}\n"
            "Were these independent?"
        )

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, EvaluationResult, self.max_retries)

    def write(self, spec: str, skill_context: str) -> WriteResult:
        system = _write_system_prompt(skill_context)
        user = f"Write: {spec}"

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, WriteResult, self.max_retries)
