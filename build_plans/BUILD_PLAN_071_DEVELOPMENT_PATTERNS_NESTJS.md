# BUILD_PLAN_071: Auxiliary Development Patterns — NestJS

**Status:** CONVERGED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

NestJS is the most popular Node.js backend framework with 68k+ GitHub stars, bringing Angular-inspired architecture (modules, decorators, dependency injection) to server-side TypeScript. It is the default choice for enterprise Node.js APIs and microservices, with first-class support for GraphQL, WebSockets, and microservice patterns.

CruxDev manages projects across 18 project types. When adopting a project built with NestJS + TypeScript, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:nestjs`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (NestJS 10+, modules, providers, middleware, interceptors)
- [ ] 1.2 Research library ecosystem (TypeORM, Prisma, MikroORM, Bull/BullMQ, Passport, class-validator)
- [ ] 1.3 Research testing patterns (Jest, supertest, @nestjs/testing, Test containers, mock providers)
- [ ] 1.4 Research deployment patterns (Docker, Kubernetes, serverless with @nestjs/serverless, PM2)
- [ ] 1.5 Research common anti-patterns and pitfalls (circular dependencies, over-abstraction, misusing providers)

## Phase 2: Write DEVELOPMENT_PATTERNS_NESTJS.md

- [ ] 2.1 Project structure conventions (modules, controllers, services, DTOs, feature-based organization)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, module boundaries, shared modules, dynamic modules)
- [ ] 2.3 State management patterns (TypeORM/Prisma repositories, caching with CacheModule, event emitters, CQRS)
- [ ] 2.4 Testing strategy (Jest + @nestjs/testing for unit, supertest for E2E, TestingModule for integration)
- [ ] 2.5 Performance optimization (Fastify adapter, caching strategies, lazy loading modules, worker threads)
- [ ] 2.6 Deployment and CI/CD (Docker multi-stage, database migrations, health checks, Swagger generation, GitHub Actions)
- [ ] 2.7 Security considerations (Helmet, CORS, rate limiting, Guards for auth, class-validator for input, throttler)
- [ ] 2.8 Common pitfalls to avoid (circular module dependencies, request-scoped providers performance, missing DTO validation, memory leaks)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect nest-cli.json, @nestjs/core in package.json
- [ ] 3.2 Add stack-specific audit dimensions if applicable (ESLint NestJS rules, TSC strict mode)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for NestJS — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
