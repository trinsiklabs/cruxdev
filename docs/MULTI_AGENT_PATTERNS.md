# Multi-Agent Parallel Execution Patterns

**Gap:** Claude Code (Agent Teams with git worktrees), Codex (6 cloud subagents).
**CruxDev current:** Sequential single-session convergence.

## Pattern: Git Worktree Isolation

For independent tasks, create git worktrees:

```bash
git worktree add ../project-task-1 -b task-1
git worktree add ../project-task-2 -b task-2
# Each task runs in its own worktree
# Merge back when complete
```

## Task Independence Detection

Tasks are parallelizable when:
1. No shared file dependencies (different files modified)
2. No ordering constraint (Phase 2 doesn't depend on Phase 1)
3. No shared state (different convergence IDs)

## Coordination

- Each parallel task gets its own convergence session
- Coordinator tracks overall progress
- Merge conflicts escalate to human
- All tasks must pass before declaring convergence

## MCP Integration

```
start_parallel_convergence(plan_file, max_parallel: 3)
  → Analyzes plan for independent phases
  → Creates worktree per parallel task
  → Returns convergence IDs for each
  → Coordinator polls all, merges when done
```

## Current Workaround

CruxDev already uses Agent tool for parallel research (7 stack patterns simultaneously). The missing piece is parallel CODE changes, which requires worktree isolation.

## Priority

Should Close — but CruxDev's research parallelism via Agent tool covers 80% of the use case. Code parallelism is the remaining 20%.
