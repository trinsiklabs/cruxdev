---
title: "Cost-Benefit Analysis: [Proposal Name]"
conducted: [YYYY-MM-DD]
valid_until: [YYYY-MM-DD]
decision_context: "[What decision does this analysis support?]"
status: [in-progress | complete | stale]
author: "[Name]"
plan_reference: "[PLAN-XXXX]"
---

# Cost-Benefit Analysis: [Proposal Name]

**Decision Context:** [Should we invest in this proposal?]
**Decision Deadline:** [YYYY-MM-DD]
**Decision Maker:** [Name or role]
**Conducted:** [YYYY-MM-DD]
**Valid Until:** [YYYY-MM-DD]

---

## 1. Executive Summary

[3-5 sentences. What is proposed, what are the total costs and benefits, what is the net outcome, and the recommendation.]

**Net Benefit (3-Year):** $[Amount] / [Positive / Negative / Break-even]
**Payback Period:** [Duration]
**Recommendation:** [Proceed / Do Not Proceed / Conditional]

---

## 2. Proposal Description

### 2.1 What Is Being Proposed

[Describe the proposed action, investment, or change in concrete terms.]

### 2.2 Baseline (Do Nothing)

[What happens if we take no action? This is the comparison point for all costs and benefits.]

- **Current state:** [Description]
- **Projected trajectory without action:** [What gets better, what gets worse, what stays the same]
- **Cost of inaction:** [Quantify if possible — lost revenue, growing tech debt, increasing risk]

### 2.3 Analysis Time Horizon

- **Time horizon:** [Number of years, typically 1, 3, or 5]
- **Discount rate:** [% used for NPV calculation — justify the rate chosen]
- **Why this horizon:** [Why this timeframe is appropriate]

---

## 3. Costs

### 3.1 Direct Costs

| Cost Item | Year 0 (One-Time) | Year 1 | Year 2 | Year 3 | Total |
|---|---|---|---|---|---|
| [Cost item: e.g., "Development labor"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| [Cost item: e.g., "Hardware/infrastructure"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| [Cost item: e.g., "Software licenses"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| [Cost item: e.g., "Training"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| [Cost item: e.g., "Ongoing maintenance"] | — | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| **Total Direct Costs** | **$[Amount]** | **$[Amount]** | **$[Amount]** | **$[Amount]** | **$[Amount]** |

### 3.2 Indirect Costs

| Cost Item | Estimate | Confidence | Notes |
|---|---|---|---|
| [Opportunity cost: team diverted from X] | $[Amount] | [H/M/L] | [How estimated] |
| [Transition/disruption cost] | $[Amount] | [H/M/L] | [Duration and impact] |
| [Learning curve productivity loss] | $[Amount] | [H/M/L] | [How estimated] |
| [Risk-weighted contingency] | $[Amount] | [H/M/L] | [What risks are priced in] |

### 3.3 Hidden/Often-Forgotten Costs

[Explicitly check for these commonly overlooked costs:]

- [ ] **Integration costs:** Connecting to existing systems
- [ ] **Data migration costs:** Moving from current solution
- [ ] **Change management:** Communication, process updates, resistance management
- [ ] **Documentation:** Creating and maintaining new documentation
- [ ] **Testing:** Comprehensive testing before and after
- [ ] **Rollback plan:** Cost to reverse if things go wrong
- [ ] **Compliance/audit:** Additional compliance requirements
- [ ] **Support burden:** New support tickets during transition

### 3.4 Total Cost Summary

| Category | Amount | Confidence |
|---|---|---|
| Direct costs (3-year) | $[Amount] | [H/M/L] |
| Indirect costs (3-year) | $[Amount] | [H/M/L] |
| **Total costs (3-year)** | **$[Amount]** | |

---

## 4. Benefits

### 4.1 Quantifiable Benefits

| Benefit | Year 1 | Year 2 | Year 3 | Total | Confidence | How Measured |
|---|---|---|---|---|---|---|
| [Benefit: e.g., "Revenue increase"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | [H/M/L] | [Measurement method] |
| [Benefit: e.g., "Cost savings from automation"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | [H/M/L] | [Measurement method] |
| [Benefit: e.g., "Reduced downtime"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | [H/M/L] | [Measurement method] |
| [Benefit: e.g., "Headcount avoidance"] | $[Amount] | $[Amount] | $[Amount] | $[Amount] | [H/M/L] | [Measurement method] |
| **Total Quantifiable Benefits** | **$[Amount]** | **$[Amount]** | **$[Amount]** | **$[Amount]** | | |

### 4.2 Qualitative Benefits

[Benefits that are real but difficult to quantify. Do NOT assign fake dollar values.]

| Benefit | Impact | Confidence | Evidence |
|---|---|---|---|
| [Benefit: e.g., "Improved developer experience"] | [Description of impact] | [H/M/L] | [What suggests this is real] |
| [Benefit: e.g., "Reduced security risk"] | [Description of impact] | [H/M/L] | [Evidence] |
| [Benefit: e.g., "Strategic positioning"] | [Description of impact] | [H/M/L] | [Evidence] |

### 4.3 Benefit Realization Timeline

```
Year 0          Year 1          Year 2          Year 3
|───────────────|───────────────|───────────────|
  Investment     [Benefit A starts]
  period         [Benefit B starts ─────────────]
                      [Benefit C starts ────────]
                           Break-even point: [X months]
```

---

## 5. Financial Analysis

### 5.1 Net Present Value (NPV)

| Year | Costs | Benefits | Net Cash Flow | Discount Factor | Present Value |
|---|---|---|---|---|---|
| 0 | $[Amount] | $[Amount] | $[Amount] | 1.000 | $[Amount] |
| 1 | $[Amount] | $[Amount] | $[Amount] | [Factor] | $[Amount] |
| 2 | $[Amount] | $[Amount] | $[Amount] | [Factor] | $[Amount] |
| 3 | $[Amount] | $[Amount] | $[Amount] | [Factor] | $[Amount] |
| **Total** | **$[Amount]** | **$[Amount]** | **$[Amount]** | | **$[NPV]** |

**NPV:** $[Amount] (using [X]% discount rate)

### 5.2 Return on Investment (ROI)

**ROI = (Total Benefits - Total Costs) / Total Costs = [X]%**

### 5.3 Payback Period

**Payback period:** [X months/years] — the point at which cumulative benefits exceed cumulative costs.

### 5.4 Break-Even Analysis

[Under what conditions does this proposal break even?]

- **Revenue needed:** [If revenue-dependent: how much revenue to break even]
- **Usage volume needed:** [If volume-dependent: how many users/transactions/etc.]
- **Time needed:** [How long until cumulative benefits exceed cumulative costs]

---

## 6. Sensitivity Analysis

[How do results change if assumptions are wrong?]

### 6.1 Key Variables

| Variable | Base Case | Optimistic | Pessimistic | NPV at Optimistic | NPV at Pessimistic |
|---|---|---|---|---|---|
| [Variable: e.g., "Development time"] | [Value] | [Value] | [Value] | $[Amount] | $[Amount] |
| [Variable: e.g., "Revenue growth rate"] | [Value] | [Value] | [Value] | $[Amount] | $[Amount] |
| [Variable: e.g., "Adoption rate"] | [Value] | [Value] | [Value] | $[Amount] | $[Amount] |

### 6.2 Scenario Analysis

| Scenario | Description | NPV | ROI | Verdict |
|---|---|---|---|---|
| **Best case** | [All optimistic assumptions] | $[Amount] | [%] | [Proceed/Not] |
| **Base case** | [Expected assumptions] | $[Amount] | [%] | [Proceed/Not] |
| **Worst case** | [All pessimistic assumptions] | $[Amount] | [%] | [Proceed/Not] |
| **Most likely** | [Mix of assumptions] | $[Amount] | [%] | [Proceed/Not] |

### 6.3 Break-Even Sensitivity

[What single change would flip the recommendation?]

- **The proposal breaks even if:** [Condition]
- **The proposal fails if:** [Condition]

---

## 7. Alternatives Comparison

| Factor | Proposed Action | [Alternative A] | [Alternative B] | Do Nothing |
|---|---|---|---|---|
| 3-Year Cost | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| 3-Year Benefit | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| NPV | $[Amount] | $[Amount] | $[Amount] | $[Amount] |
| ROI | [%] | [%] | [%] | [%] |
| Payback | [Duration] | [Duration] | [Duration] | N/A |
| Risk level | [H/M/L] | [H/M/L] | [H/M/L] | [H/M/L] |

---

## 8. Assumptions

[Every assumption that affects the numbers. Be exhaustive.]

| # | Assumption | Impact if Wrong | Confidence |
|---|---|---|---|
| 1 | [Assumption] | [What changes if this is wrong] | [H/M/L] |
| 2 | [Assumption] | [Impact] | [H/M/L] |
| 3 | [Assumption] | [Impact] | [H/M/L] |

---

## 9. Recommendation

### 9.1 Verdict

**[Proceed / Do Not Proceed / Conditional Proceed]**

### 9.2 Rationale

[3-5 sentences. Reference the NPV, ROI, payback period, and sensitivity analysis.]

### 9.3 Conditions (if Conditional)

- [Condition 1]
- [Condition 2]

### 9.4 Monitoring Plan

[How will we track whether actual costs and benefits match projections?]

| Metric | Projected | Review Frequency | Action if Off-Track |
|---|---|---|---|
| [Metric] | [Target] | [Monthly/Quarterly] | [What we do] |

### 9.5 Next Steps

1. [Next step with owner and timeline]
2. [Next step with owner and timeline]

---

## 10. Sources

1. [Source description and URL or location]

---

## Related Documents

- [Link to feasibility study, build-vs-buy analysis, or implementation plan]
