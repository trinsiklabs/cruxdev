# BUILD_PLAN_063: Auxiliary Development Patterns — Rails

**Status:** CONVERGED
**Priority:** P0
**Category:** Stack-specific development patterns

## Context

Ruby on Rails remains the productivity benchmark for web frameworks, powering Shopify, GitHub, and Basecamp. Rails 7+ with Hotwire (Turbo + Stimulus) represents a return to server-rendered HTML with targeted interactivity, eliminating the need for heavy client-side JavaScript frameworks.

CruxDev manages projects across 18 project types. When adopting a project built with Rails + Hotwire + Turbo + Stimulus, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:rails`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Rails 7/8, Hotwire, Turbo Streams/Frames, Stimulus)
- [ ] 1.2 Research component library ecosystem (ViewComponent, Phlex, Lookbook, Stimulus components)
- [ ] 1.3 Research testing patterns (RSpec, Minitest, Capybara, FactoryBot, VCR)
- [ ] 1.4 Research deployment patterns (Kamal 2, Docker, Heroku, Fly.io, Render)
- [ ] 1.5 Research common anti-patterns and pitfalls (callback hell, god models, Turbo frame nesting issues)

## Phase 2: Write DEVELOPMENT_PATTERNS_RAILS.md

- [ ] 2.1 Project structure conventions (app/ directory, concerns, service objects, form objects)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, ViewComponent, partials, Turbo Frames)
- [ ] 2.3 State management patterns (ActiveRecord, Turbo Streams, Stimulus values/targets, Kredis/Redis)
- [ ] 2.4 Testing strategy (RSpec or Minitest for unit, system tests with Capybara, FactoryBot for fixtures)
- [ ] 2.5 Performance optimization (eager loading, counter caches, fragment caching, Solid Cache)
- [ ] 2.6 Deployment and CI/CD (Kamal 2 for Docker deploy, GitHub Actions, database migrations, asset pipeline)
- [ ] 2.7 Security considerations (strong parameters, CSRF, SQL injection, Brakeman scanner, Content Security Policy)
- [ ] 2.8 Common pitfalls to avoid (N+1 queries, fat controllers, missing database indexes, Turbo Drive conflicts)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect Gemfile with rails, config/routes.rb
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Brakeman, RuboCop, bundler-audit)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Rails — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
