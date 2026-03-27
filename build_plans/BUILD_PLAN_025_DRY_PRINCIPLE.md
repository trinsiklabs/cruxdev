# BUILD_PLAN_025: DRY Principle Integration into DEVELOPMENT_PATTERNS_CRUXDEV.md

**Status:** CONVERGED
**Priority:** High

## Context

The DRY (Don't Repeat Yourself) principle is foundational but widely misunderstood. The original definition (Hunt/Thomas) is about knowledge and intent, not textual similarity. Naive DRY application causes wrong abstractions that are worse than duplication (Sandi Metz). This plan adds a DRY section to DEVELOPMENT_PATTERNS_CRUXDEV.md and integrates a duplication audit dimension into the convergence engine.

## Phase 1: Add DRY Section to DEVELOPMENT_PATTERNS_CRUXDEV.md

- [ ] 1.1 Original definition: "Every piece of knowledge must have a single, unambiguous, authoritative representation" — NOT "don't have two lines that look the same"
- [ ] 1.2 Where to apply: business logic, config, schemas, data transformations, infrastructure
- [ ] 1.3 Where NOT to apply: tests (DAMP > DRY), prototypes, cross-service boundaries, incidental similarity
- [ ] 1.4 Rule of Three: tolerate duplication until third instance, then extract
- [ ] 1.5 The abstraction tradeoff: "Duplication is far cheaper than the wrong abstraction" (Metz)
- [ ] 1.6 AHA Programming: Avoid Hasty Abstractions (Dodds, building on Metz)
- [ ] 1.7 DRY in AI-generated code: LLMs produce duplication by default, needs post-generation scanning
- [ ] 1.8 Anti-patterns: premature abstraction, wrong abstraction, coupling through sharing, DRY cargo cult
- [ ] 1.9 Detection tools: jscpd, SonarQube, PMD CPD, Clippy, Pylint

## Phase 2: Add Duplication Audit Dimension to Convergence Engine

- [ ] 2.1 Add `duplication` to CODE_DIMENSIONS in router.rs
- [ ] 2.2 Audit criteria: duplication % (flag >5%, fail >15%), business logic single-source, schema single-source
- [ ] 2.3 Exclusions: test code, intentional boundary duplication, <3 instances
- [ ] 2.4 "Intentional duplication" annotation mechanism

## Phase 3: Tests

- [ ] 3.1 Unit test: duplication dimension present in CODE_DIMENSIONS
- [ ] 3.2 Verify DEVELOPMENT_PATTERNS_CRUXDEV.md covers DRY section
