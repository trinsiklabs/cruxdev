# Lifecycle Hook Patterns

**Gap:** Claude Code has 12 lifecycle events with 3 handler types. CruxDev has MCP tools but no hook system.

## Hook Events for CruxDev

| Event | When | Use Case |
|---|---|---|
| pre_convergence_start | Before convergence begins | Validate prerequisites |
| post_round_complete | After each audit round | Notify, checkpoint |
| pre_phase_transition | Before phase change | Gate checks |
| post_convergence_complete | After convergence done | Content gen, deploy, commit |
| on_finding_discovered | When finding detected | Alert, prioritize |
| on_escalation | When convergence escalates | Emergency notification |
| on_checkpoint_saved | After checkpoint write | Backup, sync |

## Hook Configuration

```toml
# .cruxdev/hooks.toml
[[hooks]]
event = "post_convergence_complete"
type = "command"
command = "deploy.sh"
timeout = 60

[[hooks]]
event = "on_escalation"
type = "command"
command = "notify-slack.sh"
```

## Hook Types

1. **Command** — shell command with event JSON on stdin
2. **MCP tool call** — invoke another MCP tool
3. **Webhook** — POST event JSON to URL

## Current State

CruxDev's post-convergence actions (content gen, deploy trigger) are hardcoded in `convergence_submit_result`. These should be refactored into configurable hooks. The Stop hook in Claude Code settings is a client-side workaround; server-side hooks would be cleaner.

## Implementation Priority

1. `post_convergence_complete` — most impactful (replaces hardcoded content gen)
2. `on_escalation` — safety critical
3. `pre_convergence_start` — validation gate
