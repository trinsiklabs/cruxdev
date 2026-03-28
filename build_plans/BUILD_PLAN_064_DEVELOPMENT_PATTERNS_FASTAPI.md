# BUILD_PLAN_064: Auxiliary Development Patterns — FastAPI

**Status:** CONVERGED
**Priority:** P1
**Category:** Stack-specific development patterns

## Context

FastAPI is the fastest-growing Python web framework with 80k+ GitHub stars, built on modern Python async/await and type hints. It has become the default choice for Python APIs, ML model serving, and microservices due to automatic OpenAPI docs, Pydantic validation, and native async support.

CruxDev manages projects across 18 project types. When adopting a project built with FastAPI + Pydantic + SQLAlchemy, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:fastapi`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (FastAPI 0.110+, Pydantic v2, async patterns)
- [ ] 1.2 Research library ecosystem (SQLAlchemy 2.0, Alembic, httpx, SQLModel, Celery/ARQ)
- [ ] 1.3 Research testing patterns (pytest + httpx AsyncClient, pytest-asyncio, factory_boy, respx)
- [ ] 1.4 Research deployment patterns (Uvicorn + Gunicorn, Docker, Kubernetes, AWS Lambda/Mangum)
- [ ] 1.5 Research common anti-patterns and pitfalls (sync in async, missing dependency injection, Pydantic v1 vs v2)

## Phase 2: Write DEVELOPMENT_PATTERNS_FASTAPI.md

- [ ] 2.1 Project structure conventions (routers, schemas, models, services, dependency injection)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, router separation, shared dependencies)
- [ ] 2.3 State management patterns (SQLAlchemy sessions, dependency injection, background tasks, Redis caching)
- [ ] 2.4 Testing strategy (pytest + httpx for API tests, pytest-asyncio, testcontainers for DB)
- [ ] 2.5 Performance optimization (async I/O, connection pooling, response caching, Pydantic v2 speed)
- [ ] 2.6 Deployment and CI/CD (Docker multi-stage, Uvicorn workers, health checks, Alembic migrations in CI)
- [ ] 2.7 Security considerations (OAuth2/JWT, CORS, rate limiting, input validation via Pydantic, secrets management)
- [ ] 2.8 Common pitfalls to avoid (blocking calls in async routes, missing Depends() cleanup, circular imports, schema sprawl)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect FastAPI in requirements.txt/pyproject.toml
- [ ] 3.2 Add stack-specific audit dimensions if applicable (mypy, ruff, bandit)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for FastAPI — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
