# BUILD_PLAN_052: Multi-Agent Parallel Execution

**Status:** CONVERGED
**Priority:** Should Close
**Competitors:** Claude Code (Agent Teams with git worktrees), Codex (6 cloud subagents)

## Context

CruxDev currently runs convergence sequentially in a single session. Claude Code and Codex both support parallel agents in isolated environments. For large projects, parallel independent tasks would significantly reduce wall-clock convergence time.

## Phase 1: Parallel Task Detection

- [ ] 1.1 Analyze convergence tasks for independence (no shared file dependencies)
- [ ] 1.2 Mark parallelizable tasks in convergence plan
- [ ] 1.3 Report parallelism opportunities in convergence_status

## Phase 2: Git Worktree Isolation

- [ ] 2.1 For parallel tasks, create git worktrees (one per task)
- [ ] 2.2 Each task runs in its own worktree (no file conflicts)
- [ ] 2.3 Merge worktree changes back to main branch after task completion
- [ ] 2.4 Handle merge conflicts (escalate to human if unresolvable)

## Phase 3: MCP Multi-Session Coordination

- [ ] 3.1 Extend MCP server to support multiple concurrent convergence sessions
- [ ] 3.2 Each session operates on its own worktree
- [ ] 3.3 Coordinator session tracks overall progress
- [ ] 3.4 New MCP tool: `start_parallel_convergence(plan_file, max_parallel)` → spawns workers

## Phase 4: Tests

- [ ] 4.1 Test: two independent tasks run in parallel worktrees
- [ ] 4.2 Test: changes merge cleanly
- [ ] 4.3 Test: conflict detection and escalation

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
