# BUILD_PLAN_006: CruxDev Status Command

**Created:** 2026-03-22
**Status:** CONVERGED
**Goal:** Build a `cruxdev_status` MCP tool and `/cruxdev-status` slash command that verifies everything is wired and working correctly — parallel to Crux's `crux status` command.

**Rule:** TDD. Tests before code. 100% coverage.

---

## What Crux Status Does (Reference)

Crux's `crux status` command checks:
- `.crux/` directory exists and has expected structure
- `.claude/mcp.json` has crux server configured
- Session state file is valid JSON
- Active mode is set and mode file exists
- Knowledge directory exists
- MCP server is loadable (imports succeed, tools count matches expected)
- Audit backend is available (Ollama/API)
- Hook configuration present

CruxDev status should parallel this pattern.

---

## What CruxDev Status Checks

| Check | What it verifies | Pass condition |
|-------|-----------------|----------------|
| MCP server loadable | Can import `src.mcp_server` | No import errors |
| Tool count | Expected number of MCP tools registered | == 10 tools |
| `.cruxdev/` directory | Convergence state directory exists | Directory exists |
| `.claude/mcp.json` | CruxDev server is configured | `"cruxdev"` key present |
| Crux integration | Crux MCP also configured (optional but recommended) | Check and report |
| Python version | >= 3.11 | `sys.version_info >= (3, 11)` |
| Dependencies | pydantic, anthropic importable | Import succeeds |
| Methodology accessible | `docs/DEVELOPMENT_PATTERNS_CRUXDEV.md` readable | File exists and readable |
| Adoption process accessible | `docs/ADOPTION_PROCESS.md` readable | File exists and readable |
| Website planning accessible | `docs/WEBSITE_PLANNING.md` readable | File exists and readable |
| Active convergences | Any in-progress convergence runs | List with status |
| Slash commands | `.claude/commands/` has converge, plan, adopt, status | Files exist |

---

## Implementation

### MCP Tool: `cruxdev_status()`

Returns a structured status report:

```python
{
    "healthy": True,  # All critical checks pass
    "checks": [
        {"name": "MCP server", "passed": True, "message": "10 tools registered"},
        {"name": "State directory", "passed": True, "message": ".cruxdev/ exists"},
        {"name": "MCP config", "passed": True, "message": "cruxdev in .claude/mcp.json"},
        ...
    ],
    "warnings": [
        {"name": "Crux integration", "message": "Crux MCP not configured — recommended for full stack"}
    ],
    "active_convergences": [],
    "versions": {
        "python": "3.14.3",
        "pydantic": "2.12.5",
        "cruxdev_root": "/Users/user/personal/cruxdev"
    }
}
```

### Slash Command: `/cruxdev-status`

Calls `cruxdev_status()` and formats the output.

---

## Checklist

### Phase 1: Status Logic
- [ ] 1.1 `src/status.py` — health check functions for each verification
- [ ] 1.2 `cruxdev_status()` MCP tool in mcp_server.py
- [ ] 1.3 `.claude/commands/cruxdev-status.md` slash command
- [ ] 1.4 Tests for each health check (pass and fail cases)
- [ ] 1.5 Tests for overall status aggregation
- [ ] 1.6 Tests for MCP tool invocation
- [ ] 1.7 Coverage ≥ 100%

### Phase 2: Documentation + Website Convergence
- [ ] 2.1 Update docs/SESSION_UPGRADE.md with status command
- [ ] 2.2 Update cruxdev.dev site with status tool docs
- [ ] 2.3 Audit all docs for accuracy (two clean passes)
- [ ] 2.4 Audit website for accuracy (two clean passes)

**Total: 11 checkboxes**

---

## Test Commands

```bash
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100
```

## Convergence Criteria

- All checklist items complete
- All tests pass, coverage 100%
- Status command correctly reports pass/fail for all 12 checks
- Documentation converged (two clean passes)
- Website converged (two clean passes)
