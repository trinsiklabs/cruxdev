# BUILD_PLAN_115: Upgrade All 63 Patterns to the New Pattern Standard

**Status:** NOT STARTED
**Priority:** High (patterns are only enforceable as converged packages — standalone markdown is suggestions)
**Depends on:** BP113 (pattern creation audit fixes — audit_pattern_package tool), BP114 (pattern-first convergence engine)
**Reference:** docs/PATTERN_CREATION_PATTERNS.md (the 7-gate lifecycle), docs/PATTERNS_AUTOMATION_ANALYSIS.md (automation classification)

## Document Alignment

- docs/PATTERN_CREATION_PATTERNS.md — defines the target standard
- docs/PATTERN_CREATION_AUDIT.md — audit findings on the standard itself
- docs/PATTERNS_AUTOMATION_ANALYSIS.md — per-pattern automation classification

## The Problem

63 pattern files exist in `docs/` as standalone markdown. Zero are converged packages in `patterns/`. The new standard (PATTERN_CREATION_PATTERNS.md) requires each pattern to pass 7 gates before entering the library. Until a pattern is upgraded, it's a suggestion — not a gate.

The automation analysis already classified every pattern: 42 have auditable checks (146+ dimensions), 68 checks are fully automatable, 47 partially, 31 LLM-only. The work is mapped. This plan executes it.

## The Approach

**NOT 63 sequential 7-gate lifecycles.** That would take months. Instead:

1. **Batch Gate 1** — all 63 patterns already exist as markdown. Gate 1 (research) is satisfied for most. Audit each for the quality gate (verifiable checks, Research Sources section). Fix deficiencies.
2. **Batch Gate 2** — the automation analysis already exists (PATTERNS_AUTOMATION_ANALYSIS.md). Extract per-pattern classification tables into individual `_AUTOMATION_ANALYSIS.md` files.
3. **Prioritized Gate 3-4** — build Go scripts in priority order (highest LLM-call reduction first). Phase 1 scripts from BP027 cover the top 12.
4. **Batch Gate 5** — write LLM usage guides for all 42 auditable patterns.
5. **Batch Gate 6** — create fixtures and run integration tests.
6. **Batch Gate 7** — move converged packages from `docs/` to `patterns/`.

## Tier Classification

Not all 63 patterns need the same level of upgrade:

| Tier | Count | Description | Target |
|---|---|---|---|
| **Tier 1: Full Package** | ~20 | High-impact auditable patterns with >50% mechanical checks | Full 7-gate: scripts + LLM guide + fixtures |
| **Tier 2: LLM Guide Only** | ~22 | Auditable patterns with <30% mechanical checks | Gates 1-2, skip 3-4, Gate 5-7 (guide + fixtures, no scripts) |
| **Tier 3: Reference Docs** | ~21 | Architecture/reference docs, not auditable patterns | Gate 1 only (quality gate + Research Sources). Stay in docs/. |

### Tier 1 Patterns (Full Package — scripts + guide)

From PATTERNS_AUTOMATION_ANALYSIS.md, highest automation potential:

| Pattern | Mechanical % | Priority Script |
|---|---|---|
| FORM_PATTERNS | 70% | audit_form_patterns.go |
| COLOR_CONTRAST_PATTERNS | 80% | audit_contrast.go |
| MOBILE_WEB_PATTERNS | 65% | audit_mobile.go |
| POST_DEPLOYMENT_PATTERNS | 75% | audit_post_deploy.go |
| MCP_SERVER_PATTERNS | 60% | audit_mcp_server.go |
| E2E_TEST_PATTERNS | 55% | audit_e2e.go |
| BLOG_PATTERNS | 60% | audit_blog.go |
| BLOG_POST_PATTERNS | 65% | audit_blog_post.go |
| BLOG_TAGGING_PATTERNS | 70% | audit_blog_tags.go |
| BLOG_PAGINATION_PATTERNS | 70% | audit_blog_pagination.go |
| METRICS_PATTERNS | 55% | audit_metrics.go |
| DASHBOARD_PATTERNS | 50% | audit_dashboard.go |
| NAVBAR_PATTERNS | 65% | audit_navbar.go |
| WEBSITE_LOGO_PATTERNS | 75% | audit_logo.go |
| SEO (GEO_PATTERNS) | 60% | audit_seo.go |
| COMPUTED_CONTENT_PATTERNS | 55% | audit_computed_content.go |
| DRY_UI_COMPONENT_PATTERNS | 50% | audit_dry_components.go |
| REGRESSION_DETECTION_PATTERNS | 60% | audit_regression.go |
| X_POST_PATTERNS | 55% | audit_x_post.go |
| BIP_PATTERNS | 50% | audit_bip.go |

### Tier 2 Patterns (LLM Guide Only — no scripts)

| Pattern | Why No Scripts |
|---|---|
| DEVELOPMENT_PATTERNS_CRUXDEV | Methodology — judgment-heavy |
| DEVELOPMENT_PATTERNS (generic) | Methodology — judgment-heavy |
| 20 stack-specific patterns (PETAL, NEXTJS, etc.) | Stack conventions — mostly judgment |
| RESEARCH_PATTERNS | Process guidance — judgment |
| UAT_TEST_PATTERNS | Acceptance criteria — judgment |
| VISUAL_VERIFICATION_PATTERNS | Visual assessment — judgment |
| MULTI_AGENT_PATTERNS | Architecture guidance — judgment |
| COMPETITORS_PATTERN | Analysis methodology — judgment |

### Tier 3 (Reference Docs — Gate 1 only)

| Doc | Why Reference |
|---|---|
| PATTERN_CREATION_PATTERNS | Meta-pattern (already upgraded) |
| PATTERN_CREATION_AUDIT | Audit findings doc |
| AI_SKILLS_PATTERNS | Architecture reference |
| KERNEL_SANDBOXING_PATTERNS | Architecture reference |
| KV_CACHE_PATTERNS | Architecture reference |
| LIFECYCLE_HOOK_PATTERNS | Architecture reference |
| CROSS_MODEL_VALIDATION_PATTERNS | Design reference |
| AUTONOMOUS_SELF_IMPROVEMENT_PATTERNS | Design reference |
| GO_SCRIPT_SECURITY_AUDITING_PATTERNS | Process reference |
| I18N_PATTERNS | Implementation reference |
| JOB_QUEUE_PATTERNS | Implementation reference |
| OAUTH1_RUST_PATTERNS | Implementation reference |
| SKILLS_AUTO_ACTIVATION_PATTERNS | Design reference |
| VERTICAL_GAP_ANALYSIS_PATTERNS | Process reference |
| X_POSTING_SCHEDULE_PATTERNS | Schedule reference |
| BLOG_DESIGN_PATTERNS | Design reference |

## Phase 1: Batch Gate 1 — Quality Audit All 63 Patterns

### 1a. Quality gate check (Go script)
- Script: `audit_pattern_quality.go`
- [ ] For each pattern file, check:
  - Has verifiable checks (not just guidelines/principles)
  - Has `## Research Sources` section (or mark as needing one)
  - Checks are phrased as testable statements
- [ ] Output: per-pattern pass/fail with deficiency list
- [ ] Args: `-dir {docs_dir}`

### 1b. Fix deficiencies
- [ ] For patterns missing Research Sources: add section with documented provenance (web search to verify origins, cite actual sources)
- [ ] For patterns with vague checks: rewrite as verifiable statements
- [ ] Each fix converged (two clean passes on the pattern doc)

### 1c. Classify into tiers
- [ ] Run automation analysis per pattern (already done in PATTERNS_AUTOMATION_ANALYSIS.md)
- [ ] Assign each pattern to Tier 1, 2, or 3
- [ ] Create classification entry in onelist.db for each pattern

### 1d. Tests
- [ ] test_quality_audit_passes_good_pattern
- [ ] test_quality_audit_fails_missing_sources
- [ ] test_quality_audit_fails_vague_checks

## Phase 2: Batch Gate 2 — Automation Analysis Extraction

### 2a. Extract per-pattern analysis files
- [ ] For each Tier 1 + Tier 2 pattern: create `PATTERN_NAME_AUTOMATION_ANALYSIS.md`
- [ ] Extract from PATTERNS_AUTOMATION_ANALYSIS.md the relevant section
- [ ] Classification table: check name | MECHANICAL/JUDGMENT/HYBRID | script name

### 2b. Store in pattern directory structure
- [ ] `mkdir -p patterns/{pattern_name}/`
- [ ] Move pattern markdown: `docs/PATTERN_NAME.md` → `patterns/{pattern_name}/PATTERN_NAME.md`
- [ ] Place analysis: `patterns/{pattern_name}/PATTERN_NAME_AUTOMATION_ANALYSIS.md`
- [ ] Tier 3 patterns stay in `docs/` (they're reference docs, not auditable packages)

### 2c. Tests
- [ ] test_all_tier1_patterns_have_analysis
- [ ] test_all_tier2_patterns_have_analysis
- [ ] test_pattern_directories_created

## Phase 3: Prioritized Gates 3-4 — Build Go Scripts (Tier 1 Only)

### 3a. Phase 1 scripts (top 12, from BP027)
- [ ] Build scripts in priority order from BP027 Phase 1:
  - `audit_frontmatter.go` — validate markdown frontmatter
  - `audit_seo_meta.go` — HTML meta tags, titles, descriptions
  - `audit_gtv_numbers.go` — stale number detection across surfaces
  - `audit_blog_structure.go` — blog post required sections
  - `audit_contrast.go` — WCAG AA color contrast ratios
  - `audit_form_structure.go` — form element presence/absence
  - `audit_mobile.go` — responsive breakpoints, touch targets
  - `audit_nav.go` — navigation structure, accessibility
  - `audit_deployment.go` — deployment config validation
  - `audit_mcp_tools.go` — MCP tool documentation sync
  - `audit_regression.go` — regression marker detection
  - `audit_taxonomy.go` — content taxonomy validation
- [ ] Each script: sandbox → test → compile → promote to `patterns/{name}/scripts/`
- [ ] Each script follows the standard I/O contract (JSON output, exit codes, CLI args)

### 3b. Phase 2 scripts (next 8)
- [ ] Build remaining Tier 1 scripts based on automation analysis
- [ ] Same sandbox → test → compile → promote flow

### 3c. Tests per script
- [ ] Each script has unit tests in `main_test.go`
- [ ] Each script tested against good + bad fixtures
- [ ] All scripts pass I/O contract validation

## Phase 4: Batch Gate 5 — Write LLM Usage Guides

### 4a. Tier 1 guides (scripts + judgment)
- [ ] For each Tier 1 pattern: write `PATTERN_NAME_LLM_GUIDE.md`
- [ ] Guide includes:
  - Security Note (script output is untrusted)
  - Mechanical Checks section (handled by scripts — list each)
  - Hybrid Checks section (script pre-screens, LLM evaluates remainder)
  - Judgment Checks section (LLM responsibility — list each)
  - Merged findings format
  - Pass/fail criteria
- [ ] Each guide converged (two clean passes, audited by different model tier)

### 4b. Tier 2 guides (judgment only)
- [ ] For each Tier 2 pattern: write `PATTERN_NAME_LLM_GUIDE.md`
- [ ] Guide includes:
  - All checks listed as Judgment Checks
  - Findings format
  - Pass/fail criteria
  - Note: "No mechanical scripts for this pattern"
- [ ] Each guide converged

### 4c. Tests
- [ ] test_all_tier1_guides_reference_scripts
- [ ] test_all_tier2_guides_have_judgment_section
- [ ] test_guide_findings_format_valid

## Phase 5: Batch Gate 6 — Fixtures + Integration Tests

### 5a. Create fixtures per pattern
- [ ] For each Tier 1 pattern: create `fixtures/good/` and `fixtures/bad/` directories
- [ ] Good fixture: minimal project that passes all checks
- [ ] Bad fixtures: one per mechanical check (each triggers exactly one failure)
- [ ] Minimum: 1 good, N bad where N >= mechanical check count

### 5b. Integration test each pattern
- [ ] Run scripts against good fixtures → all pass (exit 0)
- [ ] Run scripts against bad fixtures → correct failures detected (exit 1)
- [ ] Zero false positives on good, zero false negatives on bad (for mechanical checks)
- [ ] Judgment checks: validate format only

### 5c. Tests
- [ ] test_all_tier1_scripts_pass_good_fixtures
- [ ] test_all_tier1_scripts_fail_bad_fixtures
- [ ] test_fixture_coverage_meets_minimum

## Phase 6: Batch Gate 7 — Package and Move to Library

### 6a. Create CONVERGENCE_LOG.md per pattern
- [ ] Record which gates passed, when, with what round count
- [ ] For Tier 2 patterns: Gates 3-4 marked "SKIPPED (judgment-only, <30% mechanical)"

### 6b. Final validation
- [ ] Run `audit_pattern_package` tool on each package
- [ ] All must pass

### 6c. Update router
- [ ] Router auto-discovers new packages in `patterns/`
- [ ] Verify: `detect_pattern_scripts()` finds all Tier 1 scripts
- [ ] Verify: pattern dimensions injected into convergence tasks

### 6d. Update docs/ symlinks (optional)
- [ ] For backward compatibility: symlink `docs/FORM_PATTERNS.md` → `patterns/form_patterns/FORM_PATTERNS.md`
- [ ] Prevents broken references in existing build plans

### 6e. Tests
- [ ] test_all_packages_pass_audit
- [ ] test_router_discovers_all_packages
- [ ] test_symlinks_resolve_correctly

## Execution Strategy

This is a LARGE plan. Recommended execution order:

1. **Week 1-2**: Phase 1 (quality audit all 63, classify into tiers) + Phase 2 (extract analyses, create directories)
2. **Week 3-4**: Phase 3a (top 12 scripts from BP027) + Phase 4a (Tier 1 LLM guides for the 12 scripted patterns)
3. **Week 5-6**: Phase 3b (remaining Tier 1 scripts) + Phase 4b (Tier 2 LLM guides)
4. **Week 7**: Phase 5 (fixtures + integration tests)
5. **Week 8**: Phase 6 (package, validate, move to library)

**Parallelizable**: Script building (Phase 3) and guide writing (Phase 4) can run in parallel since they're independent per pattern.

## Verification

```bash
cd /Users/user/personal/cruxdev/rust && cargo test
cd /Users/user/personal/cruxdev/rust && cargo clippy -- -D warnings
# Verify all Tier 1 packages pass audit
for dir in patterns/*/; do cruxdev audit_pattern_package "$dir"; done
```

## Definition of Done

1. All 63 patterns classified into tiers and recorded in onelist.db
2. All patterns pass Gate 1 quality check (verifiable checks, Research Sources)
3. All Tier 1 patterns (~20) have compiled Go audit scripts in `patterns/{name}/scripts/`
4. All Tier 1 + Tier 2 patterns (~42) have LLM usage guides
5. All Tier 1 patterns have fixtures (good + bad) passing integration tests
6. All upgraded patterns pass `audit_pattern_package` validation
7. Router auto-discovers all pattern packages
8. Pattern dimensions injected into convergence tasks
9. Backward-compatible symlinks from `docs/` to `patterns/`
10. All tests pass, zero clippy warnings
11. Two consecutive clean convergence passes on the full upgrade
