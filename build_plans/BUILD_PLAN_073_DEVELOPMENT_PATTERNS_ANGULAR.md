# BUILD_PLAN_073: Auxiliary Development Patterns — Angular

**Status:** NOT STARTED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

Angular is Google's enterprise-grade frontend framework with strong adoption in banking, government, and large corporations. Angular 17+ introduced signals, deferrable views, and the new control flow syntax, representing a major modernization. With built-in routing, forms, HTTP, and dependency injection, it remains the most opinionated and batteries-included frontend framework.

CruxDev manages projects across 18 project types. When adopting a project built with Angular + Angular Material, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:angular`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Angular 17/18+, signals, standalone components, SSR with Angular Universal)
- [ ] 1.2 Research component library ecosystem (Angular Material, PrimeNG, NG-ZORRO, Taiga UI, CDK)
- [ ] 1.3 Research testing patterns (Jest/Vitest replacing Karma, Angular Testing Library, Cypress/Playwright, Spectator)
- [ ] 1.4 Research deployment patterns (Docker, Nginx, Cloud Run, Angular Universal for SSR, prerendering)
- [ ] 1.5 Research common anti-patterns and pitfalls (RxJS memory leaks, change detection performance, over-using services)

## Phase 2: Write DEVELOPMENT_PATTERNS_ANGULAR.md

- [ ] 2.1 Project structure conventions (feature modules, standalone components, barrel exports, Nx for monorepos)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, smart/dumb components, content projection)
- [ ] 2.3 State management patterns (signals, NgRx/SignalStore, RxJS, services with BehaviorSubject, computed signals)
- [ ] 2.4 Testing strategy (Jest/Vitest for unit, Angular Testing Library for component, Playwright for E2E, Spectator for DI-heavy tests)
- [ ] 2.5 Performance optimization (OnPush change detection, signals, trackBy, deferrable views, lazy loading routes, bundle budgets)
- [ ] 2.6 Deployment and CI/CD (ng build, Docker + Nginx, Angular Universal SSR, GitHub Actions, prerendering)
- [ ] 2.7 Security considerations (built-in XSS protection, DomSanitizer, HTTP interceptors for auth, CSP, strict template checking)
- [ ] 2.8 Common pitfalls to avoid (RxJS subscription leaks, zone.js performance, circular DI, over-engineering with NgRx for simple state)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect angular.json, @angular/core in package.json
- [ ] 3.2 Add stack-specific audit dimensions if applicable (ng lint, strict mode, bundle budgets)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Angular — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
