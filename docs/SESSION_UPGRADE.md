# CruxDev Session Upgrade Guide

**Date:** 2026-03-22
**For:** Active Claude Code sessions that started before the latest CruxDev updates

Your session is running an older version of CruxDev. Significant capabilities have been added. This document tells you what's new and how to use it. Read the whole thing before continuing your current work.

---

## What Changed

### 1. CruxDev Is Now an MCP Server

CruxDev has a real MCP server with 10 tools. If your project doesn't have it configured yet, run:

```python
import sys
sys.path.insert(0, "/Users/user/personal/cruxdev")
from src.install import install
result = install(".")
for item in result["items"]:
    print(f"  ✓ {item}")
```

Then **restart Claude Code** to activate the new MCP tools.

### 2. New Slash Commands Available

After restarting with the MCP server active, you have:

| Command | What it does |
|---------|-------------|
| `/converge BUILD_PLAN_XXX.md` | Run the convergence engine on a build plan |
| `/plan "goal description"` | Create a structured build plan using the methodology |
| `/adopt .` | Install CruxDev into the current project |
| `/status` | Check convergence progress |

### 3. Convergence Engine (Request-Response Loop)

The engine is no longer theoretical — it's code. 314 tests, 100% coverage. When you `/converge`, the protocol is:

1. Call `start_convergence(plan_file)` → engine returns your first task
2. Loop:
   - Call `convergence_next_task(id)` → engine tells you what to do
   - Execute the task (audit files, fix issues, run tests)
   - Call `convergence_submit_result(id, findings_json)` → engine processes
   - Repeat until `task_type` is `"done"` or `"escalated"`
3. **You do NOT decide when to stop.** The engine decides. Two consecutive clean passes = convergence.

For a clean pass (no issues found), submit: `convergence_submit_result(id, "[]")`

For findings, submit JSON:
```json
[{"id": "f1", "file": "src/main.py", "dimension": "correctness",
  "severity": "high", "description": "Off-by-one in loop",
  "suggested_fix": "Change < to <=", "fixed": true}]
```

### 4. Model Tier System

Every task now has a `recommended_tier` field:

| Tier | Models | Used for |
|------|--------|----------|
| micro | qwen3:8b, qwen3:4b | Title generation, simple checks |
| fast | claude-haiku, gpt-5-nano, qwen3.5:27b | Plan audits, doc audits, E2E tests |
| local | qwen3-coder:30b, qwen3.5:27b | Primary coding tasks (free) |
| standard | claude-sonnet, gpt-5-mini, qwen3-coder:30b | Code audits, fix generation |
| frontier | claude-opus, gpt-5 | Security audits, complex architecture |

When the engine gives you a task with `recommended_tier: "fast"`, use a fast model. When it says `"standard"`, use a more capable one.

Crux MCP tools for model selection:
- `get_model_for_task("code_audit")` → returns the best available model for that task
- `get_available_tiers()` → shows what's available at each tier
- `get_mode_model("build-py", "primary")` → model for this mode

### 5. Escalation on Failure

If a model fails:
- **Validation failure** (bad JSON, wrong schema): Engine escalates up to 2 tiers (micro→fast→standard)
- **Quality failure** (misses obvious issues): Engine escalates 1 tier (standard→frontier)
- **Provider error** (API down, timeout): Engine switches to another provider at the same tier

You don't manage this manually — the engine handles it when you use the convergence loop.

### 6. Quality Feedback

The system tracks success rates per task type and model tier. Over time, it learns which tiers work for which tasks and automatically recommends upgrades when success rates drop below 70%.

Check stats: `get_model_quality_stats()`

### 7. Plan Validation

Before converging, validate your plan structure:

```
validate_plan_structure("BUILD_PLAN_001.md")
```

This checks: title exists, has checklist items, references tests, has convergence criteria. Fix any errors before starting convergence.

### 8. Crux Slash Commands (if Crux MCP is also active)

| Command | What it does |
|---------|-------------|
| `/crux-mode review` | Switch Crux mode |
| `/crux-knowledge "auth patterns"` | Search knowledge base |
| `/crux-session update working_on "building API"` | Update session state |
| `/crux-correct "used mock instead of DI"` | Log a correction for learning |
| `/crux-adopt . build-py` | Install Crux into a project |

---

## What to Do Right Now

1. **Read this document** (you're doing it)
2. **Check if CruxDev MCP is configured**: `cat .claude/mcp.json` — look for `"cruxdev"` entry
3. **If not configured**, run the install script above, then restart Claude Code
4. **If you have an active build plan**, you can now `/converge` it instead of manually driving audit-fix loops
5. **If you're planning**, use `/plan "goal"` to get a structured template with methodology context
6. **Continue your current work** — these tools augment what you're doing, they don't replace it

---

## Key Rules (Unchanged)

- TDD for everything. Tests before code.
- 100% coverage enforced.
- Two consecutive independent clean passes = convergence. One clean pass is NOT convergence.
- The engine owns termination. You don't decide when to stop.
- Verify all status claims empirically. Coverage numbers come from tools, not memory.
