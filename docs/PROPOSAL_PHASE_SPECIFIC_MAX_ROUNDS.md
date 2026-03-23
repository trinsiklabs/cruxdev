# CruxDev Bug Fix: Phase-Specific Max Rounds

**Date:** 2026-03-23
**Severity:** Blocking — execution phase escalates before completing checklist
**Status:** DRAFT

---

## The Bug

The `max_rounds` parameter applies uniformly across ALL phases. A setting of `max_rounds: 5` means:

- Planning: max 5 rounds ✅ (typically needs 2-3)
- Plan auditing: max 5 rounds ✅ (typically needs 2-3)
- Doc alignment: max 5 rounds ✅ (typically needs 2-3)
- Viability: max 5 rounds ✅ (typically needs 1-2)
- **Executing: max 5 rounds ❌ (needs 1 round per checklist item — could be 85+)**
- Code auditing: max 5 rounds ✅ (typically needs 2-4)
- Doc auditing: max 5 rounds ✅ (typically needs 2-3)

The executing phase is fundamentally different from audit phases:
- Audit phases converge toward zero issues (fewer rounds needed as issues decrease)
- Execution phase progresses linearly through a checklist (N items = N rounds minimum)

A plan with 85 checklist items hits `max_rounds: 5` after completing only 5 items and escalates with 80 items unbuilt.

**Real-world impact:** Today, the assessment question database plan has 85 checklist items. The engine escalated after completing ~25% of execution because it hit max_rounds=5 in the executing phase.

---

## The Fix

Phase-specific max_rounds with sensible defaults.

### Option A: Per-Phase Config (Recommended)

Replace the single `max_rounds` parameter with phase-aware defaults:

```python
PHASE_MAX_ROUNDS = {
    "planning": 5,          # Converge plan in ≤5 rounds
    "plan_auditing": 5,     # Audit-fix-reaudit ≤5 rounds
    "doc_alignment": 5,     # Align with docs ≤5 rounds
    "viability": 3,         # Viability check ≤3 rounds
    "executing": 0,         # NO LIMIT — execute until all items complete
    "code_auditing": 5,     # Code audit ≤5 rounds
    "doc_auditing": 5,      # Doc audit ≤5 rounds
    "e2e_testing": 5,       # E2E convergence ≤5 rounds
    "patterns_update": 3,   # Patterns update ≤3 rounds
}
```

The `max_rounds` parameter from `start_convergence()` becomes the DEFAULT that can be overridden per-phase. Execution phase defaults to 0 (unlimited) because its termination condition is "all checklist items complete," not "N rounds of auditing."

### Why Execution Should Be Unlimited (or Very High)

The execution phase has a DIFFERENT termination condition than audit phases:

**Audit phases terminate when:** 2 consecutive clean passes (convergence) OR max_rounds (escalation)

**Execution phase terminates when:** All checklist items are marked complete → advance to code_auditing

There is no "convergence" concept in execution — you don't audit-fix-reaudit, you BUILD sequentially. Hitting max_rounds is always wrong in execution because it means "we stopped building before we finished." The only valid escalation in execution would be:

1. A single item fails 3 times (auto-rollback per methodology)
2. Timeout (the global timeout_minutes)
3. Net-negative (items somehow getting unchecked — shouldn't happen)

### Implementation

In `check_max_rounds()`:

```python
def check_max_rounds(state: ConvergenceState) -> bool:
    phase_limits = {
        "planning": state.max_rounds,
        "plan_auditing": state.max_rounds,
        "doc_alignment": state.max_rounds,
        "viability": min(state.max_rounds, 3),
        "executing": 0,  # 0 = no limit
        "code_auditing": state.max_rounds,
        "doc_auditing": state.max_rounds,
        "e2e_testing": state.max_rounds,
        "patterns_update": min(state.max_rounds, 3),
    }

    limit = phase_limits.get(state.phase.value, state.max_rounds)
    if limit == 0:
        return False  # No limit for this phase
    return state.round >= limit
```

That's the entire change — one function, one dict lookup, one special case for 0=unlimited.

### Alternative: Auto-Scale for Execution

Instead of unlimited, auto-scale execution max_rounds to the number of unchecked items:

```python
if state.phase == ConvergencePhase.EXECUTING:
    unchecked = count_unchecked_items(state.plan_file)
    limit = unchecked + 10  # buffer for retries
    return state.round >= limit
```

This prevents truly infinite loops while allowing enough rounds for all items. The +10 buffer accounts for items that need retries.

### Recommendation

**Option A with auto-scale hybrid:** Default execution to unlimited (0), but add a safety valve: if execution has been running for more rounds than `total_items * 2`, escalate. This catches infinite loops while allowing normal long checklists to complete.

```python
if state.phase == ConvergencePhase.EXECUTING:
    total_items = count_total_items(state.plan_file)
    return state.round >= total_items * 2  # 2x safety valve
```

For 85 items: escalates at 170 rounds (should never hit). For 5 items: escalates at 10 rounds (reasonable).

---

## Backward Compatibility

- Existing `max_rounds` parameter still works — becomes the default for audit phases
- Execution phase behavior changes: no longer escalates at max_rounds
- All other phases: behavior unchanged
- `start_convergence()` API: unchanged

---

## Testing

1. Start convergence with a 20-item plan
2. Execute all 20 items (each takes 1 round)
3. Verify: execution completes at round 20, advances to code_auditing
4. Verify: audit phases still respect max_rounds=5

---

## Cost

~15 lines changed in `check_max_rounds()`. No API changes. No state format changes.
