---
name: growth
description: "Run the autonomous growth cycle — changelog, X post, README check, metrics. Use when the user asks about growth, posting, metrics, or running the growth cycle. Calls run_growth_cycle, growth_status, post_to_typefully, init_growth_config MCP tools."
disable-model-invocation: true
---

# /growth — Autonomous Growth Cycle

## Arguments

$ARGUMENTS = optional: "status", "init", or repo name

## Protocol

### Check config first

Call `growth_status(project_dir)` to verify config exists and see current state.

If no config: call `init_growth_config(project_name, repo, project_dir)`.

### Run growth cycle

Call `run_growth_cycle(repo, project_dir, dry_run)`:
1. Generates release notes from git log
2. Checks README health
3. Composes X/Twitter post
4. Posts to Typefully (if not dry-run and API key set)
5. Collects GitHub metrics (stars, forks, issues)

### Report results

Show: actions taken, metrics collected, README suggestions, post content.
