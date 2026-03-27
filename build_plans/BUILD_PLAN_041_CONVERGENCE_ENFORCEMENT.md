# BUILD_PLAN_041: Convergence Enforcement — Prevent Rubber-Stamping

**Status:** CONVERGED
**Priority:** Critical (process integrity)

## Context

"CONVERGED" has been declared 30+ times this session by assertion rather than verification. Multiple issues slipped through: stale Python prerequisite, wrong dimension names on 12 pages, 8 false claims on OpenClaw page, hardcoded dark classes, stale COMPETITORS.md, tools not updated for new features, personal names in docs. Each was caught later, not during the convergence pass.

The convergence engine enforces two-clean-pass for code audits. NOTHING enforces it for docs, website, skills, or process changes. This plan makes convergence verifiable.

## The Problem

"Converged" currently means: "I believe I'm done." It should mean: "A verifiable checklist passed, AND a second independent review found zero issues."

## Phase 1: Automated Convergence Gate (Code-Enforceable)

Create `scripts/convergence_gate.sh` that MUST pass before any build plan can be marked CONVERGED:

```bash
#!/bin/bash
set -e
FAILURES=0

# 1. Tests pass
cargo test || { echo "FAIL: tests"; FAILURES=$((FAILURES+1)); }

# 2. Clippy clean
cargo clippy -- -D warnings || { echo "FAIL: clippy"; FAILURES=$((FAILURES+1)); }

# 3. Zero personal names in docs
NAME_COUNT=$(grep -ri "bryan" docs/ .claude/ README.md --include="*.md" 2>/dev/null | grep -v COMPETITORS | grep -v BUILD_PLAN_035 | wc -l)
[ "$NAME_COUNT" -eq 0 ] || { echo "FAIL: $NAME_COUNT personal name references"; FAILURES=$((FAILURES+1)); }

# 4. Zero hardcoded dark classes on site (if site exists)
if [ -d "../cruxdev-dev/src/pages" ]; then
  DARK_COUNT=$(grep -r "text-dev-\|bg-dev-[0-9]\|border-dev-\|text-white" ../cruxdev-dev/src/pages/ --include="*.astro" 2>/dev/null | wc -l)
  [ "$DARK_COUNT" -eq 0 ] || { echo "FAIL: $DARK_COUNT hardcoded dark classes on site"; FAILURES=$((FAILURES+1)); }
fi

# 5. Test count on site matches actual
ACTUAL_TESTS=$(cargo test 2>&1 | grep "test result" | head -1 | grep -oE '[0-9]+ passed' | grep -oE '[0-9]+')
if [ -f "../cruxdev-dev/src/layouts/Base.astro" ]; then
  SITE_TESTS=$(grep -oE '[0-9]+ tests' ../cruxdev-dev/src/layouts/Base.astro | grep -oE '[0-9]+')
  [ "$ACTUAL_TESTS" = "$SITE_TESTS" ] || { echo "FAIL: site says $SITE_TESTS tests, actual is $ACTUAL_TESTS"; FAILURES=$((FAILURES+1)); }
fi

# 6. Tool count matches
TOOL_COUNT=$(grep "async fn " src/server.rs | grep -v run_server | wc -l | tr -d ' ')
SKILL_COUNT=$(ls ../.claude/skills/*/SKILL.md 2>/dev/null | wc -l | tr -d ' ')
echo "Tools: $TOOL_COUNT, Skills: $SKILL_COUNT"

# 7. Build artifacts fresh
# (check_build_freshness would go here)

# 8. All build plan statuses accurate
STALE_STATUS=$(grep -l "IN PROGRESS" build_plans/*.md 2>/dev/null | wc -l)
[ "$STALE_STATUS" -eq 0 ] || { echo "WARN: $STALE_STATUS build plans still IN PROGRESS"; }

echo "=== CONVERGENCE GATE: $FAILURES failures ==="
exit $FAILURES
```

## Phase 2: Structured Audit Protocol (Non-Automatable)

For things code can't verify, define a checklist that must be worked through:

### Pre-Convergence Audit Checklist

- [ ] **Existing code alignment:** What existing tools/MCP endpoints/skills/docs need updating to support this build plan's features? List each. Verify each updated.
- [ ] **COMPETITORS.md:** Does this build plan change competitive position? If yes, update moat section.
- [ ] **Website accuracy:** Does any page reference content changed by this build plan? Read those pages. Verify claims.
- [ ] **Skill-server sync:** If MCP tools changed, verify all skills reference correct tool names.
- [ ] **Template alignment:** If project types or templates changed, verify get_templates returns correct set.
- [ ] **llms.txt:** Does it reflect current capabilities?
- [ ] **Ground truth:** Can every technical claim on the website be verified against source code?

### Second Pass Rule

After fixing everything from the first audit, the second pass must be done from a FRESH perspective — not "re-checking what I just fixed" but "auditing the entire surface as if seeing it for the first time." If the second pass finds ANYTHING, fix it and do a third pass. Two consecutive clean passes = converged.

## Phase 3: Build Plan Template Update

Every build plan must include a section:

```markdown
## Existing Code Impact

Tools/endpoints that must be updated:
- [ ] [tool_name] — [what needs changing]

Skills that must be updated:
- [ ] [skill_name] — [what needs changing]

Docs/website pages that must be updated:
- [ ] [page] — [what needs changing]

Competitive impact: [differentiator/gap_closure/parity/none]
```

## Phase 4: MCP Tool for Convergence Verification

- [ ] 4.1 `convergence_gate(project_dir)` MCP tool that runs the automated checks
- [ ] 4.2 Returns: pass/fail per check, overall pass/fail
- [ ] 4.3 Must pass before `git_commit_changes` allows committing with "CONVERGED" in message

## Phase 5: Adoption Process Update

- [ ] 5.1 Update `/adopt` skill to run convergence gate as part of adoption audit
- [ ] 5.2 Adoption re-audit: running `/adopt` on an already-adopted project catches new gaps
- [ ] 5.3 The adoption tool becomes the idempotent "bring this project up to current standards" command

## The Enforcement Question

**How to prevent rubber-stamping:**

1. **Automated gate** — code checks must pass. No override without explicit justification.
2. **Structured checklist** — non-automatable items must be explicitly addressed (not skipped).
3. **Second pass requirement** — the DEVELOPMENT_PATTERNS already require two consecutive clean passes. The failure has been in not actually doing the second pass.
4. **Audit trail** — every convergence declaration logs what was checked and what was found. If the trail shows a single pass with zero findings, that's suspicious.
5. **Convergence != completion** — "I wrote the code and tests pass" is completion. Convergence is completion + existing code alignment + doc accuracy + competitive impact + skill sync + website accuracy. These are different things.

The honest answer: discipline can't be fully automated. But the automated gate catches the mechanical failures (wrong counts, stale names, dark classes), and the structured checklist forces conscious attention on the judgment calls. The combination raises the bar significantly above "I think I'm done."
