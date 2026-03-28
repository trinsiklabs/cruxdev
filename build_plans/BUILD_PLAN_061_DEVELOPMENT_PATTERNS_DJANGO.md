# BUILD_PLAN_061: Auxiliary Development Patterns — Django

**Status:** CONVERGED
**Priority:** P0
**Category:** Stack-specific development patterns

## Context

Django powers ~12% of all web applications and remains the dominant Python web framework. With Django REST Framework for APIs and the rising HTMX + Alpine.js pattern for interactive UIs without heavy JavaScript, it is the go-to choice for rapid full-stack Python development.

CruxDev manages projects across 18 project types. When adopting a project built with Django + DRF + HTMX + Alpine.js, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:django`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Django 5.x, class-based views, async support)
- [ ] 1.2 Research component library ecosystem (django-components, django-cotton, HTMX patterns, Alpine.js)
- [ ] 1.3 Research testing patterns (pytest-django, factory_boy, model_bakery, coverage.py)
- [ ] 1.4 Research deployment patterns (Gunicorn + Nginx, Docker, Whitenoise, Celery)
- [ ] 1.5 Research common anti-patterns and pitfalls (fat views, N+1 queries, missing select_related/prefetch_related)

## Phase 2: Write DEVELOPMENT_PATTERNS_DJANGO.md

- [ ] 2.1 Project structure conventions (apps, settings split, URL namespacing)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, template partials, HTMX fragments)
- [ ] 2.3 State management patterns (session, Django ORM, signals, caching with Redis)
- [ ] 2.4 Testing strategy (pytest-django for unit/integration, Playwright for E2E, factory_boy for fixtures)
- [ ] 2.5 Performance optimization (query optimization, caching layers, async views, database indexing)
- [ ] 2.6 Deployment and CI/CD (Docker + Gunicorn, collectstatic, migrations in CI, health checks)
- [ ] 2.7 Security considerations (CSRF, XSS via template escaping, SQL injection, SECRET_KEY management)
- [ ] 2.8 Common pitfalls to avoid (circular imports, fat models vs fat views debate, migration conflicts, missing indexes)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect manage.py, settings.py, wsgi.py
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Django check --deploy, migration lint)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Django — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
