# BUILD_PLAN_114: Pattern-First Convergence — Engine-Level Pattern Assessment, Gap Filling, and Orchestration

**Status:** NOT STARTED
**Priority:** Critical (patterns are only useful if the engine enforces them)
**Triggered by:** BP029 architectural split — CruxDev engine logic must work regardless of host (Claude Code, Cline, CruxBot)

## The Problem

CruxDev has 80+ pattern files and a convergence engine with 20+ dimension sets. But the engine doesn't know which patterns a project needs, doesn't check for gaps, and doesn't orchestrate pattern usage during convergence. Every convergence just applies all detected dimensions — there's no "this project needs FORM_PATTERNS but doesn't have it, so create it first" logic.

This is a CruxDev engine problem, not a CruxBot problem. When CruxDev runs as an MCP server in Claude Code, it can't call compiled Go scripts — Claude Code controls execution. But CruxDev CAN:
- Assess which patterns are needed for this plan
- Check if those patterns exist as converged packages
- Gate convergence until pattern gaps are filled
- Plan which patterns apply at which convergence phase
- Include the right pattern dimensions in each audit task
- Tell the LLM which pattern checks are mechanical vs judgment

**What CruxDev CANNOT do (CruxBot-only):**
- Execute compiled Go audit scripts
- Run GTV preflight with binaries
- Perform file/git operations via scripts
- Enforce script-based rollback

Those belong in BP029 (CruxBot). This plan is about the engine-level logic that works everywhere.

## Document Alignment

- docs/PATTERN_CREATION_PATTERNS.md — 7-gate pattern lifecycle
- docs/DEVELOPMENT_PATTERNS_CRUXDEV.md — master methodology
- docs/PATTERNS_AUTOMATION_ANALYSIS.md — which pattern checks are mechanical
- rust/src/engine/convergence.rs — phase order
- rust/src/engine/router.rs — dimension routing

## Phase 1: New Convergence Phases

The convergence phase order currently:
```
Planning → PlanAuditing → DocAlignment → Viability → Executing →
CodeAuditing → DocAuditing → WebsiteConvergence → E2eTesting →
PatternsUpdate → Converged
```

Insert two new phases BEFORE PlanAuditing:
```
Planning → PatternAssessment → PatternOrchestration → PlanAuditing → ...
```

### 1a. Add PatternAssessment phase
- File: `rust/src/engine/convergence.rs`
- [ ] Add `PatternAssessment` variant to `ConvergencePhase` enum
- [ ] Insert after `Planning` in `PHASE_ORDER`
- [ ] Two consecutive clean passes required (same as all phases)

### 1b. Add PatternOrchestration phase
- File: `rust/src/engine/convergence.rs`
- [ ] Add `PatternOrchestration` variant to `ConvergencePhase` enum
- [ ] Insert after `PatternAssessment` in `PHASE_ORDER`
- [ ] Two consecutive clean passes required

### 1c. Update state
- File: `rust/src/engine/state.rs`
- [ ] Add `pattern_assessment: Option<PatternAssessment>` to convergence state
- [ ] Add `pattern_orchestration: Option<PatternOrchestration>` to convergence state
- [ ] Serializable to JSON for persistence

### 1d. Tests
- [ ] test_phase_order_includes_pattern_phases
- [ ] test_pattern_assessment_before_plan_auditing
- [ ] test_advance_from_planning_goes_to_pattern_assessment

## Phase 2: Pattern Assessment Engine

The engine analyzes the build plan to determine what patterns are needed, then checks coverage.

### 2a. Domain detection from plan content
- File: `rust/src/engine/pattern_assessment.rs`
- [ ] `detect_domains(plan_content: &str) -> Vec<Domain>` — scan the plan for domain keywords
- [ ] Domain enum: `Web`, `Api`, `Database`, `Security`, `Content`, `Deployment`, `Mobile`, `Seo`, `Forms`, `Dashboard`, `Testing`, `I18n`, `Accessibility`, `Performance`, `LlmIntegration`
- [ ] Keyword mapping (from plan text):
  - "form", "input", "validation" → `Forms`
  - "deploy", "production", "fly.io", "vercel" → `Deployment`
  - "SEO", "meta tags", "sitemap" → `Seo`
  - "API", "endpoint", "REST", "GraphQL" → `Api`
  - "LLM", "prompt", "model", "embedding" → `LlmIntegration`
  - etc.
- [ ] Also detect from file paths mentioned in the plan (e.g., `src/pages/*.astro` → Web)

### 2b. Pattern mapping
- File: `rust/src/engine/pattern_assessment.rs`
- [ ] `map_domains_to_patterns(domains: &[Domain]) -> Vec<PatternRef>` — which pattern files cover each domain
- [ ] Pattern registry (hardcoded initially, later from pattern library scan):
  - `Forms` → `FORM_PATTERNS.md`
  - `Deployment` → `POST_DEPLOYMENT_PATTERNS.md`
  - `Seo` → `SEO_AND_GEO_REFERENCE.md`, `GEO_PATTERNS.md`
  - `Api` → `MCP_SERVER_PATTERNS.md`
  - `Content` → `BLOG_PATTERNS.md`, `BLOG_POST_PATTERNS.md`, `CONTENT_DIMENSIONS`
  - `LlmIntegration` → `LLM_CALL_DIMENSIONS`
  - `Mobile` → `MOBILE_WEB_PATTERNS.md`
  - `Dashboard` → `DASHBOARD_PATTERNS.md`
  - `Testing` → `E2E_TEST_PATTERNS.md`, `UAT_TEST_PATTERNS.md`
  - etc.

### 2c. Coverage check
- [ ] `check_pattern_coverage(needed: &[PatternRef], project_dir: &str) -> PatternCoverage`
  - For each needed pattern: does the file exist in `docs/` or `patterns/`?
  - If it exists in `patterns/` as a converged package: full coverage (has scripts + LLM guide)
  - If it exists in `docs/` as standalone markdown: partial coverage (no scripts, no LLM guide)
  - If it doesn't exist: gap
- [ ] `PatternCoverage` struct: `{ needed: Vec<PatternRef>, full: Vec<PatternRef>, partial: Vec<PatternRef>, gaps: Vec<PatternGap> }`
- [ ] `PatternGap` struct: `{ domain: Domain, suggested_name: String, reason: String }`

### 2d. Assessment task in router
- File: `rust/src/engine/router.rs`
- [ ] When phase is `PatternAssessment`:
  - Task type: "audit"
  - Description: "Assess pattern coverage for this plan. Report gaps."
  - Dimensions: `["pattern_coverage", "domain_detection", "gap_identification"]`
  - Files: the plan file
  - Metadata: include the detected domains and current coverage
- [ ] The LLM reviews the assessment and either confirms (clean pass) or identifies additional domains/gaps
- [ ] Engine pre-computes the domain detection and coverage check; LLM validates and adds nuance

### 2e. Gap gating
- [ ] If `gaps` is non-empty AND the LLM confirms the gaps: convergence CANNOT proceed to PlanAuditing
- [ ] The assessment task returns findings like: `{"dimension": "pattern_coverage", "status": "fail", "description": "No pattern for Forms domain. Create FORM_PATTERNS first."}`
- [ ] The LLM (or human) must create the missing patterns before the assessment can pass
- [ ] When running in Claude Code: the LLM calls `create_pattern()` to fill gaps
- [ ] When running in CruxBot: CruxBot calls `create_pattern()` as a subtask
- [ ] After gap is filled: re-run assessment (next round)

### 2f. Tests
- [ ] test_detect_domains_from_plan_text
- [ ] test_detect_web_domain_from_file_paths
- [ ] test_map_domains_to_patterns
- [ ] test_coverage_full_when_pattern_package_exists
- [ ] test_coverage_partial_when_only_markdown
- [ ] test_coverage_gap_when_missing
- [ ] test_gaps_block_convergence
- [ ] test_filled_gaps_allow_progression

## Phase 3: Pattern Orchestration Engine

After assessment passes (all patterns available), plan HOW to use them during convergence.

### 3a. Orchestration planning
- File: `rust/src/engine/pattern_orchestration.rs`
- [ ] `plan_orchestration(assessment: &PatternCoverage, plan_phases: &[String]) -> PatternOrchestration`
  - For each convergence phase (CodeAuditing, DocAuditing, etc.): which patterns apply?
  - For each pattern: which dimension set does it contribute?
  - For converged packages: which scripts are available? (metadata only — engine doesn't execute scripts)
  - For standalone markdown: which sections should the LLM read?
- [ ] `PatternOrchestration` struct:
  ```rust
  pub struct PatternOrchestration {
      pub phases: Vec<PhasePatterns>,
  }
  pub struct PhasePatterns {
      pub phase: String,
      pub patterns: Vec<AppliedPattern>,
  }
  pub struct AppliedPattern {
      pub pattern_name: String,
      pub pattern_path: String,
      pub dimensions: Vec<String>,
      pub has_scripts: bool,
      pub script_paths: Vec<String>,     // metadata for CruxBot to execute
      pub llm_guide_path: Option<String>, // for LLM to read
      pub mechanical_checks: usize,       // from automation analysis
      pub judgment_checks: usize,
  }
  ```

### 3b. Orchestration task in router
- File: `rust/src/engine/router.rs`
- [ ] When phase is `PatternOrchestration`:
  - Task type: "audit"
  - Description: "Review pattern orchestration plan. Verify each pattern is mapped to the correct phase."
  - Dimensions: `["orchestration_completeness", "phase_mapping_accuracy"]`
  - Metadata: include the full orchestration plan
- [ ] LLM validates: are the right patterns at the right phases? Any mismatches?
- [ ] Two clean passes required

### 3c. Orchestration injection into later phases
- [ ] When `CodeAuditing` phase starts: router reads `pattern_orchestration` from state
  - Adds each pattern's dimensions to the audit task dimensions list
  - Includes pattern paths in task files list (so LLM reads the pattern)
  - Includes LLM guide paths in task metadata (so LLM knows what to focus on)
  - Includes script paths in task metadata (for CruxBot to execute; ignored in Claude Code)
- [ ] Same for `DocAuditing`, `WebsiteConvergence`, etc. — each gets its relevant patterns
- [ ] The LLM guide tells the LLM: "These mechanical checks are handled by scripts (if CruxBot). If you're in Claude Code, evaluate them manually."

### 3d. Host-aware guidance
- [ ] Task metadata includes `host_hint: "cruxbot" | "claude_code" | "cline" | "unknown"`
- [ ] LLM guide sections adapt:
  - CruxBot host: "Scripts will run these checks. Focus on judgment only."
  - Claude Code host: "No scripts available. Evaluate ALL checks, both mechanical and judgment."
  - Unknown host: same as Claude Code (safe default)
- [ ] The engine NEVER assumes scripts will run. It always includes all dimensions. The difference is guidance, not enforcement.

### 3e. Tests
- [ ] test_orchestration_maps_forms_to_code_auditing
- [ ] test_orchestration_maps_seo_to_website_convergence
- [ ] test_orchestration_includes_script_paths_for_packages
- [ ] test_code_auditing_includes_pattern_dimensions
- [ ] test_host_hint_affects_guidance_not_dimensions
- [ ] test_orchestration_two_clean_passes_required

## Phase 4: MCP Tools

### 4a. assess_pattern_coverage tool
- File: `rust/src/server.rs`
- [ ] Params: `plan_file: String, project_dir: Option<String>`
- [ ] Returns: detected domains, pattern coverage, gaps
- [ ] Callable from any host (Claude Code, Cline, CruxBot)

### 4b. get_pattern_orchestration tool
- File: `rust/src/server.rs`
- [ ] Params: `convergence_id: String`
- [ ] Returns: the orchestration plan for the active convergence
- [ ] Shows which patterns apply at which phase

### 4c. Update create_pattern tool (from BP113)
- [ ] After a pattern is created via `create_pattern()`, re-run pattern assessment
- [ ] Newly created pattern should close the gap it was created for
- [ ] If it doesn't (wrong domain mapping): finding

### 4d. Tests
- [ ] test_assess_pattern_coverage_tool
- [ ] test_get_pattern_orchestration_tool
- [ ] test_create_pattern_closes_gap

## Phase 5: Update Pattern Registry

### 5a. Pattern registry file
- File: `rust/src/engine/pattern_registry.rs`
- [ ] `PatternRegistry` struct: loaded from filesystem scan
- [ ] Scans `docs/` for standalone pattern markdown files
- [ ] Scans `patterns/` for converged packages
- [ ] Caches the registry (refresh on convergence start)
- [ ] `get_patterns_for_domain(domain) -> Vec<PatternEntry>`
- [ ] `PatternEntry`: name, path, package_status (standalone/converged), domains, dimension_count

### 5b. Dynamic dimension routing
- [ ] Replace hardcoded domain→pattern mapping (Phase 2b) with registry lookup
- [ ] When a new pattern is created and added to the library, it's automatically discovered
- [ ] Pattern's automation analysis determines which dimensions it contributes

### 5c. Tests
- [ ] test_registry_scans_docs_and_patterns
- [ ] test_registry_distinguishes_standalone_vs_package
- [ ] test_new_pattern_auto_discovered

## Verification

```bash
cd /Users/user/personal/cruxdev/rust && cargo test
cd /Users/user/personal/cruxdev/rust && cargo clippy -- -D warnings
```

## Definition of Done

1. Two new convergence phases: PatternAssessment + PatternOrchestration inserted before PlanAuditing
2. Engine detects domains from plan content and maps to patterns
3. Pattern coverage check gates convergence when gaps exist
4. Pattern orchestration plans which patterns apply at which phase (to convergence)
5. Orchestration dimensions injected into CodeAuditing/DocAuditing/WebsiteConvergence
6. Host-aware guidance: CruxBot gets "scripts handle mechanical", Claude Code gets "evaluate all"
7. MCP tools: assess_pattern_coverage, get_pattern_orchestration
8. Pattern registry scans filesystem, auto-discovers new patterns
9. All tests pass, zero clippy warnings
10. Works in Claude Code (no script dependency), works in CruxBot (with script execution)
