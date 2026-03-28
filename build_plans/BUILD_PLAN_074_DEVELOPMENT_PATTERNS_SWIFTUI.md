# BUILD_PLAN_074: Auxiliary Development Patterns — SwiftUI

**Status:** CONVERGED
**Priority:** P3
**Category:** Stack-specific development patterns

## Context

SwiftUI is Apple's declarative UI framework for building native apps across iOS, macOS, watchOS, tvOS, and visionOS. With each WWDC bringing significant improvements (Observable macro, SwiftData, navigation overhaul), SwiftUI has matured into the primary way to build Apple platform apps, replacing UIKit for most new projects.

CruxDev manages projects across 18 project types. When adopting a project built with SwiftUI + Swift, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:swiftui`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (SwiftUI 5+, @Observable macro, SwiftData, NavigationStack)
- [ ] 1.2 Research library ecosystem (SwiftData, Alamofire, Kingfisher, The Composable Architecture, Swift Dependencies)
- [ ] 1.3 Research testing patterns (XCTest, Swift Testing framework, ViewInspector, snapshot testing, UI testing)
- [ ] 1.4 Research deployment patterns (Xcode Cloud, Fastlane, TestFlight, App Store Connect API)
- [ ] 1.5 Research common anti-patterns and pitfalls (view body complexity, @State misuse, navigation state management)

## Phase 2: Write DEVELOPMENT_PATTERNS_SWIFTUI.md

- [ ] 2.1 Project structure conventions (feature folders, App/Scene/View hierarchy, SPM package organization)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, view composition, ViewModifiers, PreferenceKey)
- [ ] 2.3 State management patterns (@Observable, @State, @Binding, @Environment, TCA for complex apps, SwiftData)
- [ ] 2.4 Testing strategy (Swift Testing for unit, ViewInspector for view tests, XCUITest for E2E, snapshot tests)
- [ ] 2.5 Performance optimization (lazy views, task modifiers, equatable views, Instruments profiling, draw call reduction)
- [ ] 2.6 Deployment and CI/CD (Xcode Cloud or Fastlane, TestFlight, code signing, SPM dependency caching)
- [ ] 2.7 Security considerations (Keychain for secrets, App Transport Security, certificate pinning, data protection, App Attest)
- [ ] 2.8 Common pitfalls to avoid (view body recomputation, @StateObject vs @ObservedObject lifecycle, missing task cancellation, deep navigation stacks)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect .xcodeproj/.xcworkspace, Package.swift, SwiftUI imports
- [ ] 3.2 Add stack-specific audit dimensions if applicable (SwiftLint, swift-format, Periphery for dead code)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for SwiftUI — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
