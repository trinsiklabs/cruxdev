# BUILD_PLAN_070: Auxiliary Development Patterns — SvelteKit

**Status:** CONVERGED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

SvelteKit is the official full-stack framework for Svelte, known for its compiler-first approach that ships zero runtime overhead. Svelte 5's runes system represents a fundamental shift in reactivity, and SvelteKit's adapter system enables deployment anywhere. Growing rapidly in developer satisfaction surveys and increasingly adopted for performance-critical applications.

CruxDev manages projects across 18 project types. When adopting a project built with SvelteKit + Svelte 5, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:sveltekit`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (SvelteKit 2, Svelte 5 runes, server-side rendering)
- [ ] 1.2 Research component library ecosystem (Skeleton UI, shadcn-svelte, Melt UI, Bits UI, Paraglide)
- [ ] 1.3 Research testing patterns (Vitest, Svelte Testing Library, Playwright, MSW)
- [ ] 1.4 Research deployment patterns (Vercel, Cloudflare Pages, Node adapter, static adapter, Docker)
- [ ] 1.5 Research common anti-patterns and pitfalls (Svelte 5 runes migration, store vs rune confusion, SSR data flow)

## Phase 2: Write DEVELOPMENT_PATTERNS_SVELTEKIT.md

- [ ] 2.1 Project structure conventions (src/routes/, src/lib/, +page/+layout/+server conventions)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, slots, snippets, component composition)
- [ ] 2.3 State management patterns ($state runes, $derived, $effect, context API, shared state modules)
- [ ] 2.4 Testing strategy (Vitest + Svelte Testing Library for unit, Playwright for E2E, server route testing)
- [ ] 2.5 Performance optimization (streaming, preloading, code splitting, image optimization, zero-JS pages)
- [ ] 2.6 Deployment and CI/CD (adapter selection, environment variables, GitHub Actions, preview deployments)
- [ ] 2.7 Security considerations (form actions CSRF, hooks for auth, Content Security Policy, load function security)
- [ ] 2.8 Common pitfalls to avoid (mixing Svelte 4 stores with Svelte 5 runes, load function waterfalls, missing error boundaries)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect svelte.config.js, +page.svelte files
- [ ] 3.2 Add stack-specific audit dimensions if applicable (svelte-check, Lighthouse, bundle analysis)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for SvelteKit — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
