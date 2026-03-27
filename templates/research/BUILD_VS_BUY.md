---
title: "Build vs Buy Analysis: [Capability Name]"
conducted: [YYYY-MM-DD]
valid_until: [YYYY-MM-DD]
decision_context: "[What decision does this analysis support?]"
status: [in-progress | complete | stale]
author: "[Name]"
plan_reference: "[PLAN-XXXX]"
---

# Build vs Buy Analysis: [Capability Name]

**Decision Context:** [What capability do we need, and should we build it or acquire it?]
**Decision Deadline:** [YYYY-MM-DD]
**Decision Maker:** [Name or role]
**Conducted:** [YYYY-MM-DD]
**Valid Until:** [YYYY-MM-DD]

---

## 1. Executive Summary

[2-4 sentences. What capability was evaluated, what is the recommendation (build/buy/hybrid), and why.]

**Recommendation:** [Build / Buy / Hybrid / Needs Further Investigation]
**Confidence Level:** [High / Medium / Low]

---

## 2. Capability Definition

### 2.1 What We Need

[Describe the capability in concrete terms:]
- **Core function:** [What it does]
- **Users/consumers:** [Who uses this capability]
- **Integration points:** [What it connects to]
- **Scale requirements:** [Volume, throughput, users]
- **Timeline:** [When we need it operational]

### 2.2 Requirements

| ID | Requirement | Priority | Build Feasibility | Buy Feasibility |
|---|---|---|---|---|
| R-01 | [Requirement] | Must Have | [Easy/Moderate/Hard] | [Easy/Moderate/Hard] |
| R-02 | [Requirement] | Must Have | [Easy/Moderate/Hard] | [Easy/Moderate/Hard] |
| R-03 | [Requirement] | Should Have | [Easy/Moderate/Hard] | [Easy/Moderate/Hard] |
| R-04 | [Requirement] | Nice to Have | [Easy/Moderate/Hard] | [Easy/Moderate/Hard] |

### 2.3 Strategic Importance

| Question | Answer |
|---|---|
| Is this a core competency or commodity? | [Core / Adjacent / Commodity] |
| Does this create competitive differentiation? | [Yes — how / No] |
| Would building this divert resources from core work? | [Yes — impact / No] |
| Is this capability stable or rapidly evolving? | [Stable / Evolving — implications] |

---

## 3. Build Option

### 3.1 Technical Approach

[How would we build this?]
- **Architecture:** [High-level design]
- **Technology stack:** [Languages, frameworks, infrastructure]
- **Team required:** [Skills and headcount]
- **Estimated effort:** [Person-months or sprints]

### 3.2 Build Timeline

| Phase | Duration | Deliverable |
|---|---|---|
| Design | [Duration] | [What's produced] |
| Core implementation | [Duration] | [MVP scope] |
| Integration | [Duration] | [Connected to existing systems] |
| Testing and hardening | [Duration] | [Production-ready] |
| **Total** | **[Duration]** | |

### 3.3 Build Costs

| Cost Category | One-Time | Annual Recurring | Notes |
|---|---|---|---|
| Development labor | $[Amount] | — | [FTE count x duration x rate] |
| Infrastructure | $[Amount] | $[Amount] | [Hosting, CI/CD, etc.] |
| Testing / QA | $[Amount] | — | |
| Documentation / training | $[Amount] | — | |
| Ongoing maintenance | — | $[Amount] | [% of build cost per year; typically 15-25%] |
| Bug fixes / enhancements | — | $[Amount] | |
| **Total** | **$[Amount]** | **$[Amount]** | |

**3-Year Total Cost of Ownership (Build):** $[Amount]

### 3.4 Build Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Longer than estimated | [High/Med/Low] | [Impact] | [Mitigation] |
| Key person dependency | [High/Med/Low] | [Impact] | [Mitigation] |
| Scope creep | [High/Med/Low] | [Impact] | [Mitigation] |
| Technology risk | [High/Med/Low] | [Impact] | [Mitigation] |
| Maintenance burden | [High/Med/Low] | [Impact] | [Mitigation] |

### 3.5 Build Advantages

- [Advantage 1: e.g., "Full control over feature set and roadmap"]
- [Advantage 2: e.g., "No vendor lock-in"]
- [Advantage 3: e.g., "Deep integration with existing systems"]
- [Advantage 4: e.g., "Builds internal expertise"]

### 3.6 Build Disadvantages

- [Disadvantage 1: e.g., "Significant upfront investment"]
- [Disadvantage 2: e.g., "Ongoing maintenance burden"]
- [Disadvantage 3: e.g., "Opportunity cost — diverts team from core product"]
- [Disadvantage 4: e.g., "Risk of building an inferior solution"]

---

## 4. Buy Option

### 4.1 Market Scan

[What solutions exist? Brief survey — point to VENDOR_EVALUATION.md for deep dive.]

| Vendor/Product | Fit | Price Range | Notes |
|---|---|---|---|
| [Vendor A] | [High/Med/Low] | [$X/month or /year] | [Key differentiator] |
| [Vendor B] | [High/Med/Low] | [$X/month or /year] | [Key differentiator] |
| [Vendor C] | [High/Med/Low] | [$X/month or /year] | [Key differentiator] |
| [Open source option] | [High/Med/Low] | [Free + ops cost] | [Key differentiator] |

### 4.2 Buy Costs

| Cost Category | One-Time | Annual Recurring | Notes |
|---|---|---|---|
| License / subscription | $[Amount] | $[Amount] | [Pricing tier and model] |
| Implementation / integration | $[Amount] | — | [Internal labor + any professional services] |
| Customization | $[Amount] | — | [If the product needs modification] |
| Data migration | $[Amount] | — | |
| Training | $[Amount] | — | |
| Ongoing operations | — | $[Amount] | [Monitoring, updates, support tickets] |
| **Total** | **$[Amount]** | **$[Amount]** | |

**3-Year Total Cost of Ownership (Buy):** $[Amount]

### 4.3 Buy Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Vendor lock-in | [High/Med/Low] | [Impact] | [Mitigation: data export, API standards] |
| Vendor goes out of business | [High/Med/Low] | [Impact] | [Mitigation: escrow, open source fallback] |
| Pricing increases | [High/Med/Low] | [Impact] | [Mitigation: contract terms, alternatives] |
| Feature gaps | [High/Med/Low] | [Impact] | [Mitigation: workarounds, vendor roadmap] |
| Integration complexity | [High/Med/Low] | [Impact] | [Mitigation: API quality, middleware] |
| Data security / compliance | [High/Med/Low] | [Impact] | [Mitigation: certifications, data residency] |

### 4.4 Buy Advantages

- [Advantage 1: e.g., "Faster time to value"]
- [Advantage 2: e.g., "Proven solution with existing customers"]
- [Advantage 3: e.g., "Vendor handles maintenance and updates"]
- [Advantage 4: e.g., "Broader feature set than we would build"]

### 4.5 Buy Disadvantages

- [Disadvantage 1: e.g., "Limited customization"]
- [Disadvantage 2: e.g., "Vendor dependency"]
- [Disadvantage 3: e.g., "Recurring cost that compounds over time"]
- [Disadvantage 4: e.g., "May not integrate cleanly with existing systems"]

---

## 5. Hybrid Option

[If applicable — build some, buy some.]

### 5.1 Hybrid Approach

- **Buy:** [Which parts to acquire]
- **Build:** [Which parts to build in-house]
- **Rationale:** [Why splitting makes sense]

### 5.2 Hybrid Costs

| Cost Category | One-Time | Annual Recurring |
|---|---|---|
| Buy components | $[Amount] | $[Amount] |
| Build components | $[Amount] | $[Amount] |
| Integration | $[Amount] | $[Amount] |
| **Total** | **$[Amount]** | **$[Amount]** |

**3-Year Total Cost of Ownership (Hybrid):** $[Amount]

---

## 6. Comparison

### 6.1 Side-by-Side

| Factor | Build | Buy | Hybrid |
|---|---|---|---|
| **Time to value** | [Duration] | [Duration] | [Duration] |
| **3-Year TCO** | $[Amount] | $[Amount] | $[Amount] |
| **Requirement coverage** | [% of requirements met] | [% of requirements met] | [% of requirements met] |
| **Control / flexibility** | [High/Med/Low] | [High/Med/Low] | [High/Med/Low] |
| **Vendor risk** | None | [Level] | [Level] |
| **Maintenance burden** | [High/Med/Low] | [High/Med/Low] | [High/Med/Low] |
| **Opportunity cost** | [High/Med/Low] | [High/Med/Low] | [High/Med/Low] |
| **Strategic alignment** | [Assessment] | [Assessment] | [Assessment] |

### 6.2 Scoring Matrix

| Criterion | Weight | Build Score | Buy Score | Hybrid Score |
|---|---|---|---|---|
| Cost (3-year TCO) | [W] | [1-5] | [1-5] | [1-5] |
| Time to value | [W] | [1-5] | [1-5] | [1-5] |
| Requirement fit | [W] | [1-5] | [1-5] | [1-5] |
| Strategic alignment | [W] | [1-5] | [1-5] | [1-5] |
| Risk profile | [W] | [1-5] | [1-5] | [1-5] |
| Flexibility / control | [W] | [1-5] | [1-5] | [1-5] |
| Operational burden | [W] | [1-5] | [1-5] | [1-5] |
| **Weighted Total** | | **[Total]** | **[Total]** | **[Total]** |

---

## 7. Assumptions

1. [Assumption: e.g., "Development team has capacity to start within 30 days"]
2. [Assumption: e.g., "Vendor pricing is stable for the contract period"]
3. [Assumption: e.g., "Requirements will not change significantly in the next 12 months"]
4. [Assumption: e.g., "Build estimate includes 30% buffer for unknowns"]

---

## 8. Recommendation

### 8.1 Verdict

**[Build / Buy / Hybrid / Needs Further Investigation]**

### 8.2 Rationale

[3-5 sentences summarizing the strongest arguments for the recommendation.]

### 8.3 Conditions and Caveats

- [Condition: e.g., "Recommendation assumes vendor A is selected; revisit if vendor changes"]
- [Caveat: e.g., "Build estimate carries +-40% uncertainty at this stage"]

### 8.4 Reversibility

- **If we build and it fails:** [What's the fallback? How much is lost?]
- **If we buy and it fails:** [What's the fallback? How much is lost?]
- **Switching cost (build→buy later):** [Estimated effort]
- **Switching cost (buy→build later):** [Estimated effort]

### 8.5 Next Steps

1. [Next step with owner and timeline]
2. [Next step with owner and timeline]
3. [Next step with owner and timeline]

---

## 9. Sources

1. [Source description and URL or location]
2. [Source description and URL or location]

---

## Related Documents

- [Link to vendor evaluation, technology evaluation, or implementation plan]
