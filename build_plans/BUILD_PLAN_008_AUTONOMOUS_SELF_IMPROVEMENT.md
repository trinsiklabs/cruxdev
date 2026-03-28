# BUILD_PLAN_008: Autonomous Self-Improvement System

**Created:** 2026-03-22
**Status:** CONVERGED
**Goal:** Implement all 24 axes of autonomous self-improvement across the Crux ecosystem. The system continuously improves its own code, knowledge, docs, security, performance, website, community presence, and methodology — without human prompting.

**Architecture:** The triangular flywheel — each product improves the others:
```
   Corrections & Interactions
CruxCLI ─────────────────────> Crux
   ^                            │
   │ Better convergence         │ Better knowledge, modes
   │                            ▼
   └──────────────── CruxDev
        Quality data → model routing
```

**Rule:** TDD. Tests before code. 100% coverage.
**Rule:** Every self-improvement action must be reversible (auto-rollback on failure).
**Rule:** Safety gates cannot be weakened autonomously. They are protected.
**Rule:** All axes run autonomously after implementation. No human "do it again."

---

## Phase A: Knowledge Self-Improvement (Crux)

**Purpose:** The intelligence layer learns faster and forgets less. Axes 2, 3, 6, 22.

### A1. Knowledge Clustering + Pattern Elevation (Axis 2)

When 3+ corrections share a theme (same category, same mode, same code pattern), auto-synthesize them into a knowledge entry.

- [ ] A1.1 `scripts/lib/crux_knowledge_clustering.py` — cluster corrections by category+mode+pattern
- [ ] A1.2 `synthesize_pattern()` — LLM generates knowledge entry from correction cluster
- [ ] A1.3 Integration with background processor (trigger on correction count threshold)
- [ ] A1.4 Auto-promote: project scope → user scope when pattern appears in 2+ projects
- [ ] A1.5 Tests for clustering (various cluster sizes, mixed categories)
- [ ] A1.6 Tests for synthesis (mock LLM, validate output structure)
- [ ] A1.7 Tests for promotion logic
- [ ] A1.8 Coverage ≥ 100%

### A2. Knowledge Staleness Detection (Axis 3)

Periodically validate knowledge entries against current code. Flag stale entries.

- [ ] A2.1 `scripts/lib/crux_knowledge_staleness.py` — cross-reference entries against codebase
- [ ] A2.2 Check: do referenced files/functions still exist?
- [ ] A2.3 Check: has the pattern been superseded by a newer entry?
- [ ] A2.4 Soft-retire stale entries (flag, don't delete)
- [ ] A2.5 Daily staleness scan via background processor
- [ ] A2.6 Tests for staleness detection (mock filesystem)
- [ ] A2.7 Tests for soft retirement
- [ ] A2.8 Coverage ≥ 100%

### A3. Mode Prompt Improvement from Corrections (Axis 6)

When corrections cluster around a specific mode, propose prompt changes that prevent recurrence.

- [ ] A3.1 `scripts/lib/crux_prompt_improvement.py` — analyze corrections per mode
- [ ] A3.2 Generate prompt diff proposals from correction clusters
- [ ] A3.3 Validate proposals against historical scenarios (`model_auto_evaluate.py`)
- [ ] A3.4 Apply proposals that improve evaluation scores (human-approved for now — protected files)
- [ ] A3.5 Track prompt changes over time (prompt changelog)
- [ ] A3.6 Tests for correction-to-prompt mapping
- [ ] A3.7 Tests for proposal generation
- [ ] A3.8 Tests for evaluation integration
- [ ] A3.9 Coverage ≥ 100%

### A4. Prompt Bloat Detection (Axis 22)

Detect redundant, contradictory, or irrelevant rules in mode prompts. Propose trimming.

- [ ] A4.1 `scripts/lib/crux_prompt_bloat.py` — token counting, semantic similarity, relevance checking
- [ ] A4.2 Redundancy detection (two rules that say the same thing)
- [ ] A4.3 Contradiction detection (rules that conflict)
- [ ] A4.4 Relevance check (rule references code/patterns that no longer exist)
- [ ] A4.5 Generate trimming proposals with rationale
- [ ] A4.6 Tests for each detection type
- [ ] A4.7 Coverage ≥ 100%

**Phase A total: 30 checkboxes**

---

## Phase B: Code Quality Self-Improvement (CruxDev)

**Purpose:** The codebase autonomously maintains and improves itself. Axes 1, 4, 12, 13.

### B1. Technical Debt Auto-Remediation (Axis 1)

Periodically scan for code smells, duplication, complexity hotspots. Auto-generate and auto-converge remediation plans.

- [ ] B1.1 `src/improvement/tech_debt.py` — AST-based complexity analysis, duplication detection
- [ ] B1.2 Complexity scoring per function (cyclomatic complexity)
- [ ] B1.3 Duplication detection (similar AST subtrees across files)
- [ ] B1.4 Anti-pattern detection (bare except, hardcoded paths, unused imports)
- [ ] B1.5 Auto-generate remediation build plan when debt exceeds threshold
- [ ] B1.6 Auto-converge remediation plan (full test suite must pass)
- [ ] B1.7 Tests for complexity scoring
- [ ] B1.8 Tests for duplication detection
- [ ] B1.9 Tests for anti-pattern detection
- [ ] B1.10 Tests for plan generation
- [ ] B1.11 Coverage ≥ 100%

### B2. Doc-Code Drift Detection + Auto-Repair (Axis 4)

After every code change, audit docs for accuracy. Auto-fix drift.

- [ ] B2.1 `src/improvement/doc_drift.py` — detect mismatches between code and docs
- [ ] B2.2 Function signature changes → doc update
- [ ] B2.3 Module renames → doc path update
- [ ] B2.4 Feature additions/removals → doc content update
- [ ] B2.5 Integration with convergence engine (trigger after code convergence)
- [ ] B2.6 Tests for each drift type
- [ ] B2.7 Tests for auto-repair output
- [ ] B2.8 Coverage ≥ 100%

### B3. Convergence Parameter Tuning (Axis 12)

Use historical convergence data to tune max rounds, audit dimension priorities.

- [ ] B3.1 `src/improvement/convergence_tuning.py` — analyze convergence state history
- [ ] B3.2 Calculate optimal max_rounds per phase from historical data
- [ ] B3.3 Rank audit dimensions by issue detection frequency
- [ ] B3.4 Prioritize high-yield dimensions in audit ordering
- [ ] B3.5 Safety floor: never reduce convergence_threshold below 2, max_rounds below 3
- [ ] B3.6 Tests for parameter calculation
- [ ] B3.7 Tests for safety floor enforcement
- [ ] B3.8 Coverage ≥ 100%

### B4. Test Quality Self-Improvement (Axis 13)

Beyond coverage: detect weak assertions, flaky tests, tests that never fail.

- [ ] B4.1 `src/improvement/test_quality.py` — assertion strength analysis
- [ ] B4.2 Flakiness detection (run N times, track inconsistency)
- [ ] B4.3 Weak assertion detection (assert True, assert is not None without checking value)
- [ ] B4.4 Mutation testing integration (mutmut for Python)
- [ ] B4.5 Auto-generate improvement suggestions for weak tests
- [ ] B4.6 Tests for each detection type
- [ ] B4.7 Coverage ≥ 100%

**Phase B total: 30 checkboxes**

---

## Phase C: Security + Dependencies (Crux + All)

**Purpose:** The system continuously hardens itself. Axes 7, 8.

### C1. Automated Dependency Updates (Axis 7)

Check for outdated deps, update, test, commit or rollback.

- [ ] C1.1 `src/improvement/dep_updates.py` — scan for outdated dependencies
- [ ] C1.2 Separate minor/patch (auto-merge) from major (convergence-backed)
- [ ] C1.3 Update → run full test suite → commit if pass → rollback if fail
- [ ] C1.4 Rate limit: max 1 major update per day
- [ ] C1.5 Support: pip (Python), npm (TypeScript), mix (Elixir)
- [ ] C1.6 Tests for version comparison logic
- [ ] C1.7 Tests for update+rollback flow
- [ ] C1.8 Coverage ≥ 100%

### C2. Security Self-Improvement Pipeline (Axis 8)

CVE scanning + security anti-pattern detection + learning from safety pipeline.

- [ ] C2.1 `scripts/lib/crux_cve_scanner.py` — check deps against CVE databases (OSV API)
- [ ] C2.2 Auto-patch: if CVE has a fix version, update and test
- [ ] C2.3 Security anti-pattern detection (extend existing `crux_security_audit.py`)
- [ ] C2.4 Learning: track what the safety pipeline catches, feed into audit priorities
- [ ] C2.5 Critical CVE → immediate human notification (don't just auto-fix silently)
- [ ] C2.6 Tests for CVE scanning (mock API)
- [ ] C2.7 Tests for auto-patch flow
- [ ] C2.8 Tests for learning feedback loop
- [ ] C2.9 Coverage ≥ 100%

**Phase C total: 17 checkboxes**

---

## Phase D: Cross-Project + Upstream Intelligence (Crux + CruxCLI)

**Purpose:** Learning transfers between projects and from the outside world. Axes 5, 15, 26.

### D1. Cross-Project Pattern Transfer (Axis 5)

Patterns discovered in one project auto-propagate to compatible projects.

- [ ] D1.1 `scripts/lib/crux_pattern_transfer.py` — find transferable patterns
- [ ] D1.2 Language/stack compatibility filter (Python patterns → Python projects only)
- [ ] D1.3 Transfer as "suggested pattern" in target project's knowledge
- [ ] D1.4 Validate: run target project's tests after applying pattern
- [ ] D1.5 Tests for compatibility filtering
- [ ] D1.6 Tests for transfer + validation flow
- [ ] D1.7 Coverage ≥ 100%

### D2. Upstream Intelligence Digestion (Axis 15)

Scan OpenCode's git history for useful changes, evaluate for CruxCLI adaptation.

- [ ] D2.1 `src/improvement/upstream_digest.py` — monitor upstream repo for changes
- [ ] D2.2 Filter: significant changes only (not typo fixes)
- [ ] D2.3 Evaluate: compatibility with CruxCLI architecture
- [ ] D2.4 Generate adaptation proposals (analysis only, auto-converge when clear)
- [ ] D2.5 Inspiration registry: track what was adopted and what was rejected
- [ ] D2.6 Tests for change detection and filtering
- [ ] D2.7 Tests for compatibility evaluation
- [ ] D2.8 Coverage ≥ 100%

### D3. Security Fix Cross-Application (Axis 26)

Security fix in one project → check and apply to related projects.

- [ ] D3.1 Integration with C2 (CVE scanner triggers cross-project check)
- [ ] D3.2 AST-based pattern matching for similar vulnerable code
- [ ] D3.3 Auto-apply fix + test in each target project
- [ ] D3.4 Human notification for all cross-project security fixes
- [ ] D3.5 Tests for cross-project vulnerability matching
- [ ] D3.6 Coverage ≥ 100%

**Phase D total: 19 checkboxes**

---

## Phase E: Website + Community Self-Improvement (Websites + All)

**Purpose:** Public presence stays current and grows autonomously. Axes 10, 11, 14, 17, 18, 23.

### E1. Website Metrics Auto-Update (Axis 10)

Star counts, test counts, feature counts auto-refresh on all sites.

- [ ] E1.1 `src/improvement/website_metrics.py` — fetch metrics from GitHub API, test suites
- [ ] E1.2 Inject into Astro site templates (build-time data fetching)
- [ ] E1.3 Sanity check: new value within reasonable range of old value
- [ ] E1.4 Auto-build + auto-deploy after metric update
- [ ] E1.5 Tests for metric fetching and injection
- [ ] E1.6 Coverage ≥ 100%

### E2. Release Notes Generation (Axis 11)

Auto-generate categorized release notes from git history on version tag.

- [ ] E2.1 `src/improvement/release_notes.py` — parse git log, categorize by type
- [ ] E2.2 LLM summarization of changes into human-readable notes
- [ ] E2.3 Highlight breaking changes prominently
- [ ] E2.4 Auto-publish to changelog page on website
- [ ] E2.5 Tests for categorization logic
- [ ] E2.6 Coverage ≥ 100%

### E3. Blog Post Generation (Axis 14)

When a significant feature ships, auto-generate a blog post draft.

- [ ] E3.1 `src/improvement/blog_generator.py` — build plan → blog post
- [ ] E3.2 Extract goal, approach, outcome from convergence data
- [ ] E3.3 SEO optimization (title, meta description, keywords)
- [ ] E3.4 Generate as draft in website content directory
- [ ] E3.5 Tests for content generation
- [ ] E3.6 Coverage ≥ 100%

### E4. Issue Triage (Axis 17)

Auto-label, assess severity, post diagnostics on new GitHub issues.

- [ ] E4.1 GitHub webhook or Actions workflow for issue events
- [ ] E4.2 Auto-label by category (bug, feature, docs, question)
- [ ] E4.3 Severity assessment from issue content
- [ ] E4.4 Diagnostic comment with relevant code references
- [ ] E4.5 Never auto-close issues (label and comment only)
- [ ] E4.6 Tests for labeling and severity logic
- [ ] E4.7 Coverage ≥ 100%

### E5. Comparison Page Auto-Refresh (Axis 18)

When competitor data changes, auto-update /vs/ pages.

- [ ] E5.1 Integration with COMPETITORS_PATTERN.md monitoring loop
- [ ] E5.2 Detect data changes in COMPETITORS.md
- [ ] E5.3 Regenerate affected /vs/ pages
- [ ] E5.4 Verify all links on updated pages
- [ ] E5.5 Auto-deploy
- [ ] E5.6 Tests for change detection and page generation
- [ ] E5.7 Coverage ≥ 100%

### E6. Core Web Vitals Monitoring + Auto-Fix (Axis 23)

Continuously monitor CWV, detect regressions, apply known fixes.

- [ ] E6.1 `src/improvement/cwv_monitor.py` — PageSpeed Insights API integration
- [ ] E6.2 Regression detection (compare against baseline)
- [ ] E6.3 Auto-apply safe fixes (image compression, lazy loading, preloading)
- [ ] E6.4 Visual regression check before deploy
- [ ] E6.5 Tests for regression detection
- [ ] E6.6 Coverage ≥ 100%

**Phase E total: 36 checkboxes**

---

## Phase F: Performance + Cost Optimization (CruxDev + Crux)

**Purpose:** The system gets faster and cheaper over time. Axes 9, 16, 24.

### F1. Performance Regression Detection (Axis 9)

Benchmark key operations, track over time, detect regressions at commit level.

- [ ] F1.1 `src/improvement/benchmark.py` — define benchmarks for engine operations
- [ ] F1.2 Historical tracking (benchmark results per commit)
- [ ] F1.3 Statistical significance threshold (not single-run noise)
- [ ] F1.4 Auto-flag regressions above 10% threshold
- [ ] F1.5 Tests for benchmark framework
- [ ] F1.6 Coverage ≥ 100%

### F2. CI/CD Self-Optimization (Axis 16)

Track build times, identify slow steps, optimize.

- [ ] F2.1 `src/improvement/ci_optimizer.py` — analyze GitHub Actions timing data
- [ ] F2.2 Identify parallelization opportunities
- [ ] F2.3 Identify caching opportunities
- [ ] F2.4 Generate optimized workflow (never remove test steps)
- [ ] F2.5 Validate by running full pipeline before committing
- [ ] F2.6 Tests for optimization logic
- [ ] F2.7 Coverage ≥ 100%

### F3. Infrastructure Cost Optimization (Axis 24)

Track LLM API costs per task type, optimize model tier routing.

- [ ] F3.1 Cost tracking per API call in `crux_model_quality.py`
- [ ] F3.2 Cost-aware routing: prefer cheaper model when success rate is equal
- [ ] F3.3 Never sacrifice quality for cost (quality threshold still 70%)
- [ ] F3.4 Cost dashboard in `get_model_quality_stats()`
- [ ] F3.5 Tests for cost tracking and cost-aware routing
- [ ] F3.6 Coverage ≥ 100%

**Phase F total: 18 checkboxes**

---

## Phase G: Meta-Improvement (CruxDev)

**Purpose:** The system improves its own improvement process. Axes 19, 20, 21, 25, 27, 28.

### G1. Prompt A/B Testing (Axis 19)

Test mode prompt variations against historical scenarios. Deploy winners.

- [ ] G1.1 `src/improvement/prompt_ab.py` — A/B test framework for prompts
- [ ] G1.2 Scenario curation from convergence history
- [ ] G1.3 Statistical comparison of variant outcomes
- [ ] G1.4 Integration with `model_auto_evaluate.py`
- [ ] G1.5 Never deploy a variant worse on any critical dimension
- [ ] G1.6 Tests for A/B framework
- [ ] G1.7 Coverage ≥ 100%

### G2. Self-Modifying Methodology (Axis 20)

Propose methodology improvements from convergence data. Human-approved.

- [ ] G2.1 `src/improvement/meta_analysis.py` — analyze across convergence rounds
- [ ] G2.2 Correlate methodology elements with convergence speed
- [ ] G2.3 Generate methodology diffs with evidence
- [ ] G2.4 DEVELOPMENT_PATTERNS_CRUXDEV.md is protected (human-approve only)
- [ ] G2.5 Tests for meta-analysis
- [ ] G2.6 Coverage ≥ 100%

### G3. Agent Evolution (Axis 21 — Darwin Godel Machine)

Maintain population of agent config variants, evaluate, evolve toward better performance.

- [ ] G3.1 `src/improvement/agent_evolution.py` — variant generation
- [ ] G3.2 Population management (bounded size, configurable)
- [ ] G3.3 Parallel evaluation on standardized benchmark
- [ ] G3.4 Selection: keep variants that outperform baseline
- [ ] G3.5 Protected invariants: safety pipeline, human escalation rules preserved across all variants
- [ ] G3.6 Kill switch: instant revert to known-good configuration
- [ ] G3.7 Tests for variant generation, evaluation, selection
- [ ] G3.8 Coverage ≥ 100%

### G4. Architecture Decision Transfer (Axis 25)

Architecture decisions from one project inform related projects.

- [ ] G4.1 Integration with D1 (cross-project pattern transfer)
- [ ] G4.2 Architecture decision records (ADRs) as transferable knowledge
- [ ] G4.3 Human approval required for architecture transfers
- [ ] G4.4 Tests for ADR detection and transfer proposals
- [ ] G4.5 Coverage ≥ 100%

### G5. Build Plan Template Evolution (Axis 27)

Track which plan formats produce better convergence outcomes. Evolve the template.

- [ ] G5.1 `src/improvement/plan_evolution.py` — correlate plan attributes with outcomes
- [ ] G5.2 Identify plan attributes that reduce convergence rounds
- [ ] G5.3 Update `create_plan_template()` with evidence-based improvements
- [ ] G5.4 Tests for correlation analysis
- [ ] G5.5 Coverage ≥ 100%

### G6. Audit Dimension Prioritization (Axis 28)

Track which audit dimensions find the most issues. Prioritize accordingly.

- [ ] G6.1 Frequency analysis of findings by dimension
- [ ] G6.2 Reorder audit dimensions by yield (high-yield first)
- [ ] G6.3 All dimensions still checked (reorder, never skip)
- [ ] G6.4 Tests for frequency analysis and reordering
- [ ] G6.5 Coverage ≥ 100%

### G7. Living Changelog (Axis 29)

Auto-generate CHANGELOG.md from git history, categorized, formatted.

- [ ] G7.1 `src/improvement/changelog.py` — parse conventional commits
- [ ] G7.2 Categorize: feature, fix, breaking, docs, refactor
- [ ] G7.3 Auto-update on every version tag
- [ ] G7.4 Tests for categorization
- [ ] G7.5 Coverage ≥ 100%

**Phase G total: 35 checkboxes**

---

## Progress Tracker

| Phase | Focus | Checkboxes | Products |
|-------|-------|------------|----------|
| A | Knowledge self-improvement | 30 | Crux |
| B | Code quality self-improvement | 30 | CruxDev |
| C | Security + dependencies | 17 | Crux + All |
| D | Cross-project + upstream | 19 | Crux + CruxCLI |
| E | Website + community | 36 | Websites + All |
| F | Performance + cost | 18 | CruxDev + Crux |
| G | Meta-improvement | 35 | CruxDev |

**Total: 185 checkboxes**

---

## Execution Strategy

Phases A-B are sequential (foundations first).
Phases C-F can run in parallel after B.
Phase G requires data from A-F (meta-improvement needs history to analyze).

```
A (knowledge) → B (code quality) → ┬─ C (security)
                                    ├─ D (cross-project)
                                    ├─ E (website)
                                    └─ F (performance)
                                         └─── G (meta-improvement)
```

Each phase follows DEVELOPMENT_PATTERNS_CRUXDEV.md:
- TDD, 100% coverage
- Convergence: code + docs + website + deployment
- All self-improvement axes run autonomously after implementation

---

## Safety Architecture

**Protected invariants (no axis can weaken these):**
- Convergence threshold ≥ 2 consecutive clean passes
- Max rounds ≥ 3 for code convergence
- 3-failure auto-rollback
- Full test suite must pass before any auto-commit
- Human escalation rules cannot be modified autonomously
- CLAUDE.md and mode prompts are protected files (human-approve changes)
- Security gates cannot be weakened

**Kill switch:** Every axis has an independent enable/disable flag. Any axis can be disabled instantly without affecting others.

**Observability:** Every autonomous action is logged with rationale, before/after state, and rollback path.

---

## Test Commands

```bash
# CruxDev
python3 -m pytest tests/ -v --tb=short --cov=src --cov-report=term-missing --cov-fail-under=100

# Crux
cd /Users/user/personal/crux && python3 -m pytest tests/ -v --tb=short

# CruxCLI
cd /Users/user/personal/cruxcli/packages/opencode && bun test
```

## Convergence Criteria

- All 185 checkboxes complete
- All three test suites pass
- Coverage ≥ 100% on all new CruxDev code
- All 24 axes running autonomously
- Safety architecture verified (protected invariants tested)
- Documentation converged
- All three websites updated and deployed
- Two consecutive clean passes on the full system
