# BUILD_PLAN_037: MCP Server + Skills to CruxDev Standard + Template Wiring

**Status:** CONVERGED
**Priority:** Critical
**Depends on:** BP034 (universal project management), MCP_SERVER_PATTERNS.md, AI_SKILLS_PATTERNS.md

## Context

Three things need converging:
1. CruxDev's MCP server needs auditing against MCP_SERVER_PATTERNS.md
2. CruxDev's skills need migrating from .claude/commands to SKILL.md standard
3. The 489 templates from swarm_sync need wiring into the get_templates tool

## Phase 1: Wire Template Registry

- [ ] 1.1 Copy templates from swarm_sync into `templates/` in cruxdev repo, organized by project type
- [ ] 1.2 Update `get_templates` MCP tool to load templates from the `templates/` directory based on project type classification
- [ ] 1.3 Template lookup: classify_project → project type → matching templates directory
- [ ] 1.4 Each project type subdirectory has its own documentation standard file

## Phase 2: Audit MCP Server Against Patterns

Per MCP_SERVER_PATTERNS.md audit dimensions:

- [ ] 2.1 **tool_design** — verify all 52 tools follow naming convention, descriptions are prompts not API docs
- [ ] 2.2 **security** — verify input validation, no hardcoded secrets, injection defense
- [ ] 2.3 **testing** — verify E2E coverage of protocol handshake + key tool flows
- [ ] 2.4 **error_handling** — verify isError for app errors, structured messages
- [ ] 2.5 **performance** — 52 tools is at the edge; evaluate grouping
- [ ] 2.6 **documentation** — server instructions field, tool descriptions complete
- [ ] 2.7 **skill_sync** — corresponding skills exist and reference correct tool names

## Phase 3: Migrate Skills to SKILL.md Standard

- [ ] 3.1 Migrate `.claude/commands/converge.md` → `.claude/skills/converge/SKILL.md`
- [ ] 3.2 Migrate `.claude/commands/plan.md` → `.claude/skills/plan/SKILL.md`
- [ ] 3.3 Migrate `.claude/commands/adopt.md` → `.claude/skills/adopt/SKILL.md`
- [ ] 3.4 Migrate `.claude/commands/status.md` → `.claude/skills/status/SKILL.md`
- [ ] 3.5 Add proper frontmatter: name, description (trigger condition), disable-model-invocation where appropriate
- [ ] 3.6 Verify each skill references correct MCP tool names from server.rs

## Phase 4: Skill-Server Sync Verification

- [ ] 4.1 Extract tool names from server.rs (all 52)
- [ ] 4.2 Extract tool references from each skill file
- [ ] 4.3 Verify every skill tool reference exists in the server
- [ ] 4.4 Identify tools with no skill coverage — create or document

## Phase 5: Tests

- [ ] 5.1 Template lookup returns correct templates per project type
- [ ] 5.2 Skill files parse correctly (frontmatter + body)
- [ ] 5.3 All skill tool references resolve to actual server tools
