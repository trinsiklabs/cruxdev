# BUILD_PLAN_075: Auxiliary Development Patterns — Rust + Axum

**Status:** NOT STARTED
**Priority:** P3
**Category:** Stack-specific development patterns

## Context

Axum is the leading Rust web framework built on Tokio and Tower, created by the Tokio team. Rust's memory safety, zero-cost abstractions, and raw performance make it ideal for high-throughput APIs, infrastructure tooling, and latency-sensitive services. Axum's type-safe extractors and Tower middleware ecosystem provide an ergonomic developer experience atop Rust's guarantees.

CruxDev manages projects across 18 project types. When adopting a project built with Rust + Axum + Tokio, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:axum`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Axum 0.7+, Tokio, Tower middleware, async Rust patterns)
- [ ] 1.2 Research library ecosystem (SQLx, SeaORM, serde, tracing, tower-http, askama/maud templates)
- [ ] 1.3 Research testing patterns (cargo test, axum::test, testcontainers, mockall, proptest, wiremock)
- [ ] 1.4 Research deployment patterns (Docker scratch/distroless, cross-compilation, musl, systemd, Kubernetes)
- [ ] 1.5 Research common anti-patterns and pitfalls (async lifetime issues, Arc<Mutex> overuse, error handling fragmentation)

## Phase 2: Write DEVELOPMENT_PATTERNS_AXUM.md

- [ ] 2.1 Project structure conventions (workspace layout, lib vs bin crate, modules, feature flags)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, handler composition, extractors, middleware layers)
- [ ] 2.3 State management patterns (AppState with Arc, SQLx connection pools, Tower layers, Redis with deadpool)
- [ ] 2.4 Testing strategy (cargo test for unit, TestServer/TestClient for integration, testcontainers for DB, proptest for property)
- [ ] 2.5 Performance optimization (zero-copy deserialization, connection pooling, async streaming, compile-time optimization, PGO)
- [ ] 2.6 Deployment and CI/CD (multi-stage Docker with cargo-chef, cross for cross-compilation, GitHub Actions, cargo-deny)
- [ ] 2.7 Security considerations (input validation with validator, authentication extractors, rate limiting, TLS, cargo-audit)
- [ ] 2.8 Common pitfalls to avoid (Send + Sync bound errors, blocking in async, unwrap() in production, missing graceful shutdown)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect Cargo.toml with axum dependency
- [ ] 3.2 Add stack-specific audit dimensions if applicable (clippy, cargo-audit, cargo-deny, miri)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Rust + Axum — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
