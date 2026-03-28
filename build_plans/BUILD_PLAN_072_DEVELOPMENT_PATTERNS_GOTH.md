# BUILD_PLAN_072: Auxiliary Development Patterns — Go + HTMX (GoTH)

**Status:** CONVERGED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

The GoTH stack (Go + Templ + HTMX) represents the "new simplicity" movement in web development — server-rendered HTML with targeted interactivity, no JavaScript build step, single binary deployment. Go's performance, Templ's type-safe templates, and HTMX's declarative interactivity make this an increasingly popular choice for developers who reject SPA complexity.

CruxDev manages projects across 18 project types. When adopting a project built with Go + Gin + Templ + HTMX, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:goth`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Go 1.22+, Gin/Chi/Echo, Templ, HTMX)
- [ ] 1.2 Research library ecosystem (sqlc, GORM, Ent, Air live reload, goose migrations, validator)
- [ ] 1.3 Research testing patterns (go test, testify, httptest, testcontainers-go, gomock)
- [ ] 1.4 Research deployment patterns (single binary, Docker scratch/distroless, systemd, Fly.io, Kubernetes)
- [ ] 1.5 Research common anti-patterns and pitfalls (goroutine leaks, error handling neglect, template rendering costs)

## Phase 2: Write DEVELOPMENT_PATTERNS_GOTH.md

- [ ] 2.1 Project structure conventions (cmd/, internal/, pkg/, templates/, standard Go project layout)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, Templ components, HTMX partials, handler composition)
- [ ] 2.3 State management patterns (database as source of truth, session middleware, Redis caching, context propagation)
- [ ] 2.4 Testing strategy (go test + testify for unit, httptest for handlers, testcontainers for integration, Playwright for E2E)
- [ ] 2.5 Performance optimization (connection pooling, template caching, static file embedding, HTTP/2, response compression)
- [ ] 2.6 Deployment and CI/CD (multi-stage Docker, single binary, goose/migrate for DB, GitHub Actions, Goreleaser)
- [ ] 2.7 Security considerations (input validation, SQL injection via parameterized queries, CSRF with gorilla/csrf, rate limiting, CSP headers)
- [ ] 2.8 Common pitfalls to avoid (goroutine leaks, missing context cancellation, HTMX swap target mismatches, error swallowing)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect go.mod, .templ files, HTMX script tags
- [ ] 3.2 Add stack-specific audit dimensions if applicable (golangci-lint, go vet, staticcheck)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Go + HTMX — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
