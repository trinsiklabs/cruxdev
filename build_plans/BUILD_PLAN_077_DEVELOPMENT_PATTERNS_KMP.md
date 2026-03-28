# BUILD_PLAN_077: Auxiliary Development Patterns — Kotlin Multiplatform

**Status:** NOT STARTED
**Priority:** P3
**Category:** Stack-specific development patterns

## Context

Kotlin Multiplatform (KMP) is JetBrains' solution for sharing business logic across Android, iOS, desktop, and web from a single Kotlin codebase. With Compose Multiplatform reaching stable for Android/desktop and beta for iOS, KMP enables true cross-platform UI alongside shared logic. Adopted by Netflix, Cash App, and VMware, KMP is increasingly seen as the pragmatic alternative to Flutter for teams with existing native mobile expertise.

CruxDev manages projects across 18 project types. When adopting a project built with Kotlin Multiplatform + Compose, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:kmp`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (KMP 2.0+, Compose Multiplatform, expect/actual mechanism)
- [ ] 1.2 Research library ecosystem (Ktor, SQLDelight, Koin/Kodein, Decompose, Voyager, kotlinx.serialization)
- [ ] 1.3 Research testing patterns (kotlin.test, Turbine for flows, Kotest, MockK, Paparazzi for screenshots)
- [ ] 1.4 Research deployment patterns (Gradle KMP builds, CocoaPods/SPM for iOS, Google Play/App Store, desktop packaging)
- [ ] 1.5 Research common anti-patterns and pitfalls (expect/actual overuse, coroutine scope leaks, iOS interop friction)

## Phase 2: Write DEVELOPMENT_PATTERNS_KMP.md

- [ ] 2.1 Project structure conventions (shared/, composeApp/, iosApp/, commonMain/platform-specific source sets)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, Compose components, expect/actual for platform APIs)
- [ ] 2.3 State management patterns (StateFlow, Compose state, Decompose for navigation state, SQLDelight for persistence)
- [ ] 2.4 Testing strategy (kotlin.test + commonTest for shared, Turbine for Flow testing, Paparazzi for UI, platform-specific tests)
- [ ] 2.5 Performance optimization (compose stability, lazy layouts, coroutine dispatchers, native memory management on iOS)
- [ ] 2.6 Deployment and CI/CD (Gradle builds, CocoaPods/SPM integration, GitHub Actions, Fastlane for mobile, desktop packaging)
- [ ] 2.7 Security considerations (secure storage per platform, certificate pinning with Ktor, API key management, code obfuscation)
- [ ] 2.8 Common pitfalls to avoid (iOS memory model gotchas, Gradle configuration complexity, expect/actual explosion, Swift interop limitations)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect build.gradle.kts with kotlin-multiplatform plugin
- [ ] 3.2 Add stack-specific audit dimensions if applicable (Detekt, ktlint, Gradle dependency updates)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Kotlin Multiplatform — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
