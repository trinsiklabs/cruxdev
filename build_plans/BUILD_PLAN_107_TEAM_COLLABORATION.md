# BUILD_PLAN_107: Team Collaboration — Multi-User Convergence

**Status:** PLANNED
**Priority:** Post-v1 (v1.5 or v2.0 — see Priority Assessment below)
**Depends on:** BUILD_PLAN_059/090 (Enterprise Readiness), Rust engine migration complete
**Gap source:** `/for/entrepreneurs` page: "No user roles, permissions, or team collaboration features. CruxDev is currently a single-operator tool."

---

## Context

CruxDev is a single-operator tool. One person runs convergence sessions, one person sees state, one person decides. This works for solo founders and individual contributors but blocks adoption by teams of 2+ engineers, agencies, and companies. Every serious competitor in the adjacent project management space (Linear, Notion, Asana, Monday.com, GitHub Projects) treats multi-user collaboration as table stakes.

CruxDev does not compete directly with those tools — it is a convergence harness, not a project tracker. But teams that adopt CruxDev need to know: who converged what, who can run destructive operations, what changed while I was away, and can I see what the team shipped this week. Without these answers, CruxDev stays a power tool for solo operators.

---

## Competitive Landscape: Team Features in Adjacent Tools

### Linear

- **Roles:** Owner, Admin, Member, Guest. Granular per-workspace and per-team permissions.
- **Teams:** Workspaces contain teams; each team has its own backlog, cycles, and triage.
- **Activity:** Full activity feed per issue, per project, per team. Audit log for admins.
- **Notifications:** Per-user notification preferences. Slack/Discord/email integrations. @mentions.
- **Collaboration:** Real-time presence (who's viewing an issue). Comments, reactions, assignments.
- **API/Integrations:** OAuth2, API keys, webhooks. GitHub/GitLab sync. Slack bot.
- **What CruxDev can learn:** Linear's team model is lightweight — workspace > team > member. No complex org hierarchies. Activity feeds are per-entity, not global firehose.

### Notion

- **Roles:** Owner, Admin, Member, Guest (with page-level permissions).
- **Teams:** Teamspaces with configurable visibility (open, closed, private).
- **Activity:** Page history, edit tracking, comments. No real-time presence in databases.
- **Notifications:** Mentions, page updates, comment replies. Slack integration.
- **Collaboration:** Real-time co-editing. Inline comments. Page-level permissions.
- **API/Integrations:** OAuth2, API keys. 100+ integrations via marketplace.
- **What CruxDev can learn:** Page-level (project-level) permissions are more useful than global roles for CruxDev's model. Guest access matters for client-facing agencies.

### Asana

- **Roles:** Admin, Member, Limited Access Member, Guest. Organization-level and project-level.
- **Teams:** Organization > Team > Project hierarchy. Cross-team projects supported.
- **Activity:** Activity feed per task, project, and portfolio. Full audit log (Business/Enterprise).
- **Notifications:** Granular: task assigned, due date approaching, status change, comment. Email digest.
- **Collaboration:** Task comments, approvals workflow, proofing (file review). Portfolios for executive view.
- **What CruxDev can learn:** Approval workflows map to convergence sign-off. Portfolio view maps to cross-project convergence dashboard.

### Monday.com

- **Roles:** Admin, Member, Viewer, Guest. Board-level permissions (owner, subscriber).
- **Teams:** Workspace > Board > Group > Item. Flexible structure.
- **Activity:** Activity log per board. Automation audit trail.
- **Notifications:** Per-column notifications. Slack/Teams/email. Automations as notification triggers.
- **Collaboration:** Updates (comment thread per item), @mentions, file sharing.
- **What CruxDev can learn:** Board-level (project-level) permissions with subscriber model. Automations triggering notifications = convergence events triggering team alerts.

### GitHub Projects

- **Roles:** Inherited from repository permissions (Admin, Write, Triage, Read).
- **Teams:** GitHub Organizations > Teams > Repository access. CODEOWNERS for review routing.
- **Activity:** Commits, PRs, issues, reviews all feed into project boards. Full audit via API.
- **Notifications:** Per-repo watch settings. @mentions. Review requests. CI status.
- **Collaboration:** PR reviews, issue discussions, code owners auto-assignment.
- **What CruxDev can learn:** Permission inheritance from the code repository is natural for CruxDev since convergence operates on repos. CODEOWNERS pattern maps to "who can converge this project."

---

## Gap Assessment: What CruxDev Needs

### Current State

| Capability | Status |
|-----------|--------|
| Multi-session awareness | Exists — session bus (`src/bus/broker.py`) with SQLite-backed discovery, messaging, heartbeats |
| Session identity | Exists — session ID, project name, directory. No user identity. |
| Cross-project messaging | Exists — issues, improvements, patterns, breaking changes between sessions |
| Convergence state | Single-operator — one state file per convergence run, no ownership metadata |
| Access control | None — any session can call any MCP tool on any project |
| Activity history | Partial — convergence round results stored, but no user attribution |
| Notifications | None — inbox is pull-only (check_inbox), no push notifications |
| Dashboard | None — convergence_status shows one run, no team/org view |

### Required Capabilities (Prioritized)

**P0 — Minimum Viable Team (2-5 people)**

1. **User identity on sessions.** Every session registers with a user ID (not just session ID). Convergence state records who started it, who submitted each round.
2. **Role-based tool access.** Three roles: `owner` (all tools), `contributor` (convergence + git tools, no destructive ops), `viewer` (read-only status, dashboard, inbox). Enforced at MCP tool dispatch.
3. **Team namespace.** Projects belong to a team. Team membership stored in SQLite (extending bus.db). State isolation: team A cannot see team B's convergence state.
4. **Activity feed.** Append-only log of convergence events (started, round completed, converged, escalated, git operations). Queryable by project, user, time range.
5. **Team dashboard tool.** MCP tool that returns: active convergences across team projects, recent activity, who's working on what.

**P1 — Usable Team (5-20 people)**

6. **Push notifications.** Webhook endpoint for Slack/Discord/email when: convergence completes, escalation occurs, breaking change detected, PR created.
7. **Project-level permissions.** Beyond roles: per-project overrides (user X is contributor on project A but viewer on project B).
8. **Convergence handoff.** User A starts convergence, goes home. User B picks up the same convergence run with full context (already partially supported by state persistence).
9. **Shared patterns.** Team-level DEVELOPMENT_PATTERNS that override per-project patterns. "Our team always enforces X."

**P2 — Enterprise Team (20+ people)**

10. **SSO/OIDC integration.** Authenticate via company identity provider. Map IdP groups to CruxDev teams.
11. **Audit log.** Immutable, exportable record of every tool call with user attribution (overlaps with BUILD_PLAN_090 Phase 1).
12. **Admin console.** Web UI for team management, permission grants, usage analytics.

---

## Architecture: How This Works with MCP

### The Key Constraint

CruxDev runs as an MCP server — it is called by Claude Code (or other MCP clients), not the other way around. The MCP protocol is stateless request-response. There is no persistent connection, no server-initiated push. This constrains the design:

- **Authentication** must happen per-request (token in environment or session registration).
- **Push notifications** must use external webhooks, not MCP push.
- **Real-time presence** is approximated by heartbeat polling (session bus already does this).

### Proposed Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    MCP Clients                          │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐              │
│  │ Alice's  │  │  Bob's   │  │ Carol's  │              │
│  │ Claude   │  │ Claude   │  │ Claude   │              │
│  │ Code     │  │ Code     │  │ Code     │              │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘              │
│       │              │              │                    │
│       │  MCP stdio   │  MCP stdio   │  MCP stdio        │
│       ▼              ▼              ▼                    │
│  ┌──────────────────────────────────────────────┐       │
│  │            CruxDev MCP Server                │       │
│  │                                              │       │
│  │  ┌──────────────────────────────────┐        │       │
│  │  │      Auth Middleware             │        │       │
│  │  │  ENV: CRUXDEV_USER_TOKEN         │        │       │
│  │  │  Resolves: token → user → role   │        │       │
│  │  └──────────┬───────────────────────┘        │       │
│  │             │                                │       │
│  │  ┌──────────▼───────────────────────┐        │       │
│  │  │      Permission Gate             │        │       │
│  │  │  Checks: user.role >= tool.min   │        │       │
│  │  │  Checks: user.projects ∋ target  │        │       │
│  │  └──────────┬───────────────────────┘        │       │
│  │             │                                │       │
│  │  ┌──────────▼───────────────────────┐        │       │
│  │  │      Tool Handlers               │        │       │
│  │  │  (existing tools, unchanged)     │        │       │
│  │  └──────────┬───────────────────────┘        │       │
│  │             │                                │       │
│  │  ┌──────────▼───────────────────────┐        │       │
│  │  │      Activity Logger             │        │       │
│  │  │  Appends to team activity feed   │        │       │
│  │  └──────────────────────────────────┘        │       │
│  └──────────────────────────────────────────────┘       │
│                                                          │
│  ┌──────────────────────────────────────────────┐       │
│  │         SQLite (extends bus.db)              │       │
│  │                                              │       │
│  │  teams          users          memberships   │       │
│  │  ┌─────────┐   ┌──────────┐   ┌───────────┐ │       │
│  │  │ id      │   │ id       │   │ user_id   │ │       │
│  │  │ name    │   │ name     │   │ team_id   │ │       │
│  │  │ created │   │ token_h  │   │ role      │ │       │
│  │  └─────────┘   │ created  │   │ projects  │ │       │
│  │                 └──────────┘   └───────────┘ │       │
│  │                                              │       │
│  │  activity_log        project_permissions     │       │
│  │  ┌──────────────┐   ┌───────────────────┐    │       │
│  │  │ id           │   │ user_id           │    │       │
│  │  │ team_id      │   │ project           │    │       │
│  │  │ user_id      │   │ role_override     │    │       │
│  │  │ project      │   └───────────────────┘    │       │
│  │  │ action       │                            │       │
│  │  │ details_json │                            │       │
│  │  │ timestamp    │                            │       │
│  │  └──────────────┘                            │       │
│  └──────────────────────────────────────────────┘       │
└─────────────────────────────────────────────────────────┘
```

### Authentication Model

**Per-user tokens via environment variable.** Each user's Claude Code `.claude/mcp.json` sets `CRUXDEV_USER_TOKEN` in the MCP server env config. The token is a random string hashed and stored in SQLite. On session_register, the token resolves to a user, and the session inherits that user's roles.

```jsonc
// .claude/mcp.json (per-user, not committed)
{
  "mcpServers": {
    "cruxdev": {
      "command": "python3",
      "args": ["-m", "src.mcp_server"],
      "env": {
        "CRUXDEV_USER_TOKEN": "crxd_alice_a1b2c3..."
      }
    }
  }
}
```

**Why not OAuth/SSO for v1?** MCP stdio transport has no HTTP layer. OAuth requires redirect flows that don't exist in a CLI-to-subprocess model. Token-based auth is the right fit for MCP stdio. SSO can layer on top when a future HTTP transport or web UI exists.

### Permission Enforcement

Each MCP tool is annotated with a minimum role:

| Role | Can Do |
|------|--------|
| `owner` | All tools. Manage team, invite users, delete projects, force-cancel convergence. |
| `contributor` | Start/submit/cancel convergence, git operations, create plans, research. Cannot manage team or delete. |
| `viewer` | convergence_status, team_dashboard, check_inbox, session_list. Read-only. |

Enforcement is a decorator on each MCP tool handler:

```python
@requires_role(Role.CONTRIBUTOR)
@mcp.tool()
def start_convergence(plan_file: str) -> str:
    ...
```

The decorator reads the current session's user from the resolved token, checks role against minimum, and returns a permission error if insufficient.

### Team Namespace

- Each team has a unique ID and name.
- Projects are registered to teams (a project can belong to one team).
- Convergence state paths include team ID: `~/.cruxdev/teams/{team_id}/convergence_state/`.
- Session bus queries are scoped to team: a session only sees other sessions in its team.
- Cross-team messaging is blocked by default (configurable for org-wide broadcasts).

### Activity Feed

Every MCP tool call emits an activity event:

```json
{
  "id": "evt_...",
  "team_id": "team_...",
  "user_id": "user_alice",
  "project": "cruxbot",
  "action": "convergence_started",
  "details": {"plan_file": "BUILD_PLAN_042.md", "convergence_id": "conv_..."},
  "timestamp": 1711612800.0
}
```

Events are stored in SQLite `activity_log` table. The `team_dashboard` tool queries this to show "what happened today across the team."

### Notification Webhooks

A `webhooks` table stores per-team webhook URLs:

```sql
CREATE TABLE webhooks (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    url TEXT NOT NULL,
    events TEXT NOT NULL,  -- JSON array: ["convergence_completed", "escalated", "breaking_change"]
    created_at REAL NOT NULL
);
```

After significant events, the activity logger checks for matching webhooks and fires HTTP POST requests. This enables Slack/Discord/email integration without CruxDev knowing about those platforms.

---

## Implementation Phases

### Phase 1: User Identity and Activity Log (Foundation)

**Goal:** Every convergence action is attributed to a user. No access control yet — just identity.

- [ ] 1.1 Add `users` table to bus.db schema (id, name, token_hash, created_at)
- [ ] 1.2 Add `CRUXDEV_USER_TOKEN` env var resolution in MCP server startup
- [ ] 1.3 Modify `session_register` to associate session with resolved user
- [ ] 1.4 Add `activity_log` table to bus.db schema
- [ ] 1.5 Emit activity events from convergence tools (start, submit, cancel, status)
- [ ] 1.6 Emit activity events from git tools (commit, push, PR, merge)
- [ ] 1.7 New MCP tool: `activity_feed(project?, user?, since?, limit?)` — query activity log
- [ ] 1.8 Add user_id field to ConvergenceState (who started this convergence)
- [ ] 1.9 Record user_id on each RoundResult (who submitted this round)
- [ ] 1.10 CLI command: `cruxdev user create <name>` — generates token, inserts into users table

**Tests:** User resolution, activity logging, feed queries, convergence state attribution.

### Phase 2: Teams and Role-Based Access

**Goal:** Users belong to teams. Tools enforce role-based access.

- [ ] 2.1 Add `teams` table (id, name, created_at)
- [ ] 2.2 Add `memberships` table (user_id, team_id, role, created_at)
- [ ] 2.3 `@requires_role` decorator for MCP tool handlers
- [ ] 2.4 Permission gate: check user's role in the team that owns the target project
- [ ] 2.5 Annotate all existing MCP tools with minimum role requirements
- [ ] 2.6 New MCP tools: `team_create`, `team_invite`, `team_members`, `team_remove_member`
- [ ] 2.7 Scope session bus to team (sessions only discover same-team sessions)
- [ ] 2.8 Error messages for permission failures (clear, actionable: "You need contributor role on team X")
- [ ] 2.9 Anonymous/no-token mode: all tools available (backward compatibility for solo users)

**Tests:** Role enforcement for each tool, team scoping, backward compatibility.

### Phase 3: Team Dashboard and Project Permissions

**Goal:** Team-wide visibility. Per-project permission overrides.

- [ ] 3.1 New MCP tool: `team_dashboard()` — active convergences, recent activity, who's working on what
- [ ] 3.2 Add `project_permissions` table (user_id, project, role_override)
- [ ] 3.3 Permission resolution: project override > team role > denied
- [ ] 3.4 Dashboard shows: convergence status per project, last activity per project, team member status
- [ ] 3.5 Convergence handoff support: `convergence_transfer(id, to_user)` — transfers ownership
- [ ] 3.6 Team-level patterns: `~/.cruxdev/teams/{team_id}/patterns/` directory with team overrides

**Tests:** Dashboard aggregation, permission override resolution, handoff state transfer.

### Phase 4: Notifications and Integrations

**Goal:** Push notifications via webhooks. Slack/Discord integration.

- [ ] 4.1 Add `webhooks` table to bus.db schema
- [ ] 4.2 Webhook dispatch on convergence events (async, non-blocking, fire-and-forget)
- [ ] 4.3 New MCP tools: `webhook_create`, `webhook_list`, `webhook_delete`
- [ ] 4.4 Slack webhook payload format (Slack Block Kit compatible)
- [ ] 4.5 Discord webhook payload format
- [ ] 4.6 Email via configurable SMTP (optional, not required)
- [ ] 4.7 Webhook retry with exponential backoff (3 attempts, then dead-letter)
- [ ] 4.8 Webhook event filtering (subscribe to specific event types per webhook)

**Tests:** Webhook dispatch, payload formats, retry logic, event filtering.

### Phase 5: SSO and Admin Console (Enterprise)

**Goal:** Enterprise identity integration and web-based management.

- [ ] 5.1 OIDC provider integration (map IdP groups to CruxDev teams)
- [ ] 5.2 SAML support (for enterprises that require it)
- [ ] 5.3 Admin web UI: team management, permission grants, usage analytics
- [ ] 5.4 Audit log export (JSONL, CSV, PDF — overlaps BUILD_PLAN_090)
- [ ] 5.5 Usage metering per team (convergence runs, tool calls, token consumption)

**Tests:** OIDC flow, SAML flow, admin API, export formats.

---

## Priority Assessment: v1 or Later?

**Recommendation: Post-v1. Target v1.5 or v2.0.**

### Why Not v1

1. **CruxDev's core value is convergence, not collaboration.** The single-operator model works for the initial target market: solo founders, individual ICs, and small teams where one person drives the agent. Adding team features before the convergence engine is rock-solid dilutes focus.

2. **The session bus is already multi-session.** Multiple Claude Code sessions can already discover each other, exchange messages, and coordinate. This is 80% of what a 2-person team needs. The missing piece is user identity and permissions — not fundamental architecture.

3. **Enterprise readiness (BUILD_PLAN_090) is the prerequisite.** Audit trails and access controls must exist before team features make sense. Building team collaboration without audit logging creates a compliance gap that enterprises will reject.

4. **Market validation is needed first.** Are teams actually blocked by this? Or do they use CruxDev per-engineer and coordinate via existing tools (Slack, Linear, GitHub)? Ship v1 without team features, measure demand, then build what's actually requested.

### When It Becomes Urgent

- When a paying customer says "I'd buy a team plan but I need X."
- When adoption metrics show multi-engineer teams bouncing off CruxDev.
- When competitors (Superpowers, Devin, Factory) ship team convergence features.
- When the enterprise sales conversation requires it (SOC2 audit trail + team roles as package).

### Suggested Staging

| Version | What Ships |
|---------|-----------|
| v1.0 | Single-operator convergence. Session bus for multi-session coordination. |
| v1.5 | Phase 1 (user identity + activity log) + Phase 2 (teams + roles). Minimum viable team. |
| v2.0 | Phase 3 (dashboard + project permissions) + Phase 4 (notifications). Full team product. |
| v2.5+ | Phase 5 (SSO, admin console). Enterprise tier. |

---

## Verification

```bash
# Phase 1-4: Rust tests (when migrated)
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings

# Phase 1-4: Python tests (current)
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

---

## Open Questions

1. **Shared convergence vs. isolated convergence.** Can two users submit rounds to the same convergence run simultaneously, or is convergence always serial (one active submitter at a time)? Serial is simpler and matches the current state machine. Parallel submission would require conflict resolution.

2. **Team-level vs. org-level.** Is one level of grouping (team) enough, or do enterprises need org > team hierarchy? Start with one level; add hierarchy when demanded.

3. **Token management UX.** How do users create and manage tokens? CLI tool (`cruxdev user create`) for v1.5. Web UI for v2.5. Self-service vs. admin-only token creation.

4. **Pricing model.** Per-seat? Per-team? Per-convergence-run? This affects what gets metered in the activity log. Decide before Phase 1 implementation so metering is built in from the start.

5. **Offline/local-only teams.** Some teams won't want a central SQLite database. Could team state be per-repo (`.cruxdev/team.db`)? This conflicts with cross-project dashboard but respects air-gapped environments.
