# BUILD_PLAN_043: CruxDev Self-Improvement Cycle

**Status:** NOT STARTED
**Priority:** Strategic (unique CruxDev-only pattern)

## Context

CruxDev is the only project where the product IS the process that evaluates the product. Running self-adoption after each build plan creates a bootstrapping loop: the tool finds its own gaps, fixes the tool, which makes it better at finding the next gap.

This is NOT a pattern for other projects. It's a CruxDev-internal development methodology.

## The Cycle

```
Build plan converged
    │
    ├── Run convergence gate (automated checks)
    │
    ├── Run self-adoption (classify + status + analyze_gaps + build_freshness)
    │       │
    │       ├── Zero findings → genuinely converged
    │       │
    │       └── Findings found:
    │           ├── Is the finding a real gap? → new build plan to fix it
    │           ├── Is the finding a tool false-positive? → improve the tool
    │           └── Is the finding a convergence process gap? → improve the process
    │
    ├── Re-run self-adoption with improved tools
    │       │
    │       ├── Deeper findings emerge (old tool couldn't see these)
    │       └── Repeat until zero findings
    │
    └── CONVERGED (both the build plan AND the tools that verified it)
```

## What Makes This Unique

1. **The product improves itself.** BP042 found classify_project false-matching on templates. Fixing that makes the classifier better for ALL projects, not just CruxDev.

2. **The measurement tool gets better.** Each self-adoption cycle improves the adoption tools. Better tools find deeper issues. Deeper fixes produce better tools.

3. **The convergence process evolves.** BP041 (convergence enforcement) was created because self-adoption exposed rubber-stamping. The process fix came FROM the process failure.

4. **No other project can do this.** A web app can't run its own code quality tool on itself and have the findings improve the tool. CruxDev can, because the adoption/convergence tools ARE the product.

## Implementation

### After every build plan convergence — the full cycle:

1. **Convergence gate** — `./scripts/convergence_gate.sh` (automated checks)
2. **Self-adoption** — run adoption tools on CruxDev itself:
   - `classify_project` — verify classification is accurate (no false positives)
   - `cruxdev_status` — verify tool/skill/template counts are correct
   - `analyze_gaps` — find missing docs/templates
   - `check_build_freshness` — verify binary current
3. **Ground truth verification** — verify every claim is real:
   - Every capability claim in docs → code exists that implements it
   - Every metric → matches actual test/tool output
   - Every skill → references real tools with correct names/params
   - Every website page → claims verified against source code
4. **If findings from steps 2 or 3:**
   - Categorize: real gap vs tool bug vs process gap vs false claim
   - Create build plan for each
   - Converge the build plan
   - Go back to step 1 (the full cycle reruns)
5. **Zero findings from BOTH adoption AND GTV → proceed to step 6**

### Adoption process alignment audit:

After zero findings from steps 2-5, verify the measuring tools themselves are current:

6. Audit the adoption process against current capabilities:
   - Does `/adopt` skill reference all current MCP tools it should use?
   - Does ADOPTION_PROCESS.md describe the current tool set and project types?
   - Does ADOPTION_PLAYBOOK.md reflect current templates (228+), dimensions (39+), and project types (18)?
   - Does the template registry include all filesystem templates?
   - Does the classifier detect all current project types without false positives?
   - Does the gap analysis check against the correct template set for the detected type?
5. If the adoption process is stale, fix it FIRST, then re-run self-adoption with the fixed process
6. The measurement tool must be current before its measurements are trusted

### Ground truth verification (GTV):

After self-adoption and adoption alignment, verify every truth claim:

7. GTV all claims to convergence:
   - Every capability claim in docs → verified against implementing code (file + function)
   - Every metric on the website → verified against actual test/tool output
   - Every skill tool reference → verified against server.rs tool list
   - Every template count → verified against filesystem
   - Every dimension list → verified against router.rs constants
   - Every competitive claim → verified against actual implementation
   - If a claim cannot be verified, it must be removed or marked as roadmap
8. GTV is not a single pass — it's to convergence (two consecutive clean passes)
9. The first GTV pass finds claims. The second verifies the fixes. Zero unverified claims = converged.

**GTV catches what adoption misses:**
- Adoption finds structural gaps (missing files, wrong counts)
- GTV finds semantic gaps (claims that sound right but aren't implemented)
- Both are required. Neither alone is sufficient.

### Self-adoption is part of the convergence definition:

A CruxDev build plan is NOT converged until:
- [ ] Convergence gate passes (automated)
- [ ] Self-adoption returns zero critical findings (structural)
- [ ] Ground truth verification finds zero unverified claims (semantic)
- [ ] Any tool bugs found are fixed
- [ ] Any process gaps are documented as anti-patterns
- [ ] Adoption process verified current against all capabilities

## What This Changes

The convergence criterion for CruxDev is stricter than for any other project:
- Other projects: two consecutive clean audit passes
- CruxDev: two consecutive clean audit passes + convergence gate + self-adoption + tool improvement

This is intentional. CruxDev's quality standard must exceed what it asks of the projects it manages.

## External Feedback Loop

Projects using CruxDev are field-testing the adoption tools against real codebases. When adoption finds something wrong with the TOOL rather than the PROJECT, it should be reported upstream:

```
Project runs /adopt → finds gap → is this MY gap or a CruxDev gap?
    │
    ├── Project gap → fix it locally
    │
    └── CruxDev gap → file issue at github.com/trinsiklabs/cruxdev/issues
        → CruxDev's issue monitor picks it up
        → Evaluates → build plan → converge → self-adopt
        → Improved tools ship
        → ALL projects benefit on next adoption run
```

The cruxcli session demonstrated this: it filed the `setup_competitive_analysis` empty-file bug via session bus, CruxDev fixed the root cause, and the fix benefited every project.

**How to distinguish project gap vs CruxDev gap:**
- Template missing that should exist for this project type → CruxDev gap (template registry incomplete)
- Classifier detects wrong project type → CruxDev gap (classifier bug)
- Tool returns incorrect data → CruxDev gap (tool bug)
- Gap analysis flags something that's intentionally absent → Project decision (not a gap)
- Convergence process misses a real issue → CruxDev gap (process improvement needed)

## Anti-Pattern: Skipping Self-Adoption

If self-adoption is skipped after a build plan, the tool quality stagnates and gaps accumulate silently. BP042 found 7 issues that existed across 30+ "converged" build plans because self-adoption was never run.
