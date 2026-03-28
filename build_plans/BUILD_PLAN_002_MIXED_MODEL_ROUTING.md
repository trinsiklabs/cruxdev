# BUILD_PLAN_002: Mixed Model Routing

**Created:** 2026-03-20
**Status:** CONVERGED
**Goal:** Implement intelligent model routing across the Crux ecosystem — escalation on failure, task-complexity routing, mode→model mapping, and cost-aware selection.

**Constraint:** Changes touch three repos (Crux, CruxCLI, CruxDev). Each must pass its own test suite.
**Rule:** TDD. Tests before code.
**Rule:** 100% coverage on new code.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│ Model Router (Crux — shared config)             │
│                                                  │
│  model_tiers.json:                              │
│    frontier: [claude-opus-4-5, gpt-5]           │
│    standard: [claude-sonnet-4-5, gpt-5-mini]    │
│    fast:     [claude-haiku-4-5, gpt-5-nano]     │
│    local:    [qwen3-coder:30b, qwen3.5:27b]     │
│    micro:    [qwen3:8b, qwen3:4b]               │
│                                                  │
│  task_routing:                                   │
│    plan_audit    → fast                          │
│    code_audit    → standard                      │
│    security_audit → frontier                     │
│    fix_generation → standard                     │
│    independence   → fast                         │
│    title          → micro                        │
│    compaction     → fast                         │
│                                                  │
│  escalation:                                     │
│    on_validation_failure: tier_up (max 2 levels) │
│    on_quality_failure: tier_up (max 1 level)     │
│    on_provider_error: next_provider_same_tier    │
│                                                  │
│  mode_models:                                    │
│    build-py  → {primary: local/code, audit: fast}│
│    plan      → {primary: local/think, audit: std}│
│    review    → {primary: standard}               │
│    debug     → {primary: standard}               │
└─────────────────────────────────────────────────┘
         │                    │                │
         ▼                    ▼                ▼
    ┌─────────┐        ┌──────────┐     ┌──────────┐
    │  Crux   │        │ CruxCLI  │     │ CruxDev  │
    │ audit   │        │ system.ts│     │ task     │
    │ backend │        │ mode→    │     │ router   │
    │ 8b→32b→ │        │ model    │     │ audit→   │
    │ frontier│        │ mapping  │     │ model    │
    └─────────┘        └──────────┘     └──────────┘
```

---

## Phase 1: Shared Model Tier Config (Crux)

**Purpose:** Define the model tier system in Crux so all three products use the same vocabulary. A single config file maps tiers to concrete models.

### 1A. Model Tiers Config

```python
# In Crux: scripts/lib/crux_model_tiers.py
TIERS = {
    "frontier": ["claude-opus-4-5", "gpt-5"],
    "standard": ["claude-sonnet-4-5", "gpt-5-mini", "qwen3-coder:30b"],
    "fast":     ["claude-haiku-4-5", "gpt-5-nano", "qwen3.5:27b"],
    "local":    ["qwen3-coder:30b", "qwen3.5:27b"],
    "micro":    ["qwen3:8b", "qwen3:4b"],
}

TASK_ROUTING = {
    "plan_audit":     "fast",
    "code_audit":     "standard",
    "security_audit": "frontier",
    "fix_generation": "standard",
    "independence":   "fast",
    "title":          "micro",
    "compaction":     "fast",
    "doc_audit":      "fast",
    "write":          "standard",
}
```

### 1B. Tier Resolution

```python
def resolve_tier(tier: str, available_providers: list[str]) -> str | None:
    """Given a tier name, return the best available model."""
    for model in TIERS[tier]:
        provider = model.split("/")[0] if "/" in model else detect_provider(model)
        if provider in available_providers:
            return model
    return None
```

### 1C. Mode→Model Mapping

Extend existing `OPENCODE_AGENT_META` to include tier preferences:

```python
MODE_MODELS = {
    "build-py":  {"primary": "local", "audit": "fast"},
    "build-ex":  {"primary": "local", "audit": "fast"},
    "plan":      {"primary": "fast",  "audit": "standard"},
    "review":    {"primary": "standard"},
    "debug":     {"primary": "standard"},
    "security":  {"primary": "frontier"},
    # ... etc
}
```

### Checklist — Phase 1

- [ ] 1.1 `scripts/lib/crux_model_tiers.py` — tier definitions + task routing table
- [ ] 1.2 `resolve_tier()` — find best available model for a tier
- [ ] 1.3 `get_task_model()` — given a task type, return the model to use
- [ ] 1.4 Mode→model mapping integrated with existing `OPENCODE_AGENT_META`
- [ ] 1.5 MCP tool: `get_model_for_task(task_type)` — Claude Code can ask which model to use
- [ ] 1.6 MCP tool: `get_available_tiers()` — show what's available
- [ ] 1.7 Tests for tier resolution with various provider availability
- [ ] 1.8 Tests for task routing
- [ ] 1.9 Tests for mode→model mapping
- [ ] 1.10 Coverage ≥ 100%

---

## Phase 2: Escalation on Failure (Crux)

**Purpose:** When a model fails (validation error, low-quality output, provider error), automatically escalate to a higher tier.

### 2A. Escalation Rules

```python
ESCALATION_RULES = {
    "validation_failure": {
        "action": "tier_up",
        "max_escalations": 2,  # micro → fast → standard
    },
    "quality_failure": {
        "action": "tier_up",
        "max_escalations": 1,  # standard → frontier
    },
    "provider_error": {
        "action": "next_provider_same_tier",
        "max_retries": 2,
    },
    "timeout": {
        "action": "next_provider_same_tier",
        "max_retries": 1,
    },
}
```

### 2B. Integration with Audit Backend

Extend `crux_audit_backend.py`'s fallback chain to use tiers:
- Current: Ollama → Anthropic → OpenAI → subagent → disabled
- New: micro → fast → standard → frontier (within each, try providers in order)

### Checklist — Phase 2

- [ ] 2.1 Escalation logic in `crux_model_tiers.py`
- [ ] 2.2 `escalate(current_tier, failure_type)` → returns next tier or None
- [ ] 2.3 Integration with `crux_audit_backend.py` fallback chain
- [ ] 2.4 Escalation event logging (for quality feedback loop — Phase 5)
- [ ] 2.5 Tests for each escalation rule
- [ ] 2.6 Tests for max escalation limits
- [ ] 2.7 Tests for provider fallback within same tier
- [ ] 2.8 Coverage ≥ 100%

---

## Phase 3: CruxCLI Mode→Model Mapping

**Purpose:** When Crux mode is active, CruxCLI uses the mode's preferred model tier instead of just adjusting temperature/topP.

### 3A. Extend `cruxModelParams()`

In `packages/opencode/src/session/system.ts`, extend `cruxModelParams()` to also return a model recommendation:

```typescript
export function cruxModelParams(state: CruxState): {
    temperature?: number
    topP?: number
    recommendedModel?: string  // e.g., "anthropic/claude-sonnet-4-5"
} | undefined
```

### 3B. Read Tier Config from Crux

CruxCLI reads the tier config via:
1. Direct file read: `.crux/model_tiers.json` (written by Crux on adoption)
2. Or: Crux MCP tool `get_model_for_task()`

### 3C. Model Override in LLM.stream()

In `llm.ts`, if Crux recommends a model and the user hasn't explicitly chosen one, use Crux's recommendation.

### Checklist — Phase 3

- [ ] 3.1 Extend `CruxState` interface with model tier fields
- [ ] 3.2 Extend `cruxModelParams()` to return recommended model
- [ ] 3.3 Write tier config to `.crux/model_tiers.json` on Crux adoption
- [ ] 3.4 CruxCLI reads tier config from `.crux/model_tiers.json`
- [ ] 3.5 Model recommendation applied in `llm.ts` (user choice > crux > default)
- [ ] 3.6 Tests for model recommendation flow
- [ ] 3.7 Tests for precedence (user explicit > crux > agent > default)
- [ ] 3.8 `bun test` passes
- [ ] 3.9 Coverage on new code ≥ 100%

---

## Phase 4: CruxDev Task→Model Routing

**Purpose:** The convergence engine tells Claude Code which model to use for each task, not just what to do.

### 4A. Extend Task with Model Recommendation

In `src/engine/task_router.py`, add `recommended_model` to Task:

```python
@dataclass
class Task:
    task_type: str
    description: str
    files: list[str]
    dimensions: list[str]
    recommended_model: str | None = None  # e.g., "fast" or "anthropic/claude-haiku-4-5"
    ...
```

### 4B. Route by Task Type

```python
def get_next_task(...) -> Task:
    ...
    if state.phase == ConvergencePhase.PLAN_AUDITING:
        return Task(
            task_type="audit",
            recommended_model="fast",  # Plan audits don't need frontier
            ...
        )
    if state.phase == ConvergencePhase.CODE_AUDITING:
        return Task(
            task_type="audit",
            recommended_model="standard",  # Code audits need more capability
            ...
        )
```

### 4C. Update MCP Tool Descriptions

Tell Claude Code to respect the model recommendation:
> "The engine recommends a model tier for each task. If `recommended_model` is set,
> switch to that model before executing. Use `/crux-mode` or model picker."

### Checklist — Phase 4

- [ ] 4.1 Add `recommended_model` to Task dataclass
- [ ] 4.2 Task router sets recommended_model based on task type
- [ ] 4.3 Update `/converge` command to explain model switching
- [ ] 4.4 Update MCP tool descriptions with model recommendation
- [ ] 4.5 Tests for model recommendation per task type
- [ ] 4.6 Tests for Task serialization with recommended_model
- [ ] 4.7 Coverage ≥ 100%

---

## Phase 5: Quality Feedback Loop (Crux)

**Purpose:** Track when escalation happens and feed it back into routing decisions. If the 8B model consistently fails on security audits, automatically route security audits to a higher tier.

### 5A. Escalation Log

```python
# In Crux: scripts/lib/crux_model_quality.py
def log_escalation(task_type: str, from_tier: str, to_tier: str, reason: str):
    """Log an escalation event for quality tracking."""

def get_task_success_rate(task_type: str, tier: str) -> float:
    """What percentage of tasks at this tier succeed without escalation?"""

def recommend_tier(task_type: str) -> str:
    """Based on historical success rates, what tier should this task start at?"""
```

### 5B. Adaptive Routing

If a task type's success rate at a given tier drops below a threshold (e.g., 70%), automatically route future tasks to the next tier up.

### Checklist — Phase 5

- [ ] 5.1 `crux_model_quality.py` — escalation logging + success rate tracking
- [ ] 5.2 JSONL log file for escalation events
- [ ] 5.3 `recommend_tier()` — adaptive routing based on history
- [ ] 5.4 Integration with `get_task_model()` from Phase 1
- [ ] 5.5 MCP tool: `get_model_quality_stats()` — show success rates
- [ ] 5.6 Tests for logging and retrieval
- [ ] 5.7 Tests for adaptive routing thresholds
- [ ] 5.8 Tests for cold start (no history)
- [ ] 5.9 Coverage ≥ 100%

---

## Progress Tracker

**Phase 1: Shared Tier Config (10 items)** — [ ] 1.1 – 1.10
**Phase 2: Escalation (8 items)** — [ ] 2.1 – 2.8
**Phase 3: CruxCLI Mode→Model (9 items)** — [ ] 3.1 – 3.9
**Phase 4: CruxDev Task→Model (7 items)** — [ ] 4.1 – 4.7
**Phase 5: Quality Feedback (9 items)** — [ ] 5.1 – 5.9

**Total: 43 checkboxes**

---

## Test Commands

```bash
# Crux
cd /Users/user/personal/crux && python3 -m pytest tests/ -v --tb=short --cov=scripts --cov-report=term-missing

# CruxDev
cd /Users/user/personal/cruxdev && python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100

# CruxCLI
cd /Users/user/personal/cruxcli/packages/opencode && bun test
```

## Convergence Criteria

- All checklist items complete
- All three test suites pass
- Coverage ≥ 100% on new CruxDev code
- Two consecutive clean audit passes
