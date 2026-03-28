# BUILD_PLAN_062: Auxiliary Development Patterns — TALL Stack (Laravel)

**Status:** CONVERGED
**Priority:** P0
**Category:** Stack-specific development patterns

## Context

Laravel is the most popular PHP framework with 78k+ GitHub stars and dominates the PHP ecosystem. The TALL stack (Tailwind + Alpine.js + Livewire + Laravel) is the official recommended full-stack approach, enabling reactive UIs without writing JavaScript SPAs.

CruxDev manages projects across 18 project types. When adopting a project built with Laravel + Livewire + Alpine.js + Tailwind, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:tall`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Laravel 11, Livewire 3, Volt single-file components)
- [ ] 1.2 Research component library ecosystem (Filament, Wire Elements, Laravel Jetstream, Breeze)
- [ ] 1.3 Research testing patterns (Pest PHP, Laravel Dusk, Livewire testing utilities, Mockery)
- [ ] 1.4 Research deployment patterns (Laravel Forge, Vapor/serverless, Docker, Octane)
- [ ] 1.5 Research common anti-patterns and pitfalls (Livewire performance, N+1 queries, over-engineering with events)

## Phase 2: Write DEVELOPMENT_PATTERNS_TALL.md

- [ ] 2.1 Project structure conventions (app/ directory, service providers, domain-driven modules)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, Livewire components, Blade components)
- [ ] 2.3 State management patterns (Livewire properties, session, cache, Eloquent)
- [ ] 2.4 Testing strategy (Pest for unit, Livewire test helpers for integration, Dusk for E2E)
- [ ] 2.5 Performance optimization (Eloquent eager loading, query scopes, Octane, response caching)
- [ ] 2.6 Deployment and CI/CD (Forge, Vapor, Docker + FrankenPHP, GitHub Actions, Envoy)
- [ ] 2.7 Security considerations (mass assignment, SQL injection, XSS, CSRF, authorization policies)
- [ ] 2.8 Common pitfalls to avoid (Livewire hydration issues, missing wire:key, excessive database calls in render())

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect artisan, composer.json with laravel/framework
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Laravel Pint, Larastan/PHPStan)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for the TALL Stack — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
