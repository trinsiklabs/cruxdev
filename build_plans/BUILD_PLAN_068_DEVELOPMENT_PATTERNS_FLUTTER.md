# BUILD_PLAN_068: Auxiliary Development Patterns — Flutter

**Status:** NOT STARTED
**Priority:** P2
**Category:** Stack-specific development patterns

## Context

Flutter is Google's cross-platform UI toolkit targeting mobile, web, and desktop from a single Dart codebase. With 165k+ GitHub stars and adoption by Google, BMW, and Alibaba, it is the primary alternative to React Native for cross-platform development, offering pixel-perfect rendering via its own rendering engine (Impeller).

CruxDev manages projects across 18 project types. When adopting a project built with Flutter + Dart, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:flutter`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Flutter 3.x, Dart 3, Impeller renderer, Material 3)
- [ ] 1.2 Research package ecosystem (Riverpod, BLoC, go_router, freezed, drift, dio)
- [ ] 1.3 Research testing patterns (flutter_test, widget testing, integration_test, mockito/mocktail, golden tests)
- [ ] 1.4 Research deployment patterns (Fastlane, Codemagic, GitHub Actions, Firebase App Distribution, Shorebird)
- [ ] 1.5 Research common anti-patterns and pitfalls (deep widget nesting, BuildContext misuse, state management confusion)

## Phase 2: Write DEVELOPMENT_PATTERNS_FLUTTER.md

- [ ] 2.1 Project structure conventions (feature-first folders, lib/ organization, asset management)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, widget composition, custom painters)
- [ ] 2.3 State management patterns (Riverpod, BLoC/Cubit, Provider, ValueNotifier, streams)
- [ ] 2.4 Testing strategy (widget tests for components, unit tests for logic, integration_test for E2E, golden tests for UI regression)
- [ ] 2.5 Performance optimization (const constructors, RepaintBoundary, lazy loading, Impeller, DevTools profiling)
- [ ] 2.6 Deployment and CI/CD (Fastlane + GitHub Actions, flavor configs, code signing, Shorebird OTA)
- [ ] 2.7 Security considerations (secure storage, certificate pinning, obfuscation, API key hiding, platform channels security)
- [ ] 2.8 Common pitfalls to avoid (setState in large widgets, missing dispose(), platform channel thread safety, unbounded lists)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect pubspec.yaml with flutter SDK
- [ ] 3.2 Add stack-specific audit dimensions if applicable (dart analyze, flutter analyze, DCM metrics)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Flutter — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
