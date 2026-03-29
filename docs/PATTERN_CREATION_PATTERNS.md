# Pattern Creation Patterns

How CruxDev patterns are created, hardened, and packaged. A pattern is not just a markdown file — it's a complete package: documentation + supporting code + LLM usage guide. The code does what code can do. The LLM does what only an LLM can do. The guide tells the LLM exactly which code to call and when.

## The Problem This Solves

Patterns stored as markdown files create two failure modes:

1. **LLM does mechanical work** — An LLM reading FORM_PATTERNS.md to check if labels are positioned correctly is wasting tokens on something a regex can do. Every convergence cycle burns LLM calls on checks that should be compiled binaries.

2. **Patterns decay without enforcement** — A pattern that exists only as prose gets ignored, misinterpreted, or selectively applied. Code enforces. Docs suggest.

The solution: every pattern becomes a **package** — the markdown describes the why and the judgment calls, supporting code handles the mechanical checks, and an LLM usage guide bridges the two.

## The Pattern Creation Lifecycle

Every stage produces artifacts that go through convergence (two consecutive independent clean audit passes) before advancing. Nothing enters the pattern library unaudited.

```
1. RESEARCH TO CONVERGENCE
   Research the domain → multiple passes → converge on comprehensive pattern doc
   Audit: completeness, accuracy, actionability, testability of every check
   Gate: Two clean passes on the pattern document
   Output: PATTERN_NAME.md (converged)

2. AUTOMATION ANALYSIS TO CONVERGENCE
   Analyze every check in the pattern:
   - MECHANICAL: Can be done by code (regex, AST, file scan, config validation)
   - JUDGMENT: Requires LLM understanding (quality, coherence, voice, intent)
   Audit: classification accuracy, script spec completeness, I/O contract validity
   Gate: Two clean passes on the analysis
   Output: Classification of every check with script specs (converged)

3. BUILD PLAN TO CONVERGENCE
   Write build plan for the mechanical checks as Go scripts
   Group by complexity: file validators → structure auditors → cross-file consistency
   Audit: feasibility, dependency ordering, testability, completeness
   Gate: Two clean passes on the build plan
   Output: BUILD_PLAN_NNN_PATTERN_CODE.md (converged)

4. CODE TO CONVERGENCE
   Build the Go scripts through the convergence engine
   Each script: write → sandbox test → compile → validate against fixtures
   Audit: correctness, edge cases, I/O contract compliance, performance
   Gate: All tests pass + two clean code audit passes
   Output: Compiled binaries in script library (converged)

5. LLM USAGE GUIDE TO CONVERGENCE
   Document exactly how the LLM applies this pattern:
   - Which code to run first (and how to interpret results)
   - What the LLM should focus on (only the judgment calls)
   - What findings format to use
   - What constitutes a pass vs fail
   Audit: accuracy against code, completeness of judgment checks, clarity
   Gate: Two clean passes on the guide
   Output: PATTERN_NAME_LLM_GUIDE.md (converged)

6. INTEGRATION TEST TO CONVERGENCE
   Run the complete package against known-good and known-bad fixtures:
   - Scripts produce correct mechanical findings
   - LLM guide produces correct judgment findings
   - Combined output matches expected results
   - No false positives on good fixtures, no false negatives on bad fixtures
   Audit: end-to-end accuracy, coverage, performance
   Gate: Two clean passes on integration tests
   Output: Pattern package verified and ready for library

7. ADD TO PATTERN LIBRARY
   Only after all 6 convergence gates pass:
   Pattern package = {
     pattern.md        — the full pattern documentation (converged)
     llm_guide.md      — how the LLM applies it (converged)
     scripts/           — compiled Go binaries (converged)
     tests/             — test fixtures (converged)
     integration/       — end-to-end test results (converged)
   }
```

**The rule: nothing enters the pattern library unconverged.** A pattern that hasn't been through all gates is a draft, not a pattern. Drafts live in `build_plans/` or `docs/drafts/`. Patterns live in the library.

## Step 1: Research to Convergence

The pattern markdown is created through the standard research methodology (see RESEARCH_PATTERNS.md):

- **Pass 1**: Domain survey — what exists, what's the state of the art
- **Pass 2**: Gap analysis — what's missing, what's wrong with current approaches
- **Pass 3**: Synthesis — combine into actionable pattern with specific checks
- **Pass 4**: Validation — verify against real projects, fix gaps
- **Pass 5**: Convergence — two consecutive clean audit passes

The output is a pattern file with concrete, auditable checks. Not principles. Not guidelines. Checks.

**Quality gate**: Every check in the pattern must be phrased as a verifiable statement. "Forms should be accessible" fails. "Every form input has an associated label element or aria-label attribute" passes — because code can verify it.

## Step 2: Automation Analysis

For each check in the pattern, classify:

| Classification | Criteria | Example |
|---|---|---|
| **MECHANICAL** | Can be verified by reading files, parsing structure, matching patterns, counting elements, validating config | "Every page has a meta description" → grep for `<meta name="description"` |
| **JUDGMENT** | Requires understanding meaning, quality, coherence, appropriateness, or creative merit | "Meta description accurately summarizes page content" → LLM compares description to page body |
| **HYBRID** | Mechanical pre-check narrows scope, LLM evaluates the remainder | "Color contrast meets WCAG AA" → code computes ratio (mechanical), LLM evaluates whether the color choice supports the design intent (judgment) |

**The analysis produces a script spec for each mechanical check:**

```
Script: audit_seo_meta.go
Checks:
  - Every page has <title> (fail if missing)
  - Every page has <meta name="description"> (fail if missing)
  - Title length 30-60 chars (warn if outside range)
  - Description length 120-160 chars (warn if outside range)
  - No duplicate titles across pages (fail if duplicates found)
Input: directory path containing HTML/Astro files
Output: JSON array of findings
Complexity: simple (1 day)
```

## Step 3: Build Plan

Group the scripts by complexity and dependency:

**Phase 1: File/Config Validators** (simplest, highest ROI)
- Frontmatter validation (required fields present, valid values)
- Config file schema checks (toml/json structure)
- File existence checks (required files per project type)
- Taxonomy validation (categories, tags against allowed values)

**Phase 2: Structure Auditors** (medium complexity)
- HTML element presence/absence (meta tags, headings, landmarks)
- CSS property checks (contrast ratios, responsive breakpoints)
- Code structure (function signatures, import patterns, test coverage markers)
- Cross-reference validation (links, anchors, references between files)

**Phase 3: Cross-File Consistency** (highest complexity)
- Number synchronization (test counts, tool counts across all surfaces)
- API surface consistency (MCP tool definitions match documentation)
- Navigation consistency (all pages reachable, no orphans)
- Dependency graph validation (no circular deps, all referenced files exist)

Each script follows the standard I/O contract:

```go
// Input: command-line args
//   -dir string    Project directory to audit
//   -config string Optional config file path
//
// Output: JSON to stdout
//   [{"rule": "meta_description_present", "status": "pass|fail|warn",
//     "file": "path", "evidence": "description", "line": 42}]
//
// Exit code: 0 if all pass, 1 if any fail, 2 if error
```

## Step 4: Converge the Code

Each script goes through the CruxBot script lifecycle:

```
1. CruxBot creates sandbox: /tmp/cruxbot-job-{uuid}/
2. LLM writes script + tests based on script spec
3. CruxBot runs: go test -v (must pass)
4. CruxBot compiles: go build -o audit_xxx
5. CruxBot runs compiled binary against test fixtures
6. If all pass: promote to script library
7. If fail: LLM rewrites, loop from step 2
```

**Convergence criteria**: The script must:
- Pass all unit tests
- Produce correct findings on known-good fixtures (expected: all pass)
- Produce correct findings on known-bad fixtures (expected: specific failures)
- Match the I/O contract exactly (valid JSON, correct exit codes)
- Complete in under 5 seconds for a typical project

## Step 5: LLM Usage Guide

The guide tells the LLM exactly how to apply this pattern during convergence:

```markdown
# FORM_PATTERNS — LLM Audit Guide

## Before You Start
Run the mechanical checks first:
  $ audit_form_patterns -dir {project_dir}

Review the output. Any "fail" findings are definitive — do not re-evaluate them.
Focus your audit on the remaining checks that require judgment.

## Mechanical Checks (handled by code)
These are ALREADY CHECKED by the script. Do not duplicate this work:
- [ ] Every input has a label (audit_form_patterns rule: input_has_label)
- [ ] Required fields have indicators (audit_form_patterns rule: required_indicator)
- [ ] Form has submit button (audit_form_patterns rule: submit_button_present)

## Judgment Checks (your responsibility)
Evaluate these — they require understanding context and intent:
- [ ] Label text is clear and unambiguous for the target audience
- [ ] Error messages are helpful (not just "invalid input")
- [ ] Form flow matches the user's mental model
- [ ] Progressive disclosure is appropriate for form complexity

## Findings Format
Return findings as JSON array:
  [{"dimension": "form_patterns", "rule": "label_clarity",
    "status": "pass|fail", "evidence": "Label 'Name' on email field is misleading",
    "file": "src/pages/signup.astro", "line": 42}]

## What Constitutes a Pass
- All mechanical checks pass (script exit code 0)
- All judgment checks pass
- Two consecutive clean passes = converged
```

## Step 6: Package

The final pattern package in the CruxDev ecosystem:

```
docs/
  FORM_PATTERNS.md              — The pattern (what and why)
  FORM_PATTERNS_LLM_GUIDE.md    — How the LLM applies it (code + judgment)
scripts/
  audit_form_patterns.go        — Source (git-backed)
  audit_form_patterns           — Compiled binary
  audit_form_patterns_test.go   — Tests
fixtures/
  form_good/                    — Known-good test fixtures
  form_bad/                     — Known-bad test fixtures
```

The convergence engine's router detects the pattern package and:
1. Runs the compiled script first
2. Passes script output to the LLM with the usage guide
3. LLM only evaluates judgment checks
4. Combined findings submitted to convergence

## MCP Tool: `create_pattern`

The pattern creation lifecycle is an MCP tool invoked via the `/cruxdev-create-pattern` skill.

### Tool: `create_pattern`

```
Params:
  pattern_name: string (required)   — e.g., "form_patterns"
  topic: string (required)          — research topic, e.g., "web form UX and accessibility"
  project_dir: string (optional)    — project to analyze for context
  skip_code: bool (optional)        — skip Gates 3-4 if <30% mechanical

Flow:
  Gate 1: Research → writes docs/{PATTERN_NAME}.md, runs convergence
  Gate 2: Automation analysis → classifies checks, writes script specs
  Gate 3: Build plan → writes build_plans/BUILD_PLAN_NNN_{PATTERN_NAME}_CODE.md
  Gate 4: Code → builds Go scripts through CruxBot sandbox
  Gate 5: LLM guide → writes docs/{PATTERN_NAME}_LLM_GUIDE.md
  Gate 6: Integration test → validates package against fixtures
  Gate 7: Package → moves to patterns/{pattern_name}/

Returns:
  {
    "pattern_name": "form_patterns",
    "status": "converged",
    "gates_passed": 7,
    "package_path": "patterns/form_patterns/",
    "checks_total": 17,
    "checks_mechanical": 10,
    "checks_judgment": 7,
    "scripts": ["audit_form_patterns"]
  }
```

### Skill: `/cruxdev-create-pattern`

```
Usage: /cruxdev-create-pattern <pattern_name> <topic>

Example: /cruxdev-create-pattern api_security "REST API security patterns"

The skill orchestrates the full 7-gate lifecycle:
1. Researches the topic to convergence
2. Analyzes checks for automation potential
3. Writes build plan for Go audit scripts
4. Converges the code through CruxBot sandbox
5. Writes the LLM usage guide
6. Runs integration tests
7. Packages and adds to pattern library

Each gate requires two consecutive clean passes. The skill does not
stop between gates — it runs to completion or escalates on failure.
```

### Tool: `audit_pattern_package`

```
Params:
  pattern_dir: string (required)    — path to pattern package directory

Checks:
  - Required files present (pattern.md, LLM guide, scripts/, fixtures/)
  - Scripts compile and follow I/O contract
  - Scripts pass on good fixtures, fail on bad fixtures
  - LLM guide references all scripts
  - CONVERGENCE_LOG.md lists all gates

Returns:
  { "status": "pass|fail", "findings": [...] }
```

## Pattern Library Location

Converged pattern packages live in the project's `patterns/` directory:

```
patterns/
  form_patterns/
    FORM_PATTERNS.md
    FORM_PATTERNS_LLM_GUIDE.md
    scripts/
      audit_form_patterns.go
      audit_form_patterns           # compiled binary
      audit_form_patterns_test.go
    fixtures/
      good/                         # known-good test fixtures
      bad/                          # known-bad test fixtures
    CONVERGENCE_LOG.md              # which gates passed, when
  seo_patterns/
    ...
```

Draft patterns (not yet through all gates) stay in `docs/` as standalone markdown files. A pattern moves to `patterns/` only after Gate 7.

## Integration with Convergence Router

The router in `rust/src/engine/router.rs` discovers pattern scripts via directory convention:

1. **Discovery**: Router scans `{project_dir}/patterns/*/scripts/` for compiled binaries matching `audit_*`
2. **Matching**: Script name maps to dimension set (e.g., `audit_form_patterns` → `FORM_DIMENSIONS`)
3. **Invocation**: Router adds script path to task metadata: `{"pattern_script": "patterns/form_patterns/scripts/audit_form_patterns"}`
4. **Execution**: The LLM (or CruxBot) runs the script first, then evaluates only judgment checks
5. **Merging**: Script findings (mechanical) + LLM findings (judgment) are combined and submitted

The router detects pattern packages by checking for the directory structure above. No config file needed — convention over configuration.

This reduces LLM token usage by 40-60% per convergence cycle (based on the automation analysis: 68 of 146 checks are mechanical).

## Applying This Pattern

When creating a new pattern:

1. Write the pattern doc — research to convergence (gate 1)
2. Run automation analysis to convergence (gate 2): "Which checks are mechanical?"
3. If >30% mechanical: write build plan (gate 3), converge code (gate 4)
4. If <30% mechanical: skip to step 5
5. Write LLM usage guide to convergence (gate 5)
6. Run integration tests to convergence (gate 6)
7. Add to pattern library — only after all gates pass

**Do not skip gates.** A pattern without an LLM guide is incomplete. A pattern with code that hasn't been integration-tested is dangerous. A pattern that hasn't been audited is a draft.

When updating an existing pattern:

1. Check if the pattern has a script — update the script spec if checks changed
2. Check if the pattern has an LLM guide — update the guide if checks changed
3. Run the automation analysis on the updated pattern
4. If new mechanical checks identified: add to the script
5. Re-run integration tests — the package must re-converge after any change

## Self-Application: This Pattern Through Its Own Gates

This pattern must pass its own lifecycle. Current status:

### Gate 1: Pattern Document — CONVERGING
- This file. 7-stage lifecycle, 10 auditable checks, concrete examples.

### Gate 2: Automation Analysis
6 of 10 checks are mechanical (60%):

| Check | Classification | Script |
|---|---|---|
| Pattern package has all required files | MECHANICAL | `audit_pattern_package.go` |
| Script follows I/O contract (JSON, exit codes) | MECHANICAL | `audit_pattern_package.go` |
| LLM guide references correct script names | MECHANICAL | `audit_pattern_package.go` |
| Script passes on good fixtures | MECHANICAL | `audit_pattern_package.go` |
| Script fails on bad fixtures | MECHANICAL | `audit_pattern_package.go` |
| All 7 gates documented in CONVERGENCE_LOG.md | MECHANICAL | `audit_pattern_package.go` |
| Every check phrased as verifiable statement | JUDGMENT | LLM evaluation |
| LLM guide separates mechanical vs judgment | JUDGMENT | LLM evaluation |
| Mechanical checks correctly classified | JUDGMENT | LLM evaluation |
| Integration test covers end-to-end | JUDGMENT | LLM evaluation |

### Gate 3: Build Plan
Script: `audit_pattern_package.go`
```
Input: -dir {pattern_package_dir}
Checks:
  - pattern.md exists and is non-empty
  - *_LLM_GUIDE.md exists and is non-empty
  - scripts/ directory exists with at least one .go file
  - scripts/ contains at least one compiled binary (no extension)
  - fixtures/good/ and fixtures/bad/ directories exist
  - CONVERGENCE_LOG.md exists and lists all 7 gates
  - Every script outputs valid JSON array to stdout
  - Every script exits 0 on good fixtures, 1 on bad fixtures
  - LLM guide mentions every script name found in scripts/
Output: JSON findings array
Exit: 0 all pass, 1 any fail
Complexity: simple (1 day)
```

### Gates 4-7: Pending
- Gate 4 (Code): Build `audit_pattern_package.go` through CruxBot sandbox
- Gate 5 (LLM Guide): Write PATTERN_CREATION_PATTERNS_LLM_GUIDE.md
- Gate 6 (Integration): Create good/bad fixture packages, validate end-to-end
- Gate 7 (Library): Move to `patterns/pattern_creation/` after all gates pass

## The Standard

A pattern without code is a suggestion. A pattern with code is a gate. Every pattern in CruxDev should be moving toward gate status — mechanical checks enforced by compiled binaries, judgment checks guided by structured LLM instructions, convergence verified by the engine.

The goal is not zero LLM calls. The goal is zero *unnecessary* LLM calls. The LLM should spend its tokens on judgment, not on counting meta tags.
