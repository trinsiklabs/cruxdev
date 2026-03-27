# Gauntlet Standard

<!--
  CLASSIFICATION: GOVERNANCE / PROCESS STANDARD
  This document defines the mandatory process for all gauntlet analyses.
  A gauntlet is the system's adversarial stress-testing mechanism.
  This standard supersedes informal gauntlet practices and ensures that
  gauntlet output is CONVERGED — requiring zero post-gauntlet auditing.
  Stream approval required for modifications.
-->

---

## 1. Purpose

A gauntlet produces an adversarial analysis of a subject (plan, implementation,
claim, architecture, document, or process). The gauntlet's output must be
**converged** — meaning a subsequent independent audit finds zero factual errors,
zero completeness gaps, and zero internal inconsistencies.

**The standard this document enforces:** Running any convergence audit after a
gauntlet is done with its subject is pointless because the gauntlet already did it.

---

## 2. The Problem This Standard Solves

Prior gauntlet practice produced outputs that required 5-17 additional findings
to be fixed by separate convergence audits. Root causes:

| Failure Class | Description | Example |
|---------------|-------------|---------|
| **Unverified claims** | Gauntlet stated facts without checking filesystem/DB | "18 template categories" when 19 exist |
| **Phantom references** | Paths, tools, groups referenced but never verified to exist | `templates/financial/docs/FINANCIAL_PROJECTIONS.md` — file does not exist |
| **Stale counts** | Numbers from LLM memory, not from actual queries | "48 API controllers" when only 37 are API v1 |
| **Incomplete tables** | Tables that looked complete but missed rows | Code docs table listed 12 of 18 actual files |
| **Internal contradictions** | Section A says X, Section B implies not-X | "93 tools in group" vs "206 tools in inventory" |
| **Missing sections** | Document coverage gaps invisible to the author | No post-migration maintenance, no secrets guidance |
| **Shallow self-audit** | Self-audit read for flow/clarity, not ground truth | "2 passes to convergence" then external audit finds 11 issues |

---

## 3. Mandatory Gauntlet Phases

Every gauntlet MUST execute all six phases in order. Skipping phases or
reordering them is a process violation.

### Phase 1: Adversarial Research (6+ rounds)

The existing gauntlet core. Deep adversarial analysis of the subject through
multiple attack vectors. Produces a draft output document.

**Requirements:**
- Minimum 6 adversarial rounds (more for complex subjects)
- Each round must attack from a different vector
- Rounds must include: feasibility, risk, edge cases, alternatives, consistency, completeness
- Output: draft document with all findings

### Phase 1.5: Implication Thread Walking (MANDATORY — PLAN-3392)

For every recommendation produced in Phase 1, walk the implication chain:

1. List every system state assumption
2. For each assumption: "What if this is wrong?"
3. Walk the failure chain to terminal consequence
4. If terminal = severe (server down, data loss, security breach):
   a. Add explicit verification step
   b. Add test proving terminal consequence doesn't occur
5. Cross-reference against known failures

**Format:**

```
Decision: [what]
  → Assumes: [assumption]
    → If wrong: [first failure]
      → Cascades to: [next failure]
        → Terminal: [final consequence]
  → Mitigation: [prevention]
```

**Output:** Implication walking log appended to gauntlet report.

### Phase 2: Ground Truth Verification (GTV)

Verify every factual claim in the draft output against actual system state.
This is the phase that prior gauntlets omitted entirely.

**Mandatory checks:**

| Check | Method | Example |
|-------|--------|---------|
| Every file path referenced | `test -f <path>` | Verify template paths exist |
| Every tool referenced | `which <tool>` or `test -x` | Verify tools are installed |
| Every count/number stated | Actual query or count command | `ls dir/ \| wc -l` to verify "N files" |
| Every database claim | `psql` query | Verify row counts, group membership |
| Every group/plan reference | Query plan status | Verify plans exist and have claimed status |
| Cross-document references | Read referenced doc, verify claim | "As documented in X" — read X, confirm |

**Process:**
1. Extract every verifiable claim from the draft
2. Run the actual verification command for each
3. Record: claim, verification method, result (confirmed/incorrect/unverifiable)
4. Fix every incorrect claim in the draft
5. Mark unverifiable claims explicitly: "Not independently verified: [claim]"

**Output:** GTV verification log appended to gauntlet report.

### Phase 3: Completeness Audit

Check the output document's structural completeness independent of content accuracy.

**Mandatory checks:**

- [ ] All expected sections present (per template or subject requirements)
- [ ] Table of contents matches actual sections (if TOC exists)
- [ ] Every table is complete (no missing rows vs. source data)
- [ ] Every section has substantive content (no empty/placeholder sections)
- [ ] Cross-references within the document resolve correctly
- [ ] No orphaned references (mentions of sections/tables that don't exist)
- [ ] Appendices are listed and present (if applicable)

**Process:**
1. List all sections and subsections
2. For each table, count rows vs. expected rows (from source data)
3. Check TOC against actual headings
4. Verify internal cross-references
5. Fix all gaps

### Phase 4: Issues and Opportunities Scan

Read the output as a naive outsider who has never seen the system.

**Mandatory checks:**

- [ ] Undefined jargon or acronyms
- [ ] Assumptions stated as facts without evidence
- [ ] Missing edge cases that a reader would immediately ask about
- [ ] Missing "what NOT to do" guidance (negative space)
- [ ] Weak or unsupported claims
- [ ] Missing context for someone outside the project
- [ ] Actionable recommendations that lack specificity (who, when, how)
- [ ] Opportunities the analysis missed

**Process:**
1. Read the entire output as if unfamiliar with the system
2. Flag every point of confusion, ambiguity, or missing context
3. Fix or add clarifying content for each flag
4. Identify gaps that represent missed analytical opportunities

### Phase 5: Convergence Loop

Re-run Phases 2-4 on the fixed output until two consecutive clean passes.

**Rules:**
- A "clean pass" means zero new findings across all three phases
- Maximum 5 convergence iterations (if not converged by iteration 5, report failure)
- Each iteration's findings are recorded in the convergence log
- The iteration count and finding count are reported in the final output

**Process:**
```
iteration = 1
loop:
  run Phase 2 (GTV) → findings_a
  run Phase 3 (Completeness) → findings_b
  run Phase 4 (Issues) → findings_c
  total = findings_a + findings_b + findings_c
  if total == 0 AND prior_total == 0:
    CONVERGED — exit loop
  else:
    fix all findings
    prior_total = total
    iteration += 1
    if iteration > 5:
      CONVERGENCE FAILURE — exit loop, report
    goto loop
```

### Phase 6: Sign-Off

Document the convergence result and produce the final output.

**Required metadata block** (must appear in every gauntlet output):

```
================================================================
CONVERGENCE CERTIFICATE
================================================================
Converged: [YES / NO]
Iterations to convergence: [N]
Total findings fixed: [M]
GTV checks performed: [count]
Unverifiable claims: [count] (listed below if any)
Convergence failure reason: [if NO — why iteration cap was reached]
Plan group created: [GROUP-NAME or NONE]
Plans created: [count] ([PLAN-XXXX through PLAN-YYYY])
Parent group: [GROUP-NAME or NONE]
================================================================
```

### Phase 6.5: Plan Group Generation

If the gauntlet produced actionable recommendations, generate plans and a plan
group so work is tracked and executable — not just documented.

**Rules:**

| Condition | Action |
|-----------|--------|
| 3+ related implementation items | Create a plan group |
| 1-2 implementation items | Create plans without a group |
| Purely informational gauntlet | No plans or group needed |

**Process (when plan group is warranted):**

1. Determine a descriptive group name from the gauntlet subject
2. Create the plan group: `plan-group-create <name> --description "<desc>"`
3. QCP each recommendation as a plan (create document, convert to plan)
4. Add all plans to the group: `plan-group-add <group> <plan-id>`
5. Set dependencies between plans: `plan-group-depend <group> <plan-id> --depends-on <other-plan-id>`
6. If the group is a subgroup of an existing group, link it
7. Record the group name and plan IDs in the convergence certificate

**If no plan group warranted:**

Note "No plan group created — [reason]" in the convergence certificate.
Reasons: "purely informational", "single recommendation", "recommendations
already tracked by existing plans".

---

## 4. Gauntlet Report Format and Output Location

The gauntlet report template (`gauntlet-report.md`) remains the format for
formal adversarial reports on specific subjects. This standard adds to it —
it does not replace the report template.

Every gauntlet output (whether using the formal report template or a
free-form analysis) MUST include:

1. The adversarial analysis content (Phase 1)
2. The convergence certificate (Phase 6)
3. The GTV verification log (Phase 2 results, appended or inline)

### 4.1 Output Location

Gauntlet reports MUST be written to the correct domain artifacts directory,
NOT to `/home/key/swarm_sync/tmp/`. The tmp directory is for in-progress
scratch files only — files that will exist for less than 1 hour.

**Routing rules:**

| Output Type | Destination |
|-------------|-------------|
| Gauntlet report | `/srv/sync/<domain>/artifacts/gauntlets/<filename>` |
| Convergence audit | `/srv/sync/<domain>/artifacts/audits/<filename>` |
| Competitive research | `/home/key/swarm_sync/research/competitive/<filename>` |
| Technical research | `/home/key/swarm_sync/research/technical/<filename>` |
| Other domain reports | `/srv/sync/<domain>/artifacts/reports/<filename>` |

**Domain assignment:** Route to the domain the gauntlet's subject belongs to.
Cross-domain analyses go to `/srv/sync/swarm/artifacts/gauntlets/` (the swarm
coordination domain). Competitive analyses go to `swarm_sync/research/competitive/`.

**Naming convention:** `<subject>-gauntlet.txt` for gauntlets,
`<subject>-audit.txt` for convergence audits.

---

## 5. What "Converged" Means

A gauntlet output is converged when:

1. **Every verifiable factual claim has been verified against ground truth** — no
   path, count, tool name, or status claim is based solely on LLM memory
2. **Every table is complete** against its source data
3. **Internal consistency is verified** — numbers stated in one section match
   numbers in every other section
4. **Two consecutive clean passes** of Phases 2-4 found zero issues
5. **Unverifiable claims are explicitly marked** as unverifiable

A gauntlet output is NOT converged if:
- Any path or tool reference was not checked against the filesystem
- Any count was not verified against actual data
- The self-audit only checked for "flow and clarity" without GTV
- The convergence loop was skipped or ran only one pass

---

## 6. Verification of Convergence Claims

Any agent or officer may **spot-check** a gauntlet's convergence claim:

1. Select 3-5 factual claims at random from the output
2. Re-verify each using the same methods (test -f, psql query, etc.)
3. If ANY spot-check fails, the gauntlet's convergence certificate is REVOKED
4. The gauntlet must be re-run with stricter verification

This spot-check mechanism prevents gauntlets from claiming convergence
without actually performing verification.

---

## 7. Handling Edge Cases

### 7.1 Unverifiable Claims

Some claims cannot be verified from the current machine (e.g., claims about
external services, future plans, competitive analysis). These MUST be:
- Explicitly labeled: "Not independently verified: [claim]"
- Sourced: "[claim] — per [source document/conversation/reference]"
- Never stated as verified fact

### 7.2 Multi-File Outputs

When a gauntlet produces multiple files, GTV and completeness checks apply
to EACH file independently. The convergence certificate covers all files.

### 7.3 Recommendations and Future Plans

Gauntlets that recommend new plans or actions cannot GTV future outcomes.
GTV applies to: the current state assessment, the data cited, and the
reasoning chain. Recommendations are checked for: internal consistency,
feasibility against current state, and completeness (no obvious gaps).

### 7.4 Context Limits

If the convergence loop risks exceeding context limits:
1. Prioritize GTV (Phase 2) — factual accuracy is non-negotiable
2. Then completeness (Phase 3) — structural integrity
3. Issues scan (Phase 4) can be abbreviated if context is tight
4. Record in convergence certificate: "Phase 4 abbreviated due to context constraints"

### 7.5 Convergence Failure (> 5 iterations)

If convergence is not achieved after 5 iterations, the gauntlet MUST:
1. Report CONVERGENCE FAILURE in the certificate
2. List all remaining unfixed findings
3. Explain why convergence was not achieved (new issues keep appearing = subject is unstable)
4. The output is usable but requires external review before being treated as authoritative

---

## 8. Quality Tracking

Over time, track:

| Metric | Target | Measurement |
|--------|--------|-------------|
| Convergence rate | > 95% of gauntlets converge within 5 iterations | Certificate data |
| Post-gauntlet spot-check pass rate | 100% | Random spot-checks |
| Average iterations to convergence | < 3 | Certificate data |
| Average findings fixed per gauntlet | Trending downward | Certificate data |
| GTV checks per gauntlet | > 20 | Certificate data |

---

## 9. Relationship to gauntlet-precheck

`gauntlet-precheck` (PLAN-3190) runs BEFORE the gauntlet starts and validates
that the subject is ready for adversarial analysis (invariants, templates,
open questions, etc.).

This standard governs what happens DURING and AT THE END of the gauntlet.

The full lifecycle:
```
gauntlet-precheck → Phase 1 (Research) → Phase 1.5 (Convergent Implication Walking)
                  → Phase 2 (GTV) → Phase 3 (Completeness)
                  → Phase 4 (Issues) → Phase 5 (Convergence Loop) → Phase 6 (Sign-Off)
                  → Phase 6.5 (Plan Group Generation)
```

---

## 10. Revision History

| Date | Change | Author |
|------|--------|--------|
| 2026-03-24 | Initial standard. Codifies built-in convergence requirement. | PLAN-3275 |
| 2026-03-25 | Added Section 4.1: Output location rules. Gauntlets write to domain artifacts, not tmp. | PLAN-3275 |
| 2026-03-26 | Added Phase 6.5: Plan Group Generation. Gauntlets with 3+ actionable recommendations auto-create plan groups. Updated convergence certificate format. | PLAN-3294 |
| 2026-03-26 | Added Phase 1.5: Implication Thread Walking. Mandatory chain analysis between adversarial research and GTV. | PLAN-3392 |
| 2026-03-26 | REVERTED: Convergent walking changes were made without plan approval. Reverted to PLAN-3392 implication walking. Convergent walking improvements are in GROUP-convergent-walking (PLAN-3434-3443) awaiting proper implementation. | PLAN-3431 (revert) |
