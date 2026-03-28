# BUILD_PLAN_067: Auxiliary Development Patterns — React Native + Expo

**Status:** CONVERGED
**Priority:** P1
**Category:** Stack-specific development patterns

## Context

React Native with Expo is the leading cross-platform mobile framework, used by Discord, Shopify, and Microsoft. Expo SDK 51+ with the new architecture (Fabric, TurboModules) and Expo Router for file-based navigation has made it the default path for React developers building mobile apps.

CruxDev manages projects across 18 project types. When adopting a project built with React Native + Expo, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:expo`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (Expo SDK 51+, Expo Router, New Architecture)
- [ ] 1.2 Research component library ecosystem (React Native Paper, Tamagui, NativeWind, Gluestack, React Native Reanimated)
- [ ] 1.3 Research testing patterns (Jest, React Native Testing Library, Detox, Maestro)
- [ ] 1.4 Research deployment patterns (EAS Build, EAS Submit, OTA updates, App Store/Play Store CI)
- [ ] 1.5 Research common anti-patterns and pitfalls (bridge bottlenecks, excessive re-renders, platform-specific code sprawl)

## Phase 2: Write DEVELOPMENT_PATTERNS_EXPO.md

- [ ] 2.1 Project structure conventions (app/ directory with Expo Router, feature folders, platform-specific files)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, cross-platform components, platform abstractions)
- [ ] 2.3 State management patterns (Zustand, Jotai, React Query/TanStack Query, MMKV for persistence)
- [ ] 2.4 Testing strategy (Jest + RNTL for unit/integration, Detox or Maestro for E2E, snapshot testing)
- [ ] 2.5 Performance optimization (Reanimated for animations, FlashList, memo/useMemo, hermes engine, lazy screens)
- [ ] 2.6 Deployment and CI/CD (EAS Build profiles, OTA with expo-updates, GitHub Actions, app signing)
- [ ] 2.7 Security considerations (secure storage, certificate pinning, code obfuscation, API key protection, deep link validation)
- [ ] 2.8 Common pitfalls to avoid (inline styles in lists, missing key props, blocking JS thread, native module version mismatches)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect app.json/app.config.js with expo, expo-router
- [ ] 3.2 Add stack-specific audit dimensions if applicable (expo-doctor, react-native-performance)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for React Native + Expo — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
