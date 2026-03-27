# Risk Assessment Template

<!--
  CLASSIFICATION: RISK
  Use this template for point-in-time risk assessments of specific systems,
  processes, decisions, or initiatives. For ongoing risk tracking, use risk-register.md.
  Review monthly. Stream + Key approval required.
-->

---

## Frontmatter

| Field | Value |
|-------|-------|
| **Assessment ID** | RISK-NNN |
| **Title** | _[Descriptive title, e.g., "Single Point of Failure Assessment — Onelist"]_ |
| **Created** | YYYY-MM-DD |
| **Updated** | YYYY-MM-DD |
| **Assessor** | _[Officer who conducted the assessment]_ |
| **Owner** | _[Officer responsible for the assessed area]_ |
| **Approved By** | Stream + Key |
| **Assessment Type** | _[initial / periodic / triggered / post-incident]_ |
| **Subject** | _[System, process, decision, or initiative being assessed]_ |
| **Source Plan** | PLAN-XXXX _(if applicable)_ |
| **Next Reassessment** | YYYY-MM-DD |

---

## Scope

**What is being assessed:**
_[Specific system, process, initiative, or decision under evaluation]_

**Boundaries:**
_[What is included and excluded from this assessment]_

**Assessment methodology:**
_[How risks were identified — threat modeling, historical analysis, adversarial simulation, etc.]_

---

## Risk Identification

### Risk 1: _[Risk name]_

| Attribute | Value |
|-----------|-------|
| **Description** | _[What could go wrong]_ |
| **Category** | _[operational / security / financial / reputational / strategic / compliance / technical]_ |
| **Threat Source** | _[What causes this risk — internal failure, external actor, environmental, dependency]_ |
| **Affected Assets** | _[What is impacted — systems, data, processes, missions]_ |
| **Current Controls** | _[What exists today to prevent or detect this risk]_ |

**Likelihood Assessment:**

| Factor | Rating | Justification |
|--------|--------|---------------|
| Historical frequency | _[rare / unlikely / possible / likely / almost certain]_ | _[Evidence]_ |
| Attack complexity | _[high / medium / low / N/A]_ | _[Evidence]_ |
| Detection capability | _[strong / moderate / weak / none]_ | _[Evidence]_ |
| **Overall Likelihood** | _[1-5 scale]_ | |

**Impact Assessment:**

| Dimension | Rating | Justification |
|-----------|--------|---------------|
| Operational disruption | _[negligible / minor / moderate / major / catastrophic]_ | _[Evidence]_ |
| Financial impact | _[Rating]_ | _[Estimated cost range]_ |
| Mission impact | _[Rating]_ | _[Which missions affected and how]_ |
| Reputational impact | _[Rating]_ | _[Evidence]_ |
| **Overall Impact** | _[1-5 scale]_ | |

**Risk Score:** _[Likelihood x Impact = Score]_ | **Risk Level:** _[low / medium / high / critical]_

---

### Risk 2: _[Risk name]_

_[Same structure as Risk 1]_

---

## Risk Matrix Summary

| Risk | Likelihood (1-5) | Impact (1-5) | Score | Level | Treatment |
|------|------------------|-------------|-------|-------|-----------|
| _[Risk 1]_ | _[L]_ | _[I]_ | _[LxI]_ | _[Level]_ | _[accept / mitigate / transfer / avoid]_ |
| _[Risk 2]_ | _[L]_ | _[I]_ | _[LxI]_ | _[Level]_ | _[Treatment]_ |

### Risk Matrix Visualization

```
Impact →    1-Negligible  2-Minor  3-Moderate  4-Major  5-Catastrophic
Likelihood ↓
5-Almost Certain    M          H         H          C          C
4-Likely            M          M         H          H          C
3-Possible          L          M         M          H          H
2-Unlikely          L          L         M          M          H
1-Rare              L          L         L          M          M

L = Low (accept/monitor)  M = Medium (mitigate)  H = High (mitigate urgently)  C = Critical (immediate action)
```

---

## Mitigation Plans

### Mitigation for Risk 1: _[Risk name]_

| Action | Owner | Deadline | Expected Risk Reduction | Status |
|--------|-------|----------|------------------------|--------|
| _[Action 1]_ | _[Officer]_ | _[Date]_ | _[How much does this reduce likelihood or impact]_ | _[pending / in-progress / done]_ |
| _[Action 2]_ | _[Officer]_ | _[Date]_ | _[Reduction]_ | _[Status]_ |

**Residual Risk after Mitigation:** _[Revised score and level]_

---

## Accepted Risks

_Risks that are accepted (not mitigated) must be explicitly documented with justification._

| Risk | Score | Acceptance Justification | Accepted By | Review Date |
|------|-------|------------------------|-------------|-------------|
| _[Risk name]_ | _[Score]_ | _[Why accepted — cost of mitigation exceeds impact, etc.]_ | _[Approver]_ | _[Date]_ |

---

## Recommendations

1. _[Priority recommendation 1]_
2. _[Priority recommendation 2]_
3. _[Priority recommendation 3]_

---

## Assessment Sign-off

| Role | Name | Date | Signature |
|------|------|------|-----------|
| Assessor | _[Name]_ | _[Date]_ | _[Confirmed]_ |
| Area Owner | _[Name]_ | _[Date]_ | _[Confirmed]_ |
| Approver | _[Name]_ | _[Date]_ | _[Confirmed]_ |
