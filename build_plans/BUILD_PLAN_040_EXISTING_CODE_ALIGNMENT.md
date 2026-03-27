# BUILD_PLAN_040: Existing Code Alignment — Fix Tools Lagging Behind Build Plans

**Status:** CONVERGED
**Priority:** Critical (CruxDev doesn't follow its own convergence rules)

## Context

Multiple build plans added new capabilities without updating existing tools that should use them. This violates the "existing code alignment" anti-pattern added to DEVELOPMENT_PATTERNS_CRUXDEV.md.

## Findings

| Tool | Issue | Build Plan That Created the Gap |
|------|-------|-------------------------------|
| `get_templates` | Returns 20 hardcoded templates, doesn't use 218 filesystem templates | BP037 |
| `classify_project` | Doesn't return template count or new type names in response | BP034 |
| `analyze_gaps` | Compares against 20 hardcoded templates, not 218 per project type | BP037 |
| `install_cruxdev` | Only generates .mcp.json, no OpenClaw or multi-client config | BP035 |
| `cruxdev_status` | Doesn't report skill count, template count, or domain status | BP039, BP037, BP038 |
| `setup_competitive_analysis` | Doesn't include Integration field from CompetitorProfile | BP031 |
| `run_growth_cycle` | Some values hardcoded instead of reading from growth.toml | BP023 |

## Phase 1: Fix get_templates

- [ ] 1.1 Call `get_filesystem_templates()` in addition to hardcoded templates
- [ ] 1.2 Return both built-in and filesystem templates merged
- [ ] 1.3 Include template count per project type in response

## Phase 2: Fix analyze_gaps

- [ ] 2.1 Use filesystem templates as the comparison set for the detected project type
- [ ] 2.2 Flag missing templates as findings

## Phase 3: Fix cruxdev_status

- [ ] 3.1 Include: tool count, skill count, template count, domain detection
- [ ] 3.2 Include: build artifact freshness check

## Phase 4: Fix setup_competitive_analysis

- [ ] 4.1 Accept and pass through integration data
- [ ] 4.2 Include integrations in comparison output

## Phase 5: Fix install_cruxdev

- [ ] 5.1 Generate config for multiple clients (detect which client is running)
- [ ] 5.2 Or generate all configs and let user choose

## Phase 6: Tests

- [ ] 6.1 get_templates returns filesystem templates
- [ ] 6.2 cruxdev_status includes new fields
- [ ] 6.3 analyze_gaps uses full template set
