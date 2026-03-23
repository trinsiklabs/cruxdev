# CruxDev Improvement Proposal: Green-Field Execution Mode

**Date:** 2026-03-23
**Problem:** The convergence engine gets stuck in the planning phase when building from scratch — no source files exist to audit, so it never transitions to code phase.
**Status:** DRAFT

---

## The Problem

The CruxDev convergence lifecycle assumes code EXISTS:

```
planning → code_audit → doc_audit → done
```

The engine transitions from planning to code_audit when source files are provided. But in GREEN-FIELD projects (like creating a new Phoenix app from scratch), there ARE no source files when convergence starts. The engine keeps asking for plan refinement because it has nothing to audit.

**Real-world impact:** Today, building trueassess.me from scratch. The convergence engine converged the plan (3 clean passes) but then got stuck asking for more plan refinement indefinitely. We had to drive execution manually outside the engine.

---

## Current Behavior

```
start_convergence(plan_file) → phase: "planning"
submit_result(clean) → consecutive_clean: 1
submit_result(clean) → consecutive_clean: 2
convergence_next_task() → still "planning" (no source files to transition to)
submit_result(clean) → consecutive_clean: 3
convergence_next_task() → still "planning" (stuck)
... eventually escalates: "planning_max_rounds"
```

The engine doesn't know how to: write code → then audit it. It only knows how to: audit existing code.

---

## Proposed Solution: Green-Field Execution Phase

Add a new phase between planning and code_audit: **execution**.

### New Phase: `execution`

```
planning → execution → code_audit → doc_audit → done
```

**When planning converges (2 clean passes) AND no source files exist yet:**
→ Transition to `execution` phase instead of `code_audit`

**In execution phase, the engine:**
1. Reads the plan's checklist items
2. Converts each checklist item to a task (methodology Section 2A)
3. Returns tasks one at a time: `task_type: "execute"` with the checklist item description
4. The agent executes the task (writes code, writes tests, runs tests)
5. Agent submits result with: files created, tests passing/failing
6. Engine tracks which checklist items are complete
7. When all checklist items are complete → transition to `code_audit` (now source files exist)

### Task Type: `execute`

```json
{
  "task_type": "execute",
  "description": "Phase 1, item 1.5: Write multi-axis scoring validator",
  "checklist_item": "1.5",
  "phase": "Phase 1",
  "context": "Build a function that validates cross_scores reference valid assessments/dimensions",
  "files_to_create": ["lib/trueassess/assessments/validators.ex"],
  "test_files": ["test/trueassess/assessments/validators_test.exs"],
  "dependencies": ["1.1", "1.2"]
}
```

### How the Engine Knows to Enter Execution Phase

Option A: **Auto-detect** — if `source_files` param is empty/missing when planning converges, enter execution instead of code_audit.

Option B: **Plan metadata** — the plan declares `status: NOT STARTED` and lists files to CREATE (not audit). The engine reads this.

Option C: **Explicit flag** — `start_convergence(plan_file, mode: :greenfield)` tells the engine this is a new build.

**Recommendation: Option A** — simplest, no plan format changes needed. If source files don't exist, we're green-field.

### Execution Phase Safety Gates

Same gates as normal execution (from methodology Section 2B):
- BUILD/TEST GATE: tests must pass after each task
- TIMEOUT GATE: 15 minutes per task
- 3 failed attempts → auto-rollback, log as blocked
- Mid-execution checkpoint at ~50%

### Execution Phase Completion

When all checklist items are marked complete:
1. Engine collects all created files as `source_files`
2. Engine collects all test files
3. Transitions to `code_audit` phase
4. Now normal convergence can proceed (source files exist)

---

## Implementation

### Engine Changes (~300 lines):

1. **New phase constant:** `execution` between `planning` and `code_audit`
2. **Phase transition logic:** After planning converges, check if source_files exist:
   - Yes → `code_audit` (existing behavior)
   - No → `execution` (new behavior)
3. **Checklist parser:** Read the plan's markdown checklist items and convert to executable tasks
4. **Task tracking:** Track which checklist items are complete
5. **Phase completion:** When all items done, collect created files and transition to `code_audit`

### New Task Type:

```python
class ExecuteTask:
    task_type: str = "execute"
    description: str        # from checklist item
    checklist_item: str      # item ID (e.g., "1.5")
    phase: str               # which plan phase
    dependencies: list[str]  # prerequisite items
```

### State Changes:

```python
class ConvergenceState:
    # ... existing fields ...
    checklist_items: dict[str, bool]  # NEW: item_id → completed
    created_files: list[str]         # NEW: files created during execution
```

---

## What This Enables

1. **Full autonomous lifecycle for new projects:**
   ```
   "Create a new Phoenix app for trueassess.me. Converge."
   → plan → converge plan → execute all checklist items → audit code → audit docs → done
   ```

2. **Correct sequencing:** The engine knows to BUILD before AUDITING, not skip to auditing with nothing to audit.

3. **Progress tracking:** Each checklist item is a trackable task with completion state, so the engine (and the user) can see exactly where execution is.

4. **Resume after crash:** If a session crashes during execution, the engine knows which checklist items are done and picks up from the next one.

---

## Interaction with Subagent Dispatch

For execution phase tasks, the engine can recommend subagent dispatch:

```json
{
  "task_type": "execute",
  "description": "Phase 4: Generate Attachment Style question bank (150 questions)",
  "recommended_approach": "subagent",
  "reason": "Self-contained generation task, benefits from fresh context"
}
```

vs.

```json
{
  "task_type": "execute",
  "description": "Phase 3: Build DynamicSelector with consistency analysis",
  "recommended_approach": "inline",
  "reason": "Requires understanding of QuestionBank API built in earlier phases"
}
```

The engine can use the plan's dependency graph to recommend inline vs. subagent execution.

---

## Open Questions

1. **Should execution phase have its own convergence loop?** (audit each item after execution, not just at the end?) — Probably yes for quality, but adds complexity.

2. **Should the engine parse the plan's markdown to extract checklist items automatically?** — Yes, but the format needs to be reliable. The plan template already has `- [ ] 1.1 description` format.

3. **Should execution tasks be parallelizable?** — Only if the dependency graph allows it. The engine should track dependencies from the plan.

---

## Cost

- ~300 lines of engine code
- Plan template already has the right format (checklist items with IDs)
- No breaking changes to existing behavior (code_audit still works for non-green-field)
- 1-2 days implementation
