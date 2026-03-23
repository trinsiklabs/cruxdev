# CruxDev Bug Fix: Planning Phase Never Transitions

**Date:** 2026-03-23
**Severity:** Blocking — convergence cannot proceed past planning
**Status:** DRAFT

---

## The Bug

The PLANNING phase handler in `get_next_task()` never checks convergence. It always returns a "write" task regardless of how many clean passes have been submitted.

Every other phase follows this pattern:
```python
if state.phase == ConvergencePhase.PLAN_AUDITING:
    if check_convergence(state):           # ← checks for 2 clean passes
        state.phase = ConvergencePhase.DOC_ALIGNMENT  # ← advances
        state.round = 0
        state.consecutive_clean = 0
        save_state(state, state_path)
        return get_next_task(state, state_path, ...)  # ← recurse into next phase
    return Task(task_type="audit", ...)    # ← normal task if not converged
```

The PLANNING phase does NOT follow this pattern:
```python
if state.phase == ConvergencePhase.PLANNING:
    return Task(                           # ← always returns write task
        task_type="write",                 # ← never checks convergence
        description="Create or refine the build plan...",
        files=[state.plan_file],
        dimensions=[],
    )
```

Result: `consecutive_clean` increments correctly via `submit_result`, but `get_next_task` never reads it for the PLANNING phase. The engine sits in PLANNING forever, eventually hitting `max_rounds` and escalating.

## The Fix

Add convergence check to the PLANNING handler, same pattern as every other phase:

```python
if state.phase == ConvergencePhase.PLANNING:
    if check_convergence(state):
        state.phase = ConvergencePhase.PLAN_AUDITING
        state.round = 0
        state.consecutive_clean = 0
        save_state(state, state_path)
        return get_next_task(state, state_path, source_files, doc_files, test_command)
    return Task(
        task_type="write",
        description="Create or refine the build plan. Follow the methodology from get_methodology().",
        recommended_tier=TASK_MODEL_TIERS.get("write"),
        files=[state.plan_file],
        dimensions=[],
    )
```

**One `if` block added. Same pattern as every other phase. No other changes needed.**

## Where

File: `src/mcp_server.py` (or wherever `get_next_task` lives — appears to be in `src/mcp_server.py` based on imports)

Function: `get_next_task()`

Location: The `if state.phase == ConvergencePhase.PLANNING:` block

## Testing

After fix, verify:
1. Start convergence with a plan
2. Submit 2 clean passes
3. `convergence_next_task` should return a `plan_auditing` or `doc_align` task (NOT another `write` task)
4. Continue through the full lifecycle: planning → plan_auditing → doc_alignment → viability → executing → code_auditing → done

## Impact

This bug blocks ALL convergence runs — no plan can ever advance past the planning phase. It's the reason we've been manually driving execution outside the engine.
