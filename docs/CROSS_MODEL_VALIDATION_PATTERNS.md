# Cross-Model Validation Patterns

**Purpose:** Reduce correlated errors in AI-driven convergence by using different models for generation vs audit.
**Source:** COCO paper (arxiv 2508.13815) — formal convergence guarantees via contextual rollback + bidirectional reflection.

---

## The Problem

When the same LLM generates code AND audits it, errors correlate:
- The model has the same blind spots in both roles
- It may "approve" its own output due to confirmation bias
- The COCO paper calls this the "correlated error" problem

## The Solution: Cross-Model Validation

Use a **different model** (or different prompt strategy) for audit than for generation.

### Approach 1: Different Model Family

```
Generation: Claude Opus → produces code
Audit: GPT-4o → reviews for issues the Claude model missed
```

**Pros:** Maximum decorrelation — different training data, different biases.
**Cons:** Requires multiple API keys, higher cost, slower.

### Approach 2: Different Model Tier (Same Family)

```
Generation: Claude Opus (frontier) → produces code
Audit: Claude Sonnet (standard) → reviews
```

**Pros:** Single provider, lower cost for audit passes.
**Cons:** Models from same family may share some blind spots.

### Approach 3: Different Prompt Strategy (Same Model)

```
Generation: Standard prompt → produces code
Audit: Adversarial prompt → "Find every possible issue, assume nothing is correct"
```

**Pros:** No additional API keys needed, works with any single provider.
**Cons:** Same model, some correlation remains. But adversarial framing reduces it significantly.

### Approach 4: Multi-Dimension Split

```
Security audit: Frontier model (most capable, catches subtle vulnerabilities)
Style/formatting: Fast model (cheaper, sufficient for mechanical checks)
Architecture: Standard model (good balance)
```

**Pros:** Cost-efficient — expensive models only where they matter most.
**Cons:** Complexity in routing.

## CruxDev Implementation

CruxDev's convergence engine already supports this via `recommended_tier`:

| Phase | Tier | Rationale |
|---|---|---|
| Planning | standard | Balanced capability |
| Code generation | standard | General code writing |
| Code audit (with security) | **frontier** | Security requires strongest model |
| Code audit (without security) | standard | General review |
| Doc audit | fast | Mechanical checking |
| Website convergence | standard | Visual + content review |
| E2E testing | fast | Test execution, not generation |

The MCP client reads `recommended_tier` from each task and routes to the appropriate model.

## When to Use Cross-Model Validation

| Scenario | Recommendation |
|---|---|
| Security-critical code | YES — always use frontier for security audit |
| Financial calculations | YES — use different model family if possible |
| Compliance/regulatory | YES — cross-model + human review |
| Internal tooling | MAYBE — adversarial prompt may suffice |
| Documentation | NO — correlated errors are low-risk |
| Formatting/style | NO — fast model is fine |

## Anti-Patterns

1. Using the same model + same prompt for generate and audit (maximum correlation)
2. Always using frontier for everything (wasteful, no decorrelation benefit)
3. Using a weaker model for security audit than generation (wrong direction)
4. Skipping audit entirely because "the model is good enough"

## Audit Dimensions

- **generation_audit_separation**: Are different models/prompts used for generation vs audit?
- **security_tier_escalation**: Do security audits use frontier models?
- **audit_independence**: Can the audit model see the generation model's reasoning? (It shouldn't — fresh eyes)
