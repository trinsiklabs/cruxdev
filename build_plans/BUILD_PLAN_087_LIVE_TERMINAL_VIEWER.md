# BUILD_PLAN_087: Live Terminal Viewer — Watch CruxDev Work

**Status:** NOT STARTED
**Priority:** High (marketing + transparency)
**Triggered by:** User wants visitors to see CruxDev working autonomously in real-time

## Context

A Phoenix/LiveView component that displays CruxDev's autonomous session output in a monospace terminal pane on the homepage. Visitors can watch the AI work in real-time — see build plans converge, blog posts generate, tests pass. When autonomous mode is off, the pane collapses with an expand arrow to browse past sessions.

## Data Source

Crux session logging — pull updates from `.claude/` session transcripts or a dedicated event stream. The Crux MCP server already logs interactions; this feature reads that log and streams it to a LiveView.

## Phase 1: Backend — Session Log Reader

- [ ] 1.1 Define log format: what to stream (tool calls, results, summaries — NOT full code/file contents)
- [ ] 1.2 Create Phoenix context module: `CruxdevWeb.Live.Terminal`
- [ ] 1.3 Read from Crux session log (`.claude/projects/*/` transcripts or `.cruxdev/evolution/cron.log`)
- [ ] 1.4 Tail the log file — new entries pushed via PubSub
- [ ] 1.5 Session state: `is_autonomous_active`, `current_task`, `lines[]`

## Phase 2: LiveView Component

- [ ] 2.1 Terminal pane component: monospace font, dark background (terminal colors)
- [ ] 2.2 Auto-scroll to bottom as new lines arrive
- [ ] 2.3 ANSI color code rendering (green for success, red for errors, yellow for warnings, blue for tool calls)
- [ ] 2.4 Timestamps on each line
- [ ] 2.5 Show only previous 24 hours of output, infinite scroll upward/backward through that window
- [ ] 2.6 Responsive: full-width on desktop, stacked on mobile

## Phase 3: Homepage Integration

- [ ] 3.1 Place at top of homepage, below hero but above "One clean pass isn't enough"
- [ ] 3.2 When autonomous mode active: pane is expanded, auto-scrolling, live indicator (green dot + "LIVE")
- [ ] 3.3 When autonomous mode inactive: pane is collapsed, shows "Last session: [timestamp]"
- [ ] 3.4 Dropdown arrow to expand and scroll through past session output
- [ ] 3.5 No interactivity — read-only, no input

## Phase 4: Astro → Phoenix Bridge

CruxDev's current site is Astro (static). Options:
- [ ] 4a. Embed a LiveView iframe/component in the Astro page (CORS + separate Phoenix server)
- [ ] 4b. Add a `/terminal` route on the Phoenix app, link from Astro homepage
- [ ] 4c. Migrate homepage to Phoenix (big change, deferred)
- [ ] 4d. Use Server-Sent Events (SSE) from a simple endpoint — Astro JS client consumes

**Recommendation:** Option 4d (SSE) is simplest. A Rust endpoint in CruxDev that tails the log and streams SSE events. The Astro homepage JS connects to the SSE endpoint and renders lines in a `<pre>` element.

## Phase 5: SSE Endpoint (Rust)

- [ ] 5.1 New endpoint in CruxDev binary: `cruxdev stream --port 8765`
- [ ] 5.2 Tails `.cruxdev/evolution/cron.log` and session transcripts
- [ ] 5.3 Filters: only emit summaries, tool calls, results — not full file contents
- [ ] 5.4 SSE format: `event: line\ndata: {"text": "...", "type": "info|success|error|tool"}\n\n`
- [ ] 5.5 CORS headers for cross-origin access from cruxdev.dev

## Phase 6: Astro Frontend Component

- [ ] 6.1 `<TerminalViewer />` Astro component with client-side JS
- [ ] 6.2 Connects to SSE endpoint
- [ ] 6.3 Renders lines with terminal colors (CSS classes: `.term-green`, `.term-red`, `.term-yellow`, `.term-blue`)
- [ ] 6.4 Auto-scroll, collapse/expand, live indicator
- [ ] 6.5 Graceful fallback when SSE endpoint is offline (show "Session offline")

## Phase 7: Content Generation

- [ ] 7.1 Blog post: "Watch CruxDev Work — Live Terminal on the Homepage"
- [ ] 7.2 X post announcing the feature

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
# Visual: verify terminal pane on homepage, SSE stream works
```
