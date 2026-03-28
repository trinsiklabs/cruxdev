# BUILD_PLAN_055: Skills Auto-Activate by Context

**Status:** CONVERGED
**Priority:** Should Close
**Competitor:** Superpowers (context-activated skills, 110K stars)

## Context

Superpowers auto-detects which skill to activate based on the current task context. CruxDev skills are manually invoked. Auto-activation would reduce friction and make the convergence experience smoother.

## Phase 1: Skill Context Matching

- [ ] 1.1 Add `triggers` field to SKILL.md format (keywords, file patterns, phase names)
- [ ] 1.2 Implement context analyzer: scan current task description for trigger matches
- [ ] 1.3 Rank matching skills by relevance score
- [ ] 1.4 Suggest top skill(s) in convergence_next_task response

## Phase 2: Auto-Activation

- [ ] 2.1 When convergence_next_task returns a task, include `suggested_skills` field
- [ ] 2.2 MCP client can auto-load suggested skills
- [ ] 2.3 Override: user can disable auto-activation in config

## Phase 3: Tests

- [ ] 3.1 Test: code audit task suggests convergence skill
- [ ] 3.2 Test: website task suggests website skill
- [ ] 3.3 Test: no match → no suggestion

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
