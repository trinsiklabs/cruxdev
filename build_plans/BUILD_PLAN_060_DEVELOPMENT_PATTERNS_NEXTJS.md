# BUILD_PLAN_060: Auxiliary Development Patterns — Next.js

**Status:** NOT STARTED
**Priority:** P0
**Category:** Stack-specific development patterns

## Context

Next.js dominates the React meta-framework space with 6M+ weekly npm downloads and is the default choice for new React projects. Vercel's ecosystem (hosting, edge functions, image optimization) makes it a full-stack platform.

CruxDev manages projects across 18 project types. When adopting a project built with Next.js + React + Tailwind + shadcn/ui + Prisma/Drizzle, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:nextjs`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (App Router vs Pages Router, RSC, Server Actions)
- [ ] 1.2 Research component library ecosystem (shadcn/ui, Radix UI, Headless UI, React Aria)
- [ ] 1.3 Research testing patterns (Vitest, React Testing Library, Playwright, MSW)
- [ ] 1.4 Research deployment patterns (Vercel, self-hosted, Docker, edge runtime)
- [ ] 1.5 Research common anti-patterns and pitfalls (client/server boundary, hydration mismatches, bundle bloat)

## Phase 2: Write DEVELOPMENT_PATTERNS_NEXTJS.md

- [ ] 2.1 Project structure conventions (app/ directory, route groups, colocation)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, server vs client components)
- [ ] 2.3 State management patterns (React Server Components, Zustand, Jotai, URL state)
- [ ] 2.4 Testing strategy (Vitest for unit, RTL for integration, Playwright for E2E)
- [ ] 2.5 Performance optimization (ISR, streaming, dynamic imports, image optimization)
- [ ] 2.6 Deployment and CI/CD (Vercel, Docker multi-stage, GitHub Actions)
- [ ] 2.7 Security considerations (server actions validation, CSRF, CSP, env var exposure)
- [ ] 2.8 Common pitfalls to avoid (over-clientizing, missing loading/error boundaries, N+1 queries in RSC)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect next.config.js/ts, app/ directory
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Lighthouse, bundle analysis)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Next.js — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
