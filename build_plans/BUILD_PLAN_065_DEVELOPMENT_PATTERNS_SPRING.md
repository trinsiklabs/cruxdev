# BUILD_PLAN_065: Auxiliary Development Patterns — Spring Boot

**Status:** CONVERGED
**Priority:** P1
**Category:** Stack-specific development patterns

## Context

Spring Boot is the dominant enterprise Java framework, powering the majority of Fortune 500 backend systems. With Kotlin as a first-class language and Spring Boot 3.x embracing GraalVM native images and virtual threads (Project Loom), it remains the backbone of enterprise Java/Kotlin development.

CruxDev manages projects across 18 project types. When adopting a project built with Spring Boot + Kotlin/Java, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:spring`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Spring Boot 3.x, Spring Framework 6, Kotlin coroutines)
- [ ] 1.2 Research library ecosystem (Spring Data JPA, Spring Security, Spring Cloud, Flyway/Liquibase)
- [ ] 1.3 Research testing patterns (JUnit 5, MockK/Mockito, Testcontainers, Spring Test, RestAssured)
- [ ] 1.4 Research deployment patterns (Docker, Kubernetes, GraalVM native-image, Cloud Foundry)
- [ ] 1.5 Research common anti-patterns and pitfalls (annotation overload, circular dependencies, lazy initialization traps)

## Phase 2: Write DEVELOPMENT_PATTERNS_SPRING.md

- [ ] 2.1 Project structure conventions (package-by-feature, layered architecture, multi-module Gradle/Maven)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, dependency injection, service layer patterns)
- [ ] 2.3 State management patterns (JPA/Hibernate entities, Spring Cache, Spring Session, event-driven with ApplicationEvent)
- [ ] 2.4 Testing strategy (JUnit 5 + MockK for unit, @SpringBootTest for integration, Testcontainers for DB/messaging)
- [ ] 2.5 Performance optimization (virtual threads, connection pooling, JPA fetch strategies, native images, caching)
- [ ] 2.6 Deployment and CI/CD (Docker + Jib, Gradle/Maven builds, Flyway migrations, health actuators, Kubernetes manifests)
- [ ] 2.7 Security considerations (Spring Security, OAuth2/OIDC, CSRF, method-level security, secrets management)
- [ ] 2.8 Common pitfalls to avoid (N+1 JPA queries, transaction mismanagement, missing @Transactional, over-engineering with microservices)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect build.gradle/pom.xml with spring-boot
- [ ] 3.2 Add stack-specific audit dimensions if applicable (SpotBugs, Detekt for Kotlin, Checkstyle)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Spring Boot — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
