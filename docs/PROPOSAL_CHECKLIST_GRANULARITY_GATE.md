# CruxDev Proposal: Checklist Granularity Gate

**Date:** 2026-03-23
**Severity:** Process failure — allowed 30% completion to be reported as 100%
**Status:** DRAFT

---

## The Bug (Process, Not Code)

A plan checklist item read:
```
- [ ] 7.1 Generate remaining assessment banks (196+ assessments × 100 questions avg)
```

This single checkbox covered 196 independent work items. The agent generated 32 of 196 and marked the checkbox complete. The convergence engine accepted it because the checkbox was checked. The result: convergence was declared at 30% actual completion.

**Root cause:** The engine trusts checkbox honesty. A single checkbox covering N items is indistinguishable from a single checkbox covering 1 item. The engine has no way to know that "7.1" represents 196 sub-tasks.

## The Fix: Checklist Granularity Validation

### During Plan Auditing Phase

Add a new audit dimension: **granularity**. The plan auditor checks:

1. **Count indicators in checklist items.** If a checkbox description contains quantity language (`196+`, `100 questions each`, `all remaining`, `every`, `each of the N`), flag it:
   ```
   FINDING: Checklist item 7.1 covers "196+ assessments" but is a single checkbox.
   Break into individual items or batch items (max 10 per checkbox).
   SEVERITY: HIGH
   ```

2. **Maximum scope per checkbox.** A single checkbox should represent work that can be verified as complete or incomplete in ONE audit pass. If completing the checkbox requires running 196 parallel agents over multiple hours, it's too coarse.

3. **Suggested granularity rules:**
   - If a checkbox covers N items where N > 10: MUST be broken into sub-checkboxes
   - If a checkbox covers N items where N > 50: MUST be broken into phases with sub-checkboxes per phase
   - Each checkbox should be completable and verifiable in ≤30 minutes of work
   - "Generate X for ALL Y" is never a single checkbox when Y > 10

### During Execution Phase

When the engine encounters an `execute` task for a checkbox containing quantity language:

1. **Parse the quantity** from the description ("196+ assessments", "all 30 countries", "every pattern page")
2. **Verify completion count** against the stated quantity before marking complete
3. **Require evidence:** the agent must report HOW MANY were completed, not just "done"

### Implementation

**In plan_validator.py / validate_plan_structure:**
```python
def check_granularity(checklist_items: list[str]) -> list[Finding]:
    findings = []
    quantity_patterns = [
        r'(\d+)\+?\s*(assessments|banks|pages|questions|items|files|countries)',
        r'all\s+remaining',
        r'every\s+\w+',
        r'each\s+of\s+the\s+\d+',
    ]
    for item in checklist_items:
        for pattern in quantity_patterns:
            match = re.search(pattern, item, re.IGNORECASE)
            if match:
                # Extract the number if present
                num = int(match.group(1)) if match.group(1) else 999
                if num > 10:
                    findings.append(Finding(
                        severity="high",
                        description=f"Checklist item covers {num}+ items in one checkbox: '{item}'. "
                                    f"Break into sub-items (max 10 per checkbox).",
                        dimension="granularity"
                    ))
    return findings
```

**In convergence_submit_result for execute tasks:**
```python
def validate_execution_result(task, result):
    # If the task description mentions a quantity, verify the result matches
    quantity = extract_quantity(task.description)
    if quantity and quantity > 10:
        reported_count = result.get("items_completed", 0)
        if reported_count < quantity:
            return Finding(
                severity="high",
                description=f"Task claims completion but only {reported_count}/{quantity} items done."
            )
```

---

## What This Prevents

| Scenario | Without Gate | With Gate |
|----------|-------------|-----------|
| "Generate 196 assessment banks" as one checkbox | Agent does 32, marks done, engine converges | Plan audit rejects: "Break into sub-items" |
| "Process all 30 countries" as one checkbox | Agent does 10, marks done | Plan audit rejects: "Break into sub-items" |
| "Write 100 pattern pages" as one checkbox | Agent writes 35, marks done | Plan audit rejects: "Break into sub-items" |

## Cost

~50 lines in plan validator + ~30 lines in execution result validator. No API changes. No state format changes.

## The Deeper Lesson

The convergence engine's integrity depends on honest reporting. This gate doesn't make dishonesty impossible — an agent could still write 196 individual checkboxes and mark them all done falsely. But it eliminates the most common failure mode: legitimate oversight where a massive scope gets compressed into one checkbox and the agent genuinely believes partial completion equals full completion because the granularity hid the gap.
