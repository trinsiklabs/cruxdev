# CruxDev Improvement Proposal: Document Alignment Gate

**Date:** 2026-03-23
**Problem:** Build plans drift from product decisions when the context window can't hold all relevant docs simultaneously.
**Status:** DRAFT

---

## The Problem

The current CruxDev convergence process audits build plans against THEMSELVES (internal consistency, phase dependencies, viability). It does NOT audit plans against the PRODUCT DOCS that contain the decisions the plan must implement.

This causes drift:
- A plan says "free users get Light tier only" but the pricing doc says "free users get 5 assessments at ANY depth"
- A plan says "confidence: HIGH" but the confidence calibration doc says "Full = Irrefutable"
- A plan omits badge requirements that were captured in a memory file months ago
- A plan designs a feature that contradicts a content filtering policy captured in a different doc

The drift happens because:
1. The context window can't hold 50+ docs simultaneously
2. The plan author works from memory of the docs, not from the docs themselves
3. Internal plan consistency ≠ ecosystem consistency
4. The convergence engine doesn't know about external docs — it only sees the plan

**Real-world impact:** We caught this today. The assessment question plan had 3 misalignments with existing product decisions that would have produced wrong code if not caught. In a larger codebase with hundreds of decisions captured across dozens of docs, this will get worse.

---

## Proposed Solution: Document Alignment Gate

Add a new mandatory step to the convergence lifecycle: **Document Alignment Audit**.

### Where It Fits

```
Current lifecycle:
  1. Plan writing
  2. Focused audit (per phase) → converge
  3. Full-plan audit (cross-phase) → converge
  4. Viability assessment → converge
  5. Execution
  6. Code + doc convergence

Proposed lifecycle:
  1. Plan writing
  2. Focused audit (per phase) → converge
  3. Full-plan audit (cross-phase) → converge
  4. ★ DOCUMENT ALIGNMENT AUDIT → converge ★   ← NEW
  5. Viability assessment → converge
  6. Execution
  7. Code + doc convergence
```

### How It Works

**Step 4A: Identify relevant docs**

The plan must declare which docs it needs to align with:

```markdown
## Document Alignment

### Product Docs (this plan must conform to):
- self_assessment_app/ASSESSMENT_DESIGN.md — multi-axis engine spec
- self_assessment_app/COMPATIBILITY_PRICING.md — pricing tiers, free vs. premium
- relationship_skills_app/CONFIDENCE_CALIBRATION.md — confidence framework
- [every other relevant doc]

### Memory Files (captured decisions this plan must respect):
- memory/project_xamory_profile_gating.md — assessment-gated profiles
- memory/project_assessment_badges.md — badges require Irrefutable
- memory/feedback_content_enforcement_policy.md — NSFW gating
- [every other relevant memory file]
```

**Step 4B: Subagent alignment audit**

A subagent reads EACH listed doc and audits the plan against it:

```
For each doc in alignment_docs:
  1. Read the doc fully
  2. Extract all decisions, requirements, and constraints
  3. Check each one against the plan
  4. Report misalignments as findings
```

This MUST use a subagent because the main context can't hold all docs simultaneously. The subagent gets: the plan + one doc at a time. It returns findings.

**Step 4C: Fix and re-audit**

Standard convergence: fix findings → re-audit → two consecutive clean passes.

### Engine Integration

The convergence engine needs:

1. **New phase: `doc_alignment`** between `planning` and `code_audit`
2. **New task type: `doc_align`** — audit plan against a specific external doc
3. **Plan metadata: `alignment_docs`** — list of doc paths the plan must conform to
4. **Subagent dispatch** — the engine dispatches one audit per doc (parallelizable)
5. **Finding format** — same as existing, with `dimension: "doc_alignment"` and `source_doc` field

### Schema Addition to Plan Format

```markdown
## Document Alignment

| Doc | Purpose | Path |
|-----|---------|------|
| Pricing | Free tier rules, subscription tiers | self_assessment_app/COMPATIBILITY_PRICING.md |
| Confidence | Irrefutable/Strong Signal/Hypothesis levels | relationship_skills_app/CONFIDENCE_CALIBRATION.md |
| ... | ... | ... |

| Memory File | Decision | Path |
|-------------|----------|------|
| Badges | Badges require Full/Irrefutable only | memory/project_assessment_badges.md |
| Content filtering | NSFW assessments invisible without opt-in | memory/feedback_content_enforcement_policy.md |
| ... | ... | ... |
```

This section is MANDATORY in every build plan. If it's empty, the plan fails validation ("No alignment docs declared — every plan has product context it must respect").

---

## Why Subagents Are Required

The alignment audit CANNOT be done in the main context because:
- A typical project has 20-50 product docs + 20-40 memory files
- Each doc is 3,000-15,000 words
- Total: 200K-1M+ words of product context
- The plan itself is 5,000-15,000 words
- Combined: exceeds any context window

The subagent pattern: `plan (15K words) + one doc (10K words) = 25K words` — fits easily. Run N subagents for N docs, in parallel if possible. Each returns findings. Main context synthesizes findings and fixes the plan.

---

## Anti-Patterns This Prevents

| Anti-Pattern | What Happens | How the Gate Catches It |
|-------------|-------------|------------------------|
| Plan from memory, not docs | Drifted decisions | Subagent reads actual doc, finds mismatch |
| Stale mental model | Builder remembers v1 of a decision, v3 exists | Subagent reads current doc version |
| Missing constraints | Plan omits a constraint from a memory file | Subagent finds the constraint and checks for it |
| Contradictory decisions | Plan says X, doc says Y | Subagent flags the contradiction |
| Scope creep | Plan adds features not in any product doc | Subagent finds no doc supporting the feature |
| Implicit assumptions | Plan assumes something never written down | Subagent can't find the assumption in any doc → flagged |

---

## Implementation Cost

- **Engine changes:** ~200 lines — new phase, new task type, alignment_docs metadata parsing
- **Plan template change:** Add `## Document Alignment` section to `create_plan_template()`
- **Validation change:** `validate_plan_structure()` checks for non-empty alignment docs section
- **Subagent dispatch:** Reuse existing subagent pattern from code audit
- **Timeline:** 1-2 days of implementation

---

## Open Questions

1. **Should the alignment doc list be auto-discovered or manually declared?**
   - Manual: the plan author lists what's relevant (risk: missing a doc)
   - Auto-discovered: scan for all docs in the project that might be relevant (risk: too many, slow)
   - Hybrid: auto-suggest based on keywords in the plan, human confirms

2. **How deep should the audit go?**
   - Shallow: check that major decisions are respected (fast, catches big misalignments)
   - Deep: check every claim in the plan against every claim in every doc (thorough, expensive)
   - Recommended: deep for P0 docs, shallow for P1/P2

3. **Should the alignment docs be versioned?**
   - If a doc changes after the plan was aligned, should the plan be re-audited?
   - This matters for long-running plans where product decisions evolve

4. **Should findings from alignment audit be shared back to update the docs?**
   - Sometimes the plan reveals that a doc is stale or contradicts another doc
   - The alignment audit could be bidirectional: fix the plan AND flag doc issues

---

## Recommendation

Implement this as a mandatory gate in the CruxDev convergence engine. Every build plan declares its alignment docs. The engine won't transition from planning to execution until alignment converges (two clean passes against all declared docs). The cost is low (200 lines, 1-2 days). The value is high (prevents building the wrong thing from drifted assumptions).
