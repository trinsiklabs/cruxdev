# BUILD_PLAN_039: Full Skills Coverage for All MCP Tool Workflows

**Status:** NOT STARTED
**Priority:** Critical (CruxDev doesn't meet its own MCP/Skills standard)
**Depends on:** BP037 (MCP/skills standard), AI_SKILLS_PATTERNS.md

## Context

CruxDev has 52 MCP tools and 7 skills. Per AI_SKILLS_PATTERNS.md: "New tools without skill coverage are effectively invisible to agents." 5 workflow areas have zero skill coverage: research, git, growth, issues, and build freshness.

The current skills also need auditing — they were migrated from .claude/commands but may reference old tool names or missing workflows.

## Phase 1: New Skills (5 workflow areas)

### 1.1 /research
- [ ] `.claude/skills/research/SKILL.md`
- [ ] Covers: research_topic → research_status → verify_research_sources → counter_research
- [ ] Workflow: start 5-pass research → iterate → verify sources → run adversarial check → converge
- [ ] Trigger: "research [topic]", "deep research", "5-pass research"

### 1.2 /git
- [ ] `.claude/skills/git/SKILL.md`
- [ ] Covers: git_status_check → git_commit_changes → git_push_changes → create_pull_request → merge_pull_request
- [ ] Workflow: check status → stage + commit (with safety) → push → create PR → merge
- [ ] Trigger: "commit", "push", "create PR", "merge"
- [ ] `disable-model-invocation: true` — side effects (pushes, merges)

### 1.3 /growth
- [ ] `.claude/skills/growth/SKILL.md`
- [ ] Covers: run_growth_cycle → growth_status → post_to_typefully → init_growth_config
- [ ] Workflow: check config → run cycle (changelog → post → metrics) → report
- [ ] Trigger: "run growth cycle", "growth status", "post to X"
- [ ] `disable-model-invocation: true` — posts to external services

### 1.4 /issues
- [ ] `.claude/skills/issues/SKILL.md`
- [ ] Covers: monitor_issues → issue_audit_log
- [ ] Workflow: fetch issues → sanitize → evaluate → respond (dry-run default)
- [ ] Trigger: "check issues", "monitor issues", "triage issues"

### 1.5 /build
- [ ] `.claude/skills/build/SKILL.md`
- [ ] Covers: check_build_freshness → rebuild_stale
- [ ] Workflow: detect targets → check freshness → rebuild if stale
- [ ] Trigger: "check build", "rebuild", "are binaries current"

## Phase 2: Audit Existing Skills Against Tool List

- [ ] 2.1 /converge — verify references start_convergence, convergence_submit_result, convergence_status, convergence_cancel, convergence_next_task
- [ ] 2.2 /plan — verify references create_plan_template, validate_plan_structure
- [ ] 2.3 /adopt — verify references classify_project, inventory_project, get_templates, analyze_gaps, gap_status, install_cruxdev, get_methodology, get_adoption_process
- [ ] 2.4 /status — verify references convergence_status, cruxdev_status
- [ ] 2.5 /inbox — verify references session_register, check_inbox, acknowledge_message
- [ ] 2.6 /competitor-add — verify references discover_competitors, research_competitor, setup_competitive_analysis, generate_gap_analysis
- [ ] 2.7 /cruxdev-status — verify references cruxdev_status

## Phase 3: Add Domain Skill

- [ ] 3.1 /domain — `.claude/skills/domain/SKILL.md`
- [ ] Covers: domain init, domain status, add project to domain, dependency map
- [ ] Trigger: "init domain", "domain status", "add project"

## Phase 4: Skill-Server Sync Test

- [ ] 4.1 Extract all tool names referenced in all skill files
- [ ] 4.2 Verify each against server.rs tool list
- [ ] 4.3 Zero unresolved references

## Verification

```bash
# Count skills vs tool workflows
ls .claude/skills/*/SKILL.md | wc -l
# Should be >= 13 (7 existing + 5 new + 1 domain)

# All tool references resolve
grep -h "tool\|_tool\|MCP" .claude/skills/*/SKILL.md | grep -oP '[a-z_]+' | sort -u
```
