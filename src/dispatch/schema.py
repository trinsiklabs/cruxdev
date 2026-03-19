"""Schema definitions for all LLM I/O.

Every LLM output is validated against these schemas before the engine acts on it.
"""

from pydantic import BaseModel
from typing import Literal


class AuditFinding(BaseModel):
    id: str
    file: str
    dimension: str
    severity: Literal["high", "medium", "low"]
    description: str
    suggested_fix: str


class AuditResult(BaseModel):
    findings: list[AuditFinding]
    files_audited: list[str]
    dimensions_checked: list[str]


class FixResult(BaseModel):
    success: bool
    files_modified: list[str]
    description: str


class EvaluationResult(BaseModel):
    independent: bool
    rationale: str


class WriteResult(BaseModel):
    content: str
    files_written: list[str]
    description: str
