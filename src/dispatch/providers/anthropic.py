"""Anthropic provider — calls Claude API for audit/fix/evaluate/write tasks.

HTTP-level timeout on every call. Schema validation via validate_and_retry.
"""

import json
import os
from typing import Optional

import anthropic

from ..llm import LLMDispatcher
from ..schema import AuditResult, EvaluationResult, FixResult, WriteResult
from ..validation import validate_and_retry

DEFAULT_MODEL = "claude-sonnet-4-20250514"
DEFAULT_TIMEOUT = 120.0  # seconds
DEFAULT_MAX_TOKENS = 4096


class AnthropicProvider(LLMDispatcher):
    """Anthropic Claude provider with HTTP-level timeouts and schema validation."""

    def __init__(
        self,
        api_key: Optional[str] = None,
        model: str = DEFAULT_MODEL,
        timeout: float = DEFAULT_TIMEOUT,
        max_tokens: int = DEFAULT_MAX_TOKENS,
        max_retries: int = 2,
    ):
        key = api_key or os.environ.get("ANTHROPIC_API_KEY", "")
        if not key:
            raise ValueError(
                "ANTHROPIC_API_KEY not set. Provide api_key or set the environment variable."
            )
        self.client = anthropic.Anthropic(
            api_key=key,
            timeout=timeout,
        )
        self.model = model
        self.max_tokens = max_tokens
        self.max_retries = max_retries

    def _call(
        self,
        system: str,
        user: str,
        error_feedback: Optional[str] = None,
    ) -> str:
        """Make a single API call and return the text response."""
        messages = [{"role": "user", "content": user}]
        if error_feedback:
            messages = [
                {"role": "user", "content": user},
                {
                    "role": "assistant",
                    "content": "I'll fix the formatting issues.",
                },
                {
                    "role": "user",
                    "content": f"Your previous response had a schema error: {error_feedback}\n\nPlease try again with valid JSON.",
                },
            ]

        response = self.client.messages.create(
            model=self.model,
            max_tokens=self.max_tokens,
            system=system,
            messages=messages,
        )
        # Extract text from response
        text = ""
        for block in response.content:
            if block.type == "text":
                text += block.text
        return _extract_json(text)

    def audit(
        self,
        files: list[str],
        dimensions: list[str],
        skill_context: str,
    ) -> AuditResult:
        system = _audit_system_prompt(dimensions, skill_context)
        user = _audit_user_prompt(files)

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, AuditResult, self.max_retries)

    def fix(
        self,
        finding_id: str,
        finding_description: str,
        file_path: str,
        file_content: str,
        skill_context: str,
    ) -> FixResult:
        system = _fix_system_prompt(skill_context)
        user = _fix_user_prompt(finding_id, finding_description, file_path, file_content)

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, FixResult, self.max_retries)

    def evaluate_independence(
        self,
        pass_a: AuditResult,
        pass_b: AuditResult,
    ) -> EvaluationResult:
        system = "You evaluate whether two audit passes were independent. Respond with JSON: {\"independent\": bool, \"rationale\": str}"
        user = (
            f"Pass A audited {pass_a.files_audited} on dimensions {pass_a.dimensions_checked}, "
            f"found {len(pass_a.findings)} findings.\n"
            f"Pass B audited {pass_b.files_audited} on dimensions {pass_b.dimensions_checked}, "
            f"found {len(pass_b.findings)} findings.\n\n"
            "Were these passes independent (different scope, different approach)?"
        )

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, EvaluationResult, self.max_retries)

    def write(self, spec: str, skill_context: str) -> WriteResult:
        system = _write_system_prompt(skill_context)
        user = f"Write the following:\n\n{spec}"

        def call(**kwargs):
            return self._call(system, user, **kwargs)

        return validate_and_retry(call, WriteResult, self.max_retries)


def _extract_json(text: str) -> str:
    """Extract JSON from LLM response text.

    Handles cases where the JSON is wrapped in markdown code blocks.
    """
    text = text.strip()
    # Try to find JSON in code blocks
    if "```json" in text:
        start = text.index("```json") + 7
        end = text.index("```", start)
        return text[start:end].strip()
    if "```" in text:
        start = text.index("```") + 3
        end = text.index("```", start)
        return text[start:end].strip()
    # Try raw JSON (starts with { or [)
    for i, c in enumerate(text):
        if c in "{[":
            # Find matching end
            depth = 0
            for j in range(i, len(text)):
                if text[j] in "{[":
                    depth += 1
                elif text[j] in "}]":
                    depth -= 1
                    if depth == 0:
                        return text[i : j + 1]
    return text


def _audit_system_prompt(dimensions: list[str], skill_context: str) -> str:
    dims = ", ".join(dimensions)
    context = f"\n\nContext:\n{skill_context}" if skill_context else ""
    return (
        f"You are a code auditor. Audit the provided files on these dimensions: {dims}.\n\n"
        "Respond with JSON matching this schema:\n"
        '{"findings": [{"id": str, "file": str, "dimension": str, '
        '"severity": "high"|"medium"|"low", "description": str, "suggested_fix": str}], '
        '"files_audited": [str], "dimensions_checked": [str]}\n\n'
        "If no issues found, return empty findings array."
        f"{context}"
    )


def _audit_user_prompt(files: list[str]) -> str:
    parts = ["Audit these files:\n"]
    for f in files:
        try:
            with open(f) as fh:
                content = fh.read()
            parts.append(f"--- {f} ---\n{content}\n")
        except (FileNotFoundError, OSError):
            parts.append(f"--- {f} --- (file not found)\n")
    return "\n".join(parts)


def _fix_system_prompt(skill_context: str) -> str:
    context = f"\n\nContext:\n{skill_context}" if skill_context else ""
    return (
        "You fix code issues. Apply the suggested fix and respond with JSON:\n"
        '{"success": bool, "files_modified": [str], "description": str}'
        f"{context}"
    )


def _fix_user_prompt(
    finding_id: str,
    finding_description: str,
    file_path: str,
    file_content: str,
) -> str:
    return (
        f"Finding {finding_id}: {finding_description}\n\n"
        f"File: {file_path}\n"
        f"Content:\n{file_content}\n\n"
        "Fix this issue."
    )


def _write_system_prompt(skill_context: str) -> str:
    context = f"\n\nContext:\n{skill_context}" if skill_context else ""
    return (
        "You write code and documentation. Respond with JSON:\n"
        '{"content": str, "files_written": [str], "description": str}'
        f"{context}"
    )
