# BUILD_PLAN_069: Auxiliary Development Patterns — Nuxt (Vue)

**Status:** NOT STARTED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

Nuxt is the leading Vue.js meta-framework with 55k+ GitHub stars, providing SSR, SSG, and hybrid rendering out of the box. Vue 3's Composition API combined with Nuxt 3's Nitro server engine and auto-imports makes it a highly productive full-stack framework, popular in Europe and Asia with strong enterprise adoption.

CruxDev manages projects across 18 project types. When adopting a project built with Vue + Nuxt + Vuetify/PrimeVue, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:nuxt`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Nuxt 3, Vue 3 Composition API, Nitro server engine)
- [ ] 1.2 Research component library ecosystem (Vuetify 3, PrimeVue, Naive UI, Headless UI Vue, Nuxt UI)
- [ ] 1.3 Research testing patterns (Vitest, Vue Test Utils, Nuxt Test Utils, Playwright, MSW)
- [ ] 1.4 Research deployment patterns (Vercel, Netlify, Cloudflare Workers, Node.js server, Docker)
- [ ] 1.5 Research common anti-patterns and pitfalls (Options vs Composition API mixing, reactivity gotchas, SSR hydration)

## Phase 2: Write DEVELOPMENT_PATTERNS_NUXT.md

- [ ] 2.1 Project structure conventions (pages/, components/, composables/, server/, auto-imports)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, composables, provide/inject)
- [ ] 2.3 State management patterns (Pinia, useState composable, useFetch/useAsyncData, VueUse)
- [ ] 2.4 Testing strategy (Vitest + Vue Test Utils for unit, Nuxt Test Utils for integration, Playwright for E2E)
- [ ] 2.5 Performance optimization (lazy components, payload optimization, ISR, image module, chunk splitting)
- [ ] 2.6 Deployment and CI/CD (Nitro presets for different platforms, Docker, GitHub Actions, database migrations)
- [ ] 2.7 Security considerations (server routes auth, CORS, CSP, input validation with Zod, env runtime config)
- [ ] 2.8 Common pitfalls to avoid (reactivity loss with destructuring, missing toRef/toRefs, SSR-only code in client, over-fetching)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect nuxt.config.ts, .nuxtrc
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Vue DevTools metrics, Lighthouse, bundle analysis)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Nuxt — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
