# BUILD_PLAN_017: FORM_PATTERNS.md — Web Form Design Methodology

**Status:** CONVERGED
**Priority:** High
**Depends on:** BUILD_PLAN_015 (research convergence detection available)

## Context

CruxDev manages projects with web forms (e.g. westlakeselect.net registration) that suffer from common design failures: missing validation indicators, no error states, poor spacing, ambiguous CTAs, accessibility gaps. There is no methodology document governing form design. Without one, the convergence engine has no dimensions to audit forms against.

This plan:
1. Deep-researches form design best practices to convergence using the 5-pass methodology
2. Produces `docs/FORM_PATTERNS.md` — the authoritative reference
3. Integrates form audit dimensions into the convergence engine so forms are audited like code and docs

## Phase 1: Research — 5-Pass Convergence-Detected

Execute research using the methodology in `docs/RESEARCH_PATTERNS.md`:

### Pass 1: Broad landscape (5-8 searches)
- [ ] 1.1.1 "web form design best practices 2025 2026"
- [ ] 1.1.2 "form UX patterns high conversion"
- [ ] 1.1.3 "form validation UX research"
- [ ] 1.1.4 "multi-step form wizard patterns"
- [ ] 1.1.5 "form accessibility WCAG 2.2"
- [ ] 1.1.6 "form field layout single column vs multi column"
- [ ] 1.1.7 "form error handling patterns"
- [ ] 1.1.8 "form completion rate optimization"

### Pass 2: Academic/authoritative
- [ ] 1.2.1 "form design research study 2024 2025 conversion rate"
- [ ] 1.2.2 "Baymard Institute form usability research"
- [ ] 1.2.3 "Nielsen Norman Group form design guidelines"
- [ ] 1.2.4 "Luke Wroblewski web form design best practices"
- [ ] 1.2.5 "Google Material Design form guidelines"
- [ ] 1.2.6 "Apple Human Interface Guidelines forms"

### Pass 3: Practitioner/user experience
- [ ] 1.3.1 "form design case study conversion improvement"
- [ ] 1.3.2 "developer experience form library comparison"
- [ ] 1.3.3 "real world form A/B test results"
- [ ] 1.3.4 "form design portfolio examples award winning"

### Pass 4: Contrarian/adversarial
- [ ] 1.4.1 "form design anti-patterns mistakes"
- [ ] 1.4.2 "when multi-step forms hurt conversion"
- [ ] 1.4.3 "form design myths debunked"
- [ ] 1.4.4 "inline validation problems criticism"

### Pass 5: Primary sources
- [ ] 1.5.1 WCAG 2.2 form requirements (w3.org)
- [ ] 1.5.2 HTML form spec — native validation API, constraint validation
- [ ] 1.5.3 ARIA authoring practices — form patterns
- [ ] 1.5.4 Gov.uk Design System form patterns
- [ ] 1.5.5 US Web Design System (USWDS) form components

### Convergence check
- [ ] 1.6.1 All sub-questions have ≥3 unique sources
- [ ] 1.6.2 Novelty < 10% over last 5 searches
- [ ] 1.6.3 No unresolved contradictions
- [ ] 1.6.4 All findings quality-scored

## Phase 2: Synthesize FORM_PATTERNS.md

Write `docs/FORM_PATTERNS.md` with these sections:

### 2.1 Document structure
- [ ] 2.1.1 Overview — why forms matter (conversion, trust, accessibility)
- [ ] 2.1.2 Core principles (progressive disclosure, forgiving format, immediate feedback)
- [ ] 2.1.3 Layout patterns — single column, field grouping, visual hierarchy
- [ ] 2.1.4 Field design — labels (top-aligned), placeholders (not as labels), input types, sizing
- [ ] 2.1.5 Validation — inline vs submit, error message writing, success states
- [ ] 2.1.6 Multi-step forms — when to use, progress indicators, save state
- [ ] 2.1.7 Accessibility — WCAG 2.2 compliance, ARIA, keyboard nav, screen readers
- [ ] 2.1.8 Mobile patterns — touch targets, input modes, autofill
- [ ] 2.1.9 CTAs — button text, placement, visual weight, loading states
- [ ] 2.1.10 Error recovery — inline errors, summary, focus management, persistence
- [ ] 2.1.11 Trust & conversion — field count reduction, social proof, progress
- [ ] 2.1.12 Anti-patterns — what NOT to do (with citations)
- [ ] 2.1.13 Audit dimensions (for convergence engine integration)
- [ ] 2.1.14 References — all sources with quality scores

## Phase 3: Integration into Convergence Engine

### 3.1 Form audit dimensions
- [ ] 3.1.1 Define FORM_DIMENSIONS constant in `rust/src/engine/router.rs`
  - layout, labels, validation, errors, accessibility, mobile, cta, trust, performance
- [ ] 3.1.2 Add form audit task type to router (alongside code/doc/plan audits)
- [ ] 3.1.3 Form detection: auto-detect forms in HTML/JSX/TSX files

### 3.2 Website convergence integration
- [ ] 3.2.1 When WEBSITE_CONVERGENCE phase runs, include form dimensions if forms detected
- [ ] 3.2.2 Form findings produce actionable fix tasks

### 3.3 Skill integration
- [ ] 3.3.1 `frontend-design` skill references FORM_PATTERNS.md
- [ ] 3.3.2 Convergence engine loads form methodology when auditing web projects

## Phase 4: Tests

- [ ] 4.1 Unit test: form dimension constant exists and matches doc
- [ ] 4.2 Unit test: form detection finds <form> in HTML
- [ ] 4.3 Test: FORM_PATTERNS.md passes doc audit (accuracy, completeness, clarity)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
# Verify doc exists and is non-empty
test -s docs/FORM_PATTERNS.md
```

## Sub-Questions for Research

1. What layout pattern produces highest completion rates? (single column vs multi-column vs adaptive)
2. Where should labels be placed relative to inputs? (top-aligned vs left-aligned vs floating)
3. When should validation fire? (on blur, on change, on submit, or adaptive)
4. How should errors be displayed? (inline, toast, summary, or combination)
5. What is the maximum number of fields before conversion drops significantly?
6. How should multi-step forms indicate progress?
7. What WCAG 2.2 requirements apply specifically to forms?
8. What mobile-specific patterns improve form completion on touch devices?
9. How do form design patterns differ for registration, checkout, contact, and search forms?
10. What are the measurable conversion impacts of specific form improvements?
