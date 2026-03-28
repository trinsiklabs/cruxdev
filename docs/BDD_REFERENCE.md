# BDD (Behavior-Driven Development) — Reference Guide

**Status:** Evaluated, NOT a standalone patterns doc. Conditional dimensions added to convergence engine.
**Decision:** BDD dimensions activate automatically when `.feature` files detected in a project.

---

## When BDD Is Recommended

| Project Profile | Recommendation |
|---|---|
| Software with non-technical stakeholders who actively participate | YES |
| Regulated industry (compliance, audit trails, living docs) | YES |
| Legacy system modernization (behavior preservation) | YES |
| API-first services | MAYBE (OpenAPI + contract tests may be leaner) |
| Developer-only internal tools | NO (TDD sufficient) |
| Books, courses, non-software | N/A |
| CruxDev itself | NO (Rust ecosystem doesn't favor Gherkin) |

## BDD Framework Selection by Stack

| Stack | Recommended | Avoid |
|---|---|---|
| Ruby | Cucumber, RSpec | — |
| JavaScript/TypeScript | Cucumber.js, playwright-bdd | — |
| Python | pytest-bdd, Behave | Lettuce (stale) |
| Java/Kotlin | Cucumber-JVM | JBehave (legacy) |
| Go | Godog | — |
| .NET/C# | Reqnroll | SpecFlow (EOL Dec 2024) |
| Swift | Quick/Nimble | — |
| Flutter/Dart | bdd_widget_test | — |
| Elixir | — | White Bread, Cabbage (stale) |
| Rust | — | cucumber-rs (niche, unnatural fit) |

## Anti-Patterns

1. Writing Gherkin AFTER code (defeats the purpose)
2. UI-level BDD for everything (should be mostly API/service level)
3. Incidental details in scenarios (`Given I click the blue button with id "submit"`)
4. Scenario proliferation (50+ scenarios per feature)
5. Step definition coupling (fragile regex matching)
6. BDD as unit test replacement (wrong level of abstraction)
7. No non-technical stakeholder reading the feature files

## CruxDev Integration

When `.feature` files detected, these dimensions activate:
- `feature_coverage` — all acceptance criteria have scenarios
- `scenario_quality` — declarative, no incidental details
- `step_reuse` — DRY steps, reuse rate > 60%
- `living_doc_freshness` — generated docs match current features
- `gherkin_code_traceability` — every scenario has passing step definitions

## Key Finding

AI makes BDD's automation phase nearly free. But BDD's value was never in automation — it was in collaborative discovery (Three Amigos). AI reduces the cost of the cheap part while leaving the hard part untouched.
