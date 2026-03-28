# BUILD_PLAN_066: Auxiliary Development Patterns — Blazor

**Status:** CONVERGED
**Priority:** P1
**Category:** Stack-specific development patterns

## Context

Blazor is Microsoft's answer to full-stack C# web development, enabling .NET developers to build interactive web UIs without JavaScript. With .NET 8's unified rendering model (Server, WebAssembly, and Auto modes), Blazor has matured into a production-grade framework with strong enterprise adoption in the .NET ecosystem.

CruxDev manages projects across 18 project types. When adopting a project built with ASP.NET Core + Blazor + MudBlazor, the convergence engine needs stack-specific patterns for: project structure, component architecture, testing, deployment, and common pitfalls.

## Model

Use `docs/DEVELOPMENT_PATTERNS_PETAL.md` (1000 lines, 16 sections) as the structural template. Match its depth: pinned versions, project structure, framework patterns, component library usage, testing, deployment, security, coverage, anti-patterns, and a "Report Improvements" section with GitHub issue link (label: `patterns:blazor`).

## Phase 1: Deep Research

- [ ] 1.1 Research official documentation and best practices (.NET 8/9 Blazor, render modes, streaming rendering)
- [ ] 1.2 Research component library ecosystem (MudBlazor, Radzen, FluentUI Blazor, Syncfusion)
- [ ] 1.3 Research testing patterns (bUnit, xUnit, Playwright, NSubstitute/Moq, FluentAssertions)
- [ ] 1.4 Research deployment patterns (Azure App Service, Docker, IIS, Kubernetes, Azure Container Apps)
- [ ] 1.5 Research common anti-patterns and pitfalls (render mode mismatches, excessive re-renders, WASM payload size)

## Phase 2: Write DEVELOPMENT_PATTERNS_BLAZOR.md

- [ ] 2.1 Project structure conventions (Razor class libraries, shared projects, feature folders)
- [ ] 2.2 Component/module architecture (DRY principles from DRY_UI_COMPONENT_PATTERNS.md, Razor components, cascading parameters)
- [ ] 2.3 State management patterns (cascading values, Fluxor, scoped services, browser storage, SignalR)
- [ ] 2.4 Testing strategy (bUnit for component tests, xUnit for unit, Playwright for E2E)
- [ ] 2.5 Performance optimization (virtualization, render mode selection, lazy loading assemblies, ahead-of-time compilation)
- [ ] 2.6 Deployment and CI/CD (dotnet publish, Docker multi-stage, Azure DevOps/GitHub Actions, EF Core migrations)
- [ ] 2.7 Security considerations (authentication/authorization, antiforgery, CORS, data protection API, Identity)
- [ ] 2.8 Common pitfalls to avoid (mixing render modes incorrectly, disposing services, JS interop memory leaks, oversized WASM bundles)

## Phase 3: Engine Integration

- [ ] 3.1 Add stack detection to project classifier (adoption/classify.rs) — detect .csproj with Blazor SDK, _Imports.razor
- [ ] 3.2 Add stack-specific audit dimensions if applicable (dotnet format, Roslyn analyzers, SonarQube)
- [ ] 3.3 Add stack-specific templates to templates/ directory

## Phase 4: Content Generation

- [ ] 4.1 Generate blog post: "Development Patterns for Blazor — What CruxDev Learned"
- [ ] 4.2 Generate X post announcing new stack support
- [ ] 4.3 Publish via BIP pipeline (generate_content + publish_drafts)

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
