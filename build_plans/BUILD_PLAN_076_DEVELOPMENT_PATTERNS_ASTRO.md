# BUILD_PLAN_076: Auxiliary Development Patterns — Astro

**Status:** NOT STARTED
**Priority:** P3
**Category:** Stack-specific development patterns

## Context

Astro is the content-focused web framework that pioneered the "islands architecture" — shipping zero JavaScript by default and hydrating only interactive components. With 48k+ GitHub stars and adoption for marketing sites, blogs, documentation, and e-commerce, Astro's framework-agnostic approach lets teams use React, Vue, Svelte, or Solid components within the same project.

CruxDev manages projects across 18 project types. When adopting a project built with Astro + any frontend framework, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:astro`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Astro 4+, content collections, server islands, view transitions)
- [ ] 1.2 Research integration ecosystem (React/Vue/Svelte/Solid integrations, Starlight docs, Astro DB, Keystatic)
- [ ] 1.3 Research testing patterns (Vitest, Playwright, Astro Container API, component testing per framework)
- [ ] 1.4 Research deployment patterns (Vercel, Netlify, Cloudflare Pages, Deno Deploy, static hosting, SSR adapters)
- [ ] 1.5 Research common anti-patterns and pitfalls (over-hydration, mixing too many frameworks, content collection schema drift)

## Phase 2: Write DEVELOPMENT_PATTERNS_ASTRO.md

- [ ] 2.1 Project structure conventions (src/pages/, src/content/, src/components/, src/layouts/, content collections)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, island architecture, client:* directives)
- [ ] 2.3 State management patterns (nanostores for cross-framework state, content collections, Astro DB, server endpoints)
- [ ] 2.4 Testing strategy (Vitest for unit, Container API for component, Playwright for E2E, content schema validation)
- [ ] 2.5 Performance optimization (zero-JS by default, selective hydration, image optimization, view transitions, prefetching)
- [ ] 2.6 Deployment and CI/CD (adapter selection, static vs SSR, GitHub Actions, content preview, environment variables)
- [ ] 2.7 Security considerations (SSR endpoint validation, CSP headers, API route authentication, environment variable exposure)
- [ ] 2.8 Common pitfalls to avoid (unnecessary client:load, framework interop complexity, content collection type drift, SSR cold starts)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect astro.config.mjs, .astro files
- [ ] 3.2 Add stack-specific audit dimensions if applicable (astro check, Lighthouse, bundle analysis)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Astro — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
