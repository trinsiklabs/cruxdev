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

### After every build plan convergence:

1. Run `./scripts/convergence_gate.sh` — automated checks
2. Run self-adoption:
   - `classify_project(project_dir: cruxdev)` — verify classification is accurate
   - `cruxdev_status(project_dir: cruxdev)` — verify counts are correct
   - `analyze_gaps(project_dir: cruxdev)` — find missing docs/templates
   - `check_build_freshness(project_dir: cruxdev)` — verify binary current
3. If findings:
   - Categorize: real gap vs tool bug vs process gap
   - Create build plan for each category
   - Converge the build plan
   - Go back to step 1

### Self-adoption is part of the convergence definition:

A CruxDev build plan is NOT converged until:
- [ ] Convergence gate passes
- [ ] Self-adoption returns zero critical findings
- [ ] Any tool bugs found during self-adoption are fixed
- [ ] Any process gaps found are documented as anti-patterns

## What This Changes

The convergence criterion for CruxDev is stricter than for any other project:
- Other projects: two consecutive clean audit passes
- CruxDev: two consecutive clean audit passes + convergence gate + self-adoption + tool improvement

This is intentional. CruxDev's quality standard must exceed what it asks of the projects it manages.

## Anti-Pattern: Skipping Self-Adoption

If self-adoption is skipped after a build plan, the tool quality stagnates and gaps accumulate silently. BP042 found 7 issues that existed across 30+ "converged" build plans because self-adoption was never run.
