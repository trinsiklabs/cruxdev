# AI Skills Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 50+ sources including Agent Skills Specification (agentskills.io), Anthropic, OpenAI evals methodology, Block engineering, Superpowers, OpenClaw SKILL.md, practitioner analysis
**Last updated:** 2026-03-27
**Companion:** MCP_SERVER_PATTERNS.md (tools provide capability; skills provide intent)

## Core Principle

**An MCP server without a corresponding skill is a toolbox without a carpenter.** The server exposes tools. The skill teaches the agent WHEN to call them, in WHAT order, with WHAT parameters, and HOW to interpret results. Without the skill, 52 tools are just 52 functions the agent has to figure out on its own.

**The skill IS the product. The MCP server is the implementation.**

---

## 1. Skill Design

### Single Responsibility
One skill, one job. The "God Skill" — one skill solving too many loosely related problems — causes the instruction body to become bloated and internally inconsistent. Break into smaller chainable units.

### Two-Zone Architecture (Block Engineering)
- **Rules/execution zone:** Owned by scripts, templates, and the convergence engine. Deterministic. Reproducible.
- **Interpretation/action zone:** Owned by the agent. Contextual. Creative.

Know what the agent should NOT decide (loops, scoring, termination) and what it SHOULD decide (language understanding, novel content, conversation).

### Constitutional Constraints
LLMs are "people pleasers" — they soften results, skip steps, add caveats. Counter with explicit constraints:
- "Never override, adjust, or recalculate scores from script output"
- "If the engine says failed, show it as-is"
- "Follow the formatting template exactly, not approximately"
- "Do NOT decide when to stop. The engine decides."

### Scope
- SKILL.md under 500 lines
- Instructions under 5,000 tokens
- Detailed reference material in `references/`, `scripts/`, `assets/` directories

---

## 2. Skill-MCP Relationship

### Complementary Layers

| Layer | What It Does | Example |
|-------|-------------|---------|
| **MCP tools** | Deterministic API calls with fixed schemas | `start_convergence(plan_file, max_rounds)` |
| **Skills** | Behavioral guidance — when, why, how, in what order | "After convergence, check competitive impact, update COMPETITORS.md, regenerate vs/ pages" |

### When to Use Which

| Prefer MCP Tools | Prefer Skills |
|-------------------|--------------|
| Precise operations with clear schemas | Behavioral guidance, workflow orchestration |
| Single-source-of-truth maintenance | Contextual adaptation |
| Rapidly evolving technical surfaces | Relatively static methodology |

### The CruxDev Pattern
The MCP server exposes 52 deterministic tools. The skills (converge, plan, adopt, status) provide behavioral orchestration — telling the agent how to use those tools in sequence, what to do with results, and when to stop.

---

## 3. Context Detection

### The Description Is the Routing Mechanism
AI agents format all skill descriptions into the system prompt and let the LLM decide which to activate. The description is NOT a human-readable summary — it's a **trigger condition**.

Bad: "Helps with convergence"
Good: "Drives autonomous convergence on a build plan through audit-fix-re-audit loops until two consecutive independent clean passes. Use when the user says 'converge', references a build plan, or asks to drive a plan to completion."

### Invocation Modes
- **Implicit (automatic):** Agent activates when request matches description
- **Explicit (manual):** User types `/skill-name`
- **Gated:** `disable-model-invocation: true` for skills with side effects (deploy, commit, push)

### Progressive Disclosure (3 Levels)
1. **Metadata (~100 tokens):** Name + description loaded at startup for all skills
2. **Instructions (<5,000 tokens):** Full SKILL.md body loaded when activated
3. **Resources (as needed):** Reference files loaded only when required

**Budget constraint:** Agents allocate ~2% of context for skill descriptions. Too many skills = some excluded.

### Activation Reliability
Vercel study: agents activate correct skills 79% of the time with standard descriptions. 100% requires embedding full docs in context (impractical). Progressive disclosure is the practical compromise.

---

## 4. Prompt Engineering for Skills

### Tool Descriptions Deserve Engineering Attention
"Tool definitions and specifications should be given just as much prompt engineering attention as your overall prompts." — Anthropic

### Formatting Principles
- Strict formatting with examples for every action
- Include concrete invocation examples with parameters
- Require one-line reason before a tool call and short observation after — boosts traceability, reduces loops

### Error-Proofing (Poka-yoke)
Restructure arguments to make misuse harder. Example: require absolute paths instead of relative to eliminate path confusion.

### Dynamic Context
- Shell command injection: `` !`command` `` runs before skill content is sent
- String substitutions: `$ARGUMENTS`, `$ARGUMENTS[N]`, `${CLAUDE_SESSION_ID}`

---

## 5. Skill Documentation (SKILL.md Standard)

### Required Frontmatter
```yaml
---
name: converge          # 1-64 chars, lowercase alphanumeric + hyphens
description: >          # 1-1024 chars — this is the TRIGGER CONDITION
  Drives autonomous convergence on a build plan through audit-fix-re-audit
  loops until two consecutive independent clean passes. Use when the user
  says 'converge', references a build plan, or asks to drive a plan to
  completion.
---
```

### Optional Frontmatter
- `license`, `compatibility`, `metadata`
- `disable-model-invocation` — require explicit user invocation (for side-effect skills)
- `user-invocable` — false for background knowledge skills
- `paths` — glob patterns limiting when skill loads
- `allowed-tools` — restrict which tools the skill can access

### Directory Structure
```
skill-name/
  SKILL.md          # Required
  scripts/          # Optional: executable code
  references/       # Optional: documentation
  assets/           # Optional: templates, resources
```

---

## 6. Skill Composition

### Strategies
- **Parallel:** All relevant skills load into same context. Risk: conflicts, overflow.
- **Hierarchical:** Parent dispatches to child skills/subagents per task.
- **Sequential:** Skills execute in order, each building on the prior. CruxDev's `/converge` does this: plan → converge plan → execute → converge code → update patterns.

### Conflict Resolution
- More-deeply-nested files take precedence
- Enterprise > personal > project scope
- If skill and command share a name, skill wins
- Contradictory instructions produce unpredictable behavior — be explicit about priorities

---

## 7. Skill Testing

### OpenAI's Eval Methodology (Gold Standard)

1. **Define success criteria** before building — outcome, process, style, efficiency goals
2. **Create prompt dataset** (10-20 prompts) — explicit, implicit, contextual, and negative controls
3. **Deterministic grading** — JSONL traces with rule-based checks on specific events
4. **Model-assisted rubric** — constrained JSON Schema output for qualitative assessment
5. **Progressive coverage** — failures become test cases, metrics replace guesswork

### CruxDev Convergence Applied to Skills
Two consecutive independent clean passes = skill converged. Run the skill twice independently; if both passes are clean, it works.

### What to Test
- [ ] Explicit invocation activates correctly
- [ ] Implicit activation triggers on matching requests
- [ ] Negative controls: skill does NOT activate on unrelated requests
- [ ] Tool calls use correct names and parameters
- [ ] Workflow sequence matches specification
- [ ] Error recovery works (tool returns error → skill adapts)
- [ ] No thrashing (repeated failed attempts without progress)

---

## 8. Skill Distribution

### Scopes
- **Project:** `.claude/skills/` — committed to repo
- **Personal:** `~/.claude/skills/` — all projects
- **Enterprise:** Managed settings deployment
- **Marketplace:** Published to skills.sh, ClawHub, SkillsMP

### The Open Standard
SKILL.md works across agents: Claude Code, OpenAI Codex, OpenClaw, VS Code Copilot. Published by Anthropic at agentskills.io (December 2025), adopted by OpenAI.

### Security Warning
13%+ of marketplace skills contain critical vulnerabilities. Vet before installing. Static analysis, content hashing, and human curation are minimum requirements.

---

## 9. Safety

### Side-Effect Skills
Skills that deploy, commit, push, delete, or send messages externally MUST use `disable-model-invocation: true`. The agent cannot decide to deploy on its own.

### Dry-Run Default
All destructive operations default to dry-run. Explicit `live_mode=true` required.

### Least Privilege
Use `allowed-tools` to restrict what tools a skill can access. A newsletter skill should not have access to `git_push_changes`.

---

## 10. Skill-Server Synchronization (Critical)

**When the MCP server changes, all corresponding skills are immediately stale.**

| Server Change | Skill Impact |
|--------------|-------------|
| Tool renamed | Skill references old name — breaks silently |
| Parameter changed | Skill sends wrong params — error or wrong result |
| New tool added | Agents don't know to use it — invisible |
| Tool removed | Skill references nonexistent tool — error |
| Workflow changed | Skill encodes old sequence — wrong behavior |

### Convergence Rules
1. After ANY MCP server change, all skills must be re-audited against `listTools()` output
2. Skill tool references verified against actual tool names and parameter schemas
3. New tools without skill coverage = invisible to agents
4. Build freshness gate: if server source is newer than skill file, skill is stale
5. Skill accuracy audit: every claim about tool behavior verified against actual tool behavior

---

## 11. Anti-Patterns

| Anti-Pattern | Fix |
|-------------|-----|
| **God Skill** | Single responsibility — one skill, one job |
| **Vague description** | Description is trigger condition, not summary |
| **Context overload** | Progressive disclosure — metadata → instructions → resources |
| **Contradictory instructions** | Explicit priorities when constraints conflict |
| **Vibe-testing** | Systematic evals with prompt datasets and deterministic grading |
| **Missing negative controls** | Test that skill does NOT activate on unrelated requests |
| **Stale skill** | Re-audit after every server change |
| **No constitutional constraints** | Explicit rules the agent cannot override |
| **Side-effect skills without gating** | `disable-model-invocation: true` for destructive actions |

---

## 12. Audit Dimensions

For convergence engine integration — audit skills against:

1. **design** — single responsibility, scope under 500 lines, constitutional constraints present
2. **description** — trigger condition (not summary), specific enough for reliable activation
3. **mcp_sync** — tool references match actual server tool list and schemas
4. **testing** — eval dataset exists, negative controls included, two clean passes
5. **safety** — side-effect skills gated, dry-run default, least privilege
6. **documentation** — SKILL.md frontmatter complete, examples included, edge cases covered
7. **distribution** — correct scope (project/personal/enterprise), namespace collision checked

---

## References

- Agent Skills Specification — agentskills.io
- 3 Principles for Agent Skills — Block engineering
- Building Effective Agents — Anthropic
- Testing Skills with Evals — OpenAI
- Skills vs MCP Tools — LlamaIndex
- Superpowers skill architecture — GitHub
- OpenClaw SKILL.md — docs.openclaw.ai
- Extend Claude with Skills — Claude Code docs
