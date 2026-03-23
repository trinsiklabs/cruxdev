# Proposal Implementation Handoff

**Date:** 2026-03-23
**For:** The session that created PROPOSAL_DOC_ALIGNMENT_GATE.md and PROPOSAL_GREENFIELD_EXECUTION.md

Both proposals have been implemented, tested, and converged. Here's what changed and how to use the new capabilities.

---

## What Was Built

### 1. Document Alignment Gate (from PROPOSAL_DOC_ALIGNMENT_GATE.md)

**New convergence phase:** `DOC_ALIGNMENT` sits between `PLAN_AUDITING` and `VIABILITY`.

**How it works:**
- Plans should include a `## Document Alignment` section listing product docs and memory files the plan must conform to
- The engine extracts these paths and dispatches `doc_align` tasks
- Claude Code reads each alignment doc and verifies the plan conforms to it
- Two consecutive clean passes required before advancing

**For projects with no product docs:**
- The section is optional (warning, not error)
- If no alignment docs are found, the phase auto-advances
- Add "None — new project" in the section to be explicit

**What to change in your plans:**
```markdown
## Document Alignment

### Product Docs (this plan must conform to):
- path/to/design_doc.md — what decisions it contains
- path/to/pricing_doc.md — pricing rules

### Memory Files (captured decisions this plan must respect):
- memory/project_badges.md — badge requirements
- memory/feedback_content_policy.md — content filtering rules
```

### 2. Green-Field Execution (from PROPOSAL_GREENFIELD_EXECUTION.md)

**The `EXECUTING` phase is now functional**, not a pass-through.

**How it works:**
- After planning converges, the engine parses the plan's checklist items
- Returns `execute` tasks one at a time with the checklist item description
- Each task includes: item ID, phase name, completion progress
- Claude Code builds the item (writes code, writes tests, runs tests)
- When all items complete → advances to `CODE_AUDITING`
- For existing codebases (all items already checked or no items), auto-advances

**New task type: `execute`**
```json
{
  "task_type": "execute",
  "description": "Execute checklist item 1.5: Write multi-axis scoring validator (3/15 complete, 20.0%)",
  "metadata": {
    "checklist_item": "1.5",
    "phase": "Phase 1",
    "progress": {"total": 15, "completed": 3, "remaining": 12, "percentage": 20.0}
  }
}
```

**Checklist format the parser understands:**
```markdown
## Phase 1: Setup
- [ ] 1.1 Create project structure
- [ ] 1.2 Set up database
- [x] 1.3 Configure CI (already done)
```

### 3. Updated Convergence Lifecycle

The full lifecycle is now:
```
plan → plan_audit → doc_alignment → viability → execute → code_audit → doc_audit → e2e_test → patterns → converged
```

With auto-advance for phases that don't apply:
- No alignment docs → skip doc_alignment
- No checklist items (or all complete) → skip execute
- Both are backward compatible with existing plans

---

## How to Use Going Forward

### If you're planning a green-field build:
1. Use `/plan "goal"` — the template now includes Document Alignment section
2. List any existing product docs in the alignment section
3. Use `/converge plan.md` — the engine will:
   - Audit the plan
   - Verify alignment against listed docs
   - Execute each checklist item
   - Audit the resulting code
   - Converge

### If you're working on an existing codebase:
Nothing changes. The new phases auto-advance when they don't apply.

### If you have product docs that plans should respect:
Add them to the `## Document Alignment` section. The engine catches drift like:
- Plan says "free users get X" but pricing doc says "free users get Y"
- Plan omits a constraint from a memory file
- Plan contradicts a design decision

---

## Technical Details

**Files changed:**
- `src/engine/state.py` — new `DOC_ALIGNMENT` phase (11 phases total, was 10)
- `src/engine/convergence.py` — `PHASE_ORDER` updated
- `src/engine/task_router.py` — doc_align + execute task routing, checklist parsing, alignment doc extraction
- `src/engine/checklist_parser.py` — NEW: parse/track plan checklist items
- `src/engine/plan_convergence.py` — phase-aware (doesn't reset phase for DOC_ALIGNMENT)
- `src/engine/plan_validator.py` — Document Alignment section is recommended (warning)
- `src/engine/runner.py` — DOC_ALIGNMENT + EXECUTING phase handling
- `src/mcp_server.py` — updated tool descriptions
- `.claude/commands/converge.md` — updated with doc_align + execute task types

**Tests:** 443 passing, 100% coverage, 5.2 seconds
**Backward compatible:** Yes — existing plans and convergence runs work unchanged

---

## Open Questions Resolved

From PROPOSAL_DOC_ALIGNMENT_GATE.md:
1. **Manual vs auto-discovered:** Manual declaration. Auto-advance when empty.
2. **How deep:** Standard convergence — two clean passes.
3. **Versioning:** Not implemented yet — future work.
4. **Bidirectional:** Not yet — plan→docs direction only.

From PROPOSAL_GREENFIELD_EXECUTION.md:
1. **Option A (auto-detect):** Implemented. No checklist items = auto-advance.
2. **Checklist parser:** Yes, reads standard `- [ ] N.M description` format.
3. **Parallelizable:** Not yet — items execute sequentially. Dependency tracking is future work.
