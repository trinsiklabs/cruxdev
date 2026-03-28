# BUILD_PLAN_050: Durable Execution / Checkpointing

**Status:** NOT STARTED
**Priority:** Must Close
**Depends on:** BP047 (harness gap closure)
**Competitor:** DeepAgents (LangGraph checkpointing — crash recovery from last checkpoint)

## Context

A convergence loop crash at step 47 of 50 currently means restarting. With checkpointing, it resumes at 47. CruxDev already has atomic writes and WAL — this extends to full checkpoint/resume.

## Phase 1: Full Checkpoint After Each Round

- [ ] 1.1 Extend convergence state to include checkpoint metadata (last_checkpoint_at, checkpoint_version)
- [ ] 1.2 After each round's submit_result, write a full checkpoint to `.cruxdev/convergence/checkpoints/<id>_round_<N>.json`
- [ ] 1.3 Checkpoint includes: full ConvergenceState, current phase, round, all findings history, WAL position
- [ ] 1.4 Atomic write (write-then-rename) for checkpoint files

## Phase 2: Resume from Checkpoint

- [ ] 2.1 On `start_convergence`, check for existing checkpoint for the same plan_file
- [ ] 2.2 If checkpoint exists, offer to resume from it (include checkpoint info in response)
- [ ] 2.3 New MCP tool: `resume_convergence(convergence_id)` — loads last checkpoint and continues
- [ ] 2.4 Validate checkpoint integrity before resume (version, schema, WAL consistency)

## Phase 3: Checkpoint Rotation

- [ ] 3.1 Keep last N checkpoints (default 3), delete older ones
- [ ] 3.2 Report checkpoint size in convergence_status

## Phase 4: Kill-Resume Test

- [ ] 4.1 Test: start convergence, submit 3 rounds, kill process, restart, verify resume from round 3
- [ ] 4.2 Test: corrupted checkpoint → falls back to fresh start
- [ ] 4.3 Test: checkpoint rotation (only last 3 kept)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
