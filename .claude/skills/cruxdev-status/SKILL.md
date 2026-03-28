---
name: cruxdev-status
description: /status — Check convergence status
---

# /cruxdev-status — Check convergence status

Show the current state of a convergence run.

## Arguments

$ARGUMENTS = convergence_id (optional — if not provided, list all runs)

## Protocol

If convergence_id provided:
- Call `convergence_status($ARGUMENTS)`
- Display: phase, round, consecutive clean passes, total findings, total fixed, elapsed time
- If terminal, say whether it converged or escalated (and why)

If no convergence_id:
- Look for state files in `.cruxdev/convergence_state/`
- List all runs with their status
- Highlight any that are still in progress
