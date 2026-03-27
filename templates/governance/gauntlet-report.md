# Gauntlet Report Template

<!--
  CLASSIFICATION: AUDIT
  The gauntlet report is the standard format for adversarial analysis.
  Gauntlet reports stress-test claims, plans, implementations, and assumptions.
  The gauntlet's job is to BREAK things — find weaknesses, not confirm strengths.
  Stream approval required for publication.
-->

---

## Frontmatter

| Field | Value |
|-------|-------|
| **Report ID** | GAUNTLET-NNN |
| **Title** | _[What was tested, e.g., "Revenue Model Stress Test"]_ |
| **Created** | YYYY-MM-DD |
| **Gauntlet Officer** | _[Officer who ran the adversarial analysis]_ |
| **Subject Owner** | _[Officer who owns the thing being tested]_ |
| **Approved By** | Stream |
| **Verdict** | _[PASS / CONDITIONAL PASS / FAIL]_ |
| **Source Plan** | PLAN-XXXX _(if applicable)_ |
| **Subject Type** | _[plan / implementation / claim / assumption / architecture / process]_ |

---

## Subject Under Test

_What exactly is being gauntlet-tested? Describe the claim, plan, implementation, or assumption in the subject owner's own terms. Be precise._

**Subject statement:** _[The specific assertion or artifact being tested]_

**Subject evidence:** _[What the subject owner provided as supporting evidence]_

---

## Adversarial Framework

_How was the gauntlet conducted? Which attack vectors were explored?_

### Attack Vectors Attempted

| # | Vector | Description | Outcome |
|---|--------|------------|---------|
| 1 | _[Vector name]_ | _[How the claim/plan was attacked]_ | _[survived / weakened / broken]_ |
| 2 | _[Vector name]_ | _[Attack description]_ | _[Outcome]_ |
| 3 | _[Vector name]_ | _[Attack description]_ | _[Outcome]_ |
| 4 | _[Vector name]_ | _[Attack description]_ | _[Outcome]_ |
| 5 | _[Vector name]_ | _[Attack description]_ | _[Outcome]_ |

---

## Findings

### Critical Findings (Must Fix)

_Findings that invalidate the subject or represent unacceptable risk._

#### CF-1: _[Finding title]_

**Attack vector:** _[Which vector exposed this]_
**Description:** _[What was found]_
**Evidence:** _[Specific evidence — logs, calculations, counterexamples]_
**Impact:** _[What happens if this is not addressed]_
**Required remediation:** _[What must change]_

### Major Findings (Should Fix)

_Findings that significantly weaken the subject but do not invalidate it._

#### MF-1: _[Finding title]_

**Attack vector:** _[Vector]_
**Description:** _[Finding]_
**Evidence:** _[Evidence]_
**Impact:** _[Impact]_
**Recommended remediation:** _[Recommendation]_

### Minor Findings (Could Improve)

_Findings that represent suboptimal choices or missed opportunities._

#### mF-1: _[Finding title]_

**Description:** _[Finding]_
**Suggestion:** _[How to improve]_

### Strengths Identified

_What survived the gauntlet? Adversarial analysis should also identify what is solid._

- _[Strength 1 — what attack it survived and why]_
- _[Strength 2]_

---

## Verdict Rationale

**Verdict: _[PASS / CONDITIONAL PASS / FAIL]_**

### PASS Criteria
_A subject PASSES when all attack vectors were survived or weaknesses are minor and do not threaten the subject's core claim._

### CONDITIONAL PASS Criteria
_A subject CONDITIONALLY PASSES when major findings exist but have clear, achievable remediations. Subject owner must address all major findings within the stated deadline._

### FAIL Criteria
_A subject FAILS when critical findings exist that invalidate the core claim or when the cumulative effect of findings undermines the subject's viability._

---

## Conditions (if Conditional Pass)

| Condition | Owner | Deadline | Verification Method |
|-----------|-------|----------|-------------------|
| _[What must be fixed]_ | _[Officer]_ | _[Date]_ | _[How compliance will be verified]_ |

---

## Remediations Tracker

| Finding | Severity | Owner | Status | Due Date | Verification |
|---------|----------|-------|--------|----------|-------------|
| _[CF-1]_ | Critical | _[Officer]_ | _[open / in-progress / resolved / verified]_ | _[Date]_ | _[How verified]_ |
| _[MF-1]_ | Major | _[Officer]_ | _[Status]_ | _[Date]_ | _[Verification]_ |

---

## Methodology Notes

_Any limitations of the gauntlet analysis. What was NOT tested and why? What assumptions did the gauntlet itself make?_

- _[Limitation 1]_
- _[Limitation 2]_

---

## Convergence Certificate

<!--
  MANDATORY per GAUNTLET_STANDARD.md — every gauntlet must complete Phases 2-6
  (GTV verification, completeness audit, issues scan, convergence loop, sign-off).
  See templates/governance/docs/GAUNTLET_STANDARD.md for the full standard.
-->

| Field | Value |
|-------|-------|
| **Converged** | _[YES / NO]_ |
| **Iterations to convergence** | _[N]_ |
| **Total findings fixed** | _[M]_ |
| **GTV checks performed** | _[count]_ |
| **Unverifiable claims** | _[count — list below if any]_ |
| **Convergence failure reason** | _[if NO — explain]_ |

### GTV Verification Log

_Summary of ground truth verification checks performed during Phase 2._

| # | Claim | Verification Method | Result |
|---|-------|-------------------|--------|
| 1 | _[Factual claim from report]_ | _[Command or check run]_ | _[Confirmed / Fixed / Unverifiable]_ |

### Convergence Iteration Log

| Iteration | Phase 2 Findings | Phase 3 Findings | Phase 4 Findings | Total |
|-----------|-----------------|-----------------|-----------------|-------|
| 1 | _[N]_ | _[N]_ | _[N]_ | _[N]_ |
| 2 | 0 | 0 | 0 | 0 |
| 3 | 0 | 0 | 0 | 0 |

---

## Sign-off

| Role | Name | Date |
|------|------|------|
| Gauntlet officer | _[Name]_ | _[Date]_ |
| Subject owner (acknowledged) | _[Name]_ | _[Date]_ |
| Stream (approved) | Stream | _[Date]_ |
