# BUILD_PLAN_056: Proven Unattended Evolution Run

**Status:** CONVERGED
**Priority:** Should Close
**Competitor:** yoyo-evolve (24 days unattended, 200 → 31K lines)

## Context

yoyo-evolve proved autonomous evolution works at scale (cron every 8 hours, 24 days, 700 commits). CruxDev's evolution pipeline is built but never run unattended. This plan proves it works.

## Phase 1: Cron Setup

- [ ] 1.1 Create `.cruxdev/scheduled_tasks.json` for evolution cycle scheduling
- [ ] 1.2 Write cron entry that runs `cruxdev evolution run-cycle` every 4 hours
- [ ] 1.3 Ensure evolution cycle is idempotent (safe to run multiple times)
- [ ] 1.4 Add logging to `.cruxdev/evolution/run.log`

## Phase 2: Safety Guardrails

- [ ] 2.1 Max actions per cycle (default: 5 commits, 3 posts, 1 PR)
- [ ] 2.2 Dry-run mode for first 3 cycles (log actions, don't execute)
- [ ] 2.3 Emergency stop file: `.cruxdev/evolution/STOP` halts all cycles
- [ ] 2.4 Email/notification on escalation or error

## Phase 3: 48-Hour Proof Run

- [ ] 3.1 Run evolution cycle every 4 hours for 48 hours
- [ ] 3.2 Document: what was gathered, evaluated, posted, engaged
- [ ] 3.3 Publish results as proof of autonomous evolution

## Phase 4: Tests

- [ ] 4.1 Test: evolution cycle completes without error
- [ ] 4.2 Test: STOP file halts execution
- [ ] 4.3 Test: max actions respected

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
