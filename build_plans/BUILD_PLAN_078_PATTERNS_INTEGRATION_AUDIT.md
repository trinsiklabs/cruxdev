# BUILD_PLAN_078: Patterns Integration Audit — Wire All Dimensions into Convergence

**Status:** CONVERGED
**Priority:** Critical (self-adoption gap)
**Depends on:** Self-adoption process (BP043)

## Context

Self-adoption audit revealed a systemic gap: 6 dimension sets are defined in `router.rs` but never referenced in convergence routing. Pattern docs get written, dimensions get defined, but nothing triggers them during convergence. The self-adoption loop should catch this automatically.

### Current Gaps

| Dimension Set | Defined | Used in Router | Detection Function | Gap |
|--------------|---------|----------------|-------------------|-----|
| PLAN_DIMENSIONS | Yes | Yes (Planning) | N/A | OK |
| CODE_DIMENSIONS | Yes | Yes (CodeAuditing) | N/A | OK |
| DOC_DIMENSIONS | Yes | Yes (DocAuditing) | N/A | OK |
| FORM_DIMENSIONS | Yes | Yes (WebsiteConvergence) | project_has_forms() | OK |
| UI_COMPONENT_DIMENSIONS | Yes | Yes (CodeAuditing) | detect_ui_components() | OK |
| METRICS_DIMENSIONS | Yes | **NO** | None | **GAP** |
| DASHBOARD_DIMENSIONS | Yes | **NO** | None | **GAP** |
| MCP_SERVER_DIMENSIONS | Yes | **NO** | None | **GAP** |
| SKILL_DIMENSIONS | Yes | **NO** | None | **GAP** |
| CONTENT_DIMENSIONS | Yes | **NO** | None | **GAP** |
| BUSINESS_DIMENSIONS | Yes | **NO** | None | **GAP** |
| MEDIA_DIMENSIONS | Yes | **NO** | None | **GAP** |

### Pattern Docs Without Dimensions

| Pattern Doc | Has Dimensions? | Gap |
|------------|----------------|-----|
| COLOR_CONTRAST_PATTERNS.md | No | Needs COLOR_CONTRAST_DIMENSIONS |
| WEBSITE_LOGO_PATTERNS.md | No | Needs LOGO_DIMENSIONS |

## Phase 1: Wire Existing Dimensions

- [ ] 1.1 METRICS_DIMENSIONS → detect projects with metrics/dashboards, add to CodeAuditing
- [ ] 1.2 DASHBOARD_DIMENSIONS → detect dashboard components, add to WebsiteConvergence
- [ ] 1.3 MCP_SERVER_DIMENSIONS → detect MCP server projects, add to CodeAuditing
- [ ] 1.4 SKILL_DIMENSIONS → detect projects with .claude/skills/, add to DocAuditing
- [ ] 1.5 CONTENT_DIMENSIONS → detect blog/newsletter content, add to DocAuditing
- [ ] 1.6 BUSINESS_DIMENSIONS → detect business projects, add to appropriate phase
- [ ] 1.7 MEDIA_DIMENSIONS → detect media projects, add to appropriate phase

## Phase 2: Add Missing Dimensions

- [ ] 2.1 Add COLOR_CONTRAST_DIMENSIONS (from COLOR_CONTRAST_PATTERNS.md audit criteria)
- [ ] 2.2 Add LOGO_DIMENSIONS (from WEBSITE_LOGO_PATTERNS.md audit criteria)
- [ ] 2.3 Wire both into WebsiteConvergence phase

## Phase 3: Automated Integration Check

- [ ] 3.1 Add architecture test: every `*_DIMENSIONS` constant must be referenced in get_next_task()
- [ ] 3.2 Add architecture test: every `*_PATTERNS.md` doc must have a corresponding `*_DIMENSIONS` constant
- [ ] 3.3 These tests run in the self-adoption loop, catching future gaps automatically

## Phase 4: Self-Adoption Process Update

- [ ] 4.1 Add "patterns integration audit" as a step in the self-adoption cycle
- [ ] 4.2 After any build plan that creates a patterns doc, verify dimensions exist AND are wired
- [ ] 4.3 Update DEVELOPMENT_PATTERNS_CRUXDEV.md with this requirement

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
# Architecture tests should verify all dimensions are wired
```
