# BUILD_PLAN_009: Session Bus Push Notifications

**Created:** 2026-03-23
**Status:** CONVERGED
**Goal:** Messages sent via the session bus are delivered to recipients in real-time, not just when they poll. Sessions process incoming messages without being told to check.

**Problem:** Currently the bus is write-to-SQLite (sender) + read-from-SQLite (recipient). Messages sit unread until the recipient calls `check_inbox()` or runs `/inbox`. There is no push mechanism.

**Rule:** TDD. Tests before code. 100% coverage.

---

## Document Alignment

### Product Docs:
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — post-execution convergence requires inbox check

### Memory Files:
None — new feature.

---

## Phase 1: Claude Code Hooks Integration

**Purpose:** Use Claude Code's hooks system to auto-check inbox. When Claude Code is idle or between tool calls, a hook polls the bus and surfaces new messages.

### 1A. Approach

Claude Code supports hooks in `.claude/settings.local.json`:
```json
{
  "hooks": {
    "PostToolUse": [{
      "command": "python3 -c \"from src.bus.broker import Broker; b = Broker(); msgs = b.check_inbox('PROJECT'); [print(f'[BUS] {m.type}: {m.title}') for m in msgs]\""
    }]
  }
}
```

This runs after every tool use. If there are messages, they print to stderr and the session sees them.

### 1B. Smarter: Periodic check, not every tool call

Instead of every tool call (noisy), check on a timer:
- Track last check time in a file
- Only actually poll if >60 seconds since last check
- Print a summary if messages found

### Checklist — Phase 1

- [ ] 1.1 `src/bus/hook.py` — hook script that checks inbox with rate limiting
- [ ] 1.2 Install hook into `.claude/settings.local.json` during `install()`
- [ ] 1.3 Rate limiting: check at most once per 60 seconds
- [ ] 1.4 Output format: clear, concise, actionable
- [ ] 1.5 Tests for hook logic
- [ ] 1.6 Tests for rate limiting
- [ ] 1.7 Coverage ≥ 100%

---

## Phase 2: File-Based Notification

**Purpose:** Backup mechanism — write a notification file that sessions can watch.

### 2A. Approach

When a message is sent, the broker also writes a notification file:
```
~/.cruxdev/notifications/<target_project>.notify
```

Content: JSON with message count and latest message title. The hook script checks this file (faster than SQLite query).

### Checklist — Phase 2

- [ ] 2.1 Broker writes notification file on `send_message()`
- [ ] 2.2 Hook reads notification file instead of querying SQLite
- [ ] 2.3 Notification file cleared after messages are read
- [ ] 2.4 Tests for notification file write/read/clear
- [ ] 2.5 Coverage ≥ 100%

---

## Phase 3: MCP Notification Channel

**Purpose:** Use MCP's notification mechanism if available.

### Checklist — Phase 3

- [ ] 3.1 Research: does FastMCP support server-initiated notifications?
- [ ] 3.2 If yes: implement notification on message receipt
- [ ] 3.3 If no: document limitation, rely on hooks (Phase 1)
- [ ] 3.4 Tests
- [ ] 3.5 Coverage ≥ 100%

---

## Post-Execution Convergence (Mandatory)

- [ ] Documentation convergence: audit all docs against code, two clean passes
- [ ] Website convergence: update metrics, audit content accuracy, two clean passes
- [ ] Deployment: deploy per docs/DEPLOYMENT.md
- [ ] Patterns update: capture learnings if novel
- [ ] Inbox check: process messages from other sessions

## Convergence Criteria

- All checklist items complete (including post-execution items above)
- All tests pass
- Coverage ≥ 100%
- Two consecutive clean audit passes
- Documentation verified against code
- Website metrics current
- Messages are delivered within 60 seconds of being sent
- No manual `/inbox` needed for message processing
