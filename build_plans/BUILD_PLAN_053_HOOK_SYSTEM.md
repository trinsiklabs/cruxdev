# BUILD_PLAN_053: Lifecycle Hook System

**Status:** NOT STARTED
**Priority:** Should Close
**Competitor:** Claude Code (12 lifecycle events, 3 handler types)

## Context

Claude Code has a 12-event hook system with command, prompt, and agent handlers. CruxDev's MCP tools are powerful but hooks enable external code to react to lifecycle events without being inside the engine.

## Phase 1: Hook Registry

- [ ] 1.1 Define hook events: pre_convergence_start, post_round_complete, pre_phase_transition, post_convergence_complete, pre_content_generation, post_content_generation, on_finding_discovered, on_escalation, on_checkpoint_saved
- [ ] 1.2 Hook config in `.cruxdev/hooks.toml`
- [ ] 1.3 Hooks are shell commands executed with event context as JSON env var

## Phase 2: Hook Execution

- [ ] 2.1 Execute hooks synchronously (blocking) or async (fire-and-forget) based on config
- [ ] 2.2 Hook timeout (default 30s)
- [ ] 2.3 Hook failure handling: warn (default), block (stops convergence), ignore
- [ ] 2.4 Pass event metadata as CRUXDEV_HOOK_EVENT JSON env var

## Phase 3: Built-in Hooks

- [ ] 3.1 post_convergence_complete → trigger content generation (already implemented, wire as hook)
- [ ] 3.2 on_finding_discovered → optional notification (Slack, email via shell command)
- [ ] 3.3 on_escalation → alert hook

## Phase 4: Tests

- [ ] 4.1 Test: hook fires on convergence completion
- [ ] 4.2 Test: hook timeout respected
- [ ] 4.3 Test: hook failure modes (warn, block, ignore)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
